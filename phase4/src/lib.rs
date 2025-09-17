use dec::Dec;
use namada_tx_prelude::*;
use std::str::FromStr;

pub type Denomination = u8;
pub type ChannelId = &'static str;
pub type BaseToken = &'static str;

pub type TokenMaxReward = &'static str;
pub type TokenTargetLockedAmount = u64;
pub type KpGain = &'static str;
pub type KdGain = &'static str;

const IBC_TOKENS: [(
    Denomination,
    ChannelId,
    BaseToken,
    TokenTargetLockedAmount,
    KpGain,
    KdGain,
); 14] = [
    (
        0,
        "channel-27",
        "uusdc",
        1_500_000_000, // 1.5K USDC
        "666",
        "666",
    ),
    (0, "channel-10", "utia", 0, "666", "666"),
    (0, "channel-13", "utia", 0, "666", "666"),
    (0, "channel-15", "stutia", 0, "666", "666"),
    (0, "channel-15", "stuatom", 0, "666", "666"),
    (0, "channel-15", "stuosmo", 0, "666", "666"),
    (0, "channel-16", "uatom", 0, "666", "666"),
    (0, "channel-17", "utia", 0, "666", "666"),
    (0, "channel-18", "uosmo", 0, "666", "666"),
    (0, "channel-7", "uosmo", 0, "666", "666"),
    (0, "channel-8", "stuosmo", 0, "666", "666"),
    (0, "channel-8", "stuatom", 0, "666", "666"),
    (0, "channel-8", "stutia", 0, "666", "666"),
    (0, "channel-9", "uatom", 0, "666", "666"),
];

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    // Enable shielded set rewards for ibc tokens
    for (denomination, channel_id, base_token, target_locked_amount, kp, kd) in IBC_TOKENS {
        let ibc_denom = format!("transfer/{channel_id}/{base_token}");
        let token_address = ibc::ibc_token(&ibc_denom);

        let shielded_token_target_locked_amount_key =
            token::storage_key::masp_locked_amount_target_key(&token_address);
        let shielded_token_kp_gain_key = token::storage_key::masp_kp_gain_key(&token_address);
        let shielded_token_kd_gain_key = token::storage_key::masp_kd_gain_key(&token_address);

        // Write new MASP inflation param values
        ctx.write(
            &shielded_token_target_locked_amount_key,
            token::Amount::from_uint(target_locked_amount, denomination).unwrap(),
        )?;
        ctx.write(&shielded_token_kp_gain_key, Dec::from_str(kp).unwrap())?;
        ctx.write(&shielded_token_kd_gain_key, Dec::from_str(kd).unwrap())?;
    }

    Ok(())
}
