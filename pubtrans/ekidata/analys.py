import csv
import json
import os
import pykakasi
import pandas as pd
import re

kakasi = pykakasi.kakasi()
def remove_duplicate_vowels(string):
    converted_string = re.sub(r'([aeiou])\1', r'\1', string, flags=re.IGNORECASE)
    return converted_string

def replace_with_dictionary(string, dictionary):
    for key, value in dictionary.items():
        string = string.replace(key, value)
    return string

def convert(text):
    result = kakasi.convert(text)
    base = remove_duplicate_vowels("".join([converted_word['hepburn'].capitalize() for converted_word in result ]))
    if base.endswith("sen"):
        base = base[:-3] + ""
    dictionary = {
        "Sen": "",
        "JrHigashinihon": "JR-East",
        "JrHokkaidou": "JR-Hokkaido",
        "JrKyushu": "JR-Kyusyu",
        "JrNishinihon": "JR-West",
        "JrToukai-":"JR-Central-",
        "Kabushikigaishya": "",
        "Kinkinihontetsudou": "Kintetsu",
        "Rain":"Line",
        "Jr":"JR",
        "Toukyou":"Tokyo",
        "Osaka metro":"OsakaMetro",
        "IrI":"IRI",
        "'":"",
        "Tokyotokoutsukyoku":"Toei",
        "TokyoRinkaiKousokutetsudou":"TWR",
        "Lrt":"LRT",
        "YokohamashiKoutsukyoku":"YokohamaMunicipal",
        "Hokusoutetsudou":"Hokuso",
        "KeikyuDentetsu":"Keikyu",
        "KeiouDentetsu":"Keio",
        "Keiseidentetsu":"Keisei",
        "ShutokenShintoShiTetsudou":"MIR",
        "TsukubaEkusupuresu":"TsukubaExpress",
        "OdakyuDentetsu":"Odakyu",
        "SaitamaKousokutetsudou":"SaitamaRailway",
        "SaitamaKousokutetsudouLine":"SaitamaRailway",
        "Seibutetsudou":"Seibu",
        "Shibayamatetsudou":"Shibayama",
        "ShibayamatetsudouLine":"Shibayama",
        "Sagamitetsudou":"Sotetsu",
        "TamaToshiMonoreru":"TamaMonorail",
        "Tobutetsudou":"Toubu",
        "Toukyudentetsu":"Tokyu",
        "Touyoukousokutetsudou":"ToyoRapid",
        "Toukin":"Tougane",
        "honLine":"honsen",
        "Lineseki":"Senseki",
        "JRJoubansen(Ueno~Toride)":"Joban",
        "JRJoubansen(Toride~iwaki)":"Joban",
        "JRJoubansen(Iwaki~Linedai)":"Joban",
        "JRChuousen(Kaisoku)":"ChuoRapid",
        "JRChuou・SoubuLine":"ChuoSoubuLocal",
        "JR-Tokaidouhonsen(Tokyo~Atami)":"Tokaido",
        "JRYokosukaLine":"Yokosuka",
        "TokyoMetoro":"TokyoMetro"
    }
    return replace_with_dictionary(base, dictionary)

def extract_company_data(csv_path):
    companies = []
    with open(csv_path, 'r', encoding='utf-8') as file :
        reader = csv.DictReader(file)
        for row in reader:
            cid = row['company_cd']
            name = convert(row['company_name'])
            companies.append({
                "cid": cid,
                "name":name
            })
    return companies

def extract_railway_data(csv_path,company):
    lines = []
    with open(csv_path, 'r', encoding='utf-8') as file :
        reader = csv.DictReader(file)
        for row in reader:
            if row['company_cd'] != company['cid']:
                continue
            lid = row['line_cd']
            name = convert(row['line_name'])
            lines.append({
                "lid":lid,
                "cid": company['cid'],
                "lname":name,
                "cname":company['name']
            })

    return lines

def extract_station_data(join_csv_path,station_csv_path,line):
    join_data = []
    # JOIN
    with open(join_csv_path, 'r', encoding='utf-8') as file:
        reader = csv.DictReader(file)
        for row in reader:
            try:
                if row['line_cd'] != line['lid']:
                    continue
            except:
                print(type(row['line_cd']))
                print(line)
                print(type(line['lid']))
            join_data.append(row['station_cd1'])
            join_data.append(row['station_cd2'])
    # Station
    station = list(dict.fromkeys(join_data))
    df = pd.read_csv(station_csv_path)
    st_list = []
    def getSPFID(df,st,line):
        try:
            df_key = df.query(f'{st} == station_cd').values.tolist()[0]
        except:
            print(f"駅情報が見つかりません\nコード:{st}")
            return ''
        return f"{line['cname']}.{line['lname']}.{convert(df_key[2])}"
    
    
    for index,st in enumerate(station):
        try:
            df_key = df.query(f'{st} == station_cd').values.tolist()[0]
        except:
            print(f"駅コードが見つかりません\nコード:{st}")
            continue
        st_list.append({
            'SPFID': f"{line['cname']}.{line['lname']}.{convert(df_key[2])}",
            'SRWID': f"{line['cname']}.{line['lname']}",
            'Before': getSPFID(df,station[index-1],line) if index != 0 else '',
            "After": getSPFID(df,station[index+1],line) if index+1 < len(station) else '',
            "Platform": [],
            "NumberLink": "",
            "Name": {
                "en": convert(df_key[2]),
                "ja": df_key[2]
            },
            "lat": df_key[10],
            "lon": df_key[9]
        })
    
    return st_list

def generate_json(output_json_path,output_json_data):
    # JSONファイルとして出力
    if os.path.exists(output_json_path):
        output_json_path = f"{output_json_path}_1.json"
    else:
        output_json_path = f"{output_json_path}.json"
    
    with open(output_json_path, 'w', encoding='utf-8') as file:
        json.dump(output_json_data, file, indent=2, ensure_ascii=False)

# 入力ファイルのパス
join_csv_path = './src/join.csv'
station_csv_path = './src/station.csv'
company_csv_path = './src/company.csv'
line_csv_path = './src/line.csv'

coms = extract_company_data(company_csv_path)
railways = [extract_railway_data(line_csv_path,com) for com in coms]
print(railways)
stations = [extract_station_data(join_csv_path,station_csv_path,rw)  for rwcom in railways for rw in rwcom]
[generate_json(f"output/{sts[0]['SRWID'].replace('.','-')}",sts) for sts in stations if len(sts) > 0]