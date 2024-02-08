import requests
import json

TOKEN = "1a4e53918cfd05c18091c99bd352afd3471c05f5d56877b5583f44502c572ec1"
BASE = "https://api.odpt.org/api/v4/"
 
res = requests.get(f'{BASE}odpt:Operator?acl:consumerKey={TOKEN}')
resjson = json.loads(res.content)
with open('result.json',encoding='utf-8',mode='w') as f:
    json.dump(resjson, f, indent=2, ensure_ascii=False)