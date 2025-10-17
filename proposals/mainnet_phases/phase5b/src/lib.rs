use namada_tx_prelude::*;
use namada_tx_prelude_01502 as namada_tx_prelude;

pub type ChannelId = &'static str;
pub type BaseToken = &'static str;

pub type MintTokenLimit = token::Amount;
pub type ThroughtputTokenLimit = token::Amount;

const MINT_LIMIT: MintTokenLimit = MintTokenLimit::from_u128(10_000_000_000_000_000); // 10B NAM -> Just a higher number than total supply for safety
const THROUGHPUT_LIMIT: ThroughtputTokenLimit =
    ThroughtputTokenLimit::from_u128(20_000_000_000_000); // 20M NAM

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    // 1. Enable IBC rate limits for native token
    let native_token = ctx.get_native_token()?;
    let mint_limit_token_key = ibc::mint_limit_key(&native_token);
    let throughput_limit_token_key = ibc::throughput_limit_key(&native_token);

    ctx.write(&mint_limit_token_key, MINT_LIMIT)?;
    ctx.write(&throughput_limit_token_key, THROUGHPUT_LIMIT)?;

    Ok(())
}
