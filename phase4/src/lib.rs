use dec::Dec;
use namada_tx_prelude::*;
use std::str::FromStr;

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
    TokenTargetLockedAmount,
    KpGain,
    KdGain,
); 7] = [
    (
        0,
        "channel-1",
        "uosmo",
        1_500_000_000_000, // 1.5M OSMO
        "4.74",
        "8.05",
    ),
    (
        0,
        "channel-2",
        "uatom",
        125_000_000_000, // 125k ATOM
        "126",
        "214",
    ),
    (
        0,
        "channel-3",
        "utia",
        110_000_000_000, // 110K TIA
        "50",
        "85",
    ),
    (
        0,
        "channel-0",
        "stuosmo",
        700_000_000_000, // 700k stOSMO
        "63",
        "107",
    ),
    (
        0,
        "channel-0",
        "stuatom",
        15_000_000_000, // 15k stATOM
        "2032",
        "3455",
    ),
    (
        0,
        "channel-0",
        "stutia",
        50_000_000_000, // 50k stTIA
        "563",
        "956",
    ),
    (
        0,
        "channel-5",
        "uusdc",
        1_100_000_000_000, // 1.1M USDC
        "29",
        "49",
    ),
];

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    // Enable shielded set rewards for ibc tokens
    for (denomination, channel_id, base_token, target_locked_amount, kp, kd) in IBC_TOKENS {
        let ibc_denom = format!("transfer/{channel_id}/{base_token}");
        let token_address = ibc::ibc_token(&ibc_denom);

        let shielded_token_target_locked_amount_key =
            token::storage_key::masp_locked_amount_target_key(&token_address);
        let shielded_token_kp_gain_key = token::storage_key::masp_kp_gain_key(&token_address);
        let shielded_token_kd_gain_key = token::storage_key::masp_kd_gain_key(&token_address);

        // Write new MASP inflation param values
        ctx.write(
            &shielded_token_target_locked_amount_key,
            token::Amount::from_uint(target_locked_amount, denomination).unwrap(),
        )?;
        ctx.write(&shielded_token_kp_gain_key, Dec::from_str(kp).unwrap())?;
        ctx.write(&shielded_token_kd_gain_key, Dec::from_str(kd).unwrap())?;
    }

    Ok(())
}
