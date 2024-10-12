import json
import numpy as np
from sklearn.feature_extraction.text import TfidfVectorizer
from sklearn.metrics.pairwise import cosine_similarity
import requests
from giza.datasets import DatasetsHub, DatasetsLoader
import os
import certifi
from typing import Dict, List, Any


# Giza DatasetHub ve DatasetLoader tanımlamaları
hub = DatasetsHub()
loader = DatasetsLoader()

def load_zk_compression_result(file_path: str) -> Dict[str, Any]:
    """
    zk-compression sonuç dosyasını yükle ve JSON olarak parse et.
    
    Args:
        file_path (str): Dosyanın yolu
    
    Returns:
        dict: JSON olarak yüklenen zk sonuç verisi
    """
    try:
        with open(file_path, 'r') as file:
            return json.load(file)
    except FileNotFoundError:
        raise ValueError(f"Dosya bulunamadı: {file_path}")
    except json.JSONDecodeError:
        raise ValueError(f"Geçersiz JSON formatı: {file_path}")

def get_blink_data() -> List[Dict[str, Any]]:
    """
    Giza DatasetHub üzerinden 'Blink' veri setini al.
    
    Returns:
        List[Dict]: Blink veri setinin bir listesi
    """
    # Blink etiketine sahip veri setini alıyoruz
    datasets = hub.get_by_tag('Blink')
    if not datasets:
        raise ValueError("Blink veri seti bulunamadı.")
    
    # İlk Blink veri setini yüklüyoruz
    blink_data = loader.load(datasets[0].name)
    return blink_data

def preprocess_text(text: str) -> str:
    """
    Metni ön işleme tabi tutarak küçük harflere çevir.
    
    Args:
        text (str): Önişleme yapılacak metin
    
    Returns:
        str: Küçük harflere çevrilmiş metin
    """
    return text.lower()

def calculate_similarity(text1: str, text2: str) -> float:
    """
    İki metin arasındaki benzerliği TF-IDF ve kosinüs benzerliği kullanarak hesapla.
    
    Args:
        text1 (str): Birinci metin
        text2 (str): İkinci metin
    
    Returns:
        float: İki metin arasındaki benzerlik skoru (0-1 arası)
    """
    vectorizer = TfidfVectorizer()
    tfidf_matrix = vectorizer.fit_transform([text1, text2])
    return cosine_similarity(tfidf_matrix[0:1], tfidf_matrix[1:2])[0][0]

def find_best_blink(zk_result: Dict[str, Any], blink_data: List[Dict[str, Any]]) -> Dict[str, Any]:
    """
    zk sonucu ile blink verileri arasında en iyi eşleşmeyi bul.
    
    Args:
        zk_result (dict): zk-compression sonucunun JSON formatı
        blink_data (list): Blink veri seti
    
    Returns:
        dict: En uygun eşleşme
    """
    # zk sonucu JSON verisini stringe çevirip önişleme yapıyoruz
    zk_text = preprocess_text(json.dumps(zk_result))
    
    best_similarity = -1
    best_blink = None

    # Her bir blink verisi ile benzerlik hesaplıyoruz
    for blink in blink_data:
        blink_text = preprocess_text(blink['description'])
        similarity = calculate_similarity(zk_text, blink_text)
        
        # Eğer bu benzerlik daha iyiyse, güncelliyoruz
        if similarity > best_similarity:
            best_similarity = similarity
            best_blink = blink

    return best_blink

def print_blink_info(blink: Dict[str, Any]):
    """
    Blink bilgilerini güzel bir şekilde ekrana basar.
    
    Args:
        blink (dict): Blink veri sözlüğü
    """
    print(f"En uygun sonuç bulundu:")
    print(f"Başlık: {blink['title']}")
    print(f"Açıklama: {blink['description']}")
    print(f"Bağlantı: {blink['url']}")

def main(zk_result_path: str):
    """
    Ana fonksiyon - zk-compression sonucu ve Blink verileri ile en iyi eşleşmeyi bulur.
    
    Args:
        zk_result_path (str): zk-compression sonucunun dosya yolu
    """
    # zk-compression sonucunu yükle
    try:
        zk_result = load_zk_compression_result(zk_result_path)
    except ValueError as e:
        print(e)
        return

    # Blink verilerini al
    try:
        blink_data = get_blink_data()
    except ValueError as e:
        print(e)
        return

    # En uygun Blink veri setini bul
    best_blink = find_best_blink(zk_result, blink_data)

    # En uygun eşleşme bulunduysa ekrana bas, aksi halde bilgi ver
    if best_blink:
        print_blink_info(best_blink)
    else:
        print("Uygun bir eşleşme bulunamadı.")

if __name__ == "__main__":
    # zk-compression sonucu dosyasının yolu
    zk_result_path = "path/to/zk_compression_result.json"
    main(zk_result_path)
