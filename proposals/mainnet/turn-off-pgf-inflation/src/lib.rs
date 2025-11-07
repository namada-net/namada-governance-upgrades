use dec::Dec;
use namada_tx_prelude::*;

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    // Turn off PGF inflation
    let pgf_inflation_key = governance::pgf::storage::keys::get_pgf_inflation_rate_key();
    let pgf_inflation_rate = Dec::zero();
    ctx.write(&pgf_inflation_key, pgf_inflation_rate)?;

    Ok(())
}
