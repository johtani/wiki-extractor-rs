# Extract wiki xml　 

Extract some information from Wikipedia xml.

Now, only support Japanese Wikipedia xml

## 日本語

WikipediaのXMLファイルをもとに、JSONとしてデータを出力するためのプログラムです。

現時点では日本語向けのWikipediaのデータのみを対象としています。
出力される文字列では、強調表示などの情報を抜き去った文字列と、関連情報になります。

## 出力例

アンパサンドのページの[出力例](./output_example.md)です。

## 制限

* `Wikipedia:`で始まるタイトルのページは出力対象外
* 参照タグには未対応
* テーブルタグ未対応
* 必要最低限のテンプレートへの対応
* リダイレクトページなどの対応

## TODO

* 利用方法の説明を記載
* 数あるテンプレートへの対応。。。
