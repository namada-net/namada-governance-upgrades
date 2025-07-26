use std::collections::BTreeMap;

use dec::Dec;
use masp::Precision;
use namada_tx_prelude::*;
use parameters_storage::get_gas_cost_key;

pub type ChannelId = &'static str;
pub type BaseToken = &'static str;

pub type MintTokenLimit = token::Amount;
pub type ThroughtputTokenLimit = token::Amount;
pub type CanBeUsedAsGas = bool;
pub type Gas = token::Amount;
pub type MinimumGasPrice = Option<Gas>;

// Provide a list of tokens to whitelist.
const IBC_TOKENS: [(
    ChannelId,
    BaseToken,
    MintTokenLimit,
    ThroughtputTokenLimit,
    MinimumGasPrice,
    Precision,
); 1] = [(
    "channel-1",                                        // example channel
    "utoken",                                           // example token
    MintTokenLimit::from_u64(10_000_000_000_000),       // example of token's raw units
    ThroughtputTokenLimit::from_u64(2_000_000_000_000), // example of token's raw units
    Some(Gas::from_u64(10)),                            // Ex: 10 utoken / gas unit
    100_000_000,                                        // example precision in token's raw units
)];

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    // Read the current gas cost map
    let gas_cost_key = get_gas_cost_key();
    let mut minimum_gas_price: BTreeMap<Address, token::Amount> =
        ctx.read(&gas_cost_key)?.unwrap_or_default();

    for (channel_id, base_token, mint_limit, throughput_limit, can_be_used_as_gas, precision) in
        IBC_TOKENS
    {
        let ibc_denom = format!("transfer/{channel_id}/{base_token}");
        let token_address = ibc::ibc_token(&ibc_denom).clone();

        // Write IBC deposit/withdraws limits
        let mint_limit_token_key = ibc::mint_limit_key(&token_address);
        ctx.write(&mint_limit_token_key, mint_limit)?;

        let throughput_limit_token_key = ibc::throughput_limit_key(&token_address);
        ctx.write(&throughput_limit_token_key, throughput_limit)?;

        // Write shielded rewards precision
        let shielded_token_precision_key =
            token::storage_key::masp_reward_precision_key(&token_address);
        ctx.write(&shielded_token_precision_key, precision)?;

        // Check if this ibc token should can also be used to pay for gas
        if let Some(gas) = can_be_used_as_gas {
            minimum_gas_price.insert(token_address.clone(), gas);
        }

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

    Ok(())
}
