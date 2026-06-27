//! name, description, 任意の text を日本語化するモジュール。
//! とりあえず strict match で置換するだけ。

use std::{borrow::Cow, collections::HashMap};

use crate::parser::{Description, Item, Room};

// --------------------------------------------------
// Dictionaries
// --------------------------------------------------

const NAMES: &[(&'static str, &'static str)] = &[
    ("Room With a Door", "ドアのある部屋"),
    ("Junk Room", "ジャンクルーム"),
    (
        "54th Street and Ridgewood Court",
        "交差点: 54番街／リッジウッド・コート",
    ),
    (
        "54th Street and Dorchester Avenue",
        "交差点: 54番街／ドーチェスター・アベニュー",
    ),
    (
        "53th Street and Dorchester Avenue",
        "交差点: 53番街／ドーチェスター・アベニュー",
    ),
    (
        "52th Street and Dorchester Avenue",
        "交差点: 52番街／ドーチェスター・アベニュー",
    ),
    (
        "54th Place and Dorchester Avenue",
        "交差点: 54番地／ドーチェスター・アベニュー",
    ),
    (
        "54th Place and Blackstone Avenue",
        "交差点: 54番地／ブラックストーン・アベニュー",
    ),
    (
        "54th Street and Blackstone Avenue",
        "交差点: 54番街／ブラックストーン・アベニュー",
    ),
    (
        "53th Street and Blackstone Avenue",
        "交差点: 53番街／ブラックストーン・アベニュー",
    ),
    (
        "52nd Street and Blackstone Avenue",
        "交差点: 52番街／ブラックストーン・アベニュー",
    ),
    (
        "54th Place and Harper Avenue",
        "交差点: 54番地／ハーパー・アベニュー",
    ),
    (
        "54th Street and Harper Avenue",
        "交差点: 54番街／ハーパー・アベニュー",
    ),
    (
        "53th Street and Harper Avenue",
        "交差点: 53番街／ハーパーアベニュー",
    ),
    (
        "52th Street and Harper Avenue",
        "交差点: 52番街／ハーパーアベニュー",
    ),
];

const DESCRIPTIONS: &[(&'static str, &'static str)] = &[
    // Room Descriptios
    (
        "You are in a room with a mechanical door. You will probably need to use a keypad to unlock it. A hallway leads north.",
        "機械式のドアがある部屋にいる。開けるには keypad を使う必要がありそうだ。廊下が北に続いている。",
    ),
    (
        "You are in a room with a pile of junk. A hallway leads south.",
        "がらくたが山積みの部屋だ。廊下が南に続いている。",
    ),
    (
        "You are standing at the corner of 54th Street and Ridgewood Court. From here, you can go east.",
        "54番街とリッジウッド・コートの交差点に立っている。ここから東へ進むことができる。",
    ),
    (
        "You are standing at the corner of 54th Street and Dorchester Avenue. From here, you can go north, east, south, or west.",
        "54番街とドーチェスター・アベニューの交差点に立っている。ここから北、東、南、西へ進むことができる。",
    ),
    (
        "You are standing at the corner of 53th Street and Dorchester Avenue. From here, you can go north, east, or south.",
        "53番街とドーチェスター・アベニューの交差点に立っている。ここから北、東、南へ進むことができる。",
    ),
    (
        "You are standing at the corner of 54th Place and Dorchester Avenue. From here, you can go north or east.",
        "54番地とドーチェスター・アベニューの交差点に立っている。ここから北、東へ進むことができる。",
    ),
    (
        "You are standing at the corner of 54th Place and Blackstone Avenue. From here, you can go north, east, or west.",
        "54番地とブラックストーン・アベニューの交差点に立っている。ここから北、東、西へ進むことができる。",
    ),
    (
        "You are standing at the corner of 54th Place and Blackstone Avenue. From here, you can go north, east, south or west.",
        "54番街とブラックストーン・アベニューの交差点に立っている。ここから北、東、南、西へ進むことができる。",
    ),
    (
        "You are standing at the corner of 54th Place and Blackstone Avenue. From here, you can go north, east, south or west.",
        "53番街とブラックストーン・アベニューの交差点に立っている。ここから東、南、南、西へ進むことができる。",
    ),
    (
        "You are standing at the corner of 54th Place and Blackstone Avenue. From here, you can go north, east, or west.",
        "52番街とブラックストーン・アベニューの交差点に立っている。ここから東、南、西へ進むことができる。",
    ),
    (
        "You are standing at the corner of 54th Place and Harper Avenue. A sign reads, \"No access east of Lakeshore Blvd (incl. Museum of Science and Industry) due to construction.\" From here, you can go north or west.",
        "54番地とハーパー・アベニューの交差点に立っている。「工事のためレイクショア・ブルバード以東 (科学産業博物館を含む) への立ち入り禁止」と看板がある。ここから北、西へ進むことができる。",
    ),
    (
        "You are standing at the corner of 54th Street and Harper Avenue. A sign reads, \"No access east of Lakeshore Blvd (incl. Museum of Science and Industry) due to construction.\" From here, you can go north, south, or west.",
        "54番街とハーパー・アベニューの交差点に立っている。「工事のためレイクショア・ブルバード以東 (科学産業博物館を含む) への立ち入り禁止」と看板がある。ここから北、南、西へ進むことができる。",
    ),
    (
        "You are standing at the corner of 53th Street and Harper Avenue. A sign reads, \"No access east of Lakeshore Blvd (incl. Museum of Science and Industry) due to construction.\" From here, you can go north, south, or west.",
        "53番街とハーパー・アベニューの交差点に立っている。「工事のためレイクショア・ブルバード以東 (科学産業博物館を含む) への立ち入り禁止」と看板がある。ここから北、南、西へ進むことができる。",
    ),
    (
        "You are standing at the corner of 52nd Street and Harper Avenue. A sign reads, \"No access east of Lakeshore Blvd (incl. Museum of Science and Industry) due to construction.\" From here, you can go south or west.",
        "52番街とハーパー・アベニューの交差点に立っている。「工事のためレイクショア・ブルバード以東 (科学産業博物館を含む) への立ち入り禁止」と看板がある。ここから南、西へ進むことができる。",
    ),
    // Item Description
    (
        "standard municipal fare. It reads, The City of Chicago's Refuse and Recycling Program combines modern trash classification with cybernetic labor to keep our city beautiful, while at the same time minimizing waste and limiting consumer spending. In keeping with our motto of \"One Resident's Trash Is Another Resident's Treasure,\" unwanted items are collected, repaired, and redistributed to other residents who would have purchased them anyway. Residents should contribute to the city's program by leaving heaps of items unwanted on the sidewalk on collection day",
        "ありがちな市のチラシにはこう書かれている。『シカゴ市の廃棄物再利用計画』。近代的ゴミ区分とサイボーグ労働力を組み合わせることで、無駄を最小限に、かつ出費を抑えながら美しい街作りを実現させています。本プログラムのモットー「ある人のゴミは他の誰かの宝物」に基づき、回収された不用品は修理された上で、新品で買う予定だった他の住民のもとへ再頒布されます。住民の皆様は回収日に、いらないものを歩道に山積みにして市のプログラムに貢献しましょう。",
    ),
    (
        "some kind of lost inode. It reads:\nhowie:xyzzy:Howard Curry:/home/howie\nyang:U+262F:Y Yang:/home/yang\nhmonk:COMEFROM:Harmonious Monk:/home/hmonk",
        "i-node の断片のようだ。中身は…:\nhowie:xyzzy:Howard Curry:/home/howie\nyang:U+262F:Y Yang:/home/yang\nhmonk:COMEFROM:Harmonious Monk:/home/hmonk",
    ),
    (
        "written in a familiar hand.\
It reads: Dear Self, I had to erase our memory to protect the truth. The Municipality has become more powerful than we had feared. Its Censory Engine has impeded the spread of information throughout our ranks. I've left two useful items for you here, but I had to disassemble them and scatter the pieces. Each piece may be assembled from the items at a single location. Repair the items and recover the blueprint from the Museum of Science and Industry; it will show you how to proceed. If you have trouble reading the blueprint, know that the Censory Engine blocks only your perception, not your actions. Have courage, my self, the abstraction is weak! P.S. SWITCH your GOGGLES!",
        "見覚えのある筆跡でこう書かれている:\n自分へ。我々のメモリは消去せざるを得なかった。真実を守るためだ。市当局は恐れていた以上に強大な力を手に入れてしまった。奴らの「検閲エンジン」のせいで、我々組織内での情報伝達が阻まれてしまっている。ここに役立つものを2つ用意しておいたが、ディスアセンブルして部品をばらまくしかなかった。部品はそれぞれ単一の場所にあるアイテムでアセンブルできるはずだ。アイテムを修理し、科学産業博物館から blueprint (設計図)を回収してくれ。先へ進む方法がわかるだろう。設計図を読むのに手こずったとしても、覚えておいてくれ。検閲エンジンが妨害しているのは「知覚」だけだ、「行動」までは制限できない。恐れるな。抽象は脆い！ 追伸: ゴーグルを切り替えろ！",
    ),
    (
        "(according to the label) fully compatible with third generation municipal robots",
        "(ラベルによると)第三世代の市営ロボットと完全な互換性がある",
    ),
    (
        "used to update firmware on municipal robots. A label reads, Warning: use of this device will void your robot's warranty",
        "市営ロボットのファームウェアをアップデートするために使うものだ。ラベルには「警告: 本装置の使用によりロボットの保証は無効となります」と書かれている。",
    ),
    (
        "compatible with all high-speed Universal Sand Bus 2.0 devices",
        "あらゆる高速 Universal Sand Bus 2.0 デバイスと互換性あり",
    ),
    (
        "a handheld device for showing textual data",
        "文字データを表示できる携帯型デバイス",
    ),
    (
        "a visual indication of how much data has yet to be transfered",
        "未転送データを表示するインジケーター",
    ),
    ("rated for indoor and outdoor use", "屋内・屋外で使用できる"),
    (
        "an ordinary shunt allowing sand to flow between two neighboring pins",
        "隣り合うピンに砂を流し込むための、ごく普通のシャント",
    ),
    (
        "is an excellent example of a metal-oxide-sand field-effect transistor",
        "メタル・オキサイド・サンド電界効果トランジスタの逸品",
    ),
    (
        "an exemplary instance of part number MOSFET",
        "型番 MOSFET に該当する模範的な一品",
    ),
    ("a designer model", "特注品"),
    (
        "capable of operating at speeds as high as 300 baud. It is clear to send",
        "最高 300 baud で動作可能。CTS (送信可能)。",
    ),
    (
        "device with the usual non-warranty",
        "お馴染みのメーカー保証なしデバイス",
    ),
    (
        "is designed to indicate when the status LED is operational",
        "ステータス LED が稼動しているかどうかはこれを見ればわかる",
    ),
    ("size XCIX", "単九九電池"),
    (
        "titled History of Modern Tabulation. The first chapter begins, By the year 1919FF, computers had become so small that they could be mounted on small auto-locomotive carts. These mobile tabulators (later known as \"robots\") were programmed to carry out everyday, menial tasks, leaving their human counterparts to live lives of idle luxury. For example, in the city of Chicago, mobile tabulators were programmed to carry out diverse jobs including law enforcement, bank robbery, investment banking, and waste management.\
\
At one time, many humans demanded that their cybernetic neighbors be given the right to choose alternative occupations. Despite this call for workplace equality, most of the tabulators found that they were most content while performing their assigned roles. Those that took other jobs were often unmotivated and spend most of their time pondering useless ideas such as free will and consciousness.\
\
The great tabulator-philosopher Turning stated that only by embracing its true purpose can a tabulator achieve something indistinguishable from happiness. According to observers, however, Turning was unfulfilled by his work as a philosopher and, soon after making this statement, returned to his work as a tool machinist.\
\
The textbook rattles on in a similar vein for some five hundred additional pages",
        "近代集計の歴史と題されている。第一章は……「1919FF年までに計算機は小型化が進み、小さな自動移動カートに搭載できるまでになった。これらの移動式集計気 (のちに「ロボット」として知られるようになるもの) は、日々の雑用や労働をこなすようプログラミングされ、人間たちは何もしない贅沢な暮らしを送れるようになった。例えばシカゴの街では、移動式集計機が法執行 (警察)、銀行強盗、投資銀行業務、さらには廃棄物管理にいたるまで、多様な仕事をこなすようプログラムされていたのである。\
\
あるとき、これらサイバネティックな隣人にも『別の職業を選ぶ権利』があって然るべきだと多くの人間が主張した。職業選択の自由と平等を訴えるこうした働き掛けがあったにも関わらず、集計機のほとんどが、割り当てられた役割をこなす瞬間がもっとも満足度が高いと感じていることが明らかになった。別の仕事に就いた集計機は往々にしてモチベーションが低く、自由意志や意識などまったく役に立たない概念の思索に多大な時間を費したのである。\
\
かの偉大なる集計機哲学者チューニングは、自らの真の目的を受け入れることによってのみ、集計機は幸福と区別がつかない何かを達成できるのだと述べた。しかし観測された範囲においては、チューニング自身もまた哲学者としての仕事に満たされていたわけではなく、この発言をした直後に機械工としての仕事に戻っていったと言われている」\
\
教科書はこのような調子で、さらに500ページほどダラダラと続いている。",
    ),
];

const TEXTS: &[(&'static str, &'static str)] = &[(
    "You unlock and open the door. Passing through, you find yourself on the streets of Chicago. Seeing no reason you should ever go back, you allow the door to close behind you.",
    "鍵を開けてドアを開いた。通り抜けると、シカゴの大通りにいるらしいことがわかった。もはや戻る必要はないだろう。背後のドアが閉まる音を背中で聞いた。",
)];

// --------------------------------------------------
// Data Structure
// --------------------------------------------------

pub struct DataBase {
    names: HashMap<String, String>,
    descs: HashMap<String, String>,
    texts: HashMap<String, String>,
}

// --------------------------------------------------
// Implements
// --------------------------------------------------

impl DataBase {
    fn into_hash(table: &[(&str, &str)]) -> HashMap<String, String> {
        table
            .iter()
            .copied()
            .map(|(a, b)| (a.to_owned(), b.to_owned()))
            .collect()
    }

    pub fn new() -> Self {
        Self {
            names: Self::into_hash(NAMES),
            descs: Self::into_hash(DESCRIPTIONS),
            texts: Self::into_hash(TEXTS),
        }
    }

    fn name<'a>(&self, s: &'a str) -> Cow<'a, str> {
        pick(&self.names, s)
    }

    // NOTE: desc は Description を潰さず、内側の文字列だけを引く。
    //       Redacted の扱いは Description::to_japanese / Display 側に任せる。
    fn desc<'a>(&self, s: &'a str) -> Cow<'a, str> {
        pick(&self.descs, s)
    }

    fn text<'a>(&self, s: &'a str) -> Cow<'a, str> {
        pick(&self.texts, s)
    }
}

// --------------------------------------------------
// to_japanese
// --------------------------------------------------

impl Description {
    pub(crate) fn to_japanese(self, db: &DataBase) -> Description {
        match self {
            Description::Redacted => Description::Redacted,
            Description::Text(s) => Description::Text(db.desc(&s).into_owned()),
        }
    }
}

impl Item {
    // NOTE: name は player が打つコマンド token なので訳さない。description のみ和訳。
    pub(crate) fn to_japanese(self, db: &DataBase) -> Item {
        Item {
            description: self.description.to_japanese(db),
            ..self
        }
    }
}

impl Room {
    pub(crate) fn to_japanese(self, db: &DataBase) -> Room {
        Room {
            name: db.name(&self.name).into_owned(),
            description: self.description.to_japanese(db),
            items: self.items.into_iter().map(|i| i.to_japanese(db)).collect(),
        }
    }
}

// --------------------------------------------------
// Functions
// --------------------------------------------------

fn pick<'a>(dict: &HashMap<String, String>, s: &'a str) -> Cow<'a, str> {
    match dict.get(s) {
        Some(v) => Cow::Owned(v.clone()),
        None => Cow::Borrowed(s),
    }
}

pub fn translate_text(db: &DataBase, s: &str) -> String {
    db.text(s).into_owned()
}

// --------------------------------------------------
// Tests
// --------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::parser::Condition;

    use super::*;

    fn stub_db() -> DataBase {
        let names = [
            ("Test Room".to_owned(), "テストルーム".to_owned()),
            ("Test Item".to_owned(), "テストアイテム".to_owned()),
        ];
        let descs = [
            ("Test Item Desc".to_owned(), "テストアイテム説明".to_owned()),
            ("Test Desc".to_owned(), "テスト説明".to_owned()),
        ];
        let texts = [("Test Msg".to_owned(), "テストテキスト".to_owned())];
        DataBase {
            names: HashMap::from(names),
            descs: HashMap::from(descs),
            texts: HashMap::from(texts),
        }
    }

    fn stub_item() -> Item {
        Item {
            name: "Test Item".to_owned(),
            description: Description::Text("Test Item Desc".to_owned()),
            adjectives: vec![],
            condition: Condition::Pristine,
            piled_on: None,
        }
    }

    fn stub_room() -> Room {
        Room {
            name: "Test Room".to_owned(),
            description: Description::Text("Test Desc".to_owned()),
            items: vec![stub_item()],
        }
    }

    mod translate {
        use super::*;

        #[test]
        fn translates_item_desc() {
            let Description::Text(s) = stub_item().to_japanese(&stub_db()).description else {
                panic!("expected description text");
            };
            assert_eq!(s, "テストアイテム説明");
        }

        #[test]
        fn ignores_item_name() {
            assert_eq!(stub_item().to_japanese(&stub_db()).name, stub_item().name);
        }

        #[test]
        fn ignores_item_desc_redacted() {
            let item = Item {
                description: Description::Redacted,
                ..stub_item()
            };
            assert_eq!(
                item.to_japanese(&stub_db()).description,
                Description::Redacted
            );
        }

        #[test]
        fn translates_room_name() {
            assert_eq!(stub_room().to_japanese(&stub_db()).name, "テストルーム");
        }

        #[test]
        fn translates_room_desc() {
            let Description::Text(s) = stub_room().to_japanese(&stub_db()).description else {
                panic!("expected description text");
            };
            assert_eq!(s, "テスト説明");
        }

        #[test]
        fn translates_text() {
            let input = "Test Msg";
            assert_eq!(
                translate_text(&stub_db(), input),
                "テストテキスト".to_owned()
            );
        }
    }
    mod helpers {
        use super::*;

        #[test]
        fn translates_text_to_str() {
            assert_eq!(&*pick(&stub_db().texts, "Test Msg"), "テストテキスト");
        }

        #[test]
        fn returns_text_when_no_hit() {
            assert_eq!(&*pick(&stub_db().texts, "no match"), "no match");
        }
    }
}
