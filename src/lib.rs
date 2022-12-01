mod pb;

use pb::basicexample;

use substreams::{log};
use substreams_solana::pb as solpb;

#[substreams::handlers::map]
fn sol_basic_mapper(block: solpb::sol::v1::Block) -> Result<basicexample::BasicExampleProtoData, substreams::errors::Error> {
    // Extract data from the Solana Block and log to the console.
    // The data available in the Block directly represents the related protobuf.
    // The full data model for a Solona Block is available at the following link.
    // https://github.com/streamingfast/firehose-solana/blob/develop/proto/sf/solana/type/v1/type.proto
    log::info!("block.previous_blockhash: {:#?}", block.previous_blockhash);
    log::info!("block.blockhash: {:#?}", block.blockhash);
    log::info!("block.slot: {:#?}", block.slot);

    // Copy the data in the Block's blockhash field and return it to caller.
    // Substreams developers will typically pass extracted data through a custom
    // protobuf to a store module.
    Ok(basicexample::BasicExampleProtoData {blockhash: block.blockhash})
}