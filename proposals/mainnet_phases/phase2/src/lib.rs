use dec::Dec;
use namada_proof_of_stake::storage::{read_pos_params, write_pos_params};
use namada_proof_of_stake_01502 as namada_proof_of_stake;
use namada_tx_prelude::*;
use namada_tx_prelude_01502 as namada_tx_prelude;

use std::str::FromStr;

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    // Turn on PoS inflation
    let mut pos_params = read_pos_params::<Ctx, governance::Store<Ctx>>(ctx)?.owned;
    pos_params.max_inflation_rate = Dec::from_str("0.05").unwrap();
    pos_params.target_staked_ratio = Dec::from_str("0.4").unwrap();
    pos_params.rewards_gain_p = Dec::from_str("0.5").unwrap();
    pos_params.rewards_gain_d = Dec::from_str("0.5").unwrap();
    write_pos_params(ctx, &pos_params)?;

    // Turn on PGF inflation
    let pgf_inflation_key = governance::pgf::storage::keys::get_pgf_inflation_rate_key();
    let pgf_inflation_rate = Dec::from_str("0.05").unwrap();
    ctx.write(&pgf_inflation_key, pgf_inflation_rate)?;

    Ok(())
}
