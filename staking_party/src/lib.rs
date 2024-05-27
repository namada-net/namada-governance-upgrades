use hash::Hash as CodeHash;
use namada_tx_prelude::*;
use storage::Key;

pub type ChannelId = &'static str;
pub type BaseToken = &'static str;

pub type MintTokenLimit = token::Amount;
pub type ThroughtputTokenLimit = token::Amount;

const TX_TRANSFER_NAME: &str = "tx_transfer.wasm";
const TX_TRANSFER_BYTES: &[u8] = include_bytes!(
    "../../wasms/tx_transfer.8198890dbdf67bc93b16492496413c8140a273465f0f8f3ae091e5949a5ac4e0.wasm"
);

const IBC_TOKENS: [(ChannelId, BaseToken, MintTokenLimit, ThroughtputTokenLimit); 2] = [
    (
        "channel-0",
        "tnam1q....",
        MintTokenLimit::from_u64(1000),
        ThroughtputTokenLimit::from_u64(10000),
    ),
    (
        "channel-1",
        "tnam1q....",
        MintTokenLimit::from_u64(2000),
        ThroughtputTokenLimit::from_u64(13000),
    ),
];

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    // Enable IBC deposit/withdraws limits
    for (channel_id, base_token, mint_limit, throughput_limit) in IBC_TOKENS {
        let ibc_denom = format!("transfer/{channel_id}/{base_token}");
        let token_address = ibc::ibc_token(&ibc_denom);

        let mint_limit_token_key = ibc::mint_limit_key(&token_address);
        ctx.write(&mint_limit_token_key, mint_limit)?;

        let throughput_limit_token_key = ibc::throughput_limit_key(&token_address);
        ctx.write(&throughput_limit_token_key, throughput_limit)?;
    }

    // Read the current transaction allowlist from storage
    let tx_allowlist_key = parameters_storage::get_tx_allowlist_storage_key();
    let mut current_tx_allowlist = ctx
        .read::<Vec<String>>(&tx_allowlist_key)?
        .unwrap_or_default();

    // Update the allowlist and write the addition wasm storage keys per transaction
    for (wasm_name, wasm_bytes) in [
        (TX_TRANSFER_NAME, TX_TRANSFER_BYTES)
    ] {
        let tx_hash = CodeHash::sha256(wasm_bytes);

        if current_tx_allowlist.contains(&tx_hash.to_string()) {
            continue;
        }

        let wasm_name_key = Key::wasm_code_name(wasm_name.to_string());
        ctx.write(&wasm_name_key, tx_hash)?;

        let wasm_hash_key = Key::wasm_hash(wasm_name);
        ctx.write(&wasm_hash_key, tx_hash)?;

        let code_key = Key::wasm_code(&tx_hash);
        ctx.write(&code_key, wasm_bytes)?;

        let len_key = Key::wasm_code_len(&tx_hash);
        ctx.write(&len_key, wasm_bytes.len())?;

        current_tx_allowlist.push(tx_hash.to_string());
    }

    // Write the update allowlist back to storage
    ctx.write(&tx_allowlist_key, current_tx_allowlist)?;

    Ok(())
}
