use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::{Manager, Runtime};
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use rusqlite::Connection;
use url::Url;
use sha2::{Sha256, Digest};
use base64::{Engine as _, engine::general_purpose};
use solana_sdk::{
    signature::{Keypair, Signer, Signature},
    transaction::Transaction,
    system_instruction,
    pubkey::Pubkey,
};
use solana_client::rpc_client::RpcClient;
use solana_transaction_status::UiTransactionEncoding;
use spl_memo;
use std::fs::File;
use std::io::Write;
use std::process::Command;

const BLOCKCHAIN_NETWORKS: [&str; 20] = [
    "bitcoin", "ethereum", "scroll", "polkadot", "solana", "zk-lokomotive", "cosmos",
    "algorand", "mina", "chainlink", "superteam", "aave", "compound", "maker",
    "polygon", "binance", "tron", "wormhole", "stellar", "filecoin"
];

const IGNORED_WORDS: [&str; 18] = [
    "http", "https", "www", "com", "org", "net", "search", "google", "?", "q", "=", "xyz", "&", "%", "#", "oq", "://", ":UTF-8"
];

#[derive(Clone, Serialize, Deserialize)]
struct AnalysisResult {
    most_common_word: String,
    count: u32,
    compressed_data: String,
    transaction_signature: Option<String>,
}

#[derive(Clone, Serialize)]
struct ProgressPayload {
    message: String,
    percentage: f32,
}

struct AnalysisState {
    is_running: bool,
    results: Vec<AnalysisResult>,
    client: RpcClient,
    account1: Keypair,
    account2: Keypair,
}

#[tauri::command]
async fn start_analysis<R: Runtime>(
    window: tauri::Window<R>,
    state: tauri::State<'_, Arc<Mutex<AnalysisState>>>,
) -> Result<(), String> {
    let mut analysis_state = state.lock().await;
    analysis_state.is_running = true;
    
    let steps = [
        "Extracting Chrome history",
        "Analyzing keywords",
        "Applying ZK compression",
        "Interacting with Solana blockchain",
        "Storing results",
        "Running Python script",
    ];
    
    for (i, step) in steps.iter().enumerate() {
        if !analysis_state.is_running {
            break;
        }
        
        let progress = ProgressPayload {
            message: step.to_string(),
            percentage: (i as f32 + 1.0) / steps.len() as f32,
        };
        
        window.emit("analysis_progress", &progress).map_err(|e| e.to_string())?;
        
        match i {
            0 => {
                let urls = extract_links_from_chrome().map_err(|e| e.to_string())?;
                analysis_state.results = Vec::new();
                for url in urls {
                    let mut word_counter = HashMap::new();
                    analyze_link(&url, &mut word_counter);
                    if let Some((word, count)) = get_most_common_word(&word_counter) {
                        analysis_state.results.push(AnalysisResult {
                            most_common_word: word,
                            count,
                            compressed_data: String::new(),
                            transaction_signature: None,
                        });
                    }
                }
            },
            1 => {
                // Keyword analysis is done in step 0
            },
            2 => {
                for result in analysis_state.results.iter_mut() {
                    let json_string = serde_json::to_string(&result).map_err(|e| e.to_string())?;
                    result.compressed_data = zk_compress(&json_string);
                }
            },
            3 => {
                for result in analysis_state.results.iter_mut() {
                    match transfer_compressed_hash(
                        &analysis_state.client,
                        &analysis_state.account1,
                        &analysis_state.account2.pubkey(),
                        &result.compressed_data,
                    ) {
                        Ok(signature) => {
                            result.transaction_signature = Some(signature.to_string());
                        },
                        Err(e) => println!("Error during hash transfer: {}", e),
                    }
                }
            },
            4 => {
                save_results_to_file(&analysis_state.results).map_err(|e| e.to_string())?;
            },
            5 => {
                run_python_script().map_err(|e| e.to_string())?;
            },
            _ => {}
        }
        
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    
    analysis_state.is_running = false;
    Ok(())
}

#[tauri::command]
async fn stop_analysis(state: tauri::State<'_, Arc<Mutex<AnalysisState>>>) -> Result<(), String> {
    let mut analysis_state = state.lock().await;
    analysis_state.is_running = false;
    Ok(())
}

#[tauri::command]
async fn get_results(state: tauri::State<'_, Arc<Mutex<AnalysisState>>>) -> Result<Vec<AnalysisResult>, String> {
    let analysis_state = state.lock().await;
    Ok(analysis_state.results.clone())
}