use std::collections::BTreeMap;

use namada_tx_prelude::*;
use parameters_storage::get_gas_cost_key;

pub type ChannelId = &'static str;
pub type BaseToken = &'static str;

pub type CanBeUsedAsGas = bool;
pub type Gas = token::Amount;
pub type MinimumGasPrice = Option<Gas>;

const GAS_TOKENS: [(
    ChannelId,
    BaseToken,
    MinimumGasPrice,
); 1] = [
    (
        "channel-27",
        "uusdc",
        Some(Gas::from_u64(1)),                        // 1 uusdc / gas unit
    ),
];

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    // Read the current gas cost map
    let gas_cost_key = get_gas_cost_key();
    let mut minimum_gas_price: BTreeMap<Address, token::Amount> =
        ctx.read(&gas_cost_key)?.unwrap_or_default();

    // Enable IBC deposit/withdraws limits
    for (channel_id, base_token, can_be_used_as_gas) in GAS_TOKENS {
        let ibc_denom = format!("transfer/{channel_id}/{base_token}");
        let token_address = ibc::ibc_token(&ibc_denom).clone();

        // Check if this ibc token should can also be used to pay for gas
        if let Some(gas) = can_be_used_as_gas {
            minimum_gas_price.insert(token_address.clone(), gas);
        }
    }

    // Write the gas cost map back to storage
    ctx.write(&gas_cost_key, minimum_gas_price)?;

    Ok(())
}
