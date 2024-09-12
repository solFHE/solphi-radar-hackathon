import json
import webbrowser

def read_json_file(file_path):
    with open(file_path, "r") as file:
        return json.load(file)

blink_links = {
    "superteam": "https://dial.to/developer?url=http://localhost:3001/api/action&cluster=mainnet",  # Superteam linki
    "superteamtr": "https://dial.to/developer?url=http://localhost:3001/api/action&cluster=mainnet",  # Superteamtr linki
    "earn": "https://dial.to/developer?url=http://localhost:3001/api/action&cluster=mainnet",  # Earn linki
    "zk-lokomotive": "https://dial.to/developer?url=http://localhost:3000/api/action&cluster=mainnet",  # zk-Lokomotive linki
    "zk": "https://dial.to/developer?url=http://localhost:3000/api/action&cluster=mainnet",  # zk linki
    "lokomotive": "https://dial.to/developer?url=http://localhost:3000/api/action&cluster=mainnet"  # zk linki


}

def check_for_blink(data):
    most_common_word = data.get("most_common_word", "").lower()
    if most_common_word in blink_links:
        blink_link = blink_links[most_common_word]
        print(f"{most_common_word} aktif: {blink_link}")
        webbrowser.open(blink_link)
        return blink_link
    return "No blink found"

json_file_path = "solfhe.json"
json_data = read_json_file(json_file_path)
blink_status = check_for_blink(json_data)
print(f"🚨 Matched Blink: {blink_status}")