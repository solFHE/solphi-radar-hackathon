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

fn get_chrome_history_path() -> PathBuf {
  let home = dirs::home_dir().expect("Unable to find home directory");
  if cfg!(target_os = "windows") {
      home.join(r"AppData\Local\Google\Chrome\User Data\Default\History")
  } else if cfg!(target_os = "macos") {
      home.join("Library/Application Support/Google/Chrome/Default/History")
  } else {
      home.join(".config/google-chrome/Default/History")
  }
}

fn extract_links_from_chrome() -> Result<Vec<String>, Box<dyn std::error::Error>> {
  let history_path = get_chrome_history_path();
  let temp_path = history_path.with_extension("tmp");

  fs::copy(&history_path, &temp_path)?;

  let conn = Connection::open(&temp_path)?;
  let mut stmt = conn.prepare("SELECT url FROM urls ORDER BY last_visit_time DESC LIMIT 5")?;
  
  let urls: Vec<String> = stmt.query_map([], |row| row.get(0))?
      .filter_map(Result::ok)
      .collect();

  fs::remove_file(temp_path)?;

  Ok(urls)
}

fn extract_keywords_from_url(url: &str) -> Vec<String> {
  let ignored_words: HashSet<_> = IGNORED_WORDS.iter().map(|&s| s.to_string()).collect();
  
  if let Ok(parsed_url) = Url::parse(url) {
      let domain = parsed_url.domain().unwrap_or("");
      let path = parsed_url.path();
      
      domain.split('.')
          .chain(path.split('/'))
          .filter_map(|segment| {
              let lowercase_segment = segment.to_lowercase();
              if segment.is_empty() || ignored_words.contains(&lowercase_segment) {
                  None
              } else {
                  Some(lowercase_segment)
              }
          })
          .collect()
  } else {
      Vec::new()
  }
}

fn analyze_link(link: &str, word_counter: &mut HashMap<String, u32>) {
  let keywords = extract_keywords_from_url(link);

  for word in keywords {
      if BLOCKCHAIN_NETWORKS.contains(&word.as_str()) || word.len() > 3 {
          *word_counter.entry(word).or_insert(0) += 1;
      }
  }
}

fn get_most_common_word(word_counter: &HashMap<String, u32>) -> Option<(String, u32)> {
  word_counter.iter()
      .max_by_key(|&(_, count)| count)
      .map(|(word, count)| (word.clone(), *count))
}

fn zk_compress(data: &str) -> String {
  let compressed = general_purpose::STANDARD_NO_PAD.encode(data);
  println!("Compressed data: {}", compressed);
  compressed
}

fn zk_decompress(compressed_data: &str) -> Result<String, Box<dyn std::error::Error>> {
  println!("Attempting to decompress: {}", compressed_data);
  let bytes = general_purpose::STANDARD_NO_PAD.decode(compressed_data.trim_matches('"'))?;
  let decompressed = String::from_utf8(bytes)?;
  println!("Decompressed data: {}", decompressed);
  Ok(decompressed)
}

fn create_solana_account() -> Keypair {
  Keypair::new()
}

fn airdrop_sol(client: &RpcClient, pubkey: &Pubkey, amount: u64) -> Result<(), Box<dyn std::error::Error>> {
  let sig = client.request_airdrop(pubkey, amount)?;
  client.confirm_transaction(&sig)?;
  println!("✈️ Airdrop request sent for {} lamports", amount);
  
  std::thread::sleep(Duration::from_secs(5));
  
  let balance = client.get_balance(pubkey)?;
  println!("Current balance after airdrop: {} lamports", balance);
  
  if balance == 0 {
      return Err("Airdrop failed: Balance is still 0".into());
  }
  
  Ok(())
}

fn ensure_minimum_balance(client: &RpcClient, pubkey: &Pubkey, minimum_balance: u64) -> Result<(), Box<dyn std::error::Error>> {
  let mut attempts = 0;
  while attempts < 3 {
      let balance = client.get_balance(pubkey)?;
      if balance >= minimum_balance {
          println!("Sufficient balance: {} lamports", balance);
          return Ok(());
      }
      
      println!("Insufficient balance: {} lamports. Attempting airdrop...", balance);
      if let Err(e) = airdrop_sol(client, pubkey, minimum_balance - balance) {
          println!("Airdrop attempt failed: {}. Retrying...", e);
      }
      
      attempts += 1;
      std::thread::sleep(Duration::from_secs(5));
  }
  
  Err("Failed to ensure minimum balance after multiple attempts".into())
}