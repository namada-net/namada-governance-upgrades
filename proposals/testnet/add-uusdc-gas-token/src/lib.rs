use std::collections::BTreeMap;

use namada_tx_prelude::{parameters::storage::get_gas_cost_key, *};
use namada_tx_prelude_01491 as namada_tx_prelude;

pub type ChannelId = &'static str;
pub type BaseToken = &'static str;

pub type CanBeUsedAsGas = bool;
pub type Gas = token::Amount;
pub type MinimumGasPrice = Option<Gas>;

const GAS_TOKENS: [(ChannelId, BaseToken, MinimumGasPrice); 1] =
    [("channel-27", "uusdc", Some(Gas::from_u64(1)))];

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    // Read the current gas cost map
    let gas_cost_key = get_gas_cost_key();
    let mut minimum_gas_price: BTreeMap<Address, token::Amount> =
        ctx.read(&gas_cost_key)?.unwrap_or_default();

    // Set gas cost for each token
    for (channel_id, base_token, can_be_used_as_gas) in GAS_TOKENS {
        let ibc_denom = format!("transfer/{channel_id}/{base_token}");
        let token_address = ibc::ibc_token(&ibc_denom).clone();

        if let Some(gas) = can_be_used_as_gas {
            minimum_gas_price.insert(token_address.clone(), gas);
        }
    }

    // Write the gas cost map back to storage
    ctx.write(&gas_cost_key, minimum_gas_price)?;

    Ok(())
}
