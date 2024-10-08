import json
import numpy as np
from sklearn.feature_extraction.text import TfidfVectorizer
from sklearn.metrics.pairwise import cosine_similarity
import requests
from giza.datasets import DatasetsHub, DatasetsLoader
import os
import certifi

# SSL 
os.environ['SSL_CERT_FILE'] = certifi.where()

# Giza DatasetHub DatasetLoader
hub = DatasetsHub()
loader = DatasetsLoader()

def load_zk_compression_result(file_path):
    """
    zk-compression sonuç dosyasını yükle ve parse et
    """
    with open(file_path, 'r') as file:
        return json.load(file)

def get_blink_data():
    """
    Get blink data from 
    """
    datasets = hub.get_by_tag('Blink')
    if not datasets:
        raise ValueError("no blink data")
    
    blink_data = loader.load(datasets[0].name)
    return blink_data

def preprocess_text(text):
    """
    Text
    """

    return text.lower()

def calculate_similarity(text1, text2):
    """
    Two text
    """
    vectorizer = TfidfVectorizer()
    tfidf_matrix = vectorizer.fit_transform([text1, text2])
    return cosine_similarity(tfidf_matrix[0:1], tfidf_matrix[1:2])[0][0]

def find_best_blink(zk_result, blink_data):
    """
    Calculate best result
    """
    zk_text = preprocess_text(json.dumps(zk_result))
    
    best_similarity = -1
    best_blink = None

    for blink in blink_data:
        blink_text = preprocess_text(blink['description'])
        similarity = calculate_similarity(zk_text, blink_text)
        
        if similarity > best_similarity:
            best_similarity = similarity
            best_blink = blink

    return best_blink

def main(zk_result_path):
    # zk-compression 
    zk_result = load_zk_compression_result(zk_result_path)


    blink_data = get_blink_data()


    best_blink = find_best_blink(zk_result, blink_data)

    if best_blink:
        print(f"Best result:")
        print(f"Title: {best_blink['title']}")
        print(f"Description: {best_blink['description']}")
        print(f"Link: {best_blink['url']}")
    else:
        print("😩")

if __name__ == "__main__":
    zk_result_path = "path/to/zk_compression_result.json"
    main(zk_result_path)