use pos_chain::tokenomics::{
    calculate_epoch_rewards,
    TOTAL_SUPPLY,
    EPOCH_COUNT,
    EPOCH_PERCENTAGES,
    L1_VALIDATORS_PCT,
    L2_VALIDATORS_PCT,
    P2P_REWARDS_PCT,
    DEV_GRANTS_PCT,
    BLOCK_REWARD_PCT,
    RACER_REWARD_PCT,
    TPI_REWARD_PCT,
    SNAPSHOT_REWARD_PCT,
};

#[test]
fn test_total_supply_is_33_million() {
    assert_eq!(TOTAL_SUPPLY, 33_000_000_000_000_000);
}

#[test]
fn test_epoch_percentages_sum_to_100() {
    let sum: f64 = EPOCH_PERCENTAGES.iter().sum();
    assert!((sum - 1.0).abs() < 0.0001, "Epoch percentages should sum to 1.0");
}

#[test]
fn test_category_percentages_sum_to_100() {
    let sum = L1_VALIDATORS_PCT + L2_VALIDATORS_PCT + P2P_REWARDS_PCT + DEV_GRANTS_PCT;
    assert!((sum - 1.0).abs() < 0.0001, "Category percentages should sum to 1.0");
}

#[test]
fn test_l1_internal_percentages_sum_to_100() {
    let sum = BLOCK_REWARD_PCT + RACER_REWARD_PCT + TPI_REWARD_PCT + SNAPSHOT_REWARD_PCT;
    assert!((sum - 1.0).abs() < 0.0001, "L1 internal percentages should sum to 1.0");
}

#[test]
fn test_epoch_0_rewards_are_positive() {
    let rewards = calculate_epoch_rewards(0);
    assert!(rewards.block_reward > 0, "Block reward should be positive");
    assert!(rewards.tpi_reward_per_validator > 0, "TPI reward should be positive");
    assert!(rewards.racer_reward > 0, "Racer reward should be positive");
    assert!(rewards.snapshot_reward > 0, "Snapshot reward should be positive");
}

#[test]
fn test_epoch_rewards_follow_percentage_split() {
    let epoch_0 = calculate_epoch_rewards(0);
    let epoch_1 = calculate_epoch_rewards(1);
    let epoch_2 = calculate_epoch_rewards(2);
    
    let ratio_0_to_1 = epoch_0.block_reward as f64 / epoch_1.block_reward as f64;
    let ratio_1_to_2 = epoch_1.block_reward as f64 / epoch_2.block_reward as f64;
    
    assert!((ratio_0_to_1 - 2.0).abs() < 0.1, "Epoch 0→1 should be ÷2 (60%→30%), got {}", ratio_0_to_1);
    assert!((ratio_1_to_2 - 3.0).abs() < 0.1, "Epoch 1→2 should be ÷3 (30%→10%), got {}", ratio_1_to_2);
}

#[test]
fn test_epoch_out_of_range_returns_zero() {
    let rewards = calculate_epoch_rewards(3);
    assert_eq!(rewards.block_reward, 0);
    assert_eq!(rewards.tpi_reward_per_validator, 0);
    assert_eq!(rewards.racer_reward, 0);
    assert_eq!(rewards.snapshot_reward, 0);
}

#[test]
fn test_all_epochs_have_positive_rewards() {
    for epoch in 0..EPOCH_COUNT {
        let rewards = calculate_epoch_rewards(epoch);
        assert!(rewards.block_reward > 0, "Epoch {} block reward should be positive", epoch);
        assert!(rewards.tpi_reward_per_validator > 0, "Epoch {} TPI reward should be positive", epoch);
        assert!(rewards.racer_reward > 0, "Epoch {} racer reward should be positive", epoch);
        assert!(rewards.snapshot_reward > 0, "Epoch {} snapshot reward should be positive", epoch);
    }
}
