use std::collections::BTreeMap;

use dec::Dec;
use namada_tx_prelude::*;

pub type ChannelId = &'static str;
pub type BaseToken = &'static str;

const MIGRATIONS: [(ChannelId, BaseToken, ChannelId); 7] = [
    ("channel-1", "uosmo", "channel-101"),
    ("channel-2", "uatom", "channel-102"),
    ("channel-3", "utia", "channel-103"),
    ("channel-0", "stuosmo", "channel-100"),
    ("channel-0", "stuatom", "channel-100"),
    ("channel-0", "stutia", "channel-100"),
    ("channel-5", "uusdc", "channel-105"),
];

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    let token_map_key = token::storage_key::masp_token_map_key();
    let mut token_map: masp::TokenMap = ctx.read(&token_map_key)?.unwrap_or_default();

    let gas_cost_key = parameters_storage::get_gas_cost_key();
    let mut gas_cost: BTreeMap<Address, token::Amount> =
        ctx.read(&gas_cost_key)?.unwrap_or_default();

    for (old_chan, base_token, new_chan) in MIGRATIONS {
        migrate_ibc_token(
            ctx,
            &mut token_map,
            &mut gas_cost,
            old_chan,
            base_token,
            new_chan,
        )?;
    }

    ctx.write(&gas_cost_key, gas_cost)?;
    ctx.write(&token_map_key, token_map)?;

    Ok(())
}

#[inline(always)]
fn migrate_ibc_token(
    ctx: &mut Ctx,
    token_map: &mut masp::TokenMap,
    gas_cost: &mut BTreeMap<Address, token::Amount>,
    old_chan: ChannelId,
    base_token: BaseToken,
    new_chan: ChannelId,
) -> TxResult {
    let old_denom = format!("transfer/{old_chan}/{base_token}");
    let new_denom = format!("transfer/{new_chan}/{base_token}");
    let old_token = ibc::ibc_token(&old_denom);
    let new_token = ibc::ibc_token(&new_denom);

    disable_old_masp_inflation(ctx, &old_token)?;
    remove_old_token_from_map(token_map, &old_denom);

    update_ibc_rate_limits(ctx, &old_token, &new_token)?;
    enable_new_masp_reward_state(ctx, &new_token)?;
    update_gas_token(gas_cost, &old_token, &new_token);
    add_new_token_to_map(token_map, new_denom, new_token);

    Ok(())
}

#[inline(always)]
fn disable_old_masp_inflation(ctx: &mut Ctx, old_token: &Address) -> TxResult {
    ctx.write(
        &token::storage_key::masp_max_reward_rate_key(old_token),
        Dec::zero(),
    )?;
    ctx.write(
        &token::storage_key::masp_kp_gain_key(old_token),
        Dec::zero(),
    )?;
    ctx.write(
        &token::storage_key::masp_kd_gain_key(old_token),
        Dec::zero(),
    )?;
    ctx.write(
        &token::storage_key::masp_locked_amount_target_key(old_token),
        token::Amount::zero(),
    )?;
    Ok(())
}

#[inline(always)]
fn update_gas_token(
    gas_cost: &mut BTreeMap<Address, token::Amount>,
    old_token: &Address,
    new_token: &Address,
) {
    if let Some(old_gas_price) = gas_cost.remove(old_token) {
        gas_cost.insert(new_token.clone(), old_gas_price);
    }
}

#[inline(always)]
fn remove_old_token_from_map(token_map: &mut masp::TokenMap, old_denom: &str) {
    token_map.remove(old_denom);
}

#[inline(always)]
fn update_ibc_rate_limits(ctx: &mut Ctx, old_token: &Address, new_token: &Address) -> TxResult {
    let mint_limit: token::Amount = ctx
        .read(&ibc::mint_limit_key(&old_token))?
        .unwrap_or_default();
    let throughput_limit: token::Amount = ctx
        .read(&ibc::throughput_limit_key(&old_token))?
        .unwrap_or_default();

    ctx.write(&ibc::mint_limit_key(new_token), mint_limit)?;
    ctx.write(&ibc::throughput_limit_key(new_token), throughput_limit)?;

    ctx.write(&ibc::mint_limit_key(old_token), token::Amount::zero())?;
    ctx.write(&ibc::throughput_limit_key(old_token), token::Amount::zero())?;

    Ok(())
}

#[inline(always)]
fn enable_new_masp_reward_state(ctx: &mut Ctx, new_token: &Address) -> TxResult {
    ctx.write(
        &token::storage_key::masp_last_inflation_key(new_token),
        token::Amount::zero(),
    )?;
    ctx.write(
        &token::storage_key::masp_last_locked_amount_key(new_token),
        token::Amount::zero(),
    )?;
    ctx.write(
        &token::storage_key::masp_max_reward_rate_key(new_token),
        Dec::zero(),
    )?;
    ctx.write(
        &token::storage_key::masp_locked_amount_target_key(new_token),
        token::Amount::zero(),
    )?;
    ctx.write(
        &token::storage_key::masp_kp_gain_key(new_token),
        Dec::zero(),
    )?;
    ctx.write(
        &token::storage_key::masp_kd_gain_key(new_token),
        Dec::zero(),
    )?;
    Ok(())
}

#[inline(always)]
fn add_new_token_to_map(token_map: &mut masp::TokenMap, new_denom: String, new_token: Address) {
    token_map.insert(new_denom, new_token);
}
