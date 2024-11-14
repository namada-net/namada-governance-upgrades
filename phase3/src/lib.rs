use namada_tx_prelude::*;
use parameters_storage::{get_gas_scale_key, get_masp_fee_payment_gas_limit_key, get_max_block_gas_key};

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    let gas_scale_key = get_gas_scale_key();
    ctx.write(&gas_scale_key, 50_000 as u64)?;

    let masp_fee_payment_gas_limit_key = get_masp_fee_payment_gas_limit_key();
    ctx.write(&masp_fee_payment_gas_limit_key, 80_000 as u64)?;

    let max_block_gas_key = get_max_block_gas_key();
    ctx.write(&max_block_gas_key, 3_000_000 as u64)?;

    Ok(())
}
