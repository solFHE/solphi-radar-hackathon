// 🏗️ Developed by: Baturalp Güvenç 

/* Gerekli kütüphaneleri kullanıyoruz: rusqlite (SQLite işlemleri için), url (URL ayrıştırma için), serde_json (JSON işlemleri için) ve Rust standart kütüphanesinden çeşitli modüller.
HistoryAnalyzer adında bir struct tanımlıyoruz. Bu struct, linkleri ve kelime sayımlarını tutar.
get_chrome_history_path fonksiyonu, farklı işletim sistemleri için Chrome geçmiş dosyasının konumunu belirler.
extract_links_from_chrome metodu, Chrome'un geçmiş veritabanından son 5 URL'yi çeker.
analyze_link metodu, her bir linki ayrıştırır ve içindeki anlamlı kelimeleri (özellikle blockchain ağı isimlerini) sayar.
get_most_common_word ve to_json metotları, en sık kullanılan kelimeyi bulur ve JSON formatında çıktı üretir.
run metodu, sürekli çalışan bir döngü içinde her 60 saniyede bir yeni linkleri kontrol eder. */




use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use serde_json::{json, Value};
use rusqlite::Connection;
use solana_transaction_status::option_serializer::OptionSerializer;
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
    
    thread::sleep(Duration::from_secs(5));
    
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
        thread::sleep(Duration::from_secs(5));
    }
    
    Err("Failed to ensure minimum balance after multiple attempts".into())
}

fn transfer_compressed_hash(
    client: &RpcClient,
    payer: &Keypair,
    to: &Pubkey,
    compressed_hash: &str,
    original_json: &Value,
) -> Result<Signature, Box<dyn std::error::Error>> {
    ensure_minimum_balance(client, &payer.pubkey(), 1_000_000_000)?; // Ensure 1 SOL minimum

    let rent = client.get_minimum_balance_for_rent_exemption(0)?;
    let transfer_amount = rent + 1000; // Transfer rent + 1000 lamports

    let transfer_ix = system_instruction::transfer(&payer.pubkey(), to, transfer_amount);
    let memo_ix = spl_memo::build_memo(compressed_hash.as_bytes(), &[&payer.pubkey()]);
    
    let recent_blockhash = client.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &[transfer_ix, memo_ix],
        Some(&payer.pubkey()),
        &[payer],
        recent_blockhash,
    );
    
    let signature = client.send_and_confirm_transaction(&transaction)?;
    println!("🏆 Successfully transferred compressed hash. Transaction signature: {}", signature);
    println!("⛓️✅ Transaction link: https://explorer.solana.com/tx/{}?cluster=custom", signature);

    print_formatted_json(original_json, "Original ");

    Ok(signature)
}

fn retrieve_and_decompress_hash(client: &RpcClient, signature: &Signature) -> Result<Value, Box<dyn std::error::Error>> {
    let transaction = client.get_transaction(signature, UiTransactionEncoding::Json)?;
    
    if let Some(meta) = transaction.transaction.meta {
        if let OptionSerializer::Some(log_messages) = meta.log_messages {
            for log in log_messages {
                println!("Processing log: {}", log);  
                if log.starts_with("Program log: Memo") {
                    if let Some(start_index) = log.find("): ") {
                        let compressed_hash = &log[start_index + 3..];
                        println!("Compressed hash: {}", compressed_hash);  
                        match zk_decompress(compressed_hash) {
                            Ok(decompressed_hash) => {
                                println!("Decompressed hash: {}", decompressed_hash);  
                                match serde_json::from_str(&decompressed_hash) {
                                    Ok(json_data) => {
                                        print_formatted_json(&json_data, "Retrieved ");
                                        return Ok(json_data);
                                    },
                                    Err(e) => println!("Error parsing JSON: {}. Raw data: {}", e, decompressed_hash),  
                                }
                            },
                            Err(e) => println!("Error decompressing: {}. Raw data: {}", e, compressed_hash),  
                        }
                    }
                }
            }
        }
    }

    Err("Could not find or process memo in transaction logs".into())
}

fn print_formatted_json(json_value: &Value, prefix: &str) {
    println!("{}JSON data:", prefix);
    println!("{}{}", prefix, serde_json::to_string_pretty(json_value).unwrap());
}

fn save_json_to_file(json_data: &Value, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(filename)?;
    let json_string = serde_json::to_string_pretty(json_data)?;
    file.write_all(json_string.as_bytes())?;
    println!("JSON data saved to {}", filename);
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Solfhe Analyzer");

    let client = RpcClient::new("http://localhost:8899".to_string());
    
    let account1 = create_solana_account();
    let account2 = create_solana_account();
    
    println!("Account 1 public key: {}", account1.pubkey());
    println!("Account 2 public key: {}", account2.pubkey());
    
    // Ensure minimum balance for account1
    ensure_minimum_balance(&client, &account1.pubkey(), 1_000_000_000)?;
    
    let mut links = Vec::new();
    let mut word_counter = HashMap::new();

    loop {
        match extract_links_from_chrome() {
            Ok(urls) if !urls.is_empty() => {
                for url in urls {
                    if !links.contains(&url) {
                        links.push(url.clone());
                        analyze_link(&url, &mut word_counter);
                        println!("Analyzed new link: {}", url);

                        if links.len() >= 5 {
                            let result = if let Some((word, count)) = get_most_common_word(&word_counter) {
                                json!({
                                    "most_common_word": word,
                                    "count": count
                                })
                            } else {
                                json!({"error": "No words analyzed yet"})
                            };

                            print_formatted_json(&result, "Original ");

                            let json_string = result.to_string();
                            let compressed_result = zk_compress(&json_string);
                            println!("\nSolfhe Result (ZK compressed):");
                            println!("{}", compressed_result);

                            match transfer_compressed_hash(&client, &account1, &account2.pubkey(), &compressed_result, &result) {
                                Ok(signature) => {
                                    println!("Successfully transferred hash");
                                    match retrieve_and_decompress_hash(&client, &signature) {
                                        Ok(decompressed_json) => {
                                            println!("Retrieved and decompressed JSON data:");
                                            println!("{}", serde_json::to_string_pretty(&decompressed_json)?);
                                            
                                            // Save the decompressed JSON to solfhe.json file
                                            if let Err(e) = save_json_to_file(&decompressed_json, "solfhe.json") {
                                                println!("Error saving JSON to file: {}", e);
                                            }

                                            // Execute Python script after saving JSON
                                            match Command::new("python3")
                                                .arg("blink-matcher.py")
                                                .status() {
                                                Ok(status) => println!("Python script executed with status: {}", status),
                                                Err(e) => println!("Failed to execute Python script: {}", e),
                                            }
                                        },
                                        Err(e) => println!("Error retrieving and decompressing hash: {}", e),
                                    }
                                },
                                Err(e) => println!("Error during hash transfer: {}", e),
                            }

                            links.clear();
                            word_counter.clear();
                        }
                    }
                }
            },
            Ok(_) => println!("No new links found"),
            Err(e) => println!("Error extracting links from Chrome: {}", e),
        }
        thread::sleep(Duration::from_secs(10));
    }
}