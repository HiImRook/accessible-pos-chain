use pos_chain::tpi::*;

fn main() {
    let validators = vec![
        "validator_1".to_string(),
        "validator_2".to_string(),
        "validator_3".to_string(),
        "validator_4".to_string(),
        "validator_5".to_string(),
    ];
    
    println!("Testing TPI validator selection:\n");
    
    for slot in 1000..1010 {
        let tpi_group = select_tpi_validators(slot, &validators);
        println!("Slot {}: {:?}", slot, tpi_group);
    }
    
    println!("\nTesting broadcaster selection (merit-based):\n");
    
    let validators_with_merit = vec![
        ("validator_1".to_string(), 9800),
        ("validator_2".to_string(), 8200),
        ("validator_3".to_string(), 7500),
    ];
    
    let broadcaster = select_broadcaster_by_merit(&validators_with_merit);
    println!("Broadcaster (highest merit): {}", broadcaster);
}
