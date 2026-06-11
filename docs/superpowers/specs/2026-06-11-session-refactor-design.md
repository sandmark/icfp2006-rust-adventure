# Session リファクタ設計 (2026-06-11)

## 背景

`adventure` は XML でクエリ結果を吐く「思考エンジン（≒ db サーバー）」と捉えられる。
本 crate はそれを駆動し、出力を解釈・描画（将来はログ・経路探索）する**クライアント側**である。

現状 `src/lib.rs` には spec 無しで生やした構造があり、責務が混線している:

- `run()` — 子プロセス（インタプリタ + VM）を spawn して `Child` を返す自由関数
- `relay()` — reader → writer 素通し（親stdin → 子stdin に使用）
- `Renderer` — **3役を兼ねている**:
  1. `<success>` マーカー検出（プロトコル境界の検出）
  2. English / Xml の `mode` 管理
  3. 描画
- `main.rs` — `child_stdin` / `child_stdout` を `take()` し、スレッドとループで手配線

問題:
- `main.rs` に I/O 配線と `child_*` が露出している。
- 「XML を Rust の型に落として計算する」前提なら、上記 1・2 は描画ではなく**手前の別レイヤー**。
  `Renderer` がコンテキストスイッチ（`mode`）を持つのは責務違反。

## 目的（このイテレーションの射程）

**構造のリファクタのみ。新機能はゼロ。動作は1バイトも変えない。**

- オーケストレーター構造体 `Session` を切り出し、`main.rs` から配線と `child_*` を消す。
- `Renderer` からモード／プロトコル境界検出を `Scanner` へ分離する。
- 「XML を型に落とす」ための受け皿（seam）として `Segment` 型を用意する。
- `src/lib.rs` 肥大化対策にモジュール分割する。

### 非目標 (Non-Goals)

- XML パースの実装（`Mode::Xml` は引き続き `todo!`）。
- ログ・経路探索の実装。
- consumer を差し込む trait の導入（seam は `Segment` 型のみ。trait は Logger/Pathfinder が現れた時に初めて生やす — YAGNI）。
- I/O 並行モデルの変更（writer スレッド + main 読み取りループ、EOF で writer が心中する現挙動を維持）。

## アーキテクチャ：3層パイプライン

```
親stdin ──relay(thread)──▶ 子stdin
子stdout ─▶ Session.loop ─read─▶ Scanner.feed(chunk) ─▶ [Segment...] ─▶ Renderer.render(seg, &mut out)
                                                                          Plain → 素通し / Xml → todo
EOF ─▶ ループ脱出 ─▶ Session 終了（writer スレッドは心中）
```

`Session` が `Scanner` と `Renderer` を所有して束ねる。

```
Session            ← プロセス + I/O配線 + 読み取りループを所有。main はこれだけ触る
  ├─ Scanner       ← mode 管理 + マーカー検出（旧 Renderer の責務 1・2）
  └─ Renderer      ← 描画だけ（旧 Renderer の責務 3）。状態を持たない
```

## コンポーネント

| 型 | 責務 | 状態 | 依存 |
|---|---|---|---|
| `Session` | 子プロセス生成・I/O配線・読み取りループの所有と駆動 | `Child` / `Scanner` / `Renderer` | `Scanner`, `Renderer`, `relay` |
| `Scanner` | バイト列を食って English/Xml を判定し `Segment` に切り出す | `mode` + 持ち越し `buf` | なし（純ロジック） |
| `Renderer` | `Segment` を受け取って描画する | なし | なし |
| `Segment` | `Plain(Vec<u8>)` / `Xml(Vec<u8>)` の seam | — | — |
| `relay` | stdin → 子stdin のバイトポンプ | なし | — |

### 設計契約（シグネチャ）

実装本体は実装フェーズで書く。ここでは型の輪郭（契約）のみを定める。

```rust
// scanner.rs
enum Mode { English, Xml }

pub enum Segment {
    Plain(Vec<u8>), // English 出力。素通し対象
    Xml(Vec<u8>),   // 完結した XML ドキュメント（将来パース対象）
}

pub struct Scanner { /* mode, buf */ }
impl Scanner {
    pub fn new() -> Self;
    /// 1 チャンクを食い、その時点で確定した Segment 列を返す。
    /// マーカー途中（最大 MARKER.len()-1 バイト）は内部に持ち越す。
    pub fn feed(&mut self, chunk: &[u8]) -> Vec<Segment>;
}

// renderer.rs
pub struct Renderer; // 状態なし
impl Renderer {
    pub fn new() -> Self;
    pub fn render(&mut self, seg: &Segment, out: &mut impl Write) -> io::Result<()>;
    // Plain → out へ書く / Xml → todo!
}

// session.rs
pub struct Session { /* child, scanner, renderer */ }
impl Session {
    pub fn new(interpreter: &str, vm: &str) -> anyhow::Result<Self>; // spawn を内包（旧 run）
    pub fn start(self) -> anyhow::Result<()>;                        // 配線 + ループ + EOF 終了
}

fn relay(reader: impl Read, writer: impl Write) -> io::Result<u64>; // 現状維持（session 内へ）
```

### main.rs（理想形）

```rust
fn main() -> Result<()> {
    let args = Args::parse();
    Session::new(&args.interpreter, &args.vm)?.start()
}
```

## ファイル分割

```
src/
  lib.rs       … crate doc + mod 宣言 + pub use（薄い玄関）
  session.rs   … Session, relay, spawn ヘルパ（旧 run）
  scanner.rs   … Scanner, Mode, Segment, MARKER, find_subslice
  renderer.rs  … Renderer
  main.rs      … 上記の理想形
```

## エラー処理

- 現状維持。`anyhow` / `io::Result`。
- EOF = 正常終了。
- writer スレッドは detach し、main 終了時に心中する（既存挙動）。

## テスト

挙動・意図とも不変。基本は**移設**。

- `scanner`: マーカー検出 / チャンク跨ぎ（split）/ mode 遷移
  （現 `switch_xml_mode`, `switch_xml_mode_splited` をここへ。`feed` の戻り値検証も追加可）
- `renderer`: `Segment::Plain` を渡すとそのバイトが `out` に書かれる（新規・軽量）
- `relay`: verbatim 素通し（現状維持）
- `spawn`: 存在しないインタプリタで `Err`（現状維持、`Session::new` 経由に書き換え）

## 動作不変性の根拠

- `Scanner.feed` が現 `Renderer.feed` の English ブランチ（素通し + 持ち越し + mode 切替）を再現。
- `Renderer.render(Plain)` が現素通しを再現。
- `Mode::Xml` は引き続き `todo!`。
- I/O 並行モデル・心中挙動は不変。

→ 既存テストは緑のまま維持されることが正しさの証明となる。
