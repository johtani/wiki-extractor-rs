# 出力される情報のサンプル

## 出力項目

* id: idタグの文字列
* title: ページタイトル
* timestamp: revition/timestampタグの日付
* contents(配列): 見出しごとの文字列を配列に格納
* categories(配列): カテゴリータグの文字列を配列に格納
* headings(配列): 見出しを出てきた順番に配列に格納
* images(配列): 記事に出てきた画像の情報
    * target: ファイル名 
    * target_type: タイプ(Image|Fileの2種類。違いは不明。。。)
    * text: 
        * text: 説明文(ファイル名をのぞいた文字列)
        * link_target: リンク(説明文にリンクが有る場合のみ)
* links(配列): 内部リンクを出てきた順番に格納(重複排除はしていない)
    * text: 内部リンクが貼られている文字列
    * link_target: 内部リンク文字列

## サンプル
```json
{
    "id": "5",
    "title": "アンパサンド",
    "timestamp": "2019-08-14T00:23:07Z",
    "revision_id": "1525028",
    "contents": [
        "アンパサンド\nアンパサンド (&、英語名：ampersand) とは並立助詞「…と…」を意味する記号である。ラテン語の \"et\" の合字で、Trebuchet MSフォントでは、10pxと表示され \"et\" の合字であることが容易にわかる。ampersa、すなわち \"and per se and\"、その意味は\"and [the symbol which] by itself [is] and\"である。",
        "歴史\n\nその使用は1世紀に遡ることができ、5世紀中葉から現代に至るまでの変遷がわかる。\nZ に続くラテン文字アルファベットの27字目とされた時期もある。\nアンパサンドと同じ役割を果たす文字に「ティロ式記号のet」と呼ばれる、数字の「7」に似た記号があった(⁊, U+204A)。この記号は現在もゲール文字で使われている。\n記号名の「アンパサンド」は、ラテン語まじりの英語「& はそれ自身 \"and\" を表す」(& per se and) のくずれた形である。英語以外の言語での名称は多様である。",
        "手書き\n\n日常的な手書きの場合、欧米でアンパサンドは「ε」に縦線を引く単純化されたものが使われることがある。\nまた同様に、「t」または「+（プラス）」に輪を重ねたような、無声歯茎側面摩擦音を示す発音記号「ɬ」のようなものが使われることもある。",
        "プログラミング言語\nプログラミング言語では、C など多数の言語で AND 演算子として用いられる。以下は C の例。\n  * X = A && B のように2個重ねたものは論理 AND を表す。この場合 A, B がともに真ならば X も真、それ以外は偽である。\n  * 0x12345678 & 0x0f0f0f0f のように1個であればビット AND を表す。この場合の結果は 0x02040608 である。PHPでは、変数宣言記号（$）の直前に記述することで、参照渡しを行うことができる。\nBASIC 系列の言語では文字列の連結演算子として使用される。\"foo\" & \"bar\" は \"foobar\" を返す。また、主にマイクロソフト系では整数の十六進表記に &h を用い、&h0F （十進で15）のように表現する。\nSGML、XML、HTMLでは、アンパサンドを使ってSGML実体を参照する。",
        "符号位置\n",
        "外部リンク\n"
    ],
    "categories": [
        "約物",
        "ラテン語の語句",
        "論理記号"
    ],
    "headings": [
        "アンパサンド",
        "歴史",
        "手書き",
        "プログラミング言語",
        "符号位置",
        "外部リンク"
    ],
    "images": [
        {
            "target": "Trebuchet MS ampersand.svg",
            "target_type": "Image",
            "text": {
                "text": "right|thumb|100px|Trebuchet MS フォント",
                "link_target": "Trebuchet MS"
            }
        },
        {
            "target": "Historical ampersand evolution.svg",
            "target_type": "Image",
            "text": {
                "text": "thumb|right|390px|アンパサンドの進展"
            }
        },
        {
            "target": "Ampersand-handwriting-1.png",
            "target_type": "File",
            "text": {
                "text": "right|thumb|80px|手書きのアンパサンド"
            }
        },
        {
            "target": "Ampersand-handwriting-2.svg",
            "target_type": "Image",
            "text": {
                "text": "right|thumb|80px|手書きのアンパサンド（簡素化）"
            }
        }
    ],
    "links": [
        {
            "text": "記号",
            "link_target": "記号"
        },
        {
            "text": "ラテン語",
            "link_target": "ラテン語"
        },
        {
            "text": "合字",
            "link_target": "合字"
        },
        {
            "text": "Trebuchet MS",
            "link_target": "Trebuchet MS"
        },
        {
            "text": "10px",
            "link_target": "ファイル:Trebuchet MS ampersand.svg"
        },
        {
            "text": "Z",
            "link_target": "Z"
        },
        {
            "text": "ラテン文字",
            "link_target": "ラテン文字"
        },
        {
            "text": "アルファベット",
            "link_target": "アルファベット"
        },
        {
            "text": "ゲール文字",
            "link_target": "ゲール文字"
        },
        {
            "text": "欧米",
            "link_target": "欧米"
        },
        {
            "text": "無声歯茎側面摩擦音",
            "link_target": "無声歯茎側面摩擦音"
        },
        {
            "text": "発音記号",
            "link_target": "発音記号"
        },
        {
            "text": "プログラミング言語",
            "link_target": "プログラミング言語"
        },
        {
            "text": "C",
            "link_target": "C言語"
        },
        {
            "text": "演算子",
            "link_target": "演算子"
        },
        {
            "text": "論理 AND",
            "link_target": "論理積"
        },
        {
            "text": "ビット AND",
            "link_target": "ビット演算#AND"
        },
        {
            "text": "PHP",
            "link_target": "PHP (プログラミング言語)"
        },
        {
            "text": "参照渡し",
            "link_target": "参照渡し"
        },
        {
            "text": "BASIC",
            "link_target": "BASIC"
        },
        {
            "text": "文字列",
            "link_target": "文字列"
        },
        {
            "text": "マイクロソフト",
            "link_target": "マイクロソフト"
        },
        {
            "text": "十六進表記",
            "link_target": "十六進法"
        },
        {
            "text": "SGML",
            "link_target": "Standard Generalized Markup Language"
        },
        {
            "text": "XML",
            "link_target": "Extensible Markup Language"
        },
        {
            "text": "HTML",
            "link_target": "HyperText Markup Language"
        },
        {
            "text": "SGML実体",
            "link_target": "SGML実体"
        }
    ]
}
```