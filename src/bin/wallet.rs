use pos_chain::crypto::{generate_keypair, keypair_to_address, sign_transaction, KeyPair};
use std::env;
use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct WalletFile {
    secret_key: String,
    public_key: String,
    address: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        return;
    }
    
    match args[1].as_str() {
        "new" => create_wallet(),
        "address" => show_address(),
        "balance" => check_balance(&args),
        "send" => send_transaction(&args),
        _ => print_usage(),
    }
}

fn print_usage() {
    println!("Wallet Commands:");
    println!("  wallet new                    - Create new wallet");
    println!("  wallet address                - Show your address");
    println!("  wallet balance <rpc_url>      - Check balance");
    println!("  wallet send <to> <amount> <rpc_url> - Send transaction");
    println!("\nExample:");
    println!("  wallet new");
    println!("  wallet balance http://localhost:3000");
    println!("  wallet send 5JvB...xyz 1000 http://localhost:3000");
}

fn create_wallet() {
    if fs::metadata("wallet.json").is_ok() {
        println!("Wallet already exists! Delete wallet.json to create a new one.");
        return;
    }
    
    let keypair = generate_keypair();
    let address = keypair_to_address(&keypair);
    
    let wallet = WalletFile {
        secret_key: hex::encode(keypair.signing_key.to_bytes()),
        public_key: hex::encode(keypair.verifying_key.to_bytes()),
        address: address.clone(),
    };
    
    let json = serde_json::to_string_pretty(&wallet).unwrap();
    fs::write("wallet.json", json).expect("Failed to write wallet file");
    
    println!("Wallet created!");
    println!("Address: {}", address);
    println!("Saved to wallet.json - keep this file safe!");
}

fn show_address() {
    let wallet = load_wallet();
    println!("Your address: {}", wallet.address);
}

fn check_balance(args: &[String]) {
    if args.len() < 3 {
        println!("Usage: wallet balance <rpc_url>");
        return;
    }
    
    let wallet = load_wallet();
    let rpc_url = &args[2];
    
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(format!("{}/balance", rpc_url))
        .json(&serde_json::json!({ "address": wallet.address }))
        .send();
    
    match response {
        Ok(resp) => {
            let balance: serde_json::Value = resp.json().unwrap();
            println!("Balance: {}", balance["balance"]);
        }
        Err(e) => println!("Error: {}", e),
    }
}

fn send_transaction(args: &[String]) {
    if args.len() < 5 {
        println!("Usage: wallet send <to_address> <amount> <rpc_url>");
        return;
    }
    
    let wallet = load_wallet();
    let to = &args[2];
    let amount: u64 = args[3].parse().expect("Invalid amount");
    let rpc_url = &args[4];
    
    let secret_bytes = hex::decode(&wallet.secret_key).expect("Invalid secret key");
    let secret_array: [u8; 32] = secret_bytes.try_into().expect("Wrong secret key length");
    let signing_key = ed25519_dalek::SigningKey::from_bytes(&secret_array);
    let verifying_key = signing_key.verifying_key();
    
    let keypair = KeyPair {
        signing_key,
        verifying_key,
    };
    
    let signature = sign_transaction(&keypair, &wallet.address, to, amount);
    
    println!("Sending {} to {}...", amount, to);
    
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(format!("{}/submit", rpc_url))
        .json(&serde_json::json!({
            "from": wallet.address,
            "from_pubkey": wallet.public_key,
            "to": to,
            "amount": amount,
            "signature": signature
        }))
        .send();
    
    match response {
        Ok(resp) => {
            let result: serde_json::Value = resp.json().unwrap();
            println!("Success: {}", result["message"]);
        }
        Err(e) => println!("Error: {}", e),
    }
}

fn load_wallet() -> WalletFile {
    let json = fs::read_to_string("wallet.json").expect("Wallet file not found. Run 'wallet new' first.");
    serde_json::from_str(&json).expect("Invalid wallet file")
}
