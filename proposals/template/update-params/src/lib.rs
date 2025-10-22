use namada_proof_of_stake::storage as pos_storage;
use namada_proof_of_stake_01502 as namada_proof_of_stake;
use namada_tx_prelude::*;
use namada_tx_prelude_01502 as namada_tx_prelude;

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    // Update the proof-of-stake parameters
    let mut pos_params = pos_storage::read_pos_params::<Ctx, governance::Store<Ctx>>(ctx)?.owned;
    pos_params.unbonding_len = 212_u64;

    pos_storage::write_pos_params(ctx, &pos_params)?;

    Ok(())
}
