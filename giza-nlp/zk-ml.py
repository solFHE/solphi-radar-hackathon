import json
import numpy as np
from sklearn.feature_extraction.text import TfidfVectorizer
from sklearn.metrics.pairwise import cosine_similarity
import requests
from giza.datasets import DatasetsHub, DatasetsLoader
import os
import certifi
from typing import Dict, List, Any

# SSL Sertifikası ayarı - Giza API ile uyumlu çalışmak için gerekli.
# os.environ['SSL_CERT_FILE'] = certifi.where()

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
    Giza DatasetHub üzerinden 'zk-ml' veri setini al.
    
    Returns:
        List[Dict]: zk-ml veri setinin bir listesi
    """
    # zk-ml etiketine sahip veri setini alıyoruz
    datasets = hub.get_by_tag('zk-ml')
    if not datasets:
        raise ValueError("zk-ml veri seti bulunamadı.")
    
    # İlk zk-ml veri setini yüklüyoruz
    zk_ml_data = loader.load(datasets[0].name)
    return zk_ml_data

def preprocess_text(text: str) -> str:
    """
    Metni ön işleme tabi tutarak küçük harflere çevir ve gereksiz karakterleri temizle.
    
    Args:
        text (str): Önişleme yapılacak metin
    
    Returns:
        str: Önişleme yapılmış metin
    """
    text = text.lower()
    # Gereksiz karakterleri temizle (noktalama işaretleri vs.)
    text = ''.join(char for char in text if char.isalnum() or char.isspace())
    return text

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

def find_best_match(zk_result: Dict[str, Any], zk_ml_data: List[Dict[str, Any]]) -> Dict[str, Any]:
    """
    zk sonucu ile zk-ml verileri arasında en iyi eşleşmeyi bul.
    
    Args:
        zk_result (dict): zk-compression sonucunun JSON formatı
        zk_ml_data (list): zk-ml veri seti
    
    Returns:
        dict: En uygun eşleşme
    """
    # zk sonucu JSON verisini stringe çevirip önişleme yapıyoruz
    zk_text = preprocess_text(json.dumps(zk_result))
    
    best_similarity = -1
    best_match = None

    # Her bir zk-ml verisi ile benzerlik hesaplıyoruz
    for zk_ml in zk_ml_data:
        zk_ml_text = preprocess_text(zk_ml['description'])
        similarity = calculate_similarity(zk_text, zk_ml_text)
        
        # Eğer bu benzerlik daha iyiyse, güncelliyoruz
        if similarity > best_similarity:
            best_similarity = similarity
            best_match = zk_ml

    return best_match

def print_match_info(match: Dict[str, Any]):
    """
    Eşleşen verinin bilgilerini güzel bir şekilde ekrana basar.
    
    Args:
        match (dict): Eşleşen veri sözlüğü
    """
    print(f"En uygun sonuç bulundu:")
    print(f"Başlık: {match['title']}")
    print(f"Açıklama: {match['description']}")
    print(f"Bağlantı: {match['url']}")

def main(zk_result_path: str):
    """
    Ana fonksiyon - zk-compression sonucu ve zk-ml verileri ile en iyi eşleşmeyi bulur.
    
    Args:
        zk_result_path (str): zk-compression sonucunun dosya yolu
    """
    # zk-compression sonucunu yükle
    try:
        zk_result = load_zk_compression_result(zk_result_path)
    except ValueError as e:
        print(e)
        return

    # zk-ml verilerini al
    try:
        zk_ml_data = get_blink_data()
    except ValueError as e:
        print(e)
        return

    # En uygun zk-ml veri setini bul
    best_match = find_best_match(zk_result, zk_ml_data)

    # En uygun eşleşme bulunduysa ekrana bas, aksi halde bilgi ver
    if best_match:
        print_match_info(best_match)
    else:
        print("Uygun bir eşleşme bulunamadı.")

if __name__ == "__main__":
    # zk-compression sonucu dosyasının yolu
    zk_result_path = "path/to/zk_compression_result.json"
    main(zk_result_path)
