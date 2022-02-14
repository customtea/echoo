# echooo
色付けオプション付き echo

## 概要
文字色と背景色をオプションで雑に指定できるechoコマンド

全面的にこのライブラリを利用している
[colored](https://github.com/mackwic/colored)

## usage
```
ecohoo 1.0.0
CustomTea
colorized echo

USAGE:
    echooo.exe [OPTIONS] [String]...

ARGS:
    <String>...    Text

OPTIONS:
    -b, --back-color <BACK_COLOR>      Set Background Color (black, red, green, yellow, blue,
                                       magenta, cyan, white, "#3D9979", bright_red,...)
        --blink                        blink
        --bold                         Bold
        --dimmed                       Dimmed
    -f, --front-color <FRONT_COLOR>    Set Front Color (black, red, green, yellow, blue, magenta,
                                       cyan, white, "#85144B", bright_red,...)
    -h, --help                         Print help information
        --hidden                       hidden
        --italic                       Italic
    -n, --no-newline                   no Newline
        --reverse                      Reversed
        --through                      Strike through
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
- through
  - 取り消し線

## ToDo
- 色指定を増やす
  - light系
- `\n`などの改行エスケープシーケンスが無視される
  - 無視されてもいいけど，オプションで切り替えれる方が良い？
  - 色付けと競合しそうではある
- オプションの順序をいい感じにソートしたい