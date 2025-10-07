use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

const FINALITY_THRESHOLD: f64 = 0.66;

pub struct Consensus {
    validators: HashMap<String, Validator>,
    total_stake: u64,
    current_epoch: u64,
    epoch_start: u64,
}

pub struct Validator {
    pub address: String,
    pub stake: u64,
    pub active: bool,
}

impl Consensus {
    pub fn new() -> Self {
        Consensus {
            validators: HashMap::new(),
            total_stake: 0,
            current_epoch: 0,
            epoch_start: current_timestamp(),
        }
    }

    pub fn register_validator(&mut self, address: String, stake: u64) {
        self.total_stake += stake;
        self.validators.insert(address.clone(), Validator {
            address,
            stake,
            active: true,
        });
    }

    pub fn select_producer(&self, slot: u64) -> Option<String> {
        let active: Vec<&Validator> = self.validators.values().filter(|v| v.active).collect();
        if active.is_empty() {
            return None;
        }

        let seed = slot ^ self.current_epoch;
        let random_stake = (seed % self.total_stake) as u64;
        let mut accumulated = 0;

        for validator in active {
            accumulated += validator.stake;
            if accumulated > random_stake {
                return Some(validator.address.clone());
            }
        }

        None
    }

    pub fn should_rotate_epoch(&self) -> bool {
        current_timestamp() >= self.epoch_start + 21600
    }

    pub fn rotate_epoch(&mut self) {
        self.current_epoch += 1;
        self.epoch_start = current_timestamp();
    }
}

fn current_timestamp() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}
