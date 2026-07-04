# 依存解決ソルバを Rust で書く ― 実践読本

> **この本の立ち位置**
> `docs/superpowers/specs/2026-07-04-dependency-solver-design.md`(以下「設計仕様」)を、
> Rust で手を動かしながら読める形にほどいた実践書です。設計仕様は「何を作るか」を述べ、
> 本書は「Rust でどう型に落とすか」「どこが自分で埋める楽しみとして残っているか」を案内します。
>
> **本書だけで核心3点(第0章)は実装できます。** 設計仕様の `§N` を参照する箇所には、その主張を
> その場で 1〜2 文に要約して添えてあるので、原典を開かなくても読み進められます。tutorial の
> 先にある大規模データ(第0章で定義する Chicago)まで進むときだけ、原典が要ります。
>
> **読者へ** ― プログラミング設計・TDD・抽象化はご存じの前提で進めます。ここで足す説明は
> 二種類だけです。ひとつは **Rust 低レイヤ固有の差分**(所有権・借用・`Box`・move・スタックと
> ヒープ・`drop` の効き方)、もうひとつは **この codebase 固有の事情**。一般論の再説明はしません。
>
> **重要な約束** ― 本書は「読者が自分で TDD 実装して楽しむ」ための本です。`solve` の中核
> (探索か貪欲か、順序の決め方、6枠への詰め方)は **わざと穴のまま** 残してあります。型・
> シグネチャ・テストの骨格までは示しますが、アルゴリズムの完成形は渡しません。穴は
> `// TODO:` と「【穴】」で明示します。まだ設計として決着していない点は「【コラム】」で
> 選択肢を並べます。

---

## 第0章 この本が扱う範囲

設計仕様のソルバは 3 コマンド(`walk` / `solve` / `make`)から成りますが、本書が正面から
扱うのは `solve` の **核心 3 点**(設計仕様 §8「最初の実装範囲」= tutorial で試せる範囲)だけです。

1. **再帰的な依存分解** ― `keypad → motherboard → A-1920-IXB → radio …` と壊れた木を降りる。
2. **スロット条件で止まる再帰** ― 各スロットが要求する `condition` が、そこに挿す部品の
   目標状態を決める。`radio` を「壊れたまま・`antenna` 欠品」で止め、**完成させない**。
   本書でいちばん力を入れる罠です。
3. **LIFO 掘り＋6枠制約＋在庫ゴミ(地面の山に紛れる不要品)の `incinerate`** ― 地面の
   後入れ先出しの山を `take` で掘り、同時に抱える枠数の最大を 6 以下に収めるスケジューリング。
   6 枠はインベントリ(手持ち)の上限で、ゲーム規則です。

`walk`(部屋の巡回)・`make`(手順を `UM` へ流す。`UM` は手順を実行する対象 = シェルの先にいる
本体)・部屋グラフのデータ構造は、この 3 点を位置づけるための最小限の背景としてだけ触れます。
深追いはしません。

### 前提として持っている道具

ソルバはゼロからは始めません。**シェル本体の `parser.rs` が、VM の XML 応答を Rust の型に
変換済み**です(`CLAUDE.md` 参照。本書だけで読めるよう、要る型は下に再掲します)。ソルバは
この型を入力に受け取ります。関係するのは次の 4 つ。

```rust
// src/parser.rs(既存)― ソルバはこれらを「材料」として受け取る。
// 4 つとも #[derive(Debug, PartialEq)] 付き。PartialEq があるので == で比較でき、
// Debug があるので assert_eq! で中身を表示できる(第2章・第5章で効いてくる)。

#[derive(Debug, PartialEq)]
pub enum Condition {
    Pristine,
    Broken(Broken), // ← バリアント名 Broken と、それが包む型 Broken は別物(同名なだけ)
}

#[derive(Debug, PartialEq)]
pub struct Broken {
    condition: Box<Condition>, // 欠品を全部埋めた「後」の状態(さらに Broken でありうる)
    missing: Vec<Kind>,        // 今この段で欠けているスロット
}

#[derive(Debug, PartialEq)]
struct Kind {          // pub が無いのは src の忠実な再現(Kind は非公開。書き忘れではない)
    name: String,
    condition: Condition, // ← 目標状態。ここが罠の核心(第3章)
}

#[derive(Debug, PartialEq)]
pub struct Item {
    pub name: String,
    pub adjectives: Vec<Adjective>,
    pub condition: Condition,
    pub piled_on: Option<Box<Item>>, // 地面の山(LIFO)を単方向リストで表す
    // description は本書では割愛
}
```

> **NOTE(可視性)**: `Broken.condition` / `Broken.missing`、および `Kind` とそのフィールドは src で
> **非公開**です。ソルバはこれらを読んで分解しますが、他モジュールからそのまま触れるとは限りません。
> どう可視性を設計して読み出すか(フィールドを `pub` にする / 読み取りメソッドを足す / 同じモジュールに
> 置く)は **読者の課題**として残します。ここでは答えを書かず、詰まる場所だけ示しておきます。

この 4 つに **Rust 低レイヤの見どころが 2 つ** すでに埋まっています。第1章で回収します。

- `Broken.condition` が `Box<Condition>` になっている理由(生の `Condition` ではだめ)。
- `Item.piled_on` が `Option<Box<Item>>` になっている理由(山を型でどう表すか)。

fixture は `local/tutorial.xml`。初期エリアの 2 部屋("Room With a Door" と "Junk Room")を
`look` / `go` で調べた実データで、`keypad` を `pristine` にするのがゴールです。テスト(第5章)は
これを parse して `solve` の出力手順列を検証します。

---

## 第1章 ドメインを Rust の型で表す

### 1.0 先に、低レイヤの3点セット

本書がこの後で繰り返し要求する Rust 固有の概念を、ここで先にまとめて渡します。詳細は各章で
回収するので、いまは要旨だけ掴んでください(`enum` / `match` / 再帰などご存じの道具は説明しません。
新しいのは次の 5 つだけです)。

- **所有権** ― どの値にも所有者がちょうど 1 つ。所有者(変数)がスコープを抜けると、値も
  **`drop`(破棄)**される。GC 任せだった破棄の時点が、Rust では「スコープの終わり」で決まります。
- **借用** ― 値を読む(あるいは書き換える)だけなら、所有権を移さず **借りる**(`&T` / `&mut T`)。
  借りている間、元の所有者はそのまま。読むだけの共有借用(`&T`)は同時に何本でも持てます。
- **move** ― 値を関数へ値渡ししたり別の変数へ代入すると、**所有権がそちらへ移り、元の変数は
  もう使えなくなる**。これが `combine A B` の「B を消費する」に効きます(第4章)。
- **スタックとヒープ** ― Rust は既定で値を **スタック**に置き、そのためコンパイル時に
  「その値のサイズ(バイト数)」が要る。サイズが静的に決まらない・大きい・再帰する値は
  **ヒープ**へ逃がし、スタックには **ポインタ 1 本**だけ置く。その逃がし役が `Box`(第1章1.1)。
- 回収の地図: **借用**は第2章で `required_fills`、**move と `drop`**は第4章で `combine` を
  題材に、それぞれ実物で詰めます。

### 1.1 状態は enum ― そして再帰には `Box` が要る

アイテムの状態は `pristine`(欠品なし)か `broken`(欠品あり)の 2 値。`broken` は
「欠けているスロットの一覧」と「それを全部埋めた後の状態」を持ち、後者はさらに `broken`
でありえます(多段 broken)。この構造がまさに前掲の `Condition` / `Broken` です。

ここで低レイヤの差分がひとつ。`Broken` の `condition` フィールドは、なぜ生の `Condition`
ではなく `Box<Condition>` なのか。

Rust は既定で値をスタックに置くので、型を定義した時点で「その値ひとつの大きさ」が確定して
いなければなりません。ところが次の形は大きさが決まりません。

```rust
// これはコンパイルが通らない
struct Broken {
    condition: Condition, // Condition の中に Broken、その中にまた Condition …
    missing: Vec<Kind>,
}
```

`Condition` が `Broken` を含み、`Broken` が `Condition` を含む ― この入れ子は静的には
底が見えません。コンパイラは値ひとつのバイト数を確定できず、`recursive type has infinite size`
で止まります。`Box<Condition>` にすると、フィールドの実体は **ヒープ上に確保され、構造体が
抱えるのはそこを指すポインタ 1 本**(サイズが決まっている)になります。これで大きさが有限に
閉じ、再帰的な型が定義できる。

要点は「多段 broken を表現したいなら、どこかで `Box` を挟んで再帰を切る」。この一手は
今後もあらゆる木構造で顔を出します。`missing` の側が `Vec<Kind>` なのも同じ理屈で、`Vec` は
中身をヒープに持つのでサイズが固定です。

> **【コラム】`Rc`/`Arc` ではなく `Box` でよいか**
> `Box` は「唯一の所有者がヒープの値を 1 つだけ持つ」箱です(所有権は §1.0)。同じ木のノードを
> 複数の親から共有したくなったら `Box` では表せず、`Rc`(単一スレッド)や `Arc`(複数スレッド)の
> 出番になります。本書のソルバは各アイテムの状態木を **1 つの所有者がまるごと持つ** 前提なので
> `Box` で足ります。共有が要るのは「item → (部屋, 段) 索引」の側(§1.4)ですが、そこは所有ではなく
> 参照や添字で済ませられます。まずは `Box`、共有が必要になってから重い型に上げる、で構いません。

### 1.2 アイテムの同一性 ― `(name, adjective)` を型にする

設計仕様 §2 では、アイテムの同一性は `(name, adjective)` の組で決まります(同じ `name` でも
`adjective` が違えば別バリアント。例: `transistor` の blue "PNP-complete" と red "NPN-complete")。
索引のキーや在庫の突き合わせに使うので、生の `String` を持ち回すのではなく専用の型にします。

```rust
/// アイテムの同一性(バリアント)。索引・在庫のキーになる。
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Variant {
    name: String,
    adjective: Option<String>, // adjective 無しのアイテムがある(例: bolt, screw)
}
```

`#[derive(...)]` は **トレイト実装を自動生成する仕組み**です(`derive` = 導出)。ここで並べた
トレイトの役割はそれぞれ違います。`PartialEq` / `Eq` / `Hash` が揃うと、この型をそのまま
`HashMap` のキーや `HashSet` の要素にできます(キーの条件が「等価判定できて、ハッシュを取れる」
だから)。`Clone` は値を複製したいとき(同じバリアントを 2 か所で使うなど)、`Debug` は
`assert_eq!` などで中身を表示したいときに要ります。同一性を「フィールドの一致」と定義したい
今回は、手書きの `impl` を書かず derive で十分です。

`adjective` を `Option<String>` にしたのは観測を型にしただけですが、ここには
**未決の設計判断** が乗っています。

> **【コラム / 未確定: tutorial では踏まない(Chicago で効く)】adjective 無しの同一性(設計仕様 §2 NOTE)**
> `adjective` を持たないアイテム(`bolt` など)の同一性が `name` だけで決まるのか、
> `adjective: None` まで込みで決まるのかは、設計仕様でも **未定義**。実データで確かめる論点です。
> 型の上では選択肢が 2 つあります。
> - **A案**: 上のように `adjective: Option<String>` にし、`None` も同一性の一部として扱う。
> - **B案**: `enum Identity { Named(String), Variant { name: String, adjective: String } }` のように、
>   adjective の有無を型で分岐させる(`Variant { .. }` は enum の **構造体バリアント**記法。
>   バリアントが名前付きフィールドを持つ書き方で、tuple バリアントと選べます)。
>
> `local/tutorial.xml` では `bolt` / `screw` などは `<adjectives>` が空ですが、どちらのアイテムも
> 名前が衝突しないため、どちらの案でも tutorial は通ります。効いてくるのは、`name` が同じで
> `adjective` だけ違うバリアント群が大量に出る **Chicago**(tutorial より大きい実データ。
> `local/chicago-rooms.xml`。本書の射程外の大規模データで、`X-1623-GTO` は 9 種など)に当たったとき。
> A案が破綻しないかを実データで検証してから確定するのが安全です。**ここは埋めずに開けておきます。**

### 1.3 スロットは「名前＋目標状態」で、バリアントを指定しない

スロット(欠品)は `Kind`、すなわち `(name, condition)` です。**`adjective` を持たない**
点が肝心で、これは「名前 N で状態 C のアイテムを寄こせ、色(バリアント)は問わない」という
ゲーム規則をそのまま型で言い切っています。第0章の `Kind` を再掲すると、主張は「フィールドの
不在」にあります。

```rust
struct Kind {
    name: String,
    condition: Condition, // ← このスロットが要求する目標状態
    // adjective は無い ← 「バリアントは指定しない」を型で表現している
}
```

「フィールドが無い」ことが仕様の主張になっているのは気持ちのいい設計です。`Kind` に
`adjective` を足したくなったら、それは仕様を誤解している合図になります。

### 1.4 地面の LIFO ― パーサの `Option<Box<Item>>` と、ソルバの `Vec`

地面は後入れ先出しの山で、`take` で取れるのは一番上だけ(下を出すには上から順に `take`)。
パーサはこの山を `Item.piled_on: Option<Box<Item>>` という **単方向リスト**で受け取ります
(`piled_on` を手繰ると 1 つ下のアイテム、`None` が底)。これは XML の入れ子(`<piled_on>`)を
素直に写した形で、`Box` は §1.1 と同じ「再帰を有限サイズに切る」役です。

ただしソルバが山を**掘る作業**をするときは、この形は扱いにくい。掘るとは「上から順に外す」
= 末尾操作の繰り返しで、リンクリストより **`Vec` の `pop()` が段違いに素直**です。そこで
Snapshot 側では山を `Vec` に持ち替えます。パーサの平坦化された出力(`CLAUDE.md` にある通り、
パーサ側にも `piled_on` を平らな `Vec<Item>` に均す処理があります)を、そのまま `GroundStack`
の `items` に詰めるだけの橋渡しを 1 つ書く、と考えてください。

**山と索引と部屋グラフを束ねるのが `Snapshot`** です。設計仕様 §5 の 3 要素(部屋の有向グラフ /
各部屋の `GroundStack` / `item → (部屋, 段)` 索引)を、そのまま struct のフィールドにします。

```rust
/// walk が 1 度だけ作り、以後は不変。solve / make の入力。
struct Snapshot {
    // ① 部屋の有向グラフ: 各部屋から「方角 → 隣室」への辺。逆向きは非対称でありうるので向きごとに持つ。
    graph: HashMap<RoomId, HashMap<Direction, RoomId>>,
    // ② 各部屋の地面(LIFO)
    stacks: HashMap<RoomId, GroundStack>,
    // ③ item → (部屋, 段)。段 = 一番上からの深さ = 掘るコスト。まずは単純形(後述コラムに未確定)
    index: HashMap<Variant, (RoomId, usize)>,
}

/// 一部屋の地面。items の末尾を「一番上」とする(末尾 = 上なら push/pop だけで LIFO が完結)。
struct GroundStack {
    items: Vec<Item>,
}

impl GroundStack {
    /// 一番上を取る(take 1 回に対応)。無ければ None。
    fn take_top(&mut self) -> Option<Item> {
        self.items.pop() // 末尾 = 上に統一。段(掘るコスト)は「上から何個目か」= take 回数。
    }
}
```

`RoomId` は部屋の識別子(型は後で定義。整数の別名でも文字列でもよい)。ここで低レイヤの選択が
出ます。`Vec::pop()` は末尾を O(1) で外し、`Vec::remove(0)` は先頭を外すぶん後ろを詰めるので
O(n)。山の規模は小さい(tutorial で 15 個)ので実務上どちらでも動きます。それでも「上 = 末尾」に
採れば `push`/`pop` だけで LIFO が完結し、`Box` のリンクリストより軽い。**索引の段(`index` の
`usize`)は「上から数えた深さ」に固定**し、`GroundStack` の末尾 = 上と向きを揃えておきます。

> **【コラム / 未確定: tutorial では踏まない(Chicago で効く)】item 索引のキーと重複(設計仕様 §5 NOTE)**
> 上の `index` は値を 1 個の `(部屋, 段)` にした単純形です。ただし **同一バリアントが複数の山・段に
> 現れる**場合、これでは取りこぼします。tutorial の `transistor` は blue と red で別バリアントなので
> 衝突しませんが(→ tutorial では踏まない)、Chicago では起こりえます。そのときは
> `HashMap<Variant, Vec<(RoomId, usize)>>`(同一バリアントの全出現)に上げ、どの出現を使うかを
> 「掘るコスト(段)が浅い方から」などの規則で選ぶことになります。**その選択規則は未定義**。
> 骨格は単純形を置き、確定はこのコラムに残します。

### 1.5 手順列は `Step` の `Vec`

`solve` の出力(`make` がそのまま `UM` へ流すもの)は、4 種類の操作を一列に並べたものです。
enum で表せば、後段の照合もテストの検証も `match` 一発で回せます。

```rust
/// solve が出力する 1 手。手順列は Vec<Step>。
#[derive(Debug, PartialEq, Clone)]
enum Step {
    Take(Variant),                              // 地面の一番上を取る
    Combine { target: Variant, part: Variant }, // target に part を挿す(part を消費)
    Incinerate(Variant),                        // 手持ちを 1 つ燃やす
    Go(Direction),                              // 部屋移動
}

/// "north" 等の方角トークン。1 フィールドだけのタプル構造体で別の型を作る(新型ラッパ)と、
/// ただの String と取り違えにくい。
#[derive(Debug, PartialEq, Clone)]
struct Direction(String);
```

`Step` に `#[derive(PartialEq)]` を付けておくと、テストで期待手順列と `assert_eq!` で
突き合わせられます(第5章)。

---

## 第2章 再帰的な依存分解 ― 壊れた木を降りる

`keypad` を作るには、その欠品 `motherboard` と `button` を用意する。`motherboard` を作るには
その欠品 `A-1920-IXB` と `screw` を用意する ― と、`Condition::Broken` の `missing` を手繰って
降りていくのが依存分解です。底(葉)は「何も欠いていない = すでに用意できるアイテム」。

設計仕様 §5 の要点は **「依存木を別に持たない」**こと。木は対象アイテムの `condition`
(`Broken` の入れ子)そのものから導きます。Snapshot に木を複製しません。分解とは
「`Condition` を再帰的に読む」ことに等しい。

型の上で、分解の骨格はこうなります(**本体は穴**)。

```rust
/// item を「目標状態 goal」にするのに、どのスロットを埋める必要があるかを返す。
/// 返り値は「埋めるべきスロットとその中身の再帰的な要求」の並び。
///
/// 【穴】本体は読者が TDD で埋める。ヒントは第3章。
fn required_fills(current: &Condition, goal: &Condition) -> Vec<Fill> {
    match (current, goal) {
        // 目標が今の状態と同じなら、もう埋めるものは無い(= 底)。
        // == が使えるのは Condition に PartialEq が derive してあるから(第0章)。
        (c, g) if c == g => Vec::new(),

        // 今 pristine なのに goal が broken を要求…は起こらない前提。
        // (分解は「今 broken なものを goal まで持っていく」向きにだけ進む)
        (Condition::Pristine, _) => {
            // TODO: この分岐が起きたら供給ミスか木の読み違い。どう扱うか決める。
            todo!()
        }

        // 本命: 今 broken。goal に向けて「埋めるべきスロット」を選ぶ。
        (Condition::Broken(now), goal) => {
            // TODO: ここが核心(第3章)。now.missing のうち
            //       「goal では埋まっているスロット」だけを対象に選び、
            //       各スロットの Kind.condition を目標にして再帰する。
            let _ = now;
            let _ = goal;
            todo!("第3章の罠を踏まないよう、goal 側の欠品との差を取る")
        }
    }
}

/// 1 つのスロットを埋める要求。中身(part)も再帰的に解決される。
struct Fill {
    slot: Kind,             // どのスロットを(name + 目標 condition)
    // part をどのバリアントで満たすかは索引と在庫から決まる(第4章・コラム)
}
```

`match (current, goal)` と 2 値を組で受けているのが Rust らしい書き味です。`(c, g) if c == g`
のようにガード付きアームで「同型なら底」を先に畳み、残りを構造で場合分けする。ここで
`current` / `goal` を `&Condition` の **借用**で受けているのは、分解が状態を消費せず
「読むだけ」だから(借用は §1.0)。所有権を取らないので、同じ `Condition` を何度でも読めます。

ひとつ非自明な点。`required_fills` は `current: &Condition`(借用)を受けるのに、`match` の
アームでは `Condition::Broken(now)` と書けて、`now` は自動的に **参照**(`&Broken`)に束縛されます。
借用を `match` で分解すると束縛も参照になる、という Rust の既定の振る舞いで、`&` や `ref` を
自分で書き足す必要はありません。

> **【穴】** `required_fills` の本体(`now.missing` と `goal` の差を取り、各スロットへ再帰する
> 部分)は完成させません。ここを埋めるのが第3章の主題であり、本書でいちばんの見せ場です。

底で再帰が止まる保証について。設計仕様の作業ノートは「依存の再帰には底がある(葉に着く)」を
**本命の仮説**として置いています(観測に基づく仮説であって、証明された事実ではない)。実装は
「有限で止まるはず」を仮説と承知の上で進め、テスト(第5章)で tutorial の木が実際に底を突く
ことを確かめます。

> **【コラム / 未確定: tutorial では踏まない(循環が無い)】木の同一性と、深さの底**
> 「底に着く」は観測からの仮説で、`pristine` か `broken` かとは無関係 ―
> 本質は「そのアイテムが何かを必要とするか」だけ、という立場です。Chicago 規模で循環(A が B を
> 要求し B が A を要求)が万一あれば `required_fills` は止まりません。保険として「訪問中のスロット
> 集合」を持ち回り、再訪を検出したら打ち切る、という実装もありえます。tutorial では循環が無いので
> 不要ですが、**「底がある」を確定事実として焼き込まない**立場を取り、ここはコラムに留めます。

---

## 第3章 スロット条件で止まる罠 ― `radio` を完成させない

本書の心臓部です。**各スロットの目標状態は、そのスロットの `condition` が決める**。
トップレベルの `item` だけが `pristine` を目標に取り、サブ部品の目標は「親スロットの
`condition`」に従います。これを外すと詰みます。

まず、`keypad` の依存木を一枚にしておきます(設計仕様 §8)。**radio が `antenna` 欠品のまま
止まる**のが、木のどこで起きるかを目で押さえてください。

```
keypad〈欠 motherboard, button〉
├─ motherboard〈欠 A-1920-IXB, screw〉
│  ├─ A-1920-IXB〈二重 broken〉
│  │  ├─ radio      目標 = broken・欠 antenna → transistor だけ挿す(antenna は挿さない) ★ここで止める
│  │  ├─ processor  目標 = pristine → cache を挿す
│  │  ├─ bolt       pristine(そのまま)
│  │  └─ transistor pristine(内欠。外欠 radio/processor/bolt を埋めた後に現れる)
│  └─ screw         pristine(そのまま)
└─ button           pristine(そのまま)
```

この図は「何が要るか」であって、掘る順や枠の使い方(= スケジュール、第4章)は含みません。

### 3.1 罠の実物 ― A-1920-IXB の中の radio

`local/tutorial.xml` の `A-1920-IXB` を読むと、その欠品のひとつ `radio` スロットが要求する
`condition` は **`pristine` ではなく `broken`(`antenna` 欠品)**です。

```xml
<!-- A-1920-IXB の missing の中の radio スロット(抜粋・字下げ簡略) -->
<kind>
  <name>radio</name>
  <condition>
    <broken>
      <condition><pristine/></condition>
      <missing>
        <kind><name>antenna</name><condition><pristine/></condition></kind>
      </missing>
    </broken>
  </condition>
</kind>
```

一方、地面の山にある `radio` は **`broken`(`transistor` と `antenna` の 2 つ欠品)**。
ここで素直に「部品は全部 `pristine` にしてから挿す」と考えると、`radio` に `transistor` も
`antenna` も挿して完成させてしまう。これは **バグ**です。A-1920-IXB が欲しいのは
「`antenna` が欠けたままの radio」なので、`transistor` **だけ**を挿し、`antenna` は挿さずに
止める ― これが「スロット条件で止まる」。

型で言えば、`required_fills(current, goal)` に渡す `goal` が、この場面では **概念的に**
`Condition::Broken(Broken { condition: Box::new(Condition::Pristine), missing: [antenna] })`
になります(`missing` は本来 `Vec<Kind>`、`[antenna]` はその中身を略記した擬似コード)。
`current`(山の radio)の欠品は `{transistor, antenna}`。**差 `{transistor, antenna} − {antenna}
= {transistor}`** が「今回埋めるべきスロット」です。`antenna` は goal 側でも欠けているので
触らない。この「差を取る」考え方が罠を回避する核心で、`required_fills` の穴に入れるべき
**手がかり**はまさにこれです(差集合という着想であって、そのまま貼れば動く完成コードでは
ありません ― 実際の突き合わせ・再帰の駆動は読者が書きます)。

### 3.2 目標を取り違えない設計 ― `goal` を必ず引数で運ぶ

罠を型で防ぐコツは、**「目標状態」を暗黙のグローバルにせず、必ず引数 `goal` として下へ渡す**
こと。`solve keypad` の入口だけが `goal = Pristine` を与え、以降の再帰は各 `Kind.condition` を
そのまま次の `goal` にして降ります。

```rust
/// item を goal 状態にする手順を組む。トップの呼び出しだけ goal = Pristine。
fn plan(item: &Item, goal: &Condition /* , snapshot, state */) -> Vec<Step> {
    let fills = required_fills(&item.condition, goal); // ← goal を明示的に運ぶ
    for fill in &fills {
        // 各スロットの目標は、そのスロットが要求する condition そのもの。
        let sub_goal = &fill.slot.condition; // ← ここで pristine を勝手に上書きしない！
        // TODO: sub_goal を goal として部品を用意する手順を再帰的に積む。
        //       この「積む順序」と「6枠に収める」は第4章(= 大きな穴)。
        let _ = sub_goal;
    }
    // 【穴】fills から Step 列を組む本体は第4章のスケジューラに委ねる。
    todo!("required_fills の結果を Step 列へ ― 順序と枠は第4章")
}
```

`sub_goal` を `&fill.slot.condition` から取り、**決して `Pristine` に差し替えない**。
この 1 行が「radio を完成させない」を保証します。もし途中で「部品はとにかく pristine に」と
書いてしまえば、それは §6 のいう「一意に決まる依存を解けていない = バグ」。テストで
`radio` への `antenna` 挿入が手順列に**現れないこと**を検証して守ります(第5章)。

### 3.3 二重 broken を降りる ― A-1920-IXB 自身の目標

A-1920-IXB は「二重 broken」(設計仕様 §8)。外欠 `{radio〈欠 antenna〉, processor, bolt}` を
埋めると内欠 `{transistor}` が現れ、それも埋めて `pristine` になります。`Broken.condition`
(`Box<Condition>`)がまさに「外欠を埋めた後の状態」= 内欠を抱えた `Broken` を指しており、
`required_fills` の再帰が `&*now.condition` のように **借用でたどる**ことで自然に次の段へ
降りられます。`*` で `Box` の中身を取り出し、`&` で借り直す、という 2 段の操作です。所有権を
取らずに読み進める借用の使いどころです。

`solve keypad` 全体でこの罠が守られると、`A-1920-IXB` の下では
「`radio` に `transistor` だけ挿す(`antenna` は挿さない)」「`processor` に `cache` を挿して
pristine」「`bolt` はそのまま」「外欠が埋まって現れた内欠 `transistor` を挿す」という降り方に
なります。`transistor` は需要 2(radio スロット＋A-1920-IXB 内欠)に対し供給 2(blue と red)で、
どちらの色でもスロットは受けます(バリアント無指定 = §1.3)。

> **【コラム / 未確定: tutorial では踏まない(Chicago で効く)】同一スロットが複数あるときの充填先(設計仕様 §2 NOTE)**
> `combine A B` は「A の欠けたスロットに合う B を挿す」。A に **同じ `(name, condition)` の
> スロットが複数**あるとき、どれが埋まるか(選択・順序)は未定義です。tutorial には該当が
> 無いため踏みませんが、Chicago で現れたら「`combine` を何回呼べば全部埋まるか」だけは決まる
> 一方、「どのスロットが先か」は観測で確かめる論点。ここは開けておきます。

### 3.4 全体の骨組み ― 誰が誰を呼び、データがどう流れるか

部品(`required_fills` / `plan` / `solve` / `combine`)は出そろいました。それらの **呼び出しと
データの流れ**を 1 枚にします。読みどころは 3 つ ― (a) `Variant` から `Item` をどこで引くか、
(b) `Fill` がどこで `Step` 列に変わるか、(c) `plan` と `solve` の役割差です。

```
solve(&Variant, &Snapshot, &State)
  │  (a) Snapshot.index で Variant → (部屋, 段) を引き、その山の Item を得る
  ▼
plan(&Item, goal = Pristine)                     … 1 つの item を Step 列へ
  │  ① required_fills(&item.condition, goal) -> Vec<Fill>   … 分解(第2・3章)
  │        各 Fill.slot.condition を次の goal にして再帰(罠回避)
  ▼
  ② (b) Vec<Fill> を Step 列へ組む
  │        ├ 部品を掘る take / combine / 邪魔物の incinerate / go
  │        └ この「順序決定・6枠パッキング」は……
  │           ┌─────────────────────────────────────┐
  │           │ 【穴: 層2スケジューラ】(第4章)       │  ← アルゴリズムは本書では埋めない
  │           │  貪欲 or 探索。ピーク枠数 ≤ 6 に収める │
  │           └─────────────────────────────────────┘
  ▼
Result<Vec<Step>, Unsolvable>                    … 手順列 or 「不可能」
```

(c) の役割差: **`plan` は「1 つの item を Step 列にする」**分解＋組み立ての中身。**`solve` は
入口**で、`Variant → Item` の解決、層2スケジューリングの駆動、そして「不可能」判定までを受け持ち
ます。図の破線の箱(層2)は、本書では **穴**のまま置きます。中身のアルゴリズム(どう並べれば
6 枠に収まるか)は第4章のコラムで方針だけ示し、完成形は読者が埋めます。

---

## 第4章 LIFO 掘りと 6 枠のスケジューリング

第2・3章は「何が要るか」(木)を答えました。しかし設計仕様 §6 が強調する通り、
**`solve` は木を解く関数ではなく、LIFO の山の上で `take`/`combine`/`incinerate`/`go` を
一本の列に並べるスケジューラ**です。木は順序も枠数も移動も持たない。**6 枠に収まるかは、
木を見ても分からず、手順列を実際に組んで初めて判定できる。** ここが最大の穴になります。

### 4.1 `combine` は move ― Rust の所有権にきれいに乗る

`combine A B` は「B を消費して A を 1 段進める」。この規則は Rust の move セマンティクスに
そのまま重なります。B は**所有権ごと関数へ渡して消える**(呼び出し後 B はもう使えない)、
A は**可変借用で 1 段進む**。

```rust
/// 机上シミュレーション用。combine の規則を型で言い切る。
/// b は値で受け取る = move。呼び出し後、呼び出し側の b は消費されて使えない。
/// a は &mut で受け取る = その場で 1 段進む。
fn combine(a: &mut Item, b: Item) {
    // TODO: a.condition の missing から b が埋めるスロットを 1 つ外し、
    //       全部埋まったら a.condition を Broken.condition(1 段内側)へ進める。
    //       b はこの関数の終わりで drop される = ゲームの「消費」に一致。
    let _ = a;
    let _ = b; // ここで b の寿命が尽きる
}
```

`b: Item`(値渡し)と `a: &mut Item`(可変借用)の非対称が、そのまま「B は消え、A は残る」を
表しています。ゲーム規則を覚えるのではなく、シグネチャに語らせるのが Rust の書き味です。
ここで出てくる **`drop`** は §1.0 で予告したもので、「値はスコープ(この関数)を抜けると
`drop` = 破棄される」という Rust の破棄タイミングです。`b` は関数末尾で自動的に `drop` され、
それがゲームの「B を消費」に一致します。なお **`combine` は A も B も両方インベントリに要る**
(地面の物とはその場で合成できない)ので、シミュレーションでも「両方すでに手持ちにある」
状態でしか `combine` を呼べません。

### 4.2 枠は `Vec<Item>`、掘るコストは段(深さ)

インベントリ(6 枠)とゲームの地面は、どちらも `Vec` で回します。掘る = 地面から `take` して
手持ちへ移す、は「地面 `Vec` の末尾(= 上)を外して手持ち `Vec` に push」。手持ちが 6 を
超えたらその並べ方は失格、という判定を回しながら手順を組みます。

```rust
struct Sim {
    inventory: Vec<Item>, // 手持ち。len() が枠使用数。6 を超えたら失格。
    // ground は Snapshot の GroundStack を作業用に複製したもの
}

impl Sim {
    fn holding(&self) -> usize { self.inventory.len() }
    fn over_capacity(&self) -> bool { self.holding() > 6 }
}
```

枠の増減は 1 手ごとに決まっています ― `take` が +1、`combine` が実質 −1(挿した B が消える)、
`incinerate` が −1、`go` が ±0。この増減を追いながら **「同時に抱える枠数の最大(ピーク)」が
6 を超えない**列を探すのが、スケジューラの仕事です。

### 4.3 在庫ゴミの `incinerate` は「掘りの副産物」

`keypad` の山には、どこからも要求されない `spring` や red `pill` が挟まっています。これらを
「不要品リストを別に作って消す」のではなく、**必要な item に届くために山を掘る過程で、
邪魔な物を `take` → `incinerate` する副産物**として手順に出す、というのが設計仕様 §6 の妙です。
不要判定は「手順に出てこないこと」でしか下せない、を逆手に取っています。

山から退けた物の処分は 3 択です ―「後で使うので**持ち続ける**(枠を消費)」「要らないので
**`incinerate`(燃やす)**」「**床へ戻す操作**(ゲームの `drop` コマンド。可否不明)」。どれを
選ぶかが 6 枠に収まるかを左右します。

> **【注意: 用語の衝突】** ここで出た「床へ戻す操作」はゲームの `drop` コマンドで、§4.1 の
> Rust の `drop`(値がスコープを抜けて破棄されること)とは **別物**です。同じ綴りですが、
> 前者は「アイテムを地面に置く操作」、後者は「言語が値の後始末をする仕組み」。本書は
> 混同を避けるため、ゲーム側は地の文で「床へ戻す操作」と呼びます。

> **【穴 / 未確定: tutorial で踏む(今決める)】処分 3 択の決定ロジック(設計仕様 §6 NOTE)**
> 「持ち続ける / 燃やす / 床へ戻す」のどれを選ぶかの基準は **未定義**。ゲームの `drop`(床へ戻す)は
> 可否不明なので本設計は依存しません(`Step` に床へ戻す手を用意しない、が安全側)。「後で要るか」は
> 依存木を見れば分かるので、**`incinerate` してよいのは「木のどのスロットにも現れないバリアント」
> だけ**、という不変条件は置けます。tutorial は `spring` / red `pill` の処分が実際に要るので
> ここは踏みます。その先の最適化(燃やすか持ち続けるか)は読者が埋める穴です。

### 4.4 スケジューラ本体 ― ここが本書最大の穴

`solve` の入力に `State`(現在のインベントリと現在地)が要ります。設計仕様 §6 に合わせて、
最小形はこうです。

```rust
/// solve の入力になる、いまの状態。
struct State {
    inventory: Vec<Item>, // 手持ち。len() が使用枠数
    current_room: RoomId, // 現在地(go を出すのに要る)
}
```

そのうえで `solve` の入口はこうなります(**中核は穴**)。

```rust
/// solve の入口。手順列 or 「不可能」。
///
/// Result<T, E> は「成功 Ok(T) か失敗 Err(E) のどちらか」を表す型。
/// solve は手順列を Ok(Vec<Step>) で、不可能を理由付きの Err(Unsolvable) で返す。
///
/// 【穴】中核は未確定(設計仕様 §6 NOTE: 探索か貪欲か・順序決定・6枠の詰め方)。
/// 本書はシグネチャと不変条件までを示し、本体は読者に委ねる。
fn solve(item: &Variant, snapshot: &Snapshot, state: &State) -> Result<Vec<Step>, Unsolvable> {
    // 1. 索引 snapshot.index で Variant → Item を引き、その Condition から依存分解(第2・3章、goal=Pristine)。
    // 2. 各部品を「どの山の何段目」から掘るかを索引で引く(§1.4 コラム)。
    // 3. 掘り・combine・incinerate・go を一列に並べ、ピーク枠数 <= 6 を保つ。
    //    TODO: ここが核心。層2(ターゲット内の順序)を決める。
    // 4. どう並べても 6 枠に収まらなければ Err(Unsolvable::OverCapacity)。
    let _ = (item, snapshot, state);
    todo!("層2スケジューリング ― 貪欲 or 探索。コラム参照")
}

/// 「不可能」の理由。設計仕様が確定させているのは枠オーバーのみ。
#[derive(Debug, PartialEq)]
enum Unsolvable {
    OverCapacity, // どう並べても同時保持が 6 を超える
    // NOTE: 供給不足・到達不能など他要因は設計仕様でも未整理(§6 NOTE)。増やすなら実データで。
}
```

返り値を `Result<Vec<Step>, Unsolvable>` にしたのは、「不可能」を **理由付きの `Err`** で
運べるからです。`Option` でも表せますが、理由(枠オーバーなのか供給不足なのか)を後で
足したくなるので `enum` の `Err` にしておくと将来の分岐が楽です。ただし今 **確定している
理由は枠オーバーだけ**なので、バリアントを増やすのは実データで裏が取れてからにします。

### 4.5 層1と層2 ― `solve` は層2だけ

設計仕様 §6 は「何から作るか」を 2 層に分けます。ここでいう **ターゲット**は「最終的に作りたい
item」のことです。

- **層1(ターゲット間)** ― `uploader` / `downloader` などをどの順で作り、完成品を抱えたまま
  次へ進むか。これは**プレイヤーの判断**で `solve` の外。「A を持ったまま B を作れるか？」は
  `state.inventory` に A を入れて `solve B` を呼び、`Err(OverCapacity)` が返るかで分かります。
- **層2(ターゲット内)** ― 1 回の `solve` の中で、どのサブ部品から作れば
  「同時に抱える枠数の最大 ≤ 6」に収まるか。**これが `solve` の中身**(= 4.4 の穴)。

`solve` に `state`(現在のインベントリと現在地)を渡すのは、層1の可否を層2の実行で答えるため。
`state.inventory` に既に手持ちがあれば、その分だけ 6 枠の余白が減った状態で層2を組みます。
これで「持ったまま作れるか」が自然に判定できます。

> **【コラム / 未確定: tutorial で踏む(今決める)】層2の並べ方 ― 貪欲か、探索か(設計仕様 §6 NOTE)**
> 設計仕様の作業仮説は **「最も多く枠を使う部分から先に作れば、同時に抱える枠数の最大が下がる」**。
> 直感的にはもっともらしいのですが、**この貪欲則が常に最小ピークを与えるかは未証明**(反例の
> 非存在も未確認)。各部分木のピーク枠コストの見積もり式も未定義です。6 枠パッキングは核心 3 点の
> ひとつで tutorial でも要るため、ここは踏みます。実装の選択肢は 2 つ。
>
> - **貪欲案(疑似コード)**:
>   ```text
>   各サブ部品の「作るのに要するピーク枠数」を見積もる
>   ピーク枠数の大きい順にサブ部品を並べる
>   その順に手順を連結し、通しのピークが 6 を超えないか検査
>     超えなければ採用 / 超えたら Err(OverCapacity)
>   ```
>   速いが、最小ピークの保証は無い(= 本当は解けるのに Err を返す取りこぼしがありうる)。
> - **探索案(疑似コード)**:
>   ```text
>   状態 = (手持ち集合, 各山の掘り深さ, 未達成スロット)
>   depth-first に「次に打てる 1 手(take/combine/incinerate/go)」を試す
>     手持ちが 6 を超える枝は即座に枝刈り
>     全スロット達成に到達したら、その手順列を解として返す
>   どの枝でも到達しなければ Err(OverCapacity)
>   ```
>   完全(解があれば見つかる)だが状態爆発に注意。tutorial 規模なら探索でも軽い。
>   状態を Rust の器で持つなら、手持ち集合 = `HashSet<Variant>`、各山の掘り深さ = 山ごとの `usize`、
>   未達成スロット = `Vec<Kind>` あたり(**器の型まで**。遷移や枝刈り、次の 1 手の選び方は読者が書く)。
>
> **本書はどちらも確定させません。** まず貪欲で tutorial を通し、取りこぼしが出たら探索へ
> 差し替える、という進め方が現実的です。「貪欲で最小ピークが出る」を **事実として書かない**
> ことだけ守ってください。

> **【コラム / 未確定: tutorial では踏まない(keypad は解ける)】「不可能」の完全性(設計仕様 §6・§10 NOTE)**
> 4.4 の `Err(OverCapacity)` は「層2が 6 枠に収まらない」を表しますが、「**どう並べても**
> 収まらない」を網羅的に確かめる探索の完全性は未整理です。貪欲案では「並べ方を 1 つ試して
> 失敗した」に過ぎず、真に不可能かは言えません。ここを厳密にやるなら探索案が要ります。keypad は
> 解けるので tutorial では `Err` に踏み込みませんが、供給不足・必要 item が山に無い・目的部屋が
> 到達不能といった **他の不可能要因**も設計仕様は未整理。`Unsolvable` を増やすのは実データで
> 確認してから。

---

## 第5章 テストの書き方 ― fixture を parse して手順列を検証する

`CLAUDE.md` の規約通り、各 `#[test]` は **英語の関数名＋日本語コメント**(正常系/異常系の意図)を
セットにします。ソルバのテストは `local/tutorial.xml` を fixture として parse し、`solve` が
出した手順列を検証する形が基本です。

テストの前に、いくつか自分で用意するヘルパがあります。骨格(第1・4章)が決まれば、どれも短く
書けます。

- `load_snapshot(path: &str) -> Snapshot` ― fixture を parse して `Snapshot` を作る橋渡し
  (parse は既存パーサ、詰め替えは §1.4 の橋渡し。本体は読者)。
- `variant(name: &str) -> Variant` ― テスト用に `Variant` を手早く作る(`adjective` は `None`)。
- `State::empty_at(room: &str) -> State` ― 手持ち空・現在地指定の初期状態。
- `.expect("...")` ― `Result` の `Ok` を取り出す。`Err` なら、そのメッセージ付きでテストを
  落とす(意図しない不可能を早期に露見させる)。

### 5.1 いちばん大事なテスト ― radio を完成させないこと

第3章の罠を守れているかを、まず落とします。ポイントは「**radio への `antenna` 挿入が
手順列に現れないこと**」を確かめる点です(設計仕様 §8 NOTE の具体化)。

```rust
#[test]
fn stops_recursion_at_slot_condition() {
    // 正常系: solve keypad は radio を broken・欠 antenna で止め、完成させないこと。
    //         手順列に「radio へ antenna を挿す combine」が現れてはならない。
    let snapshot = load_snapshot("local/tutorial.xml"); // parse → Snapshot
    let state = State::empty_at("Junk Room");           // 手持ち空・現在地 Junk Room
    let steps = solve(&variant("keypad"), &snapshot, &state).expect("solvable");

    let inserts_antenna_into_radio = steps.iter().any(|s| matches!(
        s,
        Step::Combine { target, part }
            if target.name == "radio" && part.name == "antenna"
    ));
    assert!(!inserts_antenna_into_radio, "radio を完成させてはいけない(罠)");
}
```

`matches!` マクロは「値がパターン(＋ガード)に一致するか」を真偽で返します。ここでは
`Step::Combine` のうち `target` が radio かつ `part` が antenna のものを探し、**無いこと**を
主張します。手順列を `enum` の `Vec` にしておいた恩恵がここで活きます。

### 5.2 transistor は挿すこと(対の正常系)

止めるだけでなく、必要な挿入は出ていることも確かめます。

```rust
#[test]
fn fills_radio_transistor_slot_only() {
    // 正常系: radio には transistor は挿す(antenna は挿さない)。
    //         スロット条件 broken(欠 antenna) へちょうど到達させる。
    let snapshot = load_snapshot("local/tutorial.xml");
    let state = State::empty_at("Junk Room");
    let steps = solve(&variant("keypad"), &snapshot, &state).expect("solvable");

    let fills_transistor = steps.iter().any(|s| matches!(
        s,
        Step::Combine { target, part }
            if target.name == "radio" && part.name == "transistor"
    ));
    assert!(fills_transistor, "radio へ transistor は挿す必要がある");
}
```

### 5.3 6 枠を破らないこと(ピークの検証)

手順列を頭から実行シミュレートして、同時保持数が 6 を超えないことを主張します。
`solve` の中身(並べ方)に踏み込まず、**出力の性質**だけを見るのが安定したテストです。
シミュレータ `simulate_peak_holding` は、中で §4.2 の `Sim` を頭から回して各手の枠増減を
たどり、最大の保持数を返す小さなインタプリタになります(本体は読者)。

```rust
#[test]
fn never_exceeds_six_slots() {
    // 正常系: 手順列を実行しても同時保持は 6 を超えないこと(層2の制約)。
    let snapshot = load_snapshot("local/tutorial.xml");
    let state = State::empty_at("Junk Room");
    let steps = solve(&variant("keypad"), &snapshot, &state).expect("solvable");

    let peak = simulate_peak_holding(&steps, &snapshot); // take/combine/inc/go を Sim で追う
    assert!(peak <= 6, "同時保持のピークは {peak}、6 を超えている");
}
```

### 5.4 在庫ゴミを掘り出すこと

`spring` や red `pill` が邪魔になる並びでは、それらが `take` → `incinerate` される
(または退けられる)ことを確かめます。ただし **処分 3 択が未確定**(4.3 コラム)なので、
「必ず `incinerate` する」と決め打つと脆いテストになります。当面は緩い性質で受けます。すなわち
「radio や keypad を掘り出すために、上に載った spring/pill が手順のどこかで手持ちから消えている
(`incinerate` された、または最後まで持ち越されていない)」くらいの主張に留めます。処分方針が
固まってから、テストを締め直せばよいでしょう。

> **【コラム / 未確定: tutorial で踏む(今決める)】期待手順列の同値判定(設計仕様 §8 NOTE)**
> 「正しい手順列」をテストでどう定めるかは未定義です。素朴に `assert_eq!(steps, expected)` と
> **完全一致**で書くと、`solve` の並べ方を 1 通りに固定してしまい、実装の自由(順序の入れ替えで
> 同じくらい良い解)を縛ります。テスト設計は着手時に決める必要があるのでここは踏みます。候補は 2 つ。
> - **順序不問の性質検証**(本章で採った形): 「radio に antenna を挿さない」「ピーク ≤ 6」
>   「keypad が最終的に pristine になる」など、順序に依らない不変条件で受ける。実装の自由を残せる。
> - **正準形に正規化して比較**: 手順列を決定的な順に並べ直してから `assert_eq!`。厳密だが、
>   正準化規則そのものを決める必要がある。
> どちらを採るかは開けておきます。まずは性質検証で TDD を回し、仕様が固まったら正準比較を
> 足す、で構いません。

---

## 終章 まだ開いている問い(`walk` / `make` / その先)

核心 3 点の外側は、本書では意図的に浅く留めます。以下は「確定していない」ことを明示する
ための一覧です。**どれも事実として断定せず、実装時に実データで詰める論点**として置きます。

- **`walk` の巡回**(設計仕様 §4 NOTE) ― 深さ優先 / 幅優先、探索の未展開集合や訪問済み判定、
  部屋の同一性、逆向きが非対称な場合のバックトラック、`... leads <方角>` の parse 具体、
  `go` 失敗時の処理。tutorial は `go north` 一本で済むため、ここは Chicago
  (`local/chicago-rooms.xml`)で詰める話。
- **経路探索**(設計仕様 §6 NOTE) ― 部屋グラフ上の `go` 列生成と、複数部屋往復時の訪問順
  最適化。有向辺(逆向き非対称ありうる)を前提にする、まで決まっているだけ。
- **バリアント割り当ての一意性**(設計仕様 §9 NOTE) ― 「6 枠で作れる物＋地面から取れる数＋
  葉から順」で一意に定まる、は**作業仮説**。崩れたら総当たり探索へ切り替えるが、「崩れた」を
  何の観測で検知するかは未定義。tutorial では transistor 2 個が交換可能で衝突しないため、
  実質テストにならない。
- **`make` の照合**(設計仕様 §7 NOTE) ― 手順を `UM`(手順を実行する対象 = シェルの先にいる
  本体)へ流し、応答を parse して想定とのずれを検知する具体手順。ずれたら全部屋を再 `walk` して
  組み直す(前の状態を引きずらない)方針だけ確定。
- **起動形態と状態の受け渡し**(設計仕様 §1・§10 NOTE) ― `walk`/`solve`/`make` をシェルの
  サブコマンドにするか対話中の内部コマンドにするか、Snapshot と手順列をメモリに置くかファイルに
  置くかは未確定。

これらは「厚みのある未完」であって欠陥ではありません。核心 3 点(再帰分解・スロット条件で
止まる罠・LIFO/6 枠)を TDD で通し切れたら、この本の目的は達成です。残りは、その土台の上で
実データと相談しながら、読者が自分の手で埋めていく余地です。

---

### 参考

本書だけで核心 3 点は実装できます(型・シグネチャ・テストの骨格は本文で閉じています)。
Chicago 以降の大規模データへ進むときは、下記の原典が要ります。

- `docs/superpowers/specs/2026-07-04-dependency-solver-design.md` ― 設計仕様(本書の主素材)
- `docs/dependency-solver.md` ― 背景の観測ノート(事実 / 仮説の二層)
- `local/tutorial.xml` ― tutorial 2 部屋の実データ(fixture)
- `src/parser.rs` ― `Condition` / `Broken` / `Kind` / `Item` の既存型(ソルバの入力)
- `CLAUDE.md` ― シェル本体(scanner/parser/renderer/session)とテスト規約
