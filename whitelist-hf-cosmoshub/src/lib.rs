use namada_tx_prelude::*;

// Channel ID(s) where the transfers are unlimited
const CHANNEL_IDS: [&str; 1] = [
    "channel-26", // new cosmoshub-4 channel (08/22/2025)
];

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    for channel_id in CHANNEL_IDS {
        let unlimited_channel_id = channel_id.parse().unwrap();
        let unlimited_channel_key =
            ibc::unlimited_channel_key(&unlimited_channel_id);
    
        ctx.write(&unlimited_channel_key, ())?;
    }

    Ok(())
}
