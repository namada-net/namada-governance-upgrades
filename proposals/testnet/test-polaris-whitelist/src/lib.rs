use std::collections::BTreeMap;

use dec::Dec;
use namada_tx_prelude::*;
use namada_tx_prelude_02512 as namada_tx_prelude;
use parameters_storage::get_gas_cost_key;

pub type ChannelId = &'static str;
pub type BaseToken = &'static str;
pub type Precision = u64;

pub type MintTokenLimit = token::Amount;
pub type ThroughtputTokenLimit = token::Amount;
pub type Gas = token::Amount;
pub type MinimumGasPrice = Option<Gas>;
pub type MayBeIncentivized = bool;

const OSMO_CHANNEL_ID: &str = "channel-29";
const MINT_LIMIT: u64 = 1_000_000;
const THROUGHPUT_LIMIT: u64 = 1_000_000;
const MINIMUM_GAS_PRICE: u64 = 1;
const MAY_BE_INCENTIVIZED: bool = true;

const OSMO_TOKENS: [(BaseToken, Precision); 5] = [
    (
        "factory/osmo1z6r6qdknhgsc0zeracktgpcxf43j6sekq07nw8sxduc9lg0qjjlqfu25e3/alloyed/allBTC", // Bitcoin (alloyed)
        8,
    ),
    ("transfer/channel-6994/utia", 6), // Celestia
    ("transfer/channel-208/uusdt", 6), // Tether USD (Ethereum via Axelar)
    (
        "factory/osmo1k6c8jln7ejuqwtqmay3yvzrg3kueaczl96pk067ldg8u835w0yhsw27twm/alloyed/allETH", // Ethereum (alloyed)
        18,
    ),
    (
        "transfer/channel-0/transfer/08-wasm-1369/0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2", // Ethereum (Eureka)
        18,
    ),
];

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    // Read the current gas cost map
    let gas_cost_key = get_gas_cost_key();
    let mut minimum_gas_price: BTreeMap<Address, token::Amount> =
        ctx.read(&gas_cost_key)?.unwrap_or_default();

    // Enable IBC deposit/withdraws limits
    for (base_token, precision) in OSMO_TOKENS {
        let ibc_denom = format!("transfer/{OSMO_CHANNEL_ID}/{base_token}");
        let token_address = ibc::ibc_token(&ibc_denom).clone();

        let mint_limit = MintTokenLimit::from_u64(MINT_LIMIT * precision);
        let throughput_limit = ThroughtputTokenLimit::from_u64(THROUGHPUT_LIMIT * precision);

        let mint_limit_token_key = ibc::mint_limit_key(&token_address);
        ctx.write(&mint_limit_token_key, mint_limit)?;

        let throughput_limit_token_key = ibc::throughput_limit_key(&token_address);
        ctx.write(&throughput_limit_token_key, throughput_limit)?;

        // Check if this ibc token should can also be used to pay for gas
        minimum_gas_price.insert(
            token_address.clone(),
            token::Amount::from_u64(MINIMUM_GAS_PRICE),
        );

        // Initialize some data if this token may be incentivized in the future
        if MAY_BE_INCENTIVIZED {
            // Add the ibc token to the masp token map
            // token_map.insert(ibc_denom, token_address.clone());

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
    }

    // Write the gas cost map back to storage
    ctx.write(&gas_cost_key, minimum_gas_price)?;

    // Write the token map back to storage
    // ctx.write(&token_map_key, token_map)?;

    Ok(())
}
