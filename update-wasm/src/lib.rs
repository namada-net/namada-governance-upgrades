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

// The following are just example strings.
// In practice, replace them with what is in and intended to go into the DB onchain.
const TX_NAME: &str = "tx_ibc.wasm";
const OLD_TX_HASH: &str = "7b0d43f4a277aadd02562d811c755e09d7f191c601ca3bffb89a7f8b599dab1e";
const NEW_TX_HASH: &str = "bd1b4230890ff8ece2733de7e770f4a8573a9541813186e5cb718bcc05c2056d";
const NEW_TX_CODE: &[u8] =
    include_bytes!("tx_ibc.bd1b4230890ff8ece2733de7e770f4a8573a9541813186e5cb718bcc05c2056d.wasm");
