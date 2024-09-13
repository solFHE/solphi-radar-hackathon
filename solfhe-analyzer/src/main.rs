// 🏗️ Developed by: Baturalp Güvenç 

/* Gerekli kütüphaneleri kullanıyoruz: rusqlite (SQLite işlemleri için), url (URL ayrıştırma için), serde_json (JSON işlemleri için) ve Rust standart kütüphanesinden çeşitli modüller.
HistoryAnalyzer adında bir struct tanımlıyoruz. Bu struct, linkleri ve kelime sayımlarını tutar.
get_chrome_history_path fonksiyonu, farklı işletim sistemleri için Chrome geçmiş dosyasının konumunu belirler.
extract_links_from_chrome metodu, Chrome'un geçmiş veritabanından son 5 URL'yi çeker.
analyze_link metodu, her bir linki ayrıştırır ve içindeki anlamlı kelimeleri (özellikle blockchain ağı isimlerini) sayar.
get_most_common_word ve to_json metotları, en sık kullanılan kelimeyi bulur ve JSON formatında çıktı üretir.
run metodu, sürekli çalışan bir döngü içinde her 60 saniyede bir yeni linkleri kontrol eder. */


use eframe::egui;
use eframe::epi;
use std::sync::{Arc, Mutex};
use std::thread;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use serde_json::{json, Value};
use rusqlite::Connection;
use solana_transaction_status::option_serializer::OptionSerializer;
use url::Url;
use sha2::Digest;
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

struct SolfheAnalyzer {
    client: RpcClient,
    account1: Keypair,
    account2: Keypair,
    links: Vec<String>,
    word_counter: HashMap<String, u32>,
    is_analyzing: bool,
    analysis_result: String,
    last_transaction_signature: Option<Signature>,
}

impl SolfheAnalyzer {
    fn new() -> Self {
        let client = RpcClient::new("http://localhost:8899".to_string());
        let account1 = create_solana_account();
        let account2 = create_solana_account();

        println!("Account 1 public key: {}", account1.pubkey());
        println!("Account 2 public key: {}", account2.pubkey());

        SolfheAnalyzer {
            client,
            account1,
            account2,
            links: Vec::new(),
            word_counter: HashMap::new(),
            is_analyzing: false,
            analysis_result: String::new(),
            last_transaction_signature: None,
        }
    }

    fn start_analysis(&mut self) {
        self.is_analyzing = true;
        self.analysis_result.clear();
        self.links.clear();
        self.word_counter.clear();
    }

    fn stop_analysis(&mut self) {
        self.is_analyzing = false;
    }

    fn analyze_step(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        match extract_links_from_chrome() {
            Ok(urls) if !urls.is_empty() => {
                for url in urls {
                    if !self.links.contains(&url) {
                        self.links.push(url.clone());
                        analyze_link(&url, &mut self.word_counter);
                        self.analysis_result.push_str(&format!("Analyzed new link: {}\n", url));

                        if self.links.len() >= 5 {
                            let result = if let Some((word, count)) = get_most_common_word(&self.word_counter) {
                                json!({
                                    "most_common_word": word,
                                    "count": count
                                })
                            } else {
                                json!({"error": "No words analyzed yet"})
                            };

                            let json_string = result.to_string();
                            let compressed_result = zk_compress(&json_string);
                            self.analysis_result.push_str(&format!("\nSolfhe Result (ZK compressed):\n{}\n", compressed_result));

                            match transfer_compressed_hash(&self.client, &self.account1, &self.account2.pubkey(), &compressed_result, &result) {
                                Ok(signature) => {
                                    self.last_transaction_signature = Some(signature);
                                    self.analysis_result.push_str(&format!("Successfully transferred hash. Signature: {}\n", signature));
                                    match retrieve_and_decompress_hash(&self.client, &signature) {
                                        Ok(decompressed_json) => {
                                            self.analysis_result.push_str(&format!("Retrieved and decompressed JSON data:\n{}\n", serde_json::to_string_pretty(&decompressed_json)?));
                                            
                                            if let Err(e) = save_json_to_file(&decompressed_json, "solfhe.json") {
                                                self.analysis_result.push_str(&format!("Error saving JSON to file: {}\n", e));
                                            }

                                            match Command::new("python3")
                                                .arg("blink-matcher.py")
                                                .status() {
                                                Ok(status) => self.analysis_result.push_str(&format!("Python script executed with status: {}\n", status)),
                                                Err(e) => self.analysis_result.push_str(&format!("Failed to execute Python script: {}\n", e)),
                                            }
                                        },
                                        Err(e) => self.analysis_result.push_str(&format!("Error retrieving and decompressing hash: {}\n", e)),
                                    }
                                },
                                Err(e) => self.analysis_result.push_str(&format!("Error during hash transfer: {}\n", e)),
                            }

                            self.links.clear();
                            self.word_counter.clear();
                        }
                    }
                }
            },
            Ok(_) => self.analysis_result.push_str("No new links found\n"),
            Err(e) => self.analysis_result.push_str(&format!("Error extracting links from Chrome: {}\n", e)),
        }

        Ok(())
    }
}

impl eframe::App for SolfheAnalyzer {
    fn update(&mut self, ctx: &egui::Context, _frame: &eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Solfhe Analyzer");
            
            if ui.button(if self.is_analyzing { "Stop Analysis" } else { "Start Analysis" }).clicked() {
                if self.is_analyzing {
                    self.stop_analysis();
                } else {
                    self.start_analysis();
                }
            }
            
            ui.label(format!("Analysis status: {}", if self.is_analyzing { "Running" } else { "Stopped" }));
            
            if let Some(signature) = self.last_transaction_signature {
                if ui.button("Open Last Transaction in Explorer").clicked() {
                    if let Err(e) = open::that(format!("https://explorer.solana.com/tx/{}?cluster=custom", signature)) {
                        eprintln!("Failed to open URL: {}", e);
                    }
                }
            }
            
            ui.label("Analysis Result:");
            ui.text_edit_multiline(&mut self.analysis_result);
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let app = SolfheAnalyzer::new();
    let app = Arc::new(Mutex::new(app));
    
    let app_clone = Arc::clone(&app);
    thread::spawn(move || {
        loop {
            {
                let mut app = app_clone.lock().unwrap();
                if app.is_analyzing {
                    if let Err(e) = app.analyze_step() {
                        eprintln!("Error during analysis: {}", e);
                    }
                }
            }
            thread::sleep(Duration::from_secs(10));
        }
    });

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Solfhe Analyzer",
        native_options,
        Box::new(|_cc| Box::new(SolfheAnalyzer::new()))
    )
}

// Diğer yardımcı fonksiyonlar buraya eklenir...

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
    compressed
}

fn zk_decompress(compressed_data: &str) -> Result<String, Box<dyn std::error::Error>> {
    let bytes = general_purpose::STANDARD_NO_PAD.decode(compressed_data.trim_matches('"'))?;
    let decompressed = String::from_utf8(bytes)?;
    Ok(decompressed)
}

fn create_solana_account() -> Keypair {
    Keypair::new()
}

fn airdrop_sol(client: &RpcClient, pubkey: &Pubkey, amount: u64) -> Result<(), Box<dyn std::error::Error>> {
    let sig = client.request_airdrop(pubkey, amount)?;
    client.confirm_transaction(&sig)?;
    
    thread::sleep(Duration::from_secs(5));
    
    let balance = client.get_balance(pubkey)?;
    
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
            return Ok(());
        }
        
        if let Err(e) = airdrop_sol(client, pubkey, minimum_balance - balance) {
            eprintln!("Airdrop attempt failed: {}. Retrying...", e);
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

    Ok(signature)
}

fn retrieve_and_decompress_hash(client: &RpcClient, signature: &Signature) -> Result<Value, Box<dyn std::error::Error>> {
    let transaction = client.get_transaction(signature, UiTransactionEncoding::Json)?;
    
    if let Some(meta) = transaction.transaction.meta {
        if let OptionSerializer::Some(log_messages) = meta.log_messages {
            for log in log_messages {
                if log.starts_with("Program log: Memo") {
                    if let Some(start_index) = log.find("): ") {
                        let compressed_hash = &log[start_index + 3..];
                        match zk_decompress(compressed_hash) {
                            Ok(decompressed_hash) => {
                                match serde_json::from_str(&decompressed_hash) {
                                    Ok(json_data) => {
                                        return Ok(json_data);
                                    },
                                    Err(e) => return Err(format!("Error parsing JSON: {}. Raw data: {}", e, decompressed_hash).into()),
                                }
                            },
                            Err(e) => return Err(format!("Error decompressing: {}. Raw data: {}", e, compressed_hash).into()),
                        }
                    }
                }
            }
        }
    }

    Err("Could not find or process memo in transaction logs".into())
}

fn save_json_to_file(json_data: &Value, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(filename)?;
    let json_string = serde_json::to_string_pretty(json_data)?;
    file.write_all(json_string.as_bytes())?;
    Ok(())
}