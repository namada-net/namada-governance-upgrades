use dec::Dec;
use namada_proof_of_stake::storage as pos_storage;
use namada_proof_of_stake_02512 as namada_proof_of_stake;
use namada_tx_prelude::parameters::storage as params_storage;
use namada_tx_prelude::*;
use namada_tx_prelude_02512 as namada_tx_prelude;

use std::str::FromStr;

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    // Update the proof-of-stake parameters
    let mut pos_params = pos_storage::read_pos_params::<Ctx, governance::Store<Ctx>>(ctx)?.owned;
    pos_params.target_staked_ratio = Dec::from_str("0.55").unwrap();

    pos_storage::write_pos_params(ctx, &pos_params)?;

    // Update other protocol parameters
    let masp_fee_payment_gas_limit_key = params_storage::get_masp_fee_payment_gas_limit_key();
    let masp_fee_payment_gas_limit: u64 = 300_000;

    ctx.write(&masp_fee_payment_gas_limit_key, masp_fee_payment_gas_limit)?;

    Ok(())
}
