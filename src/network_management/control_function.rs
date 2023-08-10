// Copyright 2023 Raven Industries inc.
use crate::driver::Address;
use crate::network_management::name::DEFAULT_NAME;
use crate::network_management::name::NAME;
use rand::Rng;
use std::time::{Duration, Instant};

use super::network_manager::{MessageQueuePriority, NetworkManager};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum AddressClaimingState {
    /// Address claiming is uninitialized
    None,
    /// State machine is waiting for the random delay time                       
    WaitForClaim,
    /// State machine is sending the request for address claim                
    SendRequestForClaim,
    /// State machine is waiting for the address claim contention period         
    WaitForRequestContentionPeriod,
    /// State machine is claiming the preferred address
    SendPreferredAddressClaim,
    /// State machine is contending the preferred address  
    ContendForPreferredAddress,
    /// State machine is claiming an address   
    SendArbitraryAddressClaim,
    /// An ECU requested address claim, inform the bus of our current address    
    SendReclaimAddressOnRequest,
    /// State machine could not claim an address
    UnableToClaim,
    /// Address claiming is complete and we have an address     
    AddressClaimingComplete,
}

pub struct AddressClaimingData {
    state: AddressClaimingState,
    name: NAME,
    timestamp: Option<Instant>,
    preferred_address: Address,
    random_delay: u8,
    enabled: bool,
}

pub enum ControlFunction {
    Internal {
        address_claim_data: AddressClaimingData,
    },
    External {
        name: NAME,
    },
}

impl AddressClaimingState {
    pub(super) fn new() -> Self {
        Self::None
    }

    pub(super) fn update_state_none(_claim_to_process: &AddressClaimingData) -> Self {
        AddressClaimingState::WaitForClaim
    }

    pub(super) fn update_state_wait_for_claim(claim_to_process: &AddressClaimingData) -> Self {
        if Instant::now().duration_since(claim_to_process.get_timestamp().unwrap())
            > Duration::from_millis(claim_to_process.get_random_delay() as u64)
        {
            AddressClaimingState::SendRequestForClaim
        } else {
            AddressClaimingState::WaitForClaim
        }
    }

    pub(super) fn update_state_send_request_for_claim(network: &mut NetworkManager) -> Self {
        network.enqueue_can_message(
            NetworkManager::construct_request_for_address_claim(),
            MessageQueuePriority::High);
        AddressClaimingState::WaitForRequestContentionPeriod
    }

    pub(super) fn update_state_wait_for_request_contention(
        claim_to_process: &AddressClaimingData,
        network: &mut NetworkManager,
    ) -> Self {
        let contention_time_ms: u64 = 250;

        if Instant::now().duration_since(claim_to_process.get_timestamp().unwrap())
            > Duration::from_millis(claim_to_process.get_random_delay() as u64 + contention_time_ms)
        {
            let is_device_at_our_address =
                network.get_control_function_by_address(claim_to_process.get_preferred_address());
            let device_at_our_address = match is_device_at_our_address {
                Some(_) => is_device_at_our_address.as_ref().unwrap(),
                None => &ControlFunction::External {
                    name: NAME {
                        raw_name: (DEFAULT_NAME),
                    },
                },
            };

            let preferred_address_name: u64 = match device_at_our_address {
                ControlFunction::External { name } => name.raw_name,
                ControlFunction::Internal {
                    address_claim_data: _,
                } => claim_to_process.get_name().raw_name,
            };

            if (!claim_to_process.get_name().get_self_configurable_address()
                && preferred_address_name > claim_to_process.get_name().raw_name)
                || DEFAULT_NAME == preferred_address_name
            {
                // Either our preferred address is free, this is the best case, or:
                // Our address is not free, but we cannot be at an arbitrary address, and the address can be stolen by us
                AddressClaimingState::SendPreferredAddressClaim
            } else if !claim_to_process.get_name().get_self_configurable_address() {
                // We cannot claim because we cannot tolerate an arbitrary address, and the CF at that spot wins due to its lower ISONAME
                AddressClaimingState::UnableToClaim
            } else {
                // We will move to another address if whoever is in our spot has a lower NAME
                if preferred_address_name < claim_to_process.get_name().raw_name {
                    // We must scan the address space and move to a free address
                    AddressClaimingState::SendArbitraryAddressClaim
                } else {
                    // Our address claim wins because it's lower than the device that's in our preferred spot
                    AddressClaimingState::SendPreferredAddressClaim
                }
            }
        } else {
            AddressClaimingState::WaitForRequestContentionPeriod
        }
    }

    pub(super) fn update_state_send_preferred_address_claim(
        claim_to_process: &AddressClaimingData,
        network: &mut NetworkManager,
    ) -> Self {
        network.enqueue_can_message(
            NetworkManager::construct_address_claim(
                claim_to_process.get_preferred_address(),
                claim_to_process.get_name(),
            ),
            MessageQueuePriority::High,
        );
        AddressClaimingState::AddressClaimingComplete
    }

    pub(super) fn update_state_send_arbitrary_address_claim(
        claim_to_process: &AddressClaimingData,
        network: &mut NetworkManager,
    ) -> Self {
        let next_address = network.get_next_free_arbitrary_address();

        if Address::NULL != next_address {
            // Found an address we can use
            network.enqueue_can_message(
                NetworkManager::construct_address_claim(next_address, claim_to_process.get_name()),
                MessageQueuePriority::High,
            );
            return AddressClaimingState::AddressClaimingComplete;
        }
        AddressClaimingState::UnableToClaim
    }
}

impl Default for AddressClaimingState {
    fn default() -> Self {
        Self::new()
    }
}

impl AddressClaimingData {
    pub fn new(name: NAME, preferred_address: Address, enabled: bool) -> AddressClaimingData {
        AddressClaimingData {
            state: AddressClaimingState::None,
            name,
            timestamp: None,
            preferred_address,
            random_delay: AddressClaimingData::generate_random_delay(),
            enabled,
        }
    }

    pub fn get_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enable: bool) {
        self.enabled = enable;

        if !enable {
            self.timestamp = None;
            self.state = AddressClaimingState::None;
        }
    }

    pub fn get_preferred_address(&self) -> Address {
        self.preferred_address
    }

    pub(super) fn set_preferred_address(&mut self, new_address: Address) {
        self.preferred_address = new_address;
    }

    pub fn get_state(&self) -> AddressClaimingState {
        self.state
    }

    pub(super) fn set_state(&mut self, new_state: AddressClaimingState) {
        self.state = new_state;
    }

    pub fn get_name(&self) -> NAME {
        self.name
    }

    pub fn set_name(&mut self, new_name: NAME) {
        if self.name.raw_name != new_name.raw_name {
            self.state = AddressClaimingState::None; // Name changed, state no longer valid
        }
        self.name = new_name;
    }

    pub fn get_timestamp(&self) -> Option<Instant> {
        self.timestamp
    }

    pub(super) fn set_timestamp(&mut self, new_timestamp: Option<Instant>) {
        self.timestamp = new_timestamp;
    }

    pub(super) fn get_random_delay(&self) -> u8 {
        self.random_delay
    }

    pub(super) fn generate_random_delay() -> u8 {
        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
        (rng.gen_range(0..255) as f32 * 0.6_f32) as u8
    }
}
