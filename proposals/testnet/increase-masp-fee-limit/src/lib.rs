use namada_tx_prelude_01491::{self as namada_tx_prelude};
use namada_tx_prelude::{parameters::storage::get_masp_fee_payment_gas_limit_key, *};

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    let masp_fee_payment_gas_limit_key = get_masp_fee_payment_gas_limit_key();
    let masp_fee_payment_gas_limit = 300_000;
    
    ctx.write(&masp_fee_payment_gas_limit_key, masp_fee_payment_gas_limit)?;

    Ok(())
}
