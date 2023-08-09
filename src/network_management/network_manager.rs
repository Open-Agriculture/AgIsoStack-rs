// Copyright 2023 Raven Industries inc.
#![allow(dead_code)]
#![allow(unused_variables)]
use std::time::{Duration, Instant};

use super::control_function::{AddressClaimingState, ControlFunction};
use crate::driver::{CanId, Type};
use crate::network_management::can_message::CANMessage;
use crate::network_management::common_parameter_group_numbers::CommonParameterGroupNumbers;
use crate::network_management::name::DEFAULT_NAME;
use crate::network_management::name::NAME;
use std::collections::VecDeque;

pub struct NetworkManager {
    control_function_table: [Option<ControlFunction>; 253],
    channel: u8,
    control_function_handle_counter: usize,
    inactive_control_functions: Vec<ControlFunction>,
    address_claim_state_machines: Vec<ControlFunction>,
    high_priority_can_message_tx_queue: VecDeque<CANMessage>,
    normal_priority_can_message_tx_queue: VecDeque<CANMessage>,
}

impl NetworkManager {
    pub fn new(channel: u8) -> Self {
        Self {
            control_function_table: std::array::from_fn(|_| None),
            channel,
            control_function_handle_counter: 0,
            inactive_control_functions: Vec::new(),
            address_claim_state_machines: Vec::new(),
            high_priority_can_message_tx_queue: VecDeque::new(),
            normal_priority_can_message_tx_queue: VecDeque::new(),
        }
    }

    pub fn get_control_function_by_address(&self, address: usize) -> &Option<ControlFunction> {
        &self.control_function_table[address]
    }

    fn send_raw_can_message(
        &self,
        source_address: u8,
        destination_address: u8,
        parameter_group_number: u32,
        priority: u8,
        data: &[u8],
    ) -> bool {
        false // Todo, link up with the driver layer
    }

    fn construct_request_for_address_claim() -> CANMessage {
        const ADDRESS_CLAIM_REQUEST_LENGTH: usize = 3;
        let pgn_to_request: u32 = CommonParameterGroupNumbers::AddressClaim as u32;
        let request = vec![
            (pgn_to_request & 0xFF) as u8,
            ((pgn_to_request >> 8) & 0xFF) as u8,
            ((pgn_to_request >> 16) & 0xFF) as u8,
        ];

        let request_id = CanId::new(0, Type::Extended); // TODO Fix the ID once encoding method is available in CanId
        CANMessage::new(request, request_id)
    }

    fn construct_address_claim(source_address: u8, name: u64) -> CANMessage {
        let pgn: u32 = CommonParameterGroupNumbers::AddressClaim as u32;
        let address_claim = vec![
            (name & 0xFF) as u8,
            ((name >> 8) & 0xFF) as u8,
            ((name >> 16) & 0xFF) as u8,
            ((name >> 24) & 0xFF) as u8,
            ((name >> 32) & 0xFF) as u8,
            ((name >> 40) & 0xFF) as u8,
            ((name >> 48) & 0xFF) as u8,
            ((name >> 56) & 0xFF) as u8,
        ];

        let request_id = CanId::new(0, Type::Extended); // TODO Fix the ID once encoding method is available in CanId
        CANMessage::new(address_claim, request_id)
    }

    fn update_address_claiming(&mut self) {
        for address_claimer in &mut self.address_claim_state_machines {
            match address_claimer {
                ControlFunction::Internal { address_claim_data } => {
                    if address_claim_data.get_enabled() {
                        match address_claim_data.get_state() {
                            AddressClaimingState::None => {
                                address_claim_data.set_state(AddressClaimingState::WaitForClaim);
                            }
                            AddressClaimingState::WaitForClaim => {
                                if address_claim_data.get_timestamp().is_none() {
                                    address_claim_data.set_timestamp(Some(Instant::now()))
                                }
                                if Instant::now()
                                    .duration_since(address_claim_data.get_timestamp().unwrap())
                                    > Duration::from_millis(
                                        address_claim_data.get_random_delay() as u64
                                    )
                                {
                                    address_claim_data
                                        .set_state(AddressClaimingState::SendRequestForClaim);
                                }
                            }
                            AddressClaimingState::SendRequestForClaim => {
                                self.high_priority_can_message_tx_queue.push_back(
                                    NetworkManager::construct_request_for_address_claim(),
                                );
                                address_claim_data.set_state(
                                    AddressClaimingState::WaitForRequestContentionPeriod,
                                );
                            }
                            AddressClaimingState::WaitForRequestContentionPeriod => {
                                let contention_time_ms: u64 = 250;

                                if Instant::now()
                                    .duration_since(address_claim_data.get_timestamp().unwrap())
                                    > Duration::from_millis(
                                        address_claim_data.get_random_delay() as u64
                                            + contention_time_ms,
                                    )
                                {
                                    let device_at_our_address = self
                                        .control_function_table
                                        .get(address_claim_data.get_preferred_address() as usize);

                                    let device_at_our_address = &device_at_our_address
                                        .unwrap()
                                        .as_ref()
                                        .unwrap_or(&ControlFunction::External {
                                            name: NAME {
                                                raw_name: DEFAULT_NAME,
                                            },
                                        });
                                    let preferred_address_name: u64 = match device_at_our_address {
                                        ControlFunction::External { name } => name.raw_name,
                                        ControlFunction::Internal { address_claim_data } => {
                                            address_claim_data.get_name().raw_name
                                        }
                                    };

                                    if (!address_claim_data
                                        .get_name()
                                        .get_self_configurable_address()
                                        && preferred_address_name
                                            > address_claim_data.get_name().raw_name)
                                        || DEFAULT_NAME == preferred_address_name
                                    {
                                        // Either our preferred address is free, this is the best case, or:
                                        // Our address is not free, but we cannot be at an arbitrary address, and the address can be stolen by us
                                        address_claim_data.set_state(
                                            AddressClaimingState::SendPreferredAddressClaim,
                                        );
                                    } else if !address_claim_data
                                        .get_name()
                                        .get_self_configurable_address()
                                    {
                                        // We cannot claim because we cannot tolerate an arbitrary address, and the CF at that spot wins due to its lower ISONAME
                                        address_claim_data
                                            .set_state(AddressClaimingState::UnableToClaim);
                                    } else {
                                        // We will move to another address if whoever is in our spot has a lower NAME
                                        if preferred_address_name
                                            < address_claim_data.get_name().raw_name
                                        {
                                            // We must scan the address space and move to a free address
                                            address_claim_data.set_state(
                                                AddressClaimingState::SendArbitraryAddressClaim,
                                            );
                                        } else {
                                            // Our address claim wins because it's lower than the device that's in our preferred spot
                                            address_claim_data.set_state(
                                                AddressClaimingState::SendPreferredAddressClaim,
                                            );
                                        }
                                    }
                                }
                            }
                            AddressClaimingState::SendPreferredAddressClaim
                            | AddressClaimingState::SendReclaimAddressOnRequest
                            | AddressClaimingState::ContendForPreferredAddress => {
                                self.high_priority_can_message_tx_queue.push_back(
                                    NetworkManager::construct_address_claim(
                                        address_claim_data.get_preferred_address(),
                                        address_claim_data.get_name().raw_name,
                                    ),
                                );
                                address_claim_data
                                    .set_state(AddressClaimingState::AddressClaimingComplete);
                            }
                            AddressClaimingState::SendArbitraryAddressClaim => {
                                for address in 129..247 {
                                    let &device_at_our_address = &self.control_function_table
                                        [address]
                                        .as_ref()
                                        .unwrap_or(&ControlFunction::External {
                                            name: NAME {
                                                raw_name: DEFAULT_NAME,
                                            },
                                        });

                                    let preferred_address_name: u64 = match device_at_our_address {
                                        ControlFunction::External { name } => name.raw_name,
                                        ControlFunction::Internal { address_claim_data } => {
                                            address_claim_data.get_name().raw_name
                                        }
                                    };

                                    if DEFAULT_NAME == preferred_address_name {
                                        // Found an address we can use
                                        self.high_priority_can_message_tx_queue.push_back(
                                            NetworkManager::construct_address_claim(
                                                address as u8,
                                                address_claim_data.get_name().raw_name,
                                            ),
                                        );
                                        address_claim_data.set_state(
                                            AddressClaimingState::AddressClaimingComplete,
                                        );
                                        break;
                                    }
                                }
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
    }

    pub fn update(mut self) {
        self.update_address_claiming();
    }
}
