use namada_tx_prelude::*;

// Channel ID(s) where the transfers are unlimited
const CHANNEL_IDS: [&str; 4] = [
    "channel-21", // bbn-1 (Babylon)
    "channel-22", // grand-1 (Noble)
    "channel-23", // cosmoshub-4
    "channel-24", // osmotest-5
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
