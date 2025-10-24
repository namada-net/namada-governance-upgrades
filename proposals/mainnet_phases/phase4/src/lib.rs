//! Enable shielded set rewards for IBC tokens.
//!
//! This is a good example that demonstrates the activation of shielded set rewards for an existing token
//! in Namada.
//!

use dec::Dec;
use namada_tx_prelude::*;
use namada_tx_prelude_02512 as namada_tx_prelude;
use std::str::FromStr;
use token::storage_key::balance_key;

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
    TokenMaxReward,
    TokenTargetLockedAmount,
    KpGain,
    KdGain,
); 7] = [
    (
        0,
        "channel-1",
        "uosmo",
        "0.009",
        13_300_000_000_000, // 13.3m OSMO
        "50",
        "85",
    ),
    (
        0,
        "channel-2",
        "uatom",
        "0.009",
        500_000_000_000, // 500k ATOM
        "50",
        "85",
    ),
    (
        0,
        "channel-3",
        "utia",
        "0.009",
        1_260_000_000_000, // 1.26 TIA
        "50",
        "85",
    ),
    (
        0,
        "channel-0",
        "stuosmo",
        "0.009",
        1_000_000_000_000, // 1m stOSMO
        "50",
        "85",
    ),
    (
        0,
        "channel-0",
        "stuatom",
        "0.009",
        31_000_000_000, // 31k stATOM
        "50",
        "85",
    ),
    (
        0,
        "channel-0",
        "stutia",
        "0.009",
        112_000_000_000, // 112k stTIA
        "50",
        "85",
    ),
    (
        0,
        "channel-5",
        "uusdc",
        "0.018",
        2_200_000_000_000, // 2.2M USDC
        "50",
        "85",
    ),
];

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    // Read the current MASP token map
    // NOTE: Not needed for the exact mainnet Phase 4 since this logic was included in the Phase 3 proposal
    // let token_map_key = token::storage_key::masp_token_map_key();
    // let mut token_map = ctx
    //     .read::<masp::TokenMap>(&token_map_key)?
    //     .unwrap_or_default();

    // Enable shielded set rewards for ibc tokens
    for (denomination, channel_id, base_token, max_reward, target_locked_amount, kp, kd) in
        IBC_TOKENS
    {
        let ibc_denom = format!("transfer/{channel_id}/{base_token}");
        let token_address = ibc::ibc_token(&ibc_denom);

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

        // Add the ibc token to the masp token map
        // NOTE: Not needed for the exact mainnet Phase 4 since this logic was included in the Phase 3 proposal
        // token_map.insert(ibc_denom, token_address.clone());

        // Read the current balance of the IBC token in MASP and set that as initial locked amount
        let ibc_balance_key = balance_key(
            &token_address,
            &Address::Internal(address::InternalAddress::Masp),
        );
        let current_ibc_amount = ctx.read::<token::Amount>(&ibc_balance_key)?.unwrap();
        ctx.write(&shielded_token_last_locked_amount_key, current_ibc_amount)?;

        // Initialize the remaining MASP inflation keys
        ctx.write(&shielded_token_last_inflation_key, token::Amount::zero())?;

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

    // Write the token map back to storage
    // NOTE: Not needed for the exact mainnet Phase 4 since this logic was included in the Phase 3 proposal
    // ctx.write(&token_map_key, token_map)?;

    Ok(())
}
