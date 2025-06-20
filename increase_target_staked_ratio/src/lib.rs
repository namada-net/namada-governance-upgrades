use namada_tx_prelude::{gov_storage::keys::get_max_proposal_code_size_key, *};

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    let max_proposal_code_size_key = get_max_proposal_code_size_key();

    ctx.write(&max_proposal_code_size_key, 2_000_000_u64)?;

    Ok(())
}
