import dataOperator as do
"""
路線情報をCSVからJSONに変換する

CSVファイルは

./input/line.csv

として保存する

初期データ取得→DB登録後にメンテナンススクリプト実行

"""

do.registerRailway('input/line.csv')

