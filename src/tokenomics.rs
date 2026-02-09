pub const TOTAL_SUPPLY: u64 = 33_000_000_000_000;
pub const DECIMALS: u8 = 6;
pub const TOKEN_SYMBOL: &str = "VLid";
pub const TOKEN_NAME: &str = "Valid";

pub const BLOCKS_PER_EPOCH: u64 = 3_150_000 * 7;
pub const EPOCH_COUNT: usize = 3;

pub const EPOCH_PERCENTAGES: [f64; 3] = [0.60, 0.30, 0.10];

pub const L1_VALIDATORS_PCT: f64 = 0.15;
pub const L2_VALIDATORS_PCT: f64 = 0.20;
pub const P2P_REWARDS_PCT: f64 = 0.40;
pub const DEV_GRANTS_PCT: f64 = 0.25;

pub const BLOCK_REWARD_PCT: f64 = 0.60;
pub const RACER_REWARD_PCT: f64 = 0.25;
pub const TPI_REWARD_PCT: f64 = 0.10;
pub const SNAPSHOT_REWARD_PCT: f64 = 0.05;

#[derive(Debug, Clone)]
pub struct EpochRewards {
    pub block_reward: u64,
    pub tpi_reward_per_validator: u64,
    pub racer_reward: u64,
    pub snapshot_reward: u64,
}

pub fn calculate_epoch_rewards(epoch: usize) -> EpochRewards {
    if epoch >= EPOCH_COUNT {
        return EpochRewards {
            block_reward: 0,
            tpi_reward_per_validator: 0,
            racer_reward: 0,
            snapshot_reward: 0,
        };
    }

    let total_epoch_supply = (TOTAL_SUPPLY as f64 * EPOCH_PERCENTAGES[epoch]) as u64;
    let l1_budget = (total_epoch_supply as f64 * L1_VALIDATORS_PCT) as u64;
    let blocks_per_epoch = BLOCKS_PER_EPOCH;

    let block_budget = (l1_budget as f64 * BLOCK_REWARD_PCT) as u64;
    let tpi_budget = (l1_budget as f64 * TPI_REWARD_PCT) as u64;
    let racer_budget = (l1_budget as f64 * RACER_REWARD_PCT) as u64;
    let snapshot_budget = (l1_budget as f64 * SNAPSHOT_REWARD_PCT) as u64;

    EpochRewards {
        block_reward: block_budget / blocks_per_epoch,
        tpi_reward_per_validator: tpi_budget / (blocks_per_epoch * 3),
        racer_reward: racer_budget / (blocks_per_epoch / 100),
        snapshot_reward: snapshot_budget / 365,
    }
}

pub fn format_vlid(micro_vlid: u64) -> String {
    let vlid = micro_vlid as f64 / 1_000_000.0;
    format!("{:.6} VLid", vlid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_supply_fits_u64() {
        assert!(TOTAL_SUPPLY <= u64::MAX);
    }

    #[test]
    fn test_epoch_0_rewards() {
        let rewards = calculate_epoch_rewards(0);
        assert!(rewards.block_reward > 0);
        println!("Epoch 0 block reward: {} ({})", rewards.block_reward, format_vlid(rewards.block_reward));
        println!("Epoch 0 TPI reward: {} ({})", rewards.tpi_reward_per_validator, format_vlid(rewards.tpi_reward_per_validator));
    }

    #[test]
    fn test_epoch_percentages_sum() {
        let sum: f64 = EPOCH_PERCENTAGES.iter().sum();
        assert!((sum - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_category_percentages_sum() {
        let sum = L1_VALIDATORS_PCT + L2_VALIDATORS_PCT + P2P_REWARDS_PCT + DEV_GRANTS_PCT;
        assert!((sum - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_all_epochs() {
        for epoch in 0..EPOCH_COUNT {
            let rewards = calculate_epoch_rewards(epoch);
            assert!(rewards.block_reward > 0, "Epoch {} block reward is zero", epoch);
            println!("Epoch {}: block={}", epoch, format_vlid(rewards.block_reward));
        }
    }
}
