use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{Keypair, Signer},
    system_transaction,
};
use std::{thread, time};

fn main() {
    let recipient_address = "";
    let recipient_pubkey = recipient_address.parse().expect("invalid recipient address");

    let rpc_url = "https://api.testnet.solana.com";
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let keypair = Keypair::new();
    println!("new wallet address: {}", keypair.pubkey());

    let airdrop_amount = 1_000_000_000; // 1 SOL
    for _ in 0..3 {
        match client.request_airdrop(&keypair.pubkey(), airdrop_amount) {
            Ok(signature) => {
                println!("Airdrop Transaction: {}", signature);

                let start = time::Instant::now();
                let mut elapsed = start.elapsed();
                while elapsed.as_secs() < 60 && client.get_signature_status(&signature).unwrap().is_none() {
                    thread::sleep(time::Duration::from_secs(1));
                    elapsed = start.elapsed();
                }

                if elapsed.as_secs() >= 60 {
                    eprintln!("Airdrop failed: confirmation time exceeded 1 minute");
                    continue;
                } else {
                    println!("Airdrop completed in {} seconds.", elapsed.as_secs());
                }
            },
            Err(e) => eprintln!("Airdrop Error: {}", e),
        }

        let wait_time = 60 - time::Instant::now().elapsed().as_secs();
        println!("Waiting for {} seconds before next airdrop...", wait_time);
        thread::sleep(time::Duration::from_secs(wait_time));
    }

    let balance = client.get_balance(&keypair.pubkey()).expect("error getting balance");
    println!("Current balance: {}", balance);

    let send_amount = (balance as f64 * 0.99) as u64;

    let transaction = system_transaction::transfer(
        &keypair,
        &recipient_pubkey,
        send_amount,
        client.get_latest_blockhash().expect("error get blockhash"),
    );

    match client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => println!("Send Transaction: {}", signature),
        Err(e) => eprintln!("Send Error: {}", e),
    }
}
