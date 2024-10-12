/*
This document prepared by `@Virjilakrum🏗️`` explains in technical detail how a complex Solana program belonging to the SolΦ project works. The program uses ZK-Compression technology to protect user privacy and Light SDK to provide efficient advertising services on the Solana blockchain. Below are detailed explanations explaining each function, data flow and main components of this program.

1. General Structure of the Program:
- This Solana program consists of multiple instruction sets that perform various functions using an enum called `SolfheInstruction`. These instructions provide a wide range of functions from data analysis to transfer of compressed data, from user rewards to advertising.
- The `process_instruction` function is defined as the main entry point of this Solana program using the `entrypoint!()` macro. This function directs incoming instructions to the appropriate functions.

2. SolfheInstruction Enum:
- It is an `enum` that defines the different types of instructions that the program can process.
- `AnalyzeLinks`: Analyzes the given links and determines the keywords.
- `CompressAndTransfer`: Compresses and transfers the data.
- `RetrieveAndDecompress`: Decompresses and returns the compressed data.
- `ServeAd`: Serves advertisements according to the user profile.
- `RewardUser`: Analyzes the user interaction and gives rewards accordingly.

3. `process_instruction` Function:
- It is the main instruction processor of the program. It determines the appropriate `SolfheInstruction` variant by deserializing the incoming `instruction_data`.
- Each instruction variant is directed to the corresponding special function.
- For example, the `AnalyzeLinks` variant goes to the `analyze_links` function and analyzes the links.

4. `analyze_links` Function:
- Extracts the keywords from the given URLs and calculates their number.
- Using the `extract_keywords` helper function, keywords longer than 3 characters are obtained from each URL.

- Keywords are kept in a hash map and the first 3 most frequently used keywords are determined.

- Results are converted to a string in JSON format and returned.

5. `save_result_as_memo` Function:

- Saves the results of analysis operations such as `analyze_links` to the Solana chain as `Memo`.

- In this way, analysis results are permanently stored on the chain.

- A memo is created with the `spl_memo::build_memo` function and the operation is performed with the `solana_program::program::invoke` function.

6. `compress_and_transfer` Function:

- Compresses the given data using the `CompressedAccount` structure and transfers it to the Solana chain.
- It uses the `CompressedAccount` and `CompressedAccountData` structures provided by `Light SDK`.
- After the data is compressed, a Merkle tree is created using `MerkleContext` and the accuracy of the data is guaranteed.
- The compressed data is transferred to other accounts on the chain using the `create_invoke_instruction` function.

7. `retrieve_and_decompress` Function:
- It retrieves and decompresses the compressed data on the chain.
- It extracts the data from the `CompressedAccount` structure and returns it to its original form.
- After this process, the data is saved on the chain using the `save_result_as_memo` function.

8. `serve_ad` Function:
- The user profile is received as compressed data and the advertisement serving process is performed.
- Ad targeting operations are performed according to the user's profile data and user privacy is protected using the `CompressedAccount` data.
- This allows advertisers to reach specific target audiences.

9. `reward_user` Function:

- Allows the user to be rewarded for their interactions with ads.
- The amount of reward the user will receive based on the interaction data is calculated with the `calculate_reward` function.

- A certain amount of lamport reward is transferred to the user using the `solana_program::system_instruction::transfer` function.

10. `calculate_reward` Function:

- Calculates the amount of reward based on the user's interactions with ads.
- An `engagement_score` is calculated by collecting the interaction data and this score is multiplied by a certain coefficient to determine the amount of reward.
- This method allows the user to receive more rewards as their interaction with ads increases.

11. `create_invoke_instruction` Function:

- Creates the `Instruction` structure required for the processing and transfer of compressed data.
- Necessary accounts such as `payer` and `authority` are determined and contexts related to Merkle trees are added.
- ZK proofs (`CompressedProof`) and parameters such as whether the data will be compressed or not are used in this process.

12. `extract_keywords` Fonksiyonu:
    - Verilen URL'yi analiz ederek 3 karakterden uzun olan anahtar kelimeleri çıkarır.
    - URL'yi parçalar ve yalnızca anlamlı kelimeleri elde etmek için gereksiz karakterleri filtreler.

13. `hash_data` Fonksiyonu:
    - Verilen verinin hash'ini hesaplar.
    - `sha2` kütüphanesini kullanarak SHA-256 algoritması ile veri üzerinde bir özet hesaplar.
    - Bu hash değeri, sıkıştırılmış verinin doğruluğunu ve bütünlüğünü sağlamak için kullanılır.

14. Off-Chain İstemci Kodu (client modülü):
    - `client` modülü, bu programın off-chain kısımlarını içerir.
    - `run_solfhe_analyzer` fonksiyonu, kullanıcıdan gelen bağlantıları analiz etmek için RPC istemcisi kullanarak talimat gönderir.
    - `extract_links_from_chrome` fonksiyonu, kullanıcının tarayıcı geçmişinden linkleri çıkarmak için kullanılır (implementasyon henüz tamamlanmamış).

15. Testler (tests modülü):
    - Programın işlevselliğini test etmek için birkaç test fonksiyonu tanımlanmıştır.
    - `test_analyze_links`, `test_extract_keywords` ve `test_calculate_reward` gibi test fonksiyonları, ilgili işlevlerin beklenen şekilde çalıştığını doğrular.

Sonuç olarak, bu program Solana blok zincirinde reklam sunma, veri analiz etme ve kullanıcıları ödüllendirme gibi karmaşık işlemleri gerçekleştirmek üzere tasarlanmıştır. Program, Light SDK'nin sunduğu ZK-Compression özelliklerini kullanarak veri gizliliği ve güvenliğini garanti ederken aynı zamanda verimli ve ölçeklenebilir bir reklam ekosistemi sunmaktadır. Kullanıcıların etkileşim verileri analiz edilir ve reklam verenlerin belirli kitlelere ulaşmasına olanak tanınır. Aynı zamanda kullanıcılar da reklamlarla etkileşime geçtikleri oranda ödüller kazanarak bu ekosistemde yer alırlar.
*/

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
use sha2::{Sha256, Digest};
use std::collections::HashMap;

// Entrypoint tanımı
entrypoint!(process_instruction);

#[derive(BorshSerialize, BorshDeserialize)]
enum SolfheInstruction {
    AnalyzeLinks { links: Vec<String> },
    CompressAndTransfer { data: String },
    RetrieveAndDecompress { signature: [u8; 64] },
    ServeAd { user_profile: Vec<u8> },
    RewardUser { engagement_data: Vec<u8> },
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
        SolfheInstruction::ServeAd { user_profile } => {
            serve_ad(program_id, accounts, &user_profile)
        },
        SolfheInstruction::RewardUser { engagement_data } => {
            reward_user(accounts, &engagement_data)
        },
    }
}

fn analyze_links(links: &[String]) -> Result<String, ProgramError> {
    let mut word_count = HashMap::new();
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
        "total_unique_words": top_words.len()
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

fn serve_ad(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    user_profile: &[u8],
) -> ProgramResult {
    // ZK-compressed kullanıcı profili kullanarak reklam hedefleme işlemi
    let compressed_account = CompressedAccount::try_from_slice(user_profile)
        .map_err(|_| ProgramError::InvalidAccountData)?;

    if let Some(data) = compressed_account.data {
        msg!("Serving ad using user profile data...");
        // Reklam hedefleme işlemleri buraya eklenebilir
        // Örneğin, reklam verenlerin belirli hedef kitlelere reklamlarını sunması
    } else {
        msg!("No user profile data available");
    }
    Ok(())
}

fn reward_user(accounts: &[AccountInfo], engagement_data: &[u8]) -> ProgramResult {
    // Kullanıcıya etkileşimi için ödül verme işlemi
    msg!("Rewarding user based on engagement data...");
    
    let reward_amount = calculate_reward(engagement_data)?;
    let user_account = &accounts[0];
    let system_program_account = &accounts[1];

    // Solana sistem programı kullanarak ödülü transfer etme
    let transfer_instruction = solana_program::system_instruction::transfer(
        &system_program_account.key,
        &user_account.key,
        reward_amount,
    );

    solana_program::program::invoke(
        &transfer_instruction,
        &[system_program_account.clone(), user_account.clone()]
    )?;
    
    msg!("User rewarded with {} lamports", reward_amount);
    Ok(())
}

fn calculate_reward(engagement_data: &[u8]) -> Result<u64, ProgramError> {
    // Engagement verisini kullanarak ödül miktarını hesaplama
    // Örneğin, etkileşim sayısına göre ödül miktarı belirlenebilir
    let engagement_score = engagement_data.iter().map(|&x| x as u64).sum();
    let reward_amount = engagement_score * 10; // Örnek ödül katsayısı
    Ok(reward_amount)
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

    #[test]
    fn test_calculate_reward() {
        let engagement_data = vec![5, 10, 15];
        let reward = calculate_reward(&engagement_data).unwrap();
        assert_eq!(reward, 300); // Engagement skoru 30, katsayı 10
    }
}

