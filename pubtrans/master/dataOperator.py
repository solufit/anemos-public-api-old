import csv
import json
from googletrans import Translator
from pymongo.mongo_client import MongoClient
import uuid
import requests
import datetime

ODPT_KEY = '1a4e53918cfd05c18091c99bd352afd3471c05f5d56877b5583f44502c572ec1'


def translate(text):
    translator = Translator()
    result = {}
    result['ja'] = text
    result['en'] = translator.translate(text, dest="en").text
    result['ko'] = translator.translate(text, dest="ko").text
    result['zh'] = translator.translate(text, dest="zh-CN").text
    return result 

def dbCtl():
    client = MongoClient('mongodb://root:solufit_siesta_password@localhost:27017/siesta?authSource=admin&retryWrites=true&w=majority')
    db = client.get_database("siesta")
    return {
        'companies': db.get_collection('companies'),
        'railways': db.get_collection('railways'),
        'stations': db.get_collection('stations')
    }

def maintenanceCtl(COMPANY_CSV,JOIN_CSV,LINE_CSV,STATION_CSV):
    dt_now_jst_aware = datetime.datetime.now(
        datetime.timezone(datetime.timedelta(hours=9))
    )
    ## 情報信頼度の低い順に更新する(Ekidata < ODPT)
    print(f"作業開始 : {dt_now_jst_aware}")
    ## 運行会社情報
    updateCompanyEkiData(COMPANY_CSV)
    updateCompanyOdpt()
    ## 路線情報
    updateRailwayEkiData(LINE_CSV)
    updateRailwayOdpt()
    dt_now_jst_aware = datetime.datetime.now(
        datetime.timezone(datetime.timedelta(hours=9))
    )
    print(f"正常終了 : {dt_now_jst_aware}")
    return

def registerOperator(csv_path):
    companies = []
    with open(csv_path, 'r', encoding='utf-8') as file :
        reader = csv.DictReader(file)
        for row in reader:
            print(f'Step: {reader.line_num}')
            items = {
                "SOPID": str(uuid.uuid4()),
                "Name": translate(row['company_name']),
                "Link": row['company_url'],
                "Arias": [
                    row['company_name'],
                    row['company_name_k'],
                    row['company_name_h'],
                    row['company_name_r'],
                    row['company_cd']
                ]
            }
            companies.append(items)

    with open('output/init_companies.json', 'w', encoding='utf-8') as file:
        json.dump(companies, file, indent=2, ensure_ascii=False)

def registerRailway(csv_path):
    collection = dbCtl()['companies']
    railways = []
    with open(csv_path, 'r', encoding='utf-8') as file :
        reader = csv.DictReader(file)
        for row in reader:
            print(f'Step: {reader.line_num}')
            items = {
                "SOPID": collection.find_one({'Arias':row['company_cd']})['SOPID'],
                "SRWID": str(uuid.uuid4()),
                "Name": translate(row['line_name']),
                "LineColor":row['line_color_c'],
                "LineCode":"",
                "ascendingRailDirection":"",
                "descendingRailDirection":"",
                "Arias": [
                    row['line_name'],
                    row['line_name_k'],
                    row['line_name_h'],
                    row['line_cd']
                ]
            }
            railways.append(items)

    with open('output/init_railways.json', 'w', encoding='utf-8') as file:
        json.dump(railways, file, indent=2, ensure_ascii=False)

def updateCompanyEkiData(COMPANY_CSV):
    with open(COMPANY_CSV, 'r', encoding='utf-8') as file :
        collection = dbCtl()['companies']
        reader = csv.DictReader(file)
        for row in reader:
            items = {
                "Name": translate(row['company_name']),
                "Link": row['company_url'],
                "Arias": [
                    row['company_name'],
                    row['company_name_k'],
                    row['company_name_h'],
                    row['company_name_r'],
                    row['company_cd']
                ]
            }
            collection.update_one({'Arias':row['company_cd']},{'$set':items})

def updateCompanyOdpt():
    collection = dbCtl()['companies']
    BASE = "https://api.odpt.org/api/v4/"
    res = requests.get(f'{BASE}odpt:Operator?acl:consumerKey={ODPT_KEY}')
    resjson = json.loads(res.content)
    for ope in resjson:
        collection.update_one({'Arias':ope['dc:title']},{'$addToSet': {'Arias': ope['owl:sameAs']}})
        collection.update_one({'Arias':ope['dc:title']},{'$set':{'Name.en':ope['odpt:operatorTitle']['en']}})

def updateRailwayEkiData(LINE_CSV):
    with open(LINE_CSV, 'r', encoding='utf-8') as file :
        collection = dbCtl()['railways']
        reader = csv.DictReader(file)
        for row in reader:
            items = {
                "Name": translate(row['line_name']),
                "LineColor":row['line_color_c'],
                "LineCode":"",
                "ascendingRailDirection":"",
                "descendingRailDirection":"",
                "Arias": [
                    row['line_name'],
                    row['line_name_k'],
                    row['line_name_h'],
                    row['line_cd']
                ]
            }
            collection.update_one({'Arias':row['line_cd']},{'$set':items})

def updateRailwayOdpt():
    collection = dbCtl()['railways']
    BASE = "https://api.odpt.org/api/v4/"
    res = requests.get(f'{BASE}odpt:Railway?acl:consumerKey={ODPT_KEY}')
    resjson = json.loads(res.content)
    for rw in resjson:
        collection.update_one({'Arias':rw['dc:title']},{'$addToSet': {'Arias': rw['owl:sameAs']}})
        collection.update_one({'Arias':rw['dc:title']},{'$set':{'Name.en':rw['odpt:operatorTitle']['en'],'ascendingRailDirection':rw['odpt:ascendingRailDirection'] if 'odpt:ascendingRailDirection' in rw else '','descendingRailDirection':rw['odpt:descendingRailDirection'] if 'odpt:descendingRailDirection' in rw else ''}})

def updateStation(JOIN_CSV,STATION_CSV):
    pass