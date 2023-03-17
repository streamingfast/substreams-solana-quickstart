mod pb;

use pb::sf::solana::block_meta::v1::BlockMeta;

use substreams_solana::pb::sol;

#[substreams::handlers::map]
fn map_block(block: sol::v1::Block) -> Result<BlockMeta, substreams::errors::Error> {
    let mut block_height : Option<u64> = None;
    if let Some(v) = block.block_height.as_ref() {
        block_height = Some(v.block_height)
    }

    Ok(BlockMeta {
        hash: block.blockhash.to_string(),
        parent_hash: block.previous_blockhash.to_string(),
        slot: block.slot,
        parent_slot: block.parent_slot,
        transaction_count: block.transactions.len() as u64,
        block_height,
    })
}