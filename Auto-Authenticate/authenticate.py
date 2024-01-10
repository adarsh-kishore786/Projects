import requests
import os
from dotenv import load_dotenv

load_dotenv()

url = "https://nac.nitk.ac.in:8090/httpclient.html"

headers = {
    'Accept': '*/*',
    'Accept-Language': 'en-GB,en;q=0.7',
    'Connection': 'keep-alive',
    'Content-Type': 'application/x-www-form-urlencoded',
    'Origin': 'https://nac.nitk.ac.in:8090',
    'Referer': url,
    'Sec-Fetch-Dest': 'empty',
    'Sec-Fetch-Mode': 'cors',
    'Sec-Fetch-Site': 'same-origin',
    'Sec-GPC': '1',
    'User-Agent': 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36',
    'sec-ch-ua': '"Not_A Brand";v="8", "Chromium";v="120", "Brave";v="120"',
    'sec-ch-ua-mobile': '?0',
    'sec-ch-ua-platform': '"Linux"',
}

data = {
    'mode': os.getenv('mode'),
    'username': os.getenv('username'),
    'password': os.getenv('password'),
    'a': os.getenv('a'),
    'producttype': os.getenv('producttype'),
}

# response = requests.get(url, headers=headers, data=data)
response = requests.post(url, headers=headers, data=data)
print(response.text)
