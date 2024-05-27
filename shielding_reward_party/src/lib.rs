use std::str::FromStr;

use dec::Dec;
use namada_tx_prelude::*;

pub type Denomination = u8;
pub type ChannelId = &'static str;
pub type BaseToken = &'static str;

pub type MintTokenLimit = token::Amount;
pub type ThroughtputTokenLimit = token::Amount;
pub type TokenMaxReward = &'static str;
pub type TokenTargetLockedAmount = &'static str;
pub type KpGain = &'static str;
pub type KdGain = &'static str;

#[allow(clippy::type_complexity)]
const IBC_TOKENS: [(
    Denomination,
    ChannelId,
    BaseToken,
    MintTokenLimit,
    ThroughtputTokenLimit,
    TokenMaxReward,
    TokenTargetLockedAmount,
    KpGain,
    KdGain,
); 1] = [(
    0,
    "channel-0",
    "tnam1q....",
    MintTokenLimit::from_u64(1000),
    ThroughtputTokenLimit::from_u64(10000),
    "0.01",
    "1_000_000_000",
    "120000",
    "120000",
)];

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    // Enable NAM transfers
    let native_token_transferable_key = parameters_storage::get_native_token_transferable_key();
    ctx.write(&native_token_transferable_key, true)?;

    // Enable NAM to be transferable via IBC
    // Enable NAM to get MASP rewards
    for (
        denomination,
        channel_id,
        base_token,
        mint_limit,
        throughput_limit,
        max_reward,
        target_locked_amount,
        kp,
        kd,
    ) in IBC_TOKENS
    {
        let ibc_denom = format!("transfer/{channel_id}/{base_token}");
        let token_address = ibc::ibc_token(&ibc_denom);

        // Enable IBC deposit/withdraws limits
        let mint_limit_token_key = ibc::mint_limit_key(&token_address);
        ctx.write(&mint_limit_token_key, mint_limit)?;
        let throughput_limit_token_key = ibc::throughput_limit_key(&token_address);
        ctx.write(&throughput_limit_token_key, throughput_limit)?;

        let shielded_token_last_inflation_key =
            token::storage_key::masp_last_inflation_key(&token_address);
        let shielded_token_last_locked_amount_key =
            token::storage_key::masp_last_locked_amount_key(&token_address);
        let shielded_token_max_rewards_key =
            token::storage_key::masp_max_reward_rate_key(&token_address);
        let shielded_token_target_locked_amount_key =
            token::storage_key::masp_locked_amount_target_key(&token_address);
        let shielded_token_kp_gain_key = token::storage_key::masp_kp_gain_key(&token_address);
        let shielded_token_kd_gain_key = token::storage_key::masp_kd_gain_key(&token_address);
        let token_map_key = token::storage_key::masp_token_map_key();

        // Add the ibc token to the masp token map
        let mut token_map: masp::TokenMap = ctx.read(&token_map_key)?.unwrap_or_default();
        token_map.insert(ibc_denom, token_address);
        ctx.write(&token_map_key, token_map)?;

        // Write the MASP inflation keys
        ctx.write(&shielded_token_last_inflation_key, token::Amount::zero())?;
        ctx.write(
            &shielded_token_last_locked_amount_key,
            token::Amount::zero(),
        )?;
        ctx.write(
            &shielded_token_max_rewards_key,
            Dec::from_str(max_reward).unwrap(),
        )?;
        ctx.write(
            &shielded_token_target_locked_amount_key,
            token::Amount::from_uint(target_locked_amount, denomination).unwrap(),
        )?;
        ctx.write(&shielded_token_kp_gain_key, Dec::from_str(kp).unwrap())?;
        ctx.write(&shielded_token_kd_gain_key, Dec::from_str(kd).unwrap())?;
    }
    Ok(())
}
