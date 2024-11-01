use std::collections::BTreeMap;

use namada_tx_prelude::*;
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
); 2] = [
    (
        "channel-4",
        "uosmo",
        MintTokenLimit::from_u64(10000000000),
        ThroughtputTokenLimit::from_u64(10000000000),
        Some(Gas::from_u64(1)),
    ),
    (
        "channel-5",
        "uatom",
        MintTokenLimit::from_u64(10000000000),
        ThroughtputTokenLimit::from_u64(10000000000),
        None,
    ),
];

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    let gas_cost_key = get_gas_cost_key();
    let mut minimum_gas_price: BTreeMap<Address, token::Amount> =
        ctx.read(&gas_cost_key)?.unwrap_or_default();

    // Enable IBC deposit/withdraws limits
    for (channel_id, base_token, mint_limit, throughput_limit, can_be_used_as_gas) in IBC_TOKENS {
        let ibc_denom = format!("transfer/{channel_id}/{base_token}");
        let token_address = ibc::ibc_token(&ibc_denom);

        let mint_limit_token_key = ibc::mint_limit_key(&token_address);
        ctx.write(&mint_limit_token_key, mint_limit)?;

        let throughput_limit_token_key = ibc::throughput_limit_key(&token_address);
        ctx.write(&throughput_limit_token_key, throughput_limit)?;

        // Check if this ibc token should can also be used to pay for gas
        if let Some(gas) = can_be_used_as_gas {
            minimum_gas_price.insert(token_address, gas);
        }
    }

    ctx.write(&gas_cost_key, minimum_gas_price)?;

    Ok(())
}
