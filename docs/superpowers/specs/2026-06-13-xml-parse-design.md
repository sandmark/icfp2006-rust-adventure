# XML パース 設計

- 日付: 2026-06-13
- 対象 TODO: 「XMLでパニックする (todo!)」「報酬コードを正しく流す」「XMLパースしてRustの型にする」
- 関連: `src/scanner.rs`, `src/renderer.rs`, `src/lib.rs`, `docs/xml.md`, `docs/ADVENTURE.md`

## 背景

`switch xml` 後、adventure エンジンはコマンド応答を pretty-print された XML
で返す（`docs/xml.md` 参照）。現状 `Scanner` の `Mode::Xml` 分岐と
`Renderer` の `Segment::Xml` 分岐が `todo!()` のままで、XMLモードに入ると
パニックする。

XML 応答には3種のルートタグがある: `<success>` / `<error>` / `<help>`。
構造は再帰的で、同名要素がネストする（`<item>` が `<piled_on>` 経由で
`<item>` を、`<condition>` が `<broken>` 経由で `<condition>` を含む）。
さらに XMLモードでは報酬コード行（例: `ADVTR.INC=5@999999|<hash>`）が
`<success>` の*前*に混じることが観測されている。

## 設計原則（この設計の土台）

**唯一許される前提は「文書は整形式 (well-formed) な XML である」こと。**
「ルートは自己ネストしない」のような有限サンプルからの観測には依存しない
（XML 自体に同名ネスト禁止のルールはなく、現にこのデータは同名ネストする）。
ヒューリスティックはアルゴリズムであって設計ではない。

その上で **malformed を正しく処理するのがアーキテクチャの仕事**。
契約の下では文書境界は構造から一意に決まり、深さを追うのは
「整形式 XML を正しく読むことの定義そのもの」であって、頑健化のための
当て推量ではない。

## 二文脈モデル（契約）

Xmlモードのバイト列は、契約により次の連なりとして扱う:

- **文書の外（深さ0）**: 報酬コード・空行・プロンプト等。XML 文法に属さない
  別チャネルの漏れ。→ **寛容**に素通し。
- **文書の中（深さ≥1）**: 厳格な整形式 XML。違反 → **エラー**。

この一般則により、報酬コードは scanner の特例 hack ではなく
「深さ0で流れてきたバイト」という一般則の帰結として自然に処理される。
エッジケースが特例でなく一般則に吸収されるのは良い設計のサイン。

## データフロー

```
VM stdout (bytes, chunked)
  → Scanner.feed()
       ├ English モード: 既存の素通し（MARKER 検出でモード遷移）
       └ Xml モード: quick-xml イベント + 深さカウンタで枠取り
            ├─ 深さ0のバイト        → Segment::Plain
            ├─ 完結した整形式文書    → Segment::Xml(Vec<u8>)
            └─ 文書内の整形式違反    → Err（コンテキストを積んで爆死）
  → Parser（Stage 3）: roxmltree DOM を手で歩いて型木へ
       Segment::Xml(bytes) → Room / Item / Condition / Description / Kind / ...
  → Renderer: Segment を出力へ描画
```

## Segment 型

```rust
enum Segment {
    Plain(Vec<u8>),  // 英語素通し ＋ 文書外バイト（報酬コード等）
    Xml(Vec<u8>),    // 完結した整形式 XML 文書 1 つ分のバイト
}
```

- **`Segment::Raw` は作らない**: 報酬コードをまだ何も活用していない以上、
  専用 variant は YAGNI 違反。当面 `Plain` で素通す。将来 reward code を
  収集・活用する必要が生じた時点で初めて分離を検討する。
- **`Segment::Malformed` は作らない**: malformed は「表示すべき出力」ではなく
  「契約違反 = エラー」。Segment 列に混ぜず `Result::Err` で上位へ伝播させる。

## crate 選定

| crate | 役割 | 理由 |
|---|---|---|
| **quick-xml** 0.40 | 枠取り（streaming） | 文書境界の検出は streaming できるパーサでしか務まらない。roxmltree も serde 系も「完結した文書」前提なので枠取り不可。quick-xml は標準・高速・現役（直近DL 71M）。 |
| **roxmltree** 0.21 | 型付け（DOM 走査） | 完結文書をランダムアクセスで走査でき、手マップが快適。直和型を子要素名 match で明示的に組める（serde の弱点を回避）。 |

**この2つは「代用不能な相補」ではなく、1つの crate でもやれる仕事を割って
各 crate に得意な半分を渡した構成。** 代償として文書は2回トークナイズされる
（quick-xml が境界検出に1回、roxmltree が DOM 構築に1回）。文書が小さく学習
目的の本件では無視できるトレード。（anyhow+thiserror のような本質的 disjoint
ではなく、有名な定番イディオムでもない点に注意。）

### 不採用

- **serde 宣言的**（serde-xml-rs / quick-xml+serde）: この文書の核心である
  「要素名で分岐する直和型」（`<condition>` の pristine|broken、
  `<description>` の text|`<redacted/>`）は serde-xml の歴史的弱点。derive 属性
  との格闘が「型に綺麗に落とす」学習を食う恐れ。serde は素直な文書で学ぶ。
- **xml-rs**: streaming できるが quick-xml より遅く非 idiomatic。
- **yaserde**: 採用少・更新も古い。

## 枠取りアルゴリズム（Stage 1）

quick-xml の Reader をイベント列で回し、深さカウンタを持つ:

- `Start` → 深さ +1
- `End`   → 深さ -1
- `Empty`（`<redacted/>` 等の自己閉じ）→ ±0
- 深さが 0 → 1 になった点が文書の開始、再び 0 に戻った点が文書の完結

トークナイズ（タグ・属性・エスケープ・自己閉じの判別）という難所は quick-xml に
委ね、こちらは深さカウンタ数行だけを持つ。文書内の整形式違反は Reader が `Err`
を返すので、それをコンテキスト付きで上位へ伝播。

### 実装時に確かめる経験的事項（設計の分岐ではない）

quick-xml が「ルート前のテキスト（報酬コード）」を無害な `Text` イベントとして
手渡すのか、プロローグ違反として `Err` にするのかは未確認。**どちらでも設計は
不変**:

- Text でくれる → 深さ0の Text を拾って `Plain` に。
- Err にする → 最初の `<` まで自前で peel して `Plain`、以降を quick-xml へ。

## 段階

### Stage 1: パニック解消 + 枠取り

- `Scanner` の `Mode::Xml => todo!("XML をまとめる")` を quick-xml 枠取りに置換。
- 完結した整形式文書を `Segment::Xml(bytes)` で emit。
- 文書内違反は `Err`。
- `Renderer` の `Segment::Xml(bytes) => todo!()` を当面 `out.write_all(bytes)`
  （エコー）に。これでパニックが消える。
- テスト: クリーンな `<success>…</success>`（報酬コードなし）で枠取りを検証。
  chunk 分割（タグ途中で切れる）ケースも。
- **完了条件**: XMLモードに入ってもパニックせず、XML がそのまま表示される。

### Stage 2: 文書外バイトの処理

- Xmlモードで深さ0のバイト（報酬コード・空行）を `Segment::Plain` として emit。
- 複数文書の連続、文書と文書外バイトの interleave に対応。
- Stage 1 の枠取りに自然に畳まれる可能性あり（独立 Stage にするか実装時判断）。
- **完了条件**: 報酬コードが混じっても文書が正しく切り出され、報酬コードは
  素通しされる。

### Stage 3: 型木へのパース（着手時に詳細設計）

- roxmltree で `Segment::Xml(bytes)` を DOM 化し、手で歩いて Rust 型へ。
- 直和型を子要素名の match で構築:
  - `enum Condition { Pristine, Broken { condition: Box<Condition>, missing: Vec<Kind> } }`
  - `enum Description { Text(String), Redacted }`
- `Room` / `Item`（再帰: `piled_on: Vec<Item>`）/ `Kind` / `Adjective` 等。
- Renderer が型木を描画（将来の TUI / 翻訳 DB / ファイル保存の土台）。
- **型の正確な定義は本 Stage 着手時に実データへ当てて設計する**（型は実物に
  対して shape させるのが学びの核。本 spec ではスケッチに留める）。

## 非目標（このサイクルの範囲外）

- TUI / html-css ベース描画（`docs/TODO.md` 参照、将来）
- 翻訳 DB / i18n
- `name` / `description` のファイル保存（Stage 3 完了後）
- カスタムコマンド `make keypad`（最短経路探索）

## 実装方針

ユーザーが学習目的でコードを書く。AI は `src` を編集せず、設計・コーチング・
チャット上での提示に留める。各 Stage は動くものを確認しながら順に進める。
