import json
import webbrowser
from typing import Dict, Any

def read_json_file(file_path: str) -> Dict[str, Any]:
    with open(file_path, "r") as file:
        return json.load(file)

blink_links = {
    "superteam": "https://dial.to/developer?url=http://localhost:3001/api/action&cluster=mainnet",
    "superteamtr": "https://dial.to/developer?url=http://localhost:3001/api/action&cluster=mainnet",
    "earn": "https://dial.to/developer?url=http://localhost:3001/api/action&cluster=mainnet",
    "zk-lokomotive": "https://dial.to/developer?url=http://localhost:3000/api/action&cluster=mainnet",
    "zk": "https://dial.to/developer?url=http://localhost:3000/api/action&cluster=mainnet",
    "lokomotive": "https://dial.to/developer?url=http://localhost:3000/api/action&cluster=mainnet",
    "solana": "https://dial.to/developer?url=http://localhost:3002/api/action&cluster=mainnet",
    "ethereum": "https://dial.to/developer?url=http://localhost:3003/api/action&cluster=mainnet",
    "bitcoin": "https://dial.to/developer?url=http://localhost:3004/api/action&cluster=mainnet",
    "polkadot": "https://dial.to/developer?url=http://localhost:3005/api/action&cluster=mainnet",
    "cardano": "https://dial.to/developer?url=http://localhost:3006/api/action&cluster=mainnet",
    "defi": "https://dial.to/developer?url=http://localhost:3007/api/action&cluster=mainnet",
    "nft": "https://dial.to/developer?url=http://localhost:3008/api/action&cluster=mainnet",
    "dao": "https://dial.to/developer?url=http://localhost:3009/api/action&cluster=mainnet",
}

def check_for_blink(data: Dict[str, Any]) -> str:
    top_words = data.get("top_words", [])
    for word, count in top_words:
        word = word.lower()
        if word in blink_links:
            blink_link = blink_links[word]
            print(f"{word} active: {blink_link}")
            webbrowser.open(blink_link)
            return blink_link
    return "No blink found"

def analyze_blockchain_relevance(data: Dict[str, Any]) -> None:
    top_words = data.get("top_words", [])
    total_count = sum(count for _, count in top_words)
    blockchain_relevance = sum(count for word, count in top_words if word.lower() in blink_links) / total_count if total_count > 0 else 0
    print(f"Blockchain relevance score: {blockchain_relevance:.2%}")

def main() -> None:
    json_file_path = "solfhe.json"
    try:
        json_data = read_json_file(json_file_path)
        blink_status = check_for_blink(json_data)
        print(f"🚨 Matched Blink: {blink_status}")
        analyze_blockchain_relevance(json_data)
        print(f"Total links analyzed: {json_data.get('total_links_analyzed', 0)}")
        print(f"Total unique words: {json_data.get('total_unique_words', 0)}")
    except FileNotFoundError:
        print(f"Error: {json_file_path} not found.")
    except json.JSONDecodeError:
        print(f"Error: Unable to parse {json_file_path}. Invalid JSON format.")
    except Exception as e:
        print(f"An unexpected error occurred: {str(e)}")

if __name__ == "__main__":
    main()