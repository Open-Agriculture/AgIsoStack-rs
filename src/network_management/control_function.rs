// Copyright 2023 Raven Industries inc.
#![allow(dead_code)]

use crate::network_management::name::NAME;
use rand::Rng;
use std::time::Instant;

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
    timestamp: Option<Instant>,
    preferred_address: u8,
    random_delay: u8,
    enabled: bool,
}

pub enum ControlFunction {
    Internal {
        name: NAME,
        address_claim_data: AddressClaimingData,
    },
    External {
        name: NAME,
    },
}

impl AddressClaimingData {
    pub fn new(preferred_address: u8, enabled: bool) -> AddressClaimingData {
        AddressClaimingData {
            state: AddressClaimingState::None,
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

    pub fn get_preferred_address(&self) -> u8 {
        self.preferred_address
    }

    pub fn get_state(&self) -> AddressClaimingState {
        self.state
    }

    pub(super) fn set_state(&mut self, new_state: AddressClaimingState) {
        self.state = new_state;
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

impl Default for AddressClaimingData {
    fn default() -> AddressClaimingData {
        AddressClaimingData {
            state: AddressClaimingState::None,
            timestamp: None,
            preferred_address: 0xFE_u8,
            random_delay: AddressClaimingData::generate_random_delay(),
            enabled: true,
        }
    }
}
