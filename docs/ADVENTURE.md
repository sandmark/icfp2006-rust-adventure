# Adventure に関する記録

## 概要

    [Building vocabulary]
    [Initializing command processor]
    [Populating environment]
    Room With a Door
    
    You are in a room with a mechanical door. You will probably need
    to use a keypad to unlock it. A hallway leeds north.
    There is a pamphlet here.
    Underneath the pamphlet, there is a manifesto.
    
    >:

UMIX ユーザー `howie` (Howard Curry) が作ったテキストアドベンチャー…の
皮を被った dependency hell を丁重に皮肉ったパズル (?) ゲーム。

地面 (`pop` のみ許された LIFO 構造) に落ちているゴミ (再帰的依存関係を持つ) を
インベントリ (size[6]) の中で解決 (`combine`) しながら進んでいく。

**なぜかアイテムは地面に `drop` できないので退避先はない。**

**本当に必要のないものは `incinarate` で焼却できる。**

**インベントリと地面表層にあるアイテムの合成はできないため、どちらも拾う必要がある。**

## 規模

XX 番街の YY ストリート... という位置フォーマットがある。
`dunnet` を思い出すが、さすがに端と端は循環しないようだ。
それでも宇宙的規模ではある。

## コマンド

- `help <command>`

よくあるやつ。

- `go <direction>`

`(n)orth`, `(e)ast`, `(s)outh`, `(w)est` に移動する。
`go` そのものを省略して `<direction>` のみで移動できる。

隠しエイリアス: `cd <direction>`.

- `take <item>`

地面の表層にある `<item>` をインベントリに移動させる(拾う)。

エイリアス: `get`, `grab`.

- `drop <item>`

インベントリからアイテムを落とす(未確認)。

- `incinerate <item>`
インベントリにあるアイテムを焼却する。

`state-of-the-art DVNUL-9000 incinerator` という
焼却装置を装備しているようだ。もしかして: `/dev/null`.

エイリアス: `inc`
  
- `combine <item> with <another>`

欠けたパーツを装着することでアイテムを修復する。

エイリアス: `comb`, `c`
  
- `use <item>`

アイテムの機能を活用する。

原文 `"use: Exploit the functionality of an item."` には
あまりにも含みがある。

- `examine [target]`

アイテムや周りを観察する。
埋もれたアイテムも観察することができる。

エイリアス: `ex`, `x`, `look`, `l`

隠しエイリアス: `ls`

- `show`

現在のインベントリを表示する。

エイリアス: `invent`, `inv`, `i`

- `quit`

ゲームを終了する。

エイリアス: `exit`

### 隠しコマンド

    Things look slightly different, but you can't put a diode on
    exactly how.

- `switch <mode>`

身に付けている `Insta-Read(tm) Goggles` のモードを切り替える。

- `English`, `Reading`

今のところ両者の違いはわからない。
デフォルトのようだ。

- `XML`

何もかも XML 形式で見えるようになる。
**なぜかちゃんと pretty-print される。**

See: [XML レスポンス](./xml.md)

- `sexp`

何もかも S 式に見えるようになる。
pprint されるが、閉じ括弧など一部のコンテキストの間にはスペースが入る。

- `ML`

謎。

- `ANSI`

ANSI エスケープシーケンスで描写されるようになる。

**上部にスコアと場所を表示するステータスバーが追加される。**

## チュートリアル: Room With a Door からの脱出

    n
    take bolt
    take spring
    take button
    take processor
    take pill
    inc pill
    take radio
    take cache
    combine processor with cache
    take blue transistor
    combine radio with transistor
    take antenna
    inc antenna
    inc spring
    take screw
    take motherboard
    combine motherboard with screw
    take a-1920-ixb
    combine a-1920-ixb with radio
    combine a-1920-ixb with bolt
    combine a-1920-ixb with processor
    take transistor
    take keypad
    combine a-1920-ixb with transistor
    combine motherboard with a-1920-ixb
    combine keypad with motherboard
    combine keypad with button
    s
    use keypad
    
