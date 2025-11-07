use dec::Dec;
use namada_tx_prelude::*;
use namada_tx_prelude_01502 as namada_tx_prelude;
use std::str::FromStr;

pub type ChannelId = &'static str;
pub type BaseToken = &'static str;

const NEW_REWARD_RATE: &str = "0.0";
const IBC_TOKENS: [(ChannelId, BaseToken); 6] = [
    ("channel-1", "uosmo"),
    ("channel-2", "uatom"),
    ("channel-3", "utia"),
    ("channel-0", "stuosmo"),
    ("channel-0", "stuatom"),
    ("channel-0", "stutia"),
];

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    for (channel_id, base_token) in IBC_TOKENS {
        let ibc_denom = format!("transfer/{channel_id}/{base_token}");
        let token_address = ibc::ibc_token(&ibc_denom);

        let shielded_token_max_rewards_key =
            token::storage_key::masp_max_reward_rate_key(&token_address);

        ctx.write(
            &shielded_token_max_rewards_key,
            Dec::from_str(NEW_REWARD_RATE).unwrap(),
        )?;
    }

    Ok(())
}
