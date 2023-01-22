use anyhow::Result;
use bee_block::output::{dto::OutputDto, OutputId};
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::{fs::File, io::BufWriter, path::Path};

mod snapshot;
use snapshot::get_snapshot_data;

const SNAPSHOT_PATH: &str = "latest-full_snapshot.bin";
// The ledger index will get prepended
const OUTPUT_PATH: &str = "snapshot.json";

// Data of all outputs for a certain ledger_index
#[derive(Clone, Serialize, Deserialize)]
pub struct SnapshotData {
    // The milestone index
    #[serde(rename = "ledgerIndex")]
    ledger_index: u32,
    // All outputs
    #[serde(rename = "outputIdsWithOutput")]
    output_ids_with_output: HashMap<OutputId, OutputDto>,
    // The remaining amount in the treasury (legacy network)
    #[serde(rename = "treasuryOutputAmount")]
    treasury_output_amount: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    let snapshot_data = read_snapshot_outputs_data(SNAPSHOT_PATH)?;
    write_to_file(
        format!("{}{}", snapshot_data.ledger_index, OUTPUT_PATH),
        snapshot_data,
    )?;
    Ok(())
}

// Read snapshot file and write outputs to a file in json format
fn read_snapshot_outputs_data(snapshot_path: &str) -> Result<SnapshotData> {
    let mut output_ids_with_output = HashMap::new();
    let (outputs, treasury_output_amount, ledger_index) = get_snapshot_data(snapshot_path)?;
    for output_data in outputs.into_iter() {
        output_ids_with_output.insert(output_data.output_id, OutputDto::from(&output_data.output));
    }
    Ok(SnapshotData {
        ledger_index,
        output_ids_with_output,
        treasury_output_amount,
    })
}

/// Function to write snapshot data to a file
pub fn write_to_file<P: AsRef<Path>>(path: P, snapshot_data: SnapshotData) -> Result<()> {
    let jsonvalue = serde_json::to_value(snapshot_data)?;
    let file = File::create(path)?;
    let bw = BufWriter::new(file);
    serde_json::to_writer_pretty(bw, &jsonvalue)?;
    Ok(())
}
