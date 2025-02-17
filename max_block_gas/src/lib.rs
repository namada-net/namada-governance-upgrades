use namada_tx_prelude::*;

pub const MAX_BLOCK_GAS: u64 = 5_000_000;

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    // Raise max block gas
    let max_block_gas_key = parameters_storage::get_max_block_gas_key();
    ctx.write(&max_block_gas_key, MAX_BLOCK_GAS)?;

    Ok(())
}
