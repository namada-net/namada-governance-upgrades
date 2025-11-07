use dec::Dec;
use namada_tx_prelude::*;
use namada_tx_prelude_01502 as namada_tx_prelude;
use std::str::FromStr;

pub type ChannelId = &'static str;
pub type BaseToken = &'static str;

const NEW_REWARD_RATE: &str = "0.0";
const IBC_TOKENS: [(ChannelId, BaseToken); 12] = [
    ("channel-10", "utia"),
    ("channel-13", "utia"),
    ("channel-16", "uatom"),
    ("channel-15", "stuosmo"),
    ("channel-15", "stuatom"),
    ("channel-15", "stutia"),
    ("channel-17", "utia"),
    ("channel-7", "uosmo"),
    ("channel-8", "stuosmo"),
    ("channel-8", "stutia"),
    ("channel-8", "statom"),
    ("channel-9", "uatom"),
];

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    for (channel_id, base_token) in IBC_TOKENS {
        let ibc_denom = format!("transfer/{channel_id}/{base_token}");
        let token_address = ibc::ibc_token(&ibc_denom);

        let shielded_token_max_reward_rate_key =
            token::storage_key::masp_max_reward_rate_key(&token_address);

        ctx.write(
            &shielded_token_max_reward_rate_key,
            Dec::from_str(NEW_REWARD_RATE).unwrap(),
        )?;
    }

    Ok(())
}
