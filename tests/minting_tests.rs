use pos_chain::types::{Block, ChainState};
use pos_chain::tokenomics::{TOTAL_SUPPLY, calculate_epoch_rewards};

fn create_test_block(slot: u64, producer: &str) -> Block {
    Block {
        slot,
        parent_hash: "genesis".to_string(),
        hash: "test_hash".to_string(),
        producer: producer.to_string(),
        timestamp: 0,
        transactions: vec![],
    }
}

#[test]
fn test_block_reward_minting() {
    let mut state = ChainState::new();
    let block = create_test_block(0, "validator1");
    
    let initial_supply = state.total_supply;
    assert!(state.add_block(block.clone()));
    
    let epoch_0_rewards = calculate_epoch_rewards(0);
    assert_eq!(state.total_supply, initial_supply + epoch_0_rewards.block_reward);
    assert_eq!(state.get_balance("validator1"), epoch_0_rewards.block_reward);
}

#[test]
fn test_supply_cap_enforcement() {
    let mut state = ChainState::new();
    state.total_supply = TOTAL_SUPPLY - 1000;
    
    let block = create_test_block(0, "validator1");
    assert!(state.add_block(block));
    
    assert!(state.total_supply <= TOTAL_SUPPLY);
}

#[test]
fn test_multiple_blocks_increase_supply() {
    let mut state = ChainState::new();
    
    for i in 0..10 {
        let block = create_test_block(i, "validator1");
        state.add_block(block);
    }
    
    let epoch_0_rewards = calculate_epoch_rewards(0);
    let expected_supply = epoch_0_rewards.block_reward * 10;
    assert_eq!(state.total_supply, expected_supply);
}

#[test]
fn test_validator_earns_block_rewards() {
    let mut state = ChainState::new();
    
    let block = create_test_block(0, "validator1");
    state.add_block(block);
    
    let epoch_0_rewards = calculate_epoch_rewards(0);
    assert_eq!(state.get_balance("validator1"), epoch_0_rewards.block_reward);
}

#[test]
fn test_epoch_transition() {
    let mut state = ChainState::new();
    
    let epoch_1_start_slot = 3_150_000 * 7;
    
    let epoch_1_block = create_test_block(epoch_1_start_slot, "validator1");
    state.add_block(epoch_1_block);
    
    let epoch_1_rewards = calculate_epoch_rewards(1);
    assert_eq!(state.total_supply, epoch_1_rewards.block_reward);
    assert_eq!(state.get_balance("validator1"), epoch_1_rewards.block_reward);
}

#[test]
fn test_minting_stops_at_supply_cap() {
    let mut state = ChainState::new();
    state.total_supply = TOTAL_SUPPLY;
    
    let block = create_test_block(0, "validator1");
    state.add_block(block);
    
    assert_eq!(state.total_supply, TOTAL_SUPPLY);
    assert_eq!(state.get_balance("validator1"), 0);
}

#[test]
fn test_different_validators_earn_rewards() {
    let mut state = ChainState::new();
    
    let block1 = create_test_block(0, "validator1");
    let block2 = create_test_block(1, "validator2");
    let block3 = create_test_block(2, "validator3");
    
    state.add_block(block1);
    state.add_block(block2);
    state.add_block(block3);
    
    let epoch_0_rewards = calculate_epoch_rewards(0);
    assert_eq!(state.get_balance("validator1"), epoch_0_rewards.block_reward);
    assert_eq!(state.get_balance("validator2"), epoch_0_rewards.block_reward);
    assert_eq!(state.get_balance("validator3"), epoch_0_rewards.block_reward);
}
