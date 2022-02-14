# echooo
色付けオプション付き echo

![cmdline](https://github.com/customtea/echoo/blob/fig/fig/commandline.png)

## 概要
文字色と背景色をオプションで雑に指定できるechoコマンド

全面的にこのライブラリを利用している
[colored](https://github.com/mackwic/colored)

## usage
```
ecohoo 1.2.0
CustomTea
colorized echo

USAGE:
    echooo [OPTIONS] [String]...

ARGS:
    <String>...    Text

OPTIONS:
    -b, --back-color <BACK_COLOR>      Set Background Color
        --blink                        blink
        --bold                         Bold
        --dimmed                       Dimmed
    -f, --front-color <FRONT_COLOR>    Set Front Color
        --file <FILE>                  File
    -h, --help                         Print help information
        --hidden                       hidden
        --italic                       Italic
    -l, --list                         Color list
    -n, --no-newline                   no Newline
        --reverse                      Reversed
        --strike                       Strike through
        --under                        Underline
    -V, --version                      Print version information
```

## 指定可能な色
依存ライブラリ側が指定できるもの
- black
- red
- green
- yellow
- blue
- magenta(or purple)
- cyan
- white
- #NNNNNN
  - 16進数カラーコード
各色に `bright_` をつけると明るいものを指定できる

追加した色 参考サイト[Colors](http://clrs.cc/)
- silver
- gray
- maroon
- olive
- lime
- aqua
- teal
- navy
- fuchsia
利用可能な色リストを `-l` オプションで表示できる
ターミナルのカラースキームで変わってくる

## 指定可能デコレーション
依存ライブラリ側が指定できるもの
- bold
  - 太字
  - 背景色を指定しているとはみ出す事がある
- underline
  - 下線
- italic
  - 斜体
  - 背景色を指定しているとはみ出す事がある
- dimmed
  - 薄くなる
  - 現環境では違いがわからない
- reverse
  - 文字色と背景色を入れ替え
- blink
  - 点滅
    - たぶんしない
- hidden
  - 消える
  - 要らない気がする…
  - 文字自体は消えてないのでコピー可能
- strike
  - 取り消し線(strike-through)

## ファイル読み込み（埋め込み指定）
- テキストファイルに，色の指定などを埋め込んで自動的に表示するモード
- `--file` でファイルを指定
- 使えるコマンドは
  - `\{f_red}` : 文字色を赤に
  - `\{b_blue}` : 背景色を青に
  - `\{f_clear}` : 文字色を初期値に 
  - `\{b_clear}` : 背景色を初期値に 
  - `\{bold}` : 太字に
  - `\{n_bold}` : 太字を解除
  - その他色や，装飾は上の指定可能オプションを参照
- 設定した色や装飾は，解除指示をするまで行をまたがって継続する

```
日本語の途中で\{f_red}色を\{f_blue}変更．
次の行の色はどうなるのか
\{b_red}\{f_clear}
行頭でのコマンド処理
\{strike}取り消して\{n_strike}取り消し線の解除
```
![metafile](https://github.com/customtea/echoo/blob/fig/fig/fileload.png)

## ToDo
- 色指定を増やす
  - light系
- オプションの順序をいい感じにソートしたい