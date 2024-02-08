import os
import json

# 収集するフォルダーのパスを指定します
folder_path = "output/"

# まとめるための空のリストを作成します
combined_data = []

# フォルダー内のファイルを走査します
for filename in os.listdir(folder_path):
    if filename.endswith(".json"):  # 拡張子が.jsonのファイルのみを処理します
        file_path = os.path.join(folder_path, filename)
        with open(file_path, "r") as file:
            # JSONファイルを読み込み、データをリストに追加します
            json_data = json.load(file)
            combined_data.append(json_data)

combined_data = [ com  for comb in combined_data for com in comb]

# まとめたデータを1つのJSONファイルに書き込みます
output_file_path = "comp.json"
with open(output_file_path, "w") as output_file:
    json.dump(combined_data, output_file,ensure_ascii=False)

print("ファイルの結合が完了しました。")
