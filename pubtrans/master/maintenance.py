import dataOperator as do
"""
列車情報・駅情報の取得更新を行う

データソースについて

全国の路線情報・駅情報のCSVファイルは https://ekidata.jp/dl/ から取得する


./input/company.csv

として保存する

"""

COMPANY_CSV = 'input/company.csv'
JOIN_CSV = 'input/join.csv'
LINE_CSV = 'input/line.csv'
STATION_CSV = 'input/station.csv'

do.maintenanceCtl(COMPANY_CSV,JOIN_CSV,LINE_CSV,STATION_CSV)

