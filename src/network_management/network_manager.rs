// Copyright 2023 Raven Industries inc.
use std::time::Instant;

use super::control_function::{AddressClaimingState, ControlFunction};
use crate::driver::{Address, CanId, Pgn, Priority};
use crate::network_management::can_message::CANMessage;
use crate::network_management::common_parameter_group_numbers::CommonParameterGroupNumbers;
use crate::network_management::name::NAME;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
pub(super) enum MessageQueuePriority {
    /// High priority messages are always sent to the driver before normal ones
    High,
    /// Normal messages are sent to the driver when no high priority messages are in the queue (todo)
    Normal,
}

pub struct NetworkManager {
    control_function_table: [Option<ControlFunction>; 253],
    // Todo inactive_control_functions: Vec<ControlFunction>,
    address_claim_state_machines: Vec<ControlFunction>,
    high_priority_can_message_tx_queue: VecDeque<CANMessage>,
    normal_priority_can_message_tx_queue: VecDeque<CANMessage>,
}

impl NetworkManager {
    pub fn new() -> Self {
        Self {
            control_function_table: std::array::from_fn(|_| None),
            // Todo inactive_control_functions: Vec::new(),
            address_claim_state_machines: Vec::new(),
            high_priority_can_message_tx_queue: VecDeque::new(),
            normal_priority_can_message_tx_queue: VecDeque::new(),
        }
    }

    pub fn get_control_function_by_address(&self, address: Address) -> &Option<ControlFunction> {
        &self.control_function_table[address.0 as usize]
    }

    pub(super) fn get_next_free_arbitrary_address(&self) -> Address {
        let default_external_cf = ControlFunction::External {
            name: NAME::default(),
        };
        for address in 129..247 {
            let is_device_at_address = self.get_control_function_by_address(Address(address));
            let device_at_our_address = match is_device_at_address {
                Some(_) => is_device_at_address.as_ref().unwrap(),
                None => &default_external_cf,
            };

            let preferred_address_name: u64 = match device_at_our_address {
                ControlFunction::External { name } => (*name).into(),
                ControlFunction::Internal { address_claim_data } => {
                    address_claim_data.get_name().into()
                }
            };

            if <NAME as Into<u64>>::into(NAME::default()) == preferred_address_name {
                return Address(address);
            }
        }
        Address::NULL
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

    pub(super) fn construct_address_claim(source_address: Address, name: NAME) -> CANMessage {
        let address_claim = <NAME as Into<u64>>::into(name).to_le_bytes().to_vec();

        let request_id = CanId::try_encode(
            Pgn::from_raw(CommonParameterGroupNumbers::AddressClaim as u32),
            source_address,
            Address::BROADCAST,
            Priority::Default,
        );
        CANMessage::new(address_claim, request_id.unwrap())
    }

    pub(super) fn enqueue_can_message(
        &mut self,
        message: CANMessage,
        queue_priority: MessageQueuePriority,
    ) {
        match queue_priority {
            MessageQueuePriority::High => {
                self.high_priority_can_message_tx_queue.push_back(message)
            }
            MessageQueuePriority::Normal => {
                self.normal_priority_can_message_tx_queue.push_back(message)
            }
        }
    }

    fn update_address_claiming(&mut self) {
        let mut state_machines = std::mem::take(&mut self.address_claim_state_machines);
        for address_claimer in &mut state_machines {
            match address_claimer {
                ControlFunction::Internal { address_claim_data } => {
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

    pub fn update(mut self) {
        self.update_address_claiming();
    }
}

impl Default for NetworkManager {
    fn default() -> Self {
        Self::new()
    }
}
