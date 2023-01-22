use anyhow::anyhow;
use anyhow::Result;
use bee_block::payload::milestone::MilestoneIndex;
use bee_block::{
    output::{Output, OutputId},
    BlockId,
};
use bee_ledger::types::snapshot::{FullSnapshotHeader, SnapshotHeader, SnapshotKind};
use bee_tangle::solid_entry_point::SolidEntryPoint;
use packable::PackableExt;
use packable::{unpacker::Unpacker, Packable};

#[derive(Debug, Clone)]
pub struct OutputData {
    pub output_id: OutputId,
    pub output: Output,
    pub block_id: BlockId,
}

/// Get outputs, treasury_output_amount and ledger_index from a snapshot
pub fn get_snapshot_data(path: &str) -> Result<(Vec<OutputData>, u64, u32)> {
    let mut slice = &std::fs::read(path)?[..];

    let header = SnapshotHeader::unpack_verified(slice)?;

    println!("Network ID:\t\t\t{}", header.network_id());
    println!("Ledger index:\t\t\t{}", header.ledger_index());

    if let SnapshotKind::Full = header.kind() {
        let full_header = FullSnapshotHeader::unpack::<_, true>(&mut slice)?;

        // Read solid entry points so we can access the outputs that come after them
        for _ in 0..full_header.sep_count() {
            SolidEntryPoint::unpack_verified(slice)?;
        }

        println!("Output count:\t\t\t{}", full_header.output_count());
        let outputs = import_outputs(&mut slice, full_header.output_count())?;
        Ok((
            outputs,
            full_header.treasury_output_amount(),
            *header.ledger_index(),
        ))
    } else {
        Err(anyhow!("Full snapshot required"))
    }
}

// Read OutputData
fn import_outputs<U: Unpacker>(unpacker: &mut U, output_count: u64) -> Result<Vec<OutputData>> {
    let mut outputs = Vec::new();
    for _ in 0..output_count {
        let output_id =
            OutputId::unpack::<_, true>(unpacker).map_err(|_| anyhow!("unpacking failed"))?;
        let block_id =
            BlockId::unpack::<_, true>(unpacker).map_err(|_| anyhow!("unpacking failed"))?;
        let _milestone_index_booked =
            MilestoneIndex::unpack::<_, true>(unpacker).map_err(|_| anyhow!("unpacking failed"))?;
        let _milestone_timestamp_booked =
            u32::unpack::<_, true>(unpacker).map_err(|_| anyhow!("unpacking failed"))?;
        let _output_length =
            u32::unpack::<_, true>(unpacker).map_err(|_| anyhow!("unpacking failed"))?;
        let output =
            Output::unpack::<_, true>(unpacker).map_err(|_| anyhow!("unpacking failed"))?;

        outputs.push(OutputData {
            block_id,
            output,
            output_id,
        })
    }
    Ok(outputs)
}
