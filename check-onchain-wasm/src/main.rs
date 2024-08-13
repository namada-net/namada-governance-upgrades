pub mod config;

use std::process;

use clap::Parser;
use config::AppConfig;
use namada_sdk::{hash::Hash, rpc::query_storage_value};
use sha2::{Digest, Sha256};
use tendermint_rpc::HttpClient;
use tokio::{fs::File, io::AsyncWriteExt};

#[tokio::main]
async fn main() {
    let config = AppConfig::parse();

    let client = HttpClient::new(config.tendermint_url.as_str()).unwrap();

    let proposal_code_key =
        namada_governance::storage::keys::get_proposal_code_key(config.proposal_id);
    let proposal_code: Vec<u8> = query_storage_value(&client, &proposal_code_key)
        .await
        .unwrap();

    let mut hasher = Sha256::new();
    hasher.update(&proposal_code);

    let proposal_code_hash = format!("{:x}", hasher.finalize());
    println!("Proposal Code Hash: {}", proposal_code_hash.to_ascii_uppercase());

    if let Some(expected_hash) = config.expected_hash {
        if let Err(_) = Hash::try_from(expected_hash.as_str()) {
            println!("The supplied hash via --expected-hash is not valid");
            process::exit(1)
        };
        let hash_is_correct = proposal_code_hash.eq(&expected_hash);
        if hash_is_correct {
            println!("The expected hash correspond to the wasm associated with proposal id: {}", config.proposal_id)
        } else {
            println!("The expected hash DOES NOT correspond to the wasm associated with proposal id: {}", config.proposal_id)
        }
    }

    if let Some(filepath) = config.dump_to {
        let mut file = File::create(filepath).await.unwrap();
        file.write_all(&proposal_code).await.unwrap();
    }
}
