use dec::Dec;
use hash::Hash as CodeHash;
use namada_tx_prelude::*;
use storage::Key;

use std::str::FromStr;

use namada_proof_of_stake::storage::{read_pos_params, write_pos_params};

const TX_UPDATE_ACCOUNT_NAME: &str = "tx_update_account.wasm";
const TX_UPDATE_ACCOUNT_BYTES: &[u8] = include_bytes!("../../wasms/tx_update_account.04256c4087de46e71e6f323b696ec412e41cfd3ec337010c1f2a590037c0cf8c.wasm");

const TX_INIT_ACCOUNT_NAME: &str = "tx_init_account.wasm";
const TX_INIT_ACCOUNT_BYTES: &[u8] = include_bytes!("../../wasms/tx_init_account.4078c07a356691b23049055df6fc7fb99c32b75f488f435538c1a960cdb41153.wasm");

const TX_CLAIM_REWARDS_NAME: &str = "tx_claim_rewards.wasm";
const TX_CLAIM_REWARDS_BYTES: &[u8] = include_bytes!("../../wasms/tx_claim_rewards.af4cd35b30f17cf2440be74bedeb74296aac21e4ab1ea7143f3a3dede722a0ed.wasm");

const TX_UPDATE_STEWARD_COMMISSION_NAME: &str = "tx_update_steward_commission.wasm";
const TX_UPDATE_STEWARD_COMMISSION_BYTES: &[u8] = include_bytes!("../../wasms/tx_update_steward_commission.01e2395fab96f0ac8251329370149144159d378abcf5a414df1c030a329bda82.wasm");

const TX_RESIGN_STEWARD_NAME: &str = "tx_resign_steward.wasm";
const TX_RESIGN_STEWARD_BYTES: &[u8] = include_bytes!("../../wasms/tx_resign_steward.b5dbddb91e2627d400fa6ca83970a8396dd968ab25ff896b845b19a813729e04.wasm");

#[transaction(gas = 10000)]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    // PoS inflation
    let mut pos_params = read_pos_params(ctx)?.owned;
    pos_params.max_inflation_rate = Dec::from_str("0.1").unwrap();
    pos_params.target_staked_ratio = Dec::from_str("0.666667").unwrap();
    pos_params.rewards_gain_p = Dec::from_str("2.5").unwrap();
    pos_params.rewards_gain_d = Dec::from_str("2.5").unwrap();
    write_pos_params(ctx, &pos_params)?;

    // PGF inflation
    let pgf_inflation_key = governance::pgf::storage::keys::get_pgf_inflation_rate_key();
    let pgf_inflation_rate = Dec::from_str("0.025").unwrap(); // set PGF inflaton inflation to 2.5%
    ctx.write(&pgf_inflation_key, pgf_inflation_rate)?;

    // PGF stewards inflation
    let steward_inflation_key = governance::pgf::storage::keys::get_steward_inflation_rate_key();
    let steward_inflation_rate = Dec::from_str("0.001").unwrap(); // set PGF stewards inflation to 0.01%
    ctx.write(&steward_inflation_key, steward_inflation_rate)?;

    // Read the current transaction allowlist from storage
    let tx_allowlist_key = parameters_storage::get_tx_allowlist_storage_key();
    let mut current_tx_allowlist = ctx
        .read::<Vec<String>>(&tx_allowlist_key)?
        .unwrap_or_default();

    // Update the allowlist and write the addition wasm storage keys per transaction
    for (wasm_name, wasm_bytes) in [
        (TX_UPDATE_ACCOUNT_NAME, TX_UPDATE_ACCOUNT_BYTES),
        (TX_INIT_ACCOUNT_NAME, TX_INIT_ACCOUNT_BYTES),
        (TX_CLAIM_REWARDS_NAME, TX_CLAIM_REWARDS_BYTES),
        (
            TX_UPDATE_STEWARD_COMMISSION_NAME,
            TX_UPDATE_STEWARD_COMMISSION_BYTES,
        ),
        (TX_RESIGN_STEWARD_NAME, TX_RESIGN_STEWARD_BYTES),
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
