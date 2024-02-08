import requests
import json
import re

TOKEN = "1a4e53918cfd05c18091c99bd352afd3471c05f5d56877b5583f44502c572ec1"
BASE = "https://api.odpt.org/api/v4/"
 
res = requests.get(f'{BASE}odpt:Station?acl:consumerKey={TOKEN}&odpt:railway=odpt.Railway:HakoneTozan.HakoneTozan')
resjson = json.loads(res.content)
resjson = sorted(resjson,key=lambda x:int(re.findall(r'\d+',x['odpt:stationCode'])[0]))
result=[]
before = ''
for rw in resjson:
    item = {
        'SPFID':rw['owl:sameAs'].split(':')[1],
        'SRWID':rw['odpt:railway'].split(':')[1],
        'Before':before,
        'After':'',
        'Platform':[],
        'NumberLink':rw['odpt:stationCode'] if 'odpt:stationCode' in rw else '',
        "Name":rw['odpt:stationTitle'],
        "lat":rw['geo:lat'] if 'geo:lat' in rw else '',
        "long":rw['geo:long'] if 'geo:long' in rw else ''
    }
    if len(result) > 0:
        result[-1]['After'] = rw['owl:sameAs'].split(':')[1]
    result.append(item)
    before = rw['owl:sameAs'].split(':')[1]

with open('result.json',encoding='utf-8',mode='w') as f:
    json.dump(result, f, indent=2, ensure_ascii=False)