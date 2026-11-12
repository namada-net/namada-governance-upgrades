use dec::Dec;
use namada_tx_prelude::*;

pub type ChannelId = &'static str;
pub type BaseToken = &'static str;

const CHANS: [(ChannelId, BaseToken); 7] = [
    ("channel-1", "uosmo"),
    ("channel-2", "uatom"),
    ("channel-3", "utia"),
    ("channel-0", "stuosmo"),
    ("channel-0", "stuatom"),
    ("channel-0", "stutia"),
    ("channel-5", "uusdc"),
];

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    for (old_chan, base_token) in CHANS {
        let old_denom = format!("transfer/{old_chan}/{base_token}");
        let old_token = ibc::ibc_token(&old_denom);

        // Zero out the MASP rewards of the old IBC token.
        ctx.write(
            &token::storage_key::masp_max_reward_rate_key(&old_token),
            Dec::zero(),
        )?;
    }

    Ok(())
}
