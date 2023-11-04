// Copyright 2023 Raven Industries inc.
use std::time::Instant;

use super::control_function::{AddressClaimingState, ControlFunction};
use crate::driver::{Address, CanId, Pgn, Priority};
use crate::network_management::can_message::CANMessage;
use crate::network_management::common_parameter_group_numbers::CommonParameterGroupNumbers;
use crate::network_management::name::{DEFAULT_NAME, NAME};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, Clone, Copy)]
pub(super) enum MessageQueuePriority {
    /// High priority messages are always sent to the driver before normal ones
    High,
    /// Normal messages are sent to the driver when no high priority messages are in the queue (todo)
    Normal,
}

#[derive(Debug, Clone, Copy)]
pub enum CANTransmitState {
    /// Used to describe that a CAN message was accepted by the CAN stack to be sent
    Success,
    /// Used to describe that a CAN message was not accepted by the stack and will not be sent
    Fail,
}

pub struct NetworkManager {
    control_function_table: [Option<Rc<RefCell<ControlFunction>>>; 253],
    inactive_control_functions: Vec<Rc<RefCell<ControlFunction>>>,
    address_claim_state_machines: Vec<Rc<RefCell<ControlFunction>>>,
    high_priority_can_message_tx_queue: VecDeque<CANMessage>,
    normal_priority_can_message_tx_queue: VecDeque<CANMessage>,
    receive_message_queue: VecDeque<CANMessage>,
}

impl NetworkManager {
    pub fn new() -> Self {
        Self {
            control_function_table: std::array::from_fn(|_| None),
            inactive_control_functions: Vec::new(),
            address_claim_state_machines: Vec::new(),
            high_priority_can_message_tx_queue: VecDeque::new(),
            normal_priority_can_message_tx_queue: VecDeque::new(),
            receive_message_queue: VecDeque::new(),
        }
    }

    pub fn get_control_function_by_address(
        &self,
        address: Address,
    ) -> &Option<Rc<RefCell<ControlFunction>>> {
        &self.control_function_table[address.0 as usize]
    }

    pub fn get_control_function_address_by_name(&self, name: NAME) -> Address {
        for (i, cf) in self.control_function_table.iter().enumerate() {
            if let Some(extant_cf) = cf {
                if extant_cf.borrow().get_name().raw_name == name.raw_name {
                    return Address(i as u8);
                }
            }
        }
        Address::NULL
    }

    pub(super) fn on_new_internal_control_function(
        &mut self,
        new_cf: Rc<RefCell<ControlFunction>>,
    ) {
        self.inactive_control_functions.push(new_cf.clone());
        self.address_claim_state_machines.push(new_cf);
    }

    pub(super) fn get_next_free_arbitrary_address(&self) -> Address {
        for address in 129..247 {
            let is_device_at_address = self.get_control_function_by_address(Address(address));
            let is_valid_device: bool = is_device_at_address.is_some();

            if !is_valid_device {
                return Address(address);
            } else {
                let device_at_our_address = is_device_at_address.as_ref().unwrap().borrow();

                let preferred_address_name: u64 = match &*device_at_our_address {
                    ControlFunction::External { name } => name.raw_name,
                    ControlFunction::Internal { address_claim_data } => {
                        address_claim_data.get_name().raw_name
                    }
                };

                if DEFAULT_NAME == preferred_address_name {
                    return Address(address);
                }
            }
        }
        Address::NULL
    }

    pub(super) fn construct_address_claim(source_address: Address, name: NAME) -> CANMessage {
        let address_claim = name.raw_name.to_le_bytes().to_vec();

        let request_id = CanId::try_encode(
            Pgn::from_raw(CommonParameterGroupNumbers::AddressClaim as u32),
            source_address,
            Address::BROADCAST,
            Priority::Default,
        );
        CANMessage::new(address_claim, request_id.unwrap())
    }

    pub(super) fn construct_request_for_address_claim() -> CANMessage {
        let pgn_to_request: u32 = CommonParameterGroupNumbers::AddressClaim as u32;
        let request = pgn_to_request.to_le_bytes().to_vec();
        let request_id = CanId::try_encode(
            Pgn::from_raw(CommonParameterGroupNumbers::ParameterGroupNumberRequest as u32),
            Address::NULL,
            Address::BROADCAST,
            Priority::Three,
        );
        CANMessage::new(request, request_id.unwrap())
    }

    pub(super) fn enqueue_can_message(
        &mut self,
        message: CANMessage,
        queue_priority: MessageQueuePriority,
    ) {
        // Todo, max queue depth?
        match queue_priority {
            MessageQueuePriority::High => {
                self.high_priority_can_message_tx_queue.push_back(message)
            }
            MessageQueuePriority::Normal => {
                self.normal_priority_can_message_tx_queue.push_back(message)
            }
        }
    }

    pub fn send_can_message(
        &mut self,
        parameter_group_number: Pgn,
        data: &[u8],
        source: Rc<RefCell<ControlFunction>>,
        destination: Rc<RefCell<ControlFunction>>,
        priority: Priority,
    ) -> CANTransmitState {
        if !data.is_empty() {
            // Todo, handle lengths greater than 8

            if data.len() <= 8 {
                let source = source.borrow();
                let destination = destination.borrow();
                let message_id = CanId::try_encode(
                    parameter_group_number,
                    self.get_control_function_address_by_name(source.get_name()),
                    self.get_control_function_address_by_name(destination.get_name()),
                    priority,
                )
                .unwrap_or_default();

                if message_id.raw() != CanId::default().raw() {
                    self.enqueue_can_message(
                        CANMessage::new(data.to_vec(), message_id),
                        MessageQueuePriority::Normal,
                    );
                    return CANTransmitState::Success;
                }
            }
        }
        CANTransmitState::Fail
    }

    fn update_address_claiming(&mut self) {
        let mut state_machines = std::mem::take(&mut self.address_claim_state_machines);
        for address_claimer in &mut state_machines {
            let mut address_claimer = address_claimer.borrow_mut();
            match *address_claimer {
                ControlFunction::Internal {
                    ref mut address_claim_data,
                } => {
                    if address_claim_data.get_enabled() {
                        match address_claim_data.get_state() {
                            AddressClaimingState::None => {
                                address_claim_data.set_state(
                                    AddressClaimingState::update_state_none(address_claim_data),
                                );
                            }
                            AddressClaimingState::WaitForClaim => {
                                if address_claim_data.get_timestamp().is_none() {
                                    address_claim_data.set_timestamp(Some(Instant::now()))
                                }

                                address_claim_data.set_state(
                                    AddressClaimingState::update_state_wait_for_claim(
                                        address_claim_data,
                                    ),
                                );
                            }
                            AddressClaimingState::SendRequestForClaim => {
                                address_claim_data.set_state(
                                    AddressClaimingState::update_state_send_request_for_claim(self),
                                );
                            }
                            AddressClaimingState::WaitForRequestContentionPeriod => {
                                address_claim_data.set_state(
                                    AddressClaimingState::update_state_wait_for_request_contention(
                                        address_claim_data,
                                        self,
                                    ),
                                );
                            }
                            AddressClaimingState::SendPreferredAddressClaim
                            | AddressClaimingState::SendReclaimAddressOnRequest
                            | AddressClaimingState::ContendForPreferredAddress => {
                                address_claim_data.set_state(
                                    AddressClaimingState::update_state_send_preferred_address_claim(
                                        address_claim_data,
                                        self,
                                    ),
                                );
                            }
                            AddressClaimingState::SendArbitraryAddressClaim => {
                                address_claim_data.set_state(
                                    AddressClaimingState::update_state_send_arbitrary_address_claim(
                                        address_claim_data,
                                        self,
                                    ),
                                );
                                address_claim_data
                                    .set_preferred_address(self.get_next_free_arbitrary_address());
                            }
                            AddressClaimingState::AddressClaimingComplete
                            | AddressClaimingState::UnableToClaim => {
                                // Nothing to do
                            }
                        }
                    }
                }
                _ => panic!("Only Internal CFs can perform address claiming"),
            }
        }
        std::mem::swap(&mut state_machines, &mut self.address_claim_state_machines);
    }

    fn update_receive_messages(&mut self) {
        while !self.receive_message_queue.is_empty() {
            // Todo receive messages, need to generalize message handling
            let current_message = self.receive_message_queue.front().unwrap();

            // Process address claims and requests to claim
            if DEFAULT_NAME == current_message.get_destination_name().raw_name {
                // Broadcast Message
                if current_message.get_identifier().pgn()
                    == Pgn::from_raw(CommonParameterGroupNumbers::AddressClaim as u32)
                {
                    // Todo
                } else if current_message.get_identifier().pgn()
                    == Pgn::from_raw(
                        CommonParameterGroupNumbers::ParameterGroupNumberRequest as u32,
                    )
                    && current_message.get_data().len() >= 3
                {
                    let message_data = current_message.get_data();
                    let requested_pgn: u32 = (message_data[0] as u32)
                        | ((message_data[1] as u32) << 8)
                        | ((message_data[2] as u32) << 16);

                    if requested_pgn
                        == CommonParameterGroupNumbers::ParameterGroupNumberRequest as u32
                    {
                        for internal_cf in &mut self.address_claim_state_machines {
                            let mut address_claimer = internal_cf.borrow_mut();
                            match *address_claimer {
                                ControlFunction::Internal {
                                    ref mut address_claim_data,
                                } => {
                                    if address_claim_data.get_state()
                                        == AddressClaimingState::AddressClaimingComplete
                                    {
                                        address_claim_data.set_state(
                                            AddressClaimingState::SendReclaimAddressOnRequest,
                                        );
                                    }
                                }
                                ControlFunction::External { name: _ } => {}
                            }
                        }
                    }
                } else {
                    // Destination specific
                }

                self.receive_message_queue.pop_front();
            }
        }
    }

    fn update_transmit_messages(&mut self) {
        let should_continue_sending: bool = true; // Todo, check driver return values.

        while !self.high_priority_can_message_tx_queue.is_empty() {
            // todo hand off to driver
            self.high_priority_can_message_tx_queue.pop_front();
        }

        while should_continue_sending && !self.normal_priority_can_message_tx_queue.is_empty() {
            // todo hand off to driver
            self.normal_priority_can_message_tx_queue.pop_front();
        }
    }

    pub fn update(mut self) {
        self.update_receive_messages();
        self.update_address_claiming();
        self.update_transmit_messages();
    }
}

impl Default for NetworkManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creating_network_manager() {
        let network = NetworkManager::new();
        network.update();
    }

    #[test]
    fn test_creating_internal_control_function() {
        let mut network = NetworkManager::new();
        let test_name = NAME::build(4, 0, 8, 5, 6, 3, 2, 7, 1, true);

        let new_cf = ControlFunction::new_internal_control_function(
            test_name,
            Address(0x81),
            true,
            &mut network,
        );

        assert_eq!(new_cf.borrow().get_name().raw_name, test_name.raw_name);
    }
}
