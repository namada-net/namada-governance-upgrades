use namada_ibc::storage::unlimited_channel_key;
use namada_tx_prelude::*;

// Channel ID where the transfers are unlimited
const CHANNEL_IDS: [&str; 2] = ["channel-22", "channel-23"];

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    for channel_id in CHANNEL_IDS {
        let unlimited_channel_id = channel_id.parse().unwrap();
        let unlimited_channel_key = unlimited_channel_key(&unlimited_channel_id);

        ctx.write(&unlimited_channel_key, ())?;
    }
    Ok(())
}
