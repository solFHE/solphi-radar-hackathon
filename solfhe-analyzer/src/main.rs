use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    program_error::ProgramError,
    instruction::{AccountMeta, Instruction},
    system_program,
    sysvar::{rent::Rent, Sysvar},
};
use borsh::{BorshDeserialize, BorshSerialize};
use light_sdk::{
    compressed_account::{CompressedAccount, CompressedAccountData},
    merkle_context::MerkleContext,
    proof::CompressedProof,
    constants::PROGRAM_ID_LIGHT_TOKEN,
};
use spl_memo::build_memo;
use serde_json::json;

// Entrypoint tanımı
entrypoint!(process_instruction);

#[derive(BorshSerialize, BorshDeserialize)]
enum SolfheInstruction {
    AnalyzeLinks { links: Vec<String> },
    CompressAndTransfer { data: String },
    RetrieveAndDecompress { signature: [u8; 64] },
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = SolfheInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    match instruction {
        SolfheInstruction::AnalyzeLinks { links } => {
            let result = analyze_links(&links)?;
            save_result_as_memo(accounts, &result)
        },
        SolfheInstruction::CompressAndTransfer { data } => {
            compress_and_transfer(program_id, accounts, &data)
        },
        SolfheInstruction::RetrieveAndDecompress { signature } => {
            retrieve_and_decompress(program_id, accounts, &signature)
        },
    }
}

fn analyze_links(links: &[String]) -> Result<String, ProgramError> {
    let mut word_count = std::collections::HashMap::new();
    for link in links {
        let words = extract_keywords(link);
        for word in words {
            *word_count.entry(word).or_insert(0) += 1;
        }
    }

    let mut top_words: Vec<_> = word_count.into_iter().collect();
    top_words.sort_by(|a, b| b.1.cmp(&a.1));
    top_words.truncate(3);

    let result = json!({
        "top_words": top_words,
        "total_links_analyzed": links.len(),
        "total_unique_words": word_count.len()
    });

    Ok(result.to_string())
}

fn save_result_as_memo(accounts: &[AccountInfo], result: &str) -> ProgramResult {
    let memo_program_id = spl_memo::id();
    let memo_instruction = spl_memo::build_memo(result.as_bytes(), &[&accounts[0].key]);
    
    solana_program::program::invoke(
        &memo_instruction,
        accounts,
    )
}

fn compress_and_transfer(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &str,
) -> ProgramResult {
    let compressed_account = CompressedAccount {
        owner: *program_id,
        lamports: 0,
        address: None,
        data: Some(CompressedAccountData {
            discriminator: [0; 8],
            data: data.as_bytes().to_vec(),
            data_hash: hash_data(data),
        }),
    };

    let merkle_tree_pubkey = Pubkey::new_unique();
    let merkle_context = MerkleContext {
        merkle_tree_pubkey,
        nullifier_queue_pubkey: Pubkey::new_unique(),
        leaf_index: 0,
        queue_index: None,
    };

    let instruction = create_invoke_instruction(
        accounts[0].key,
        accounts[1].key,
        &[],
        &[compressed_account],
        &[merkle_context],
        &[merkle_tree_pubkey],
        &[0],
        &[],
        None,
        None,
        false,
        None,
        true,
    );

    solana_program::program::invoke(&instruction, accounts)?;
    
    msg!("Data compressed and transferred successfully");
    Ok(())
}

fn retrieve_and_decompress(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _signature: &[u8; 64],
) -> ProgramResult {
    let compressed_account = CompressedAccount::try_from_slice(&accounts[2].data.borrow())
        .map_err(|_| ProgramError::InvalidAccountData)?;

    if let Some(data) = compressed_account.data {
        let decompressed_data = String::from_utf8(data.data)
            .map_err(|_| ProgramError::InvalidAccountData)?;
        msg!("Decompressed data: {}", decompressed_data);
        save_result_as_memo(accounts, &decompressed_data)?;
    } else {
        msg!("No data found in the compressed account");
    }

    Ok(())
}

fn create_invoke_instruction(
    payer: &Pubkey,
    authority: &Pubkey,
    input_compressed_accounts: &[CompressedAccount],
    output_compressed_accounts: &[CompressedAccount],
    merkle_context: &[MerkleContext],
    merkle_tree_pubkeys: &[Pubkey],
    root_indices: &[u16],
    _new_address_params: &[()],
    proof: Option<CompressedProof>,
    compress_or_decompress_lamports: Option<u64>,
    is_compress: bool,
    decompression_recipient: Option<Pubkey>,
    _sort: bool,
) -> Instruction {
    let mut accounts = vec![
        AccountMeta::new(*payer, true),
        AccountMeta::new_readonly(*authority, true),
    ];

    for pubkey in merkle_tree_pubkeys {
        accounts.push(AccountMeta::new(*pubkey, false));
    }

    let mut data = Vec::new();
    data.extend_from_slice(&(input_compressed_accounts.len() as u32).to_le_bytes());
    data.extend_from_slice(&(output_compressed_accounts.len() as u32).to_le_bytes());
    data.extend_from_slice(&(merkle_context.len() as u32).to_le_bytes());
    data.extend_from_slice(&(root_indices.len() as u32).to_le_bytes());
    
    if let Some(proof) = proof {
        data.push(1);
        data.extend_from_slice(&proof.a);
        data.extend_from_slice(&proof.b);
        data.extend_from_slice(&proof.c);
    } else {
        data.push(0);
    }

    if let Some(lamports) = compress_or_decompress_lamports {
        data.push(1);
        data.extend_from_slice(&lamports.to_le_bytes());
    } else {
        data.push(0);
    }

    data.push(is_compress as u8);

    if let Some(recipient) = decompression_recipient {
        data.push(1);
        data.extend_from_slice(&recipient.to_bytes());
    } else {
        data.push(0);
    }

    Instruction {
        program_id: PROGRAM_ID_LIGHT_TOKEN,
        accounts,
        data,
    }
}

fn extract_keywords(url: &str) -> Vec<String> {
    url.split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty() && s.len() > 3)
        .map(|s| s.to_lowercase())
        .collect()
}

fn hash_data(data: &str) -> [u8; 32] {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

// Off-chain istemci kodu
#[cfg(not(target_os = "solana"))]
pub mod client {
    use solana_client::rpc_client::RpcClient;
    use solana_sdk::{
        signature::{Keypair, Signer},
        transaction::Transaction,
        instruction::Instruction,
    };
    use super::*;

    pub async fn run_solfhe_analyzer(client: &RpcClient, payer: &Keypair) -> Result<(), Box<dyn std::error::Error>> {
        let links = extract_links_from_chrome()?;
        
        let instruction_data = SolfheInstruction::AnalyzeLinks { links }.try_to_vec()?;
        let instruction = Instruction::new_with_borsh(
            *program_id(),
            &instruction_data,
            vec![AccountMeta::new(payer.pubkey(), true)],
        );

        let recent_blockhash = client.get_latest_blockhash()?;
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
            &[payer],
            recent_blockhash,
        );

        let signature = client.send_and_confirm_transaction(&transaction)?;
        println!("Transaction sent: {}", signature);

        let transaction_data = client.get_transaction(&signature, UiTransactionEncoding::Json)?;
        if let Some(meta) = transaction_data.transaction.meta {
            if let Some(log_messages) = meta.log_messages {
                for log in log_messages {
                    if log.starts_with("Program log: Memo") {
                        println!("Analysis result: {}", log);
                        break;
                    }
                }
            }
        }

        Ok(())
    }

    fn extract_links_from_chrome() -> Result<Vec<String>, Box<dyn std::error::Error>> {
        // Chrome geçmişinden linkleri çıkarma işlemi
        // Bu fonksiyonun implementasyonu, önceki off-chain versiyonunuzla aynı olabilir
        unimplemented!()
    }

    fn program_id() -> Pubkey {
        // Program ID'nizi buraya ekleyin
        Pubkey::new_unique()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_links() {
        let links = vec![
            "https://example.com/test1".to_string(),
            "https://example.com/test2".to_string(),
            "https://another.com/page".to_string(),
        ];

        let result = analyze_links(&links).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

        assert_eq!(parsed["total_links_analyzed"], 3);
        assert!(parsed["top_words"].as_array().unwrap().len() > 0);
    }

    #[test]
    fn test_extract_keywords() {
        let url = "https://www.example.com/page?param=value";
        let keywords = extract_keywords(url);
        assert!(keywords.contains(&"example".to_string()));
        assert!(keywords.contains(&"page".to_string()));
        assert!(!keywords.contains(&"www".to_string()));  // 3 karakterden kısa
        assert!(!keywords.contains(&"com".to_string()));  // 3 karakterden kısa
    }
}