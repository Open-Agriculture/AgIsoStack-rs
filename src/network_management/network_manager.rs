// Copyright 2023 Raven Industries inc.
use std::time::Instant;

use super::control_function::{AddressClaimingState, ControlFunction};
use crate::j1939::{Address, ExtendedId, Frame, Pgn, Priority};
use crate::network_management::common_parameter_group_numbers::CommonParameterGroupNumbers;
use crate::network_management::name::NAME;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, Clone, Copy)]
pub(super) enum MessageQueuePriority {
    /// High priority messages are always sent to the j1939 before normal ones
    High,
    /// Normal messages are sent to the j1939 when no high priority messages are in the queue (todo)
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
    high_priority_can_message_tx_queue: VecDeque<Frame>,
    normal_priority_can_message_tx_queue: VecDeque<Frame>,
    receive_message_queue: VecDeque<Frame>,
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
        &self.control_function_table[address.raw() as usize]
    }

    pub fn get_control_function_address_by_name(&self, name: NAME) -> Address {
        for (i, cf) in self.control_function_table.iter().enumerate() {
            if let Some(extant_cf) = cf {
                if extant_cf.borrow().get_name() == name {
                    return Address::new(i as u8);
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
            let is_device_at_address = self.get_control_function_by_address(Address::new(address));
            let is_valid_device: bool = is_device_at_address.is_some();

            if !is_valid_device {
                return Address::new(address);
            } else {
                let device_at_our_address = is_device_at_address.as_ref().unwrap().borrow();

                let preferred_address_name: u64 = match &*device_at_our_address {
                    ControlFunction::External { name } => (*name).into(),
                    ControlFunction::Internal { address_claim_data } => {
                        address_claim_data.get_name().into()
                    }
                };

                if <NAME as Into<u64>>::into(NAME::default()) == preferred_address_name {
                    return Address::new(address);
                }
            }
        }
        Address::NULL
    }

    pub(super) fn construct_address_claim(source_address: Address, name: NAME) -> Frame {
        let address_claim = <NAME as Into<u64>>::into(name).to_le_bytes().to_vec();

        let request_id = ExtendedId::new(
            Priority::DEFAULT,
            CommonParameterGroupNumbers::AddressClaim.get_pgn(),
            source_address,
        );
        Frame::new(request_id, address_claim).unwrap()
    }

    pub(super) fn construct_request_for_address_claim() -> Frame {
        let pgn_to_request: u32 = CommonParameterGroupNumbers::AddressClaim as u32;
        let request = pgn_to_request.to_le_bytes().to_vec();
        let mut pgn = CommonParameterGroupNumbers::ParameterGroupNumberRequest.get_pgn();
        pgn.set_destination_address(Address::BROADCAST);
        let request_id = ExtendedId::new(Priority::Three, pgn, Address::NULL);
        Frame::new(request_id, request).unwrap()
    }

    pub(super) fn enqueue_can_message(
        &mut self,
        message: Frame,
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
                let mut pgn = parameter_group_number;
                pgn.set_destination_address(
                    self.get_control_function_address_by_name(destination.get_name()),
                );
                let message_id = ExtendedId::new(
                    priority,
                    pgn,
                    self.get_control_function_address_by_name(source.get_name()),
                );

                self.enqueue_can_message(
                    Frame::new(message_id, data.to_vec()).unwrap(),
                    MessageQueuePriority::Normal,
                );
                return CANTransmitState::Success;
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
            if NAME::default() == NAME::default()
            /*TODO!: Replaced following code line by upper NAME::default(). There needs to be another abstraction of ISO 11783 specific frames which includes the NAME*/
            /*current_message.get_destination_name()*/
            {
                // Broadcast Message
                if current_message.id().pgn().pdu_format()
                    == CommonParameterGroupNumbers::AddressClaim
                        .get_pgn()
                        .pdu_format()
                {
                    // Todo
                } else if current_message.id().pgn().pdu_format()
                    == CommonParameterGroupNumbers::ParameterGroupNumberRequest
                        .get_pgn()
                        .pdu_format()
                    && current_message.clone().data().len() >= 3
                {
                    let message_data = current_message.clone().data();
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
        let should_continue_sending: bool = true; // Todo, check j1939 return values.

        while !self.high_priority_can_message_tx_queue.is_empty() {
            // todo hand off to j1939
            self.high_priority_can_message_tx_queue.pop_front();
        }

        while should_continue_sending && !self.normal_priority_can_message_tx_queue.is_empty() {
            // todo hand off to j1939
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
        let test_name = NAME::builder()
            .device_class(0)
            .device_class_instance(0)
            .ecu_instance(0)
            .function_code(130)
            .function_instance(0)
            .identity_number(123_u32)
            .industry_group(2)
            .function_instance(0)
            .build();

        let new_cf = ControlFunction::new_internal_control_function(
            test_name,
            Address::new(0x81),
            true,
            &mut network,
        );

        assert_eq!(
            <NAME as Into<u64>>::into(new_cf.borrow().get_name()),
            test_name.into()
        );
    }
}
