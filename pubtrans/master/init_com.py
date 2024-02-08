import dataOperator as do
"""
鉄道運行会社情報をCSVからJSONに変換する

CSVファイルは

./input/company.csv

として保存する

初期データ取得→DB登録後にメンテナンススクリプト実行

"""

do.registerOperator('input/company.csv')

