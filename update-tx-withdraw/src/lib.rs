use std::str::FromStr;

use namada_tx_prelude::hash::Hash;
use namada_tx_prelude::*;

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    let new_code_hash = Hash::from_str(NEW_TX_HASH).unwrap();
    let new_code_len = u64::try_from(NEW_TX_CODE.len()).unwrap();
    let old_code_hash = Hash::from_str(OLD_TX_HASH).unwrap();

    // Update the tx allowlist parameter
    let tx_allowlist_key = parameters_storage::get_tx_allowlist_storage_key();
    let tx_allowlist: Vec<String> = ctx.read(&tx_allowlist_key)?.unwrap();
    assert!(tx_allowlist.iter().any(|hash_str| hash_str == OLD_TX_HASH));
    let tx_allowlist: Vec<String> = tx_allowlist
        .into_iter()
        .map(|hash_str| {
            if hash_str == OLD_TX_HASH {
                new_code_hash.to_string().to_lowercase()
            } else {
                hash_str
            }
        })
        .collect();
    ctx.write(&tx_allowlist_key, tx_allowlist)?;

    // Delete the old tx code
    let old_code_key = storage::Key::wasm_code(&old_code_hash);
    let old_code_len_key = storage::Key::wasm_code_len(&old_code_hash);
    ctx.delete(&old_code_key)?;
    ctx.delete(&old_code_len_key)?;

    // Write the new tx code into storage
    let code_key = storage::Key::wasm_code(&new_code_hash);
    let code_len_key = storage::Key::wasm_code_len(&new_code_hash);
    let hash_key = storage::Key::wasm_hash(TX_NAME);
    let code_name_key = storage::Key::wasm_code_name(TX_NAME.to_owned());

    ctx.write(&code_key, NEW_TX_CODE)?;
    ctx.write(&code_len_key, new_code_len)?;
    ctx.write(&hash_key, new_code_hash)?;
    ctx.write(&code_name_key, new_code_hash)?;

    Ok(())
}

const TX_NAME: &str = "tx_withdraw.wasm";
const OLD_TX_HASH: &str = "8a9df03a1a8f5e9e606e14a97fdfb2097dba062da1b3b2158bbfa7deabeeadfb";
const NEW_TX_HASH: &str = "991043b8dc468f5d0ad54b22bfbe75b355a535126c6d3e9dcf511a00d6c8b331";
const NEW_TX_CODE: &[u8] = include_bytes!(
    "tx_withdraw.991043b8dc468f5d0ad54b22bfbe75b355a535126c6d3e9dcf511a00d6c8b331.wasm"
);
