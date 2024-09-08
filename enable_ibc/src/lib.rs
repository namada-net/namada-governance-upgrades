use namada_ibc::parameters::IbcParameters;
use namada_tx_prelude::*;

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    let ibc_parameters = IbcParameters {
        default_mint_limit: token::Amount::native_whole(10000000),
        default_per_epoch_throughput_limit: token::Amount::native_whole(10000000),
    };

    let ibc_parameters_key = namada_ibc::storage::params_key();
    ctx.write(&ibc_parameters_key, ibc_parameters)?;

    Ok(())
}
