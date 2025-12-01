use namada_tx_prelude::*;
use namada_tx_prelude_01502::{self as namada_tx_prelude};

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    let max_proposal_code_size_key = gov_storage::keys::get_max_proposal_code_size_key();
    ctx.write(&max_proposal_code_size_key, 3_000_000_u64)?;

    Ok(())
}
