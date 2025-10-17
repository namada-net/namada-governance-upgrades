use namada_tx_prelude::*;

pub const MAX_BLOCK_GAS: u64 = 10_000_000;
pub const MASP_FEE_PAYMENT_GAS_LIMIT: u64 = 300_000;

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    let max_block_gas_key = parameters_storage::get_max_block_gas_key();
    let masp_fee_payment_gas_limit_key = parameters_storage::get_masp_fee_payment_gas_limit_key();
    ctx.write(&max_block_gas_key, MAX_BLOCK_GAS)?;
    ctx.write(&masp_fee_payment_gas_limit_key, MASP_FEE_PAYMENT_GAS_LIMIT)?;

    Ok(())
}
