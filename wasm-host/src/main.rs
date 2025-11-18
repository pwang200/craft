extern crate core;

mod data_provider;
mod decoding;
mod hashing;
mod host_functions_wamr;
mod mock_data;
mod sfield;
mod vm_wamr;

use crate::mock_data::MockData;
use clap::Parser;
use log::{debug, error, info};
use std::fs;
use std::path::PathBuf;

/// Wasm WASM testing utility
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    wasm_file: String,
    fixtures:  String,
}

#[allow(clippy::type_complexity)]
fn load_test_data(
    fixtures: &str,
) -> Result<(String, String, String, String, String), Box<dyn std::error::Error>> {
    let base_path = PathBuf::from(fixtures);
    if !base_path.exists() {
        return Err(format!(
            "Fixtures not found at expected location: {}",
            base_path.display()
        )
        .into());
    }

    let tx_path = base_path.join("tx.json");
    let lo_path = base_path.join("ledger_object.json");
    let lh_path = base_path.join("ledger_header.json");
    let l_path = base_path.join("ledger.json");
    let nfts_path = base_path.join("nfts.json");

    let tx_json = fs::read_to_string(tx_path)?;
    let lo_json = fs::read_to_string(lo_path)?;
    let lh_json = fs::read_to_string(lh_path)?;
    let l_json = fs::read_to_string(l_path)?;
    let nft_json = fs::read_to_string(nfts_path)?;

    Ok((tx_json, lo_json, lh_json, l_json, nft_json))
}

fn main() {
    let args = Args::parse();

    println!("{} {}", args.wasm_file, args.fixtures);

    let (tx_json, lo_json, lh_json, l_json, nft_json) =
        match load_test_data(&args.fixtures) {
            Ok((tx, lo, lh, l, nft)) => {
                debug!("Test data loaded successfully");
                (tx, lo, lh, l, nft)
            }
            Err(e) => {
                error!("Failed to load test data: {}", e);
                return;
            }
        };

    let data_source = MockData::new(&tx_json, &lo_json, &lh_json, &l_json, &nft_json);
    match vm_wamr::run_func(args.wasm_file, "finish", Some(1000000), data_source) {
        Ok(result) => {
            println!("-------------------------------------------------");
            println!("| WASM FUNCTION EXECUTION RESULT                |");
            println!("-------------------------------------------------");
            println!("| Result:     {:<33} |", result);
            println!("-------------------------------------------------");
            info!("Function completed successfully with result: {}", result);
        }
        Err(e) => {
            println!("-------------------------------------------------");
            println!("| WASM FUNCTION EXECUTION ERROR                 |");
            println!("-------------------------------------------------");
            println!("| Error:      {:<33} |", e);
            println!("-------------------------------------------------");
            error!("Function execution failed: {}", e);
        }
    }

    info!("Wasm host application execution completed");
}
