//! Whitelist initial set of IBC tokens to enter Namada.
//!
//! This is a good example that demonstrates several different operations:
//! - Setting rate limits for new tokens
//! - Writing to the token map, needed for future incentivized rewards
//! - Whitelisting a token for use as a gas token and setting the gas cost
//! - Initializing shielded inflation data for a token
//!

use std::collections::BTreeMap;

use dec::Dec;
use namada_tx_prelude::*;
use namada_tx_prelude_02512 as namada_tx_prelude;
use parameters_storage::get_gas_cost_key;

pub type ChannelId = &'static str;
pub type BaseToken = &'static str;

pub type MintTokenLimit = token::Amount;
pub type ThroughtputTokenLimit = token::Amount;
pub type CanBeUsedAsGas = bool;
pub type Gas = token::Amount;
pub type MinimumGasPrice = Option<Gas>;

const IBC_TOKENS: [(
    ChannelId,
    BaseToken,
    MintTokenLimit,
    ThroughtputTokenLimit,
    MinimumGasPrice,
); 6] = [
    (
        "channel-1",
        "uosmo",
        MintTokenLimit::from_u64(10752692000000), // 10,752,692 OSMO
        ThroughtputTokenLimit::from_u64(2150539000000), // 2,150,539 OSMO
        Some(Gas::from_u64(10)),                  // 10 uosmo / gas unit
    ),
    (
        "channel-2",
        "uatom",
        MintTokenLimit::from_u64(759878000000), // 759,878 ATOM
        ThroughtputTokenLimit::from_u64(151976000000), // 151,976 ATOM
        Some(Gas::from_u64(1)),                 // 1 uatom / gas unit;
    ),
    (
        "channel-3",
        "utia",
        MintTokenLimit::from_u64(1018330000000), // 1,018,330 TIA
        ThroughtputTokenLimit::from_u64(203666000000), // 203,666 TIA
        Some(Gas::from_u64(1)),                  // 1 utia / gas unit;
    ),
    (
        "channel-0",
        "stuosmo",
        MintTokenLimit::from_u64(8196721000000), // 8,196,721 stOSMO
        ThroughtputTokenLimit::from_u64(1639344000000), // 1,639,344 stOSMO
        Some(Gas::from_u64(10)),                 // 10 stuosmo / gas unit
    ),
    (
        "channel-0",
        "stuatom",
        MintTokenLimit::from_u64(512821000000), // 512,821 stATOM
        ThroughtputTokenLimit::from_u64(102564000000), // 102,564 stATOM
        Some(Gas::from_u64(1)),                 // 1 stuatom / gas unit;
    ),
    (
        "channel-0",
        "stutia",
        MintTokenLimit::from_u64(946970000000), // 946,970 stTIA
        ThroughtputTokenLimit::from_u64(189394000000), // 189,394 stTIA
        Some(Gas::from_u64(1)),                 // 1 stutia / gas unit;
    ),
];

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    // Read the current gas cost map
    let gas_cost_key = get_gas_cost_key();
    let mut minimum_gas_price: BTreeMap<Address, token::Amount> =
        ctx.read(&gas_cost_key)?.unwrap_or_default();

    // Read the current MASP token map
    let token_map_key = token::storage_key::masp_token_map_key();
    let mut token_map = ctx
        .read::<masp::TokenMap>(&token_map_key)?
        .unwrap_or_default();

    // Enable IBC deposit/withdraws limits
    for (channel_id, base_token, mint_limit, throughput_limit, can_be_used_as_gas) in IBC_TOKENS {
        let ibc_denom = format!("transfer/{channel_id}/{base_token}");
        let token_address = ibc::ibc_token(&ibc_denom).clone();

        let mint_limit_token_key = ibc::mint_limit_key(&token_address);
        ctx.write(&mint_limit_token_key, mint_limit)?;

        let throughput_limit_token_key = ibc::throughput_limit_key(&token_address);
        ctx.write(&throughput_limit_token_key, throughput_limit)?;

        // Check if this ibc token should can also be used to pay for gas
        if let Some(gas) = can_be_used_as_gas {
            minimum_gas_price.insert(token_address.clone(), gas);
        }

        // Add the ibc token to the masp token map
        token_map.insert(ibc_denom, token_address.clone());

        // Write some null MASP reward data
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

        ctx.write(
            &shielded_token_last_locked_amount_key,
            token::Amount::zero(),
        )?;
        ctx.write(&shielded_token_last_inflation_key, token::Amount::zero())?;
        ctx.write(&shielded_token_max_rewards_key, Dec::zero())?;
        ctx.write(
            &shielded_token_target_locked_amount_key,
            token::Amount::zero(),
        )?;
        ctx.write(&shielded_token_kp_gain_key, Dec::zero())?;
        ctx.write(&shielded_token_kd_gain_key, Dec::zero())?;
    }

    // Write the gas cost map back to storage
    ctx.write(&gas_cost_key, minimum_gas_price)?;

    // Write the token map back to storage
    ctx.write(&token_map_key, token_map)?;

    Ok(())
}
