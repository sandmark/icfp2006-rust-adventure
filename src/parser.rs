use anyhow::{Context, Result, bail};
use roxmltree::{Document, Node};
use std::fmt::Display;

/// DOM: アイテムの説明
#[derive(Debug, PartialEq)]
pub enum Description {
    Text(String),
    Redacted,
}

/// DOM: アイテムの特徴
#[derive(Debug, PartialEq)]
pub struct Adjective(String);

/// DOM: アイテムの状態
#[derive(Debug, PartialEq)]
pub enum Condition {
    Pristine,
    Broken(Broken),
}

/// DOM: 壊れたアイテムと必要な依存関係
#[derive(Debug, PartialEq)]
pub struct Broken {
    condition: Box<Condition>,
    missing: Vec<Kind>,
}

#[derive(Debug, PartialEq)]
struct Kind {
    name: String,
    condition: Condition,
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Switch(String),
    Look(Room),
    Go(Room),
    Show(Vec<Item>),
    Take(Item),
    Incinerate(Item),
    Combine(Vec<Item>),
    Use((Item, String)),
    Examine(Item),
}

#[derive(Debug, PartialEq)]
pub enum Response {
    Success(Command),
    Error(String),
    Help(String),
    Failed(String),
}

#[derive(Debug, PartialEq)]
pub struct Item {
    pub name: String,
    pub description: Description,
    pub adjectives: Vec<Adjective>,
    pub condition: Condition,
    pub piled_on: Option<Box<Item>>,
}

#[derive(Debug, PartialEq)]
pub struct Room {
    pub name: String,
    pub description: Description,
    pub items: Vec<Item>,
}

impl Broken {
    fn new(condition: Condition, missing: Vec<Kind>) -> Self {
        Self {
            condition: Box::new(condition),
            missing,
        }
    }
}

impl Kind {
    fn new(name: String, condition: Condition) -> Self {
        Self { name, condition }
    }
}

impl Item {
    fn new(
        name: String,
        description: Description,
        adjectives: Vec<Adjective>,
        condition: Condition,
        piled_on: Option<Box<Item>>,
    ) -> Self {
        Self {
            name,
            description,
            adjectives,
            condition,
            piled_on,
        }
    }
}

impl Room {
    fn new(name: String, description: Description, items: Vec<Item>) -> Self {
        Self {
            name,
            description,
            items,
        }
    }
}

// --------------------------------------------------
// impl Display
// --------------------------------------------------

impl Display for Description {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Description::Text(s) => write!(f, "{s}"),
            Description::Redacted => write!(f, "[検閲]"),
        }
    }
}

impl Display for Adjective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Condition::Pristine => write!(f, "無傷"),
            Condition::Broken(b) => write!(f, "{}", b),
        }
    }
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.condition {
            // 無傷の部品は名前だけ
            Condition::Pristine => write!(f, "{}", self.name),
            // 壊れた部品は「(欠けているもの)を欠いた 名前」(1段だけ前置修飾する)
            Condition::Broken(b) => {
                let lack = b
                    .missing
                    .iter()
                    .map(|k| k.to_string())
                    .collect::<Vec<String>>()
                    .join("・");
                write!(f, "{lack} を欠いた {}", self.name)
            }
        }
    }
}

impl Display for Broken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "壊れている: ")?;

        // 修理の段階を、外側(今すぐの不足)から内側(その先の不足)へ歩く。
        // NOTE: 「破損」を繰り返さず、連体修飾を入れ子にしないことで括弧の山を避ける
        let mut cur = self;
        let mut first = true;
        loop {
            let parts = cur
                .missing
                .iter()
                .map(|k| k.to_string())
                .collect::<Vec<String>>()
                .join(", ");

            match cur.condition.as_ref() {
                // この段で無傷に届く = 連鎖の終端
                Condition::Pristine => {
                    return if first {
                        write!(f, "{parts} が足りない")
                    } else {
                        write!(f, "{parts} を欠く")
                    };
                }
                // まだ段が続く
                Condition::Broken(next) => {
                    if first {
                        let pron = if cur.missing.len() == 1 {
                            "それを"
                        } else {
                            "それらを"
                        };
                        write!(f, "{parts} が足りず、{pron}補ってもなお ")?;
                    } else {
                        write!(f, "{parts} を欠き、さらに ")?;
                    }
                    cur = next;
                    first = false;
                }
            }
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let adjectives = if self.adjectives.is_empty() {
            "なし"
        } else {
            &self
                .adjectives
                .iter()
                .map(|a| a.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        };
        write!(
            f,
            "{}: \"{}\"\n  特徴: {}\n  状態: {}",
            self.name, self.description, adjectives, self.condition
        )
    }
}

impl Display for Room {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let items = self
            .items
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        write!(
            f,
            "--- {} ---\n{}\n\n{}",
            self.name, self.description, items
        )
    }
}

// --------------------------------------------------
// Parser
// --------------------------------------------------

fn parse_command(node: Node) -> Result<Command> {
    let child = node.first_element_child().context("empty command tag")?;
    match child.tag_name().name() {
        "switch" => Ok(Command::Switch(
            child
                .text()
                .context("empty switch result")?
                .trim()
                .to_owned(),
        )),
        "look" => Ok(Command::Look(parse_room(
            child.first_element_child().context("look: no room tag")?,
        )?)),
        "go" => Ok(Command::Go(parse_room(
            child.first_element_child().context("look: no room tag")?,
        )?)),
        "show" => Ok(Command::Show(parse_items(child)?)),
        "take" => Ok(Command::Take(parse_item(
            child.first_element_child().context("take: no item tag")?,
        )?)),
        "incinerate" => Ok(Command::Incinerate(parse_item(
            child
                .first_element_child()
                .context("incinerate: no item tag")?,
        )?)),
        "combine" => Ok(Command::Combine(parse_items(child)?)),
        "use" => {
            let message = child
                .children()
                .filter(|n| n.is_text())
                .filter_map(|n| n.text())
                .collect::<String>()
                .trim()
                .to_owned();
            let item = parse_item(child.first_element_child().context("use: no item tag")?)?;
            Ok(Command::Use((item, message)))
        }
        "examine" => Ok(Command::Examine(parse_item(
            child
                .first_element_child()
                .context("examine: no item tag")?,
        )?)),
        other => {
            let src = &child.document().input_text()[child.range()];
            bail!("NOT IMPLEMENTED: {other}\n{src}");
        }
    }
}

fn parse_response_failed(node: Node) -> Result<Response> {
    let reason = node
        .children()
        .find(|n| n.has_tag_name("reason"))
        .context("failed tag has no reason")?
        .text()
        .context("reason tag has no text")?
        .trim();
    let command = node
        .children()
        .find(|n| n.has_tag_name("command"))
        .context("failed tag has no command")?
        .first_element_child()
        .context("command tag has no children")?
        .tag_name()
        .name()
        .trim();
    Ok(Response::Failed(format!(
        "failed to `{command}`: {reason}\n"
    )))
}

fn parse_response(node: Node) -> Result<Response> {
    match node.tag_name().name() {
        "error" => Ok(Response::Error(
            node.children()
                .find(|n| n.has_tag_name("response"))
                .context("error has no response")?
                .text()
                .context("response has no text")?
                .trim()
                .to_owned(),
        )),
        "help" => Ok(Response::Help(
            node.text().context("no message in help")?.trim().to_owned(),
        )),
        "success" => {
            let command_node = node
                .children()
                .find(|n| n.has_tag_name("command"))
                .context("success has no command")?;
            let command = parse_command(command_node)?;
            Ok(Response::Success(command))
        }
        "failed" => parse_response_failed(node),
        other => {
            let src = &node.document().input_text()[node.range()];
            bail!("unknown response: {other}\n{src}")
        }
    }
}

fn parse_description(node: Node) -> Result<Description> {
    if let Some(child) = node.first_element_child() {
        match child.tag_name().name() {
            "redacted" => Ok(Description::Redacted),
            _ => bail!("undefined description inner tag"),
        }
    } else {
        Ok(Description::Text(
            node.text().unwrap_or_default().trim().to_owned(),
        ))
    }
}

fn parse_adjective(node: Node) -> Result<Adjective> {
    let text = node.text().unwrap_or_default().trim();
    Ok(Adjective(text.to_owned()))
}

fn parse_adjectives(node: Node) -> Result<Vec<Adjective>> {
    node.children()
        .filter(|child| child.is_element())
        .map(parse_adjective)
        .collect::<Result<Vec<_>>>()
}

fn parse_condition(node: Node) -> Result<Condition> {
    let child = node
        .first_element_child()
        .context("taking first element of condition")?;

    match child.tag_name().name() {
        "pristine" => Ok(Condition::Pristine),
        "broken" => {
            let condition_node = child
                .children()
                .find(|n| n.has_tag_name("condition"))
                .context("no condition tag in broken")?;
            let missing_node = child
                .children()
                .find(|n| n.has_tag_name("missing"))
                .context("no missing tag in broken")?;
            let condition = parse_condition(condition_node)?;
            let missing = parse_missing(missing_node)?;
            Ok(Condition::Broken(Broken::new(condition, missing)))
        }
        _ => bail!("malformed condition inner"),
    }
}

fn parse_kind(node: Node) -> Result<Kind> {
    let name_node = node
        .children()
        .find(|n| n.has_tag_name("name"))
        .context("kind without inner name")?;
    let condition_node = node
        .children()
        .find(|n| n.has_tag_name("condition"))
        .context("kind without inner condition")?;
    let condition = parse_condition(condition_node)?;
    let name = name_node.text().unwrap_or_default().trim().to_owned();

    Ok(Kind::new(name, condition))
}

fn parse_missing(node: Node) -> Result<Vec<Kind>> {
    node.children()
        .filter(|n| n.is_element())
        .map(parse_kind)
        .collect::<Result<Vec<_>>>()
}

fn sanitize_help(text: &str) -> Result<String> {
    let inner = text
        .strip_prefix("<help>")
        .and_then(|s| s.strip_suffix("</help>"))
        .context("malformed <help> tags")?
        .trim();
    let escaped = inner.replace('<', "&lt;").replace('>', "&gt;");
    Ok(format!("<help>{escaped}</help>"))
}

pub fn parse(bytes: &[u8]) -> Result<Response> {
    let text = str::from_utf8(bytes)?.trim();

    let xml = if text.starts_with("<help>") {
        sanitize_help(text)?
    } else {
        text.to_owned()
    };

    let doc = Document::parse(&xml).context("parsing xml from bytes")?;
    parse_response(doc.root_element())
}

fn parse_item(node: Node) -> Result<Item> {
    let name = node
        .children()
        .find(|n| n.has_tag_name("name"))
        .context("item tag has no name")?
        .text()
        .context("item name is empty")?
        .trim();
    let description = parse_description(
        node.children()
            .find(|n| n.has_tag_name("description"))
            .context("item tag has no description")?,
    )?;
    let adjectives = parse_adjectives(
        node.children()
            .find(|n| n.has_tag_name("adjectives"))
            .context("item tag has no adjectives")?,
    )?;
    let condition = parse_condition(
        node.children()
            .find(|n| n.has_tag_name("condition"))
            .context("item tag has no condition")?,
    )?;

    if let Some(piled_on) = node
        .children()
        .find(|n| n.has_tag_name("piled_on"))
        .context("item tag has no piled_on")?
        .first_element_child()
    {
        Ok(Item::new(
            name.to_owned(),
            description,
            adjectives,
            condition,
            Some(Box::new(
                parse_item(piled_on).context("malformed inner piled_on")?,
            )),
        ))
    } else {
        Ok(Item::new(
            name.to_owned(),
            description,
            adjectives,
            condition,
            None,
        ))
    }
}

/// 複数の `<item>` を [Vec] にして返す。
/// フラットな `<item></item>` の並びであればそのまま [Vec] にし、
/// ネストした `<item><piled_on><item>...` であればフラットにして [Vec] へ push する。
fn parse_items(node: Node) -> Result<Vec<Item>> {
    let mut items = Vec::new();

    for top in node.children().filter(|n| n.has_tag_name("item")) {
        let mut cur = Some(Box::new(parse_item(top)?));
        while let Some(mut b) = cur {
            cur = b.piled_on.take();
            items.push(*b);
        }
    }

    Ok(items)
}

fn parse_room(node: Node) -> Result<Room> {
    let name = node
        .children()
        .find(|n| n.has_tag_name("name"))
        .context("room tag has no name")?
        .text()
        .context("name tag is empty")
        .unwrap_or_default()
        .trim();
    let description = parse_description(
        node.children()
            .find(|n| n.has_tag_name("description"))
            .context("room tag has no description")?,
    )?;
    let items = parse_items(
        node.children()
            .find(|n| n.has_tag_name("items"))
            .context("room tag has no items")?,
    )?;
    Ok(Room::new(name.to_owned(), description, items))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parsed<T>(input: &str, f: impl FnOnce(Node) -> Result<T>) -> Result<T> {
        let doc = Document::parse(input).unwrap();
        f(doc.root_element())
    }

    mod display {
        use super::*;

        #[test]
        fn test_description() {
            assert_eq!(Description::Redacted.to_string(), "[検閲]");
            assert_eq!(Description::Text("TEXT".to_owned()).to_string(), "TEXT");
        }

        #[test]
        fn test_adjective() {
            assert_eq!(Adjective("red".to_owned()).to_string(), "red");
        }

        #[test]
        fn test_condition_pristine() {
            assert_eq!(Condition::Pristine.to_string(), "無傷");
        }

        #[test]
        fn test_broken_missing_is_broken() {
            // 正常系: missing の材料自体が壊れている (radio が破損)
            assert_eq!(
                Broken::new(
                    Condition::Pristine,
                    vec![Kind::new(
                        "radio".to_owned(),
                        Condition::Broken(Broken::new(
                            Condition::Pristine,
                            vec![Kind::new("antenna".to_owned(), Condition::Pristine)],
                        )),
                    )],
                )
                .to_string(),
                "壊れている: antenna を欠いた radio が足りない"
            );
        }

        #[test]
        fn test_condition_broken() {
            // 正常系: A-1920-IXB の二段階修理を style B で表示する
            assert_eq!(
                Condition::Broken(Broken::new(
                    // 補充後に残る不足: transistor
                    Condition::Broken(Broken::new(
                        Condition::Pristine,
                        vec![Kind::new("transistor".to_owned(), Condition::Pristine)],
                    )),
                    // 今すぐの不足: 壊れた radio・processor・bolt
                    vec![
                        Kind::new(
                            "radio".to_owned(),
                            Condition::Broken(Broken::new(
                                Condition::Pristine,
                                vec![Kind::new("antenna".to_owned(), Condition::Pristine)],
                            )),
                        ),
                        Kind::new("processor".to_owned(), Condition::Pristine),
                        Kind::new("bolt".to_owned(), Condition::Pristine),
                    ],
                ))
                .to_string(),
                "壊れている: antenna を欠いた radio, processor, bolt が足りず、それらを補ってもなお transistor を欠く"
            );
        }

        #[test]
        fn test_kind() {
            assert_eq!(
                Kind::new("test".to_owned(), Condition::Pristine).to_string(),
                "test"
            );
        }

        #[test]
        fn test_broken_to_pristine() {
            assert_eq!(
                Broken::new(
                    Condition::Pristine,
                    vec![Kind::new("test".to_owned(), Condition::Pristine)]
                )
                .to_string(),
                "壊れている: test が足りない"
            );
            assert_eq!(
                Broken::new(
                    Condition::Pristine,
                    vec![
                        Kind::new("foo".to_owned(), Condition::Pristine),
                        Kind::new("bar".to_owned(), Condition::Pristine)
                    ]
                )
                .to_string(),
                "壊れている: foo, bar が足りない"
            );
        }

        #[test]
        fn test_broken_to_broken() {
            assert_eq!(
                Broken::new(
                    Condition::Broken(Broken::new(
                        Condition::Pristine,
                        vec![Kind::new("transistor".to_owned(), Condition::Pristine)]
                    )),
                    vec![Kind::new("radio".to_owned(), Condition::Pristine)]
                )
                .to_string(),
                "壊れている: radio が足りず、それを補ってもなお transistor を欠く"
            );
        }

        #[test]
        fn test_item() {
            assert_eq!(
                Item::new(
                    "foo".to_owned(),
                    Description::Redacted,
                    vec![],
                    Condition::Pristine,
                    None
                )
                .to_string(),
                "foo: \"[検閲]\"\n  特徴: なし\n  状態: 無傷"
            )
        }

        #[test]
        fn test_room() {
            assert_eq!(
                Room::new(
                    "testroom".to_owned(),
                    Description::Redacted,
                    vec![Item::new(
                        "foo".to_owned(),
                        Description::Redacted,
                        vec![],
                        Condition::Pristine,
                        None
                    ),]
                )
                .to_string(),
                "--- testroom ---\n[検閲]\n\nfoo: \"[検閲]\"\n  特徴: なし\n  状態: 無傷"
            )
        }
    }
    mod room {
        use super::*;

        // 部屋の名前、詳細、アイテムすべてを [Room] にする。
        #[test]
        fn test_room_with_a_door() {
            let input = " <room> <name> Room With a Door </name> <description> You are in a room with a mechanical door. You will probably need to use a keypad to unlock it. A hallway leads north. </description> <items> <item> <name> pamphlet </name> <description> standard municipal fare. It reads, The City of Chicago's Refuse and Recycling Program combines modern trash classification with cybernetic labor to keep our city beautiful, while at the same time minimizing waste and limiting consumer spending. In keeping with our motto of \"One Resident's Trash Is Another Resident's Treasure,\" unwanted items are collected, repaired, and redistributed to other residents who would have purchased them anyway. Residents should contribute to the city's program by leaving heaps of items unwanted on the sidewalk on collection day </description> <adjectives> </adjectives> <condition> <pristine> </pristine> </condition> <piled_on> <item> <name> manifesto </name> <description> <redacted/> </description> <adjectives> </adjectives> <condition> <pristine> </pristine> </condition> <piled_on> </piled_on> </item> </piled_on> </item> </items> </room>";
            let room = parsed(input, parse_room).unwrap();

            assert_eq!(room.name, "Room With a Door".to_owned());
            assert_eq!(room.description, Description::Text("You are in a room with a mechanical door. You will probably need to use a keypad to unlock it. A hallway leads north.".to_owned()));
            assert_eq!(room.items.len(), 2);
        }
    }
    mod item {
        use super::*;

        // フラットなアイテムを Vec にする。
        #[test]
        fn test_parse_flat_items() {
            let input = "<show> <item> <name> manifesto </name> <description> <redacted/> </description> <adjectives> </adjectives> <condition> <pristine> </pristine> </condition> <piled_on> </piled_on> </item> <item> <name> pamphlet </name> <description> standard municipal fare. It reads, The City of Chicago's Refuse and Recycling Program combines modern trash classification with cybernetic labor to keep our city beautiful, while at the same time minimizing waste and limiting consumer spending. In keeping with our motto of \"One Resident's Trash Is Another Resident's Treasure,\" unwanted items are collected, repaired, and redistributed to other residents who would have purchased them anyway. Residents should contribute to the city's program by leaving heaps of items unwanted on the sidewalk on collection day </description> <adjectives> </adjectives> <condition> <pristine> </pristine> </condition> <piled_on> </piled_on> </item> </show>";
            assert_eq!(
                parsed(input, parse_items).unwrap(),
                vec![
                    Item {
                        name: "manifesto".to_owned(),
                        description: Description::Redacted,
                        adjectives: vec![],
                        condition: Condition::Pristine,
                        piled_on: None
                    },
                    Item {
                        name: "pamphlet".to_owned(),
                        description: Description::Text("standard municipal fare. It reads, The City of Chicago's Refuse and Recycling Program combines modern trash classification with cybernetic labor to keep our city beautiful, while at the same time minimizing waste and limiting consumer spending. In keeping with our motto of \"One Resident's Trash Is Another Resident's Treasure,\" unwanted items are collected, repaired, and redistributed to other residents who would have purchased them anyway. Residents should contribute to the city's program by leaving heaps of items unwanted on the sidewalk on collection day".to_owned()),
                        adjectives: vec![],
                        condition: Condition::Pristine,
                        piled_on: None
                    },
                ]
            );
        }

        // ネストしたアイテムをフラットな Vec にする。
        #[test]
        fn test_parse_items() {
            let input = "<items><item> <name> pamphlet </name> <description> standard municipal fare. It reads, The City of Chicago's Refuse and Recycling Program combines modern trash classification with cybernetic labor to keep our city beautiful, while at the same time minimizing waste and limiting consumer spending. In keeping with our motto of \"One Resident's Trash Is Another Resident's Treasure,\" unwanted items are collected, repaired, and redistributed to other residents who would have purchased them anyway. Residents should contribute to the city's program by leaving heaps of items unwanted on the sidewalk on collection day </description> <adjectives> </adjectives> <condition> <pristine> </pristine> </condition> <piled_on> <item> <name> manifesto </name> <description> <redacted/> </description> <adjectives> </adjectives> <condition> <pristine> </pristine> </condition> <piled_on> </piled_on> </item> </piled_on> </item></items>";
            assert_eq!(
                parsed(input, parse_items).unwrap(),
                vec![
                    Item {
                        name: "pamphlet".to_owned(),
                        description: Description::Text("standard municipal fare. It reads, The City of Chicago's Refuse and Recycling Program combines modern trash classification with cybernetic labor to keep our city beautiful, while at the same time minimizing waste and limiting consumer spending. In keeping with our motto of \"One Resident's Trash Is Another Resident's Treasure,\" unwanted items are collected, repaired, and redistributed to other residents who would have purchased them anyway. Residents should contribute to the city's program by leaving heaps of items unwanted on the sidewalk on collection day".to_owned()),
                        adjectives: vec![],
                        condition: Condition::Pristine,
                        piled_on: None
                    },
                    Item {
                        name: "manifesto".to_owned(),
                        description: Description::Redacted,
                        adjectives: vec![],
                        condition: Condition::Pristine,
                        piled_on: None
                    }
                ]
            );
        }

        // 積み上がった (ネストした) アイテムを構造体にする。
        #[test]
        fn test_item_nested() {
            let input = "<item> <name> pamphlet </name> <description> standard municipal fare. It reads, The City of Chicago's Refuse and Recycling Program combines modern trash classification with cybernetic labor to keep our city beautiful, while at the same time minimizing waste and limiting consumer spending. In keeping with our motto of \"One Resident's Trash Is Another Resident's Treasure,\" unwanted items are collected, repaired, and redistributed to other residents who would have purchased them anyway. Residents should contribute to the city's program by leaving heaps of items unwanted on the sidewalk on collection day </description> <adjectives> </adjectives> <condition> <pristine> </pristine> </condition> <piled_on> <item> <name> manifesto </name> <description> <redacted/> </description> <adjectives> </adjectives> <condition> <pristine> </pristine> </condition> <piled_on> </piled_on> </item> </piled_on> </item>";
            assert_eq!(
                parsed(input, parse_item).unwrap(),
                Item {
                    name: "pamphlet".to_owned(),
                    description: Description::Text(
                        "standard municipal fare. It reads, The City of Chicago's Refuse and Recycling Program combines modern trash classification with cybernetic labor to keep our city beautiful, while at the same time minimizing waste and limiting consumer spending. In keeping with our motto of \"One Resident's Trash Is Another Resident's Treasure,\" unwanted items are collected, repaired, and redistributed to other residents who would have purchased them anyway. Residents should contribute to the city's program by leaving heaps of items unwanted on the sidewalk on collection day".to_owned()
                    ),
                    adjectives: vec![],
                    condition: Condition::Pristine,
                    piled_on: Some(Box::new(Item {
                        name: "manifesto".to_owned(),
                        description: Description::Redacted,
                        adjectives: vec![],
                        condition: Condition::Pristine,
                        piled_on: None
                    }))
                }
            );
        }

        // 積み上がっていないアイテムを構造体にする。
        #[test]
        fn test_item_on_top() {
            let input = "<item> <name> manifesto </name> <description> <redacted/> </description> <adjectives> </adjectives> <condition> <pristine> </pristine> </condition> <piled_on> </piled_on></item>";
            assert_eq!(
                parsed(input, parse_item).unwrap(),
                Item {
                    name: "manifesto".to_owned(),
                    description: Description::Redacted,
                    adjectives: vec![],
                    condition: Condition::Pristine,
                    piled_on: None
                }
            );
        }
    }
    mod entry {
        use super::*;

        // <help> に含まれる `<command>` をタグとしてパースしない
        #[test]
        fn test_help_command() {
            let bytes = b"<help>\nTry 'help <command>'\n</help>";
            assert_eq!(
                parse(bytes).unwrap(),
                Response::Help("Try 'help <command>'".to_owned())
            );
        }

        // 正常系: bytes を str→Document 化し、root から Response を組む（ここでは error 経路）
        #[test]
        fn test_bytes() {
            let bytes = b"<error><response>Huh?</response></error>";
            assert_eq!(parse(bytes).unwrap(), Response::Error("Huh?".to_owned()));
        }
    }
    mod response {
        use super::*;

        // 異常系: 不明なレスポンスの XML をエラーに含める
        #[test]
        fn test_unknown() {
            let input = "<unknown><message>unknown message</message></unknown>";
            let msg = format!("{}", parsed(input, parse_response).unwrap_err());

            assert_eq!(
                msg,
                "unknown response: unknown\n<unknown><message>unknown message</message></unknown>"
            );
        }

        // 異常系: `go` failed
        #[test]
        fn test_go_failed() {
            let input = "<failed><command><go>north</go></command><reason>there is no way north from here</reason></failed>";
            assert_eq!(
                parsed(input, parse_response).unwrap(),
                Response::Failed("failed to `go`: there is no way north from here\n".to_string())
            );
        }

        // 異常系: アイテムが拾えない
        #[test]
        fn test_take_failed() {
            let input = "<failed> <command> <take> <item> <name> manifesto </name> <description> <redacted/> </description> <adjectives> </adjectives> <condition> <pristine> </pristine> </condition> <piled_on> </piled_on> </item> </take> </command> <reason> there is another item on top of it (take the other item first) </reason> </failed>";

            assert_eq!(
                parsed(input, parse_response).unwrap(),
                Response::Failed(
                    "failed to `take`: there is another item on top of it (take the other item first)\n".to_owned()
                )
            );
        }

        // 正常系: <help> 直下のテキストを trim して Response::Help に
        #[test]
        fn test_help() {
            let input = "<help>\n  examine: Inspect an item or your environment. Synonyms include ex, x, look, and l.\n </help>";
            assert_eq!(
                parsed(input, parse_response).unwrap(),
                Response::Help("examine: Inspect an item or your environment. Synonyms include ex, x, look, and l.".to_owned())
            );
        }

        // 正常系: <error> 内の <response> テキストを trim して Response::Error に
        #[test]
        fn test_error() {
            let input = "<error> <response>\nHuh? Try 'help'.\n</response></error>";
            assert_eq!(
                parsed(input, parse_response).unwrap(),
                Response::Error("Huh? Try 'help'.".to_owned())
            );
        }

        // 正常系: <success> → <command> → <switch> を辿り Response::Success(Command::Switch) に
        #[test]
        fn test_success() {
            let input = "\n<success>\n <command>\n<switch>XML</switch></command></success>";
            assert_eq!(
                parsed(input, parse_response).unwrap(),
                Response::Success(Command::Switch("XML".to_owned()))
            );
        }
    }
    mod command {
        use super::*;

        // 異常系: 未実装コマンドはタグ名だけでなく元 XML 片をエラーに載せる
        #[test]
        fn test_unimplemented_command_dumps_xml() {
            let input = "<command><test_command><room>x</room></test_command></command>";
            let msg = format!("{}", parsed(input, parse_command).unwrap_err());
            assert_eq!(
                msg,
                "NOT IMPLEMENTED: test_command\n<test_command><room>x</room></test_command>"
            );
        }

        // 正常系: <command> 直下の <switch> テキストを trim して Command::Switch に
        #[test]
        fn test_switch() {
            let input = "\n<command>\n<switch>\n XML\n</switch></command>";

            assert_eq!(
                parsed(input, parse_command).unwrap(),
                Command::Switch("XML".to_owned())
            );
        }

        // 正常系: look
        #[test]
        fn test_look() {
            let input = "<command><look> <room> <name> Room With a Door </name> <description> You are in a room with a mechanical door. You will probably need to use a keypad to unlock it. A hallway leads north. </description> <items> <item> <name> pamphlet </name> <description> standard municipal fare. It reads, The City of Chicago's Refuse and Recycling Program combines modern trash classification with cybernetic labor to keep our city beautiful, while at the same time minimizing waste and limiting consumer spending. In keeping with our motto of \"One Resident's Trash Is Another Resident's Treasure,\" unwanted items are collected, repaired, and redistributed to other residents who would have purchased them anyway. Residents should contribute to the city's program by leaving heaps of items unwanted on the sidewalk on collection day </description> <adjectives> </adjectives> <condition> <pristine> </pristine> </condition> <piled_on> <item> <name> manifesto </name> <description> <redacted/> </description> <adjectives> </adjectives> <condition> <pristine> </pristine> </condition> <piled_on> </piled_on> </item> </piled_on> </item> </items> </room> </look> </command> ";

            assert!(matches!(
                parsed(input, parse_command).unwrap(),
                Command::Look(_)
            ))
        }

        // 正常系: go
        #[test]
        fn test_go() {
            let input = "<command><go> <room> <name> Room With a Door </name> <description> You are in a room with a mechanical door. You will probably need to use a keypad to unlock it. A hallway leads north. </description> <items> <item> <name> pamphlet </name> <description> standard municipal fare. It reads, The City of Chicago's Refuse and Recycling Program combines modern trash classification with cybernetic labor to keep our city beautiful, while at the same time minimizing waste and limiting consumer spending. In keeping with our motto of \"One Resident's Trash Is Another Resident's Treasure,\" unwanted items are collected, repaired, and redistributed to other residents who would have purchased them anyway. Residents should contribute to the city's program by leaving heaps of items unwanted on the sidewalk on collection day </description> <adjectives> </adjectives> <condition> <pristine> </pristine> </condition> <piled_on> <item> <name> manifesto </name> <description> <redacted/> </description> <adjectives> </adjectives> <condition> <pristine> </pristine> </condition> <piled_on> </piled_on> </item> </piled_on> </item> </items> </room> </go> </command> ";

            assert!(matches!(
                parsed(input, parse_command).unwrap(),
                Command::Go(_)
            ))
        }

        // 正常系: 空のインベントリ
        #[test]
        fn test_show_empty() {
            let input = "<command><show> </show></command>";

            assert!(matches!(
                parsed(input, parse_command).unwrap(),
                Command::Show(_)
            ))
        }

        // 正常系: 空ではないインベントリ
        #[test]
        fn test_show() {
            let input = "<command><show> <item> <name> manifesto </name> <description> <redacted/> </description> <adjectives> </adjectives> <condition> <pristine> </pristine> </condition> <piled_on> </piled_on> </item> <item> <name> pamphlet </name> <description> standard municipal fare. It reads, The City of Chicago's Refuse and Recycling Program combines modern trash classification with cybernetic labor to keep our city beautiful, while at the same time minimizing waste and limiting consumer spending. In keeping with our motto of \"One Resident's Trash Is Another Resident's Treasure,\" unwanted items are collected, repaired, and redistributed to other residents who would have purchased them anyway. Residents should contribute to the city's program by leaving heaps of items unwanted on the sidewalk on collection day </description> <adjectives> </adjectives> <condition> <pristine> </pristine> </condition> <piled_on> </piled_on> </item> </show></command>";

            let Command::Show(items) = parsed(input, parse_command).unwrap() else {
                panic!("expected Show");
            };
            assert_eq!(items.len(), 2);
        }

        // 正常系: アイテムを拾う
        #[test]
        fn test_take() {
            let input = "<command><take> <item> <name> manifesto </name> <description> <redacted/> </description> <adjectives> </adjectives> <condition> <pristine> </pristine> </condition> <piled_on> </piled_on> </item> </take></command>";

            let Command::Take(item) = parsed(input, parse_command).unwrap() else {
                panic!("expected Take");
            };
            assert_eq!(item.name, "manifesto");
        }

        // 正常系: アイテムを削除する
        #[test]
        fn test_incinerate() {
            let input = "<command><incinerate> <item> <name> pamphlet </name> <description> standard municipal fare. It reads, The City of Chicago's Refuse and Recycling Program combines modern trash classification with cybernetic labor to keep our city beautiful, while at the same time minimizing waste and limiting consumer spending. In keeping with our motto of \"One Resident's Trash Is Another Resident's Treasure,\" unwanted items are collected, repaired, and redistributed to other residents who would have purchased them anyway. Residents should contribute to the city's program by leaving heaps of items unwanted on the sidewalk on collection day </description> <adjectives> </adjectives> <condition> <pristine> </pristine> </condition> <piled_on> </piled_on> </item> </incinerate></command>";

            let Command::Incinerate(item) = parsed(input, parse_command).unwrap() else {
                panic!("expected Incinerate");
            };
            assert_eq!(item.name, "pamphlet");
        }

        // 正常系: アイテムを組み合わせる
        #[test]
        fn test_combine() {
            let input = "<command><combine> <item> <name> processor </name> <description> from the elusive 19x86 line </description> <adjectives> </adjectives> <condition> <broken> <condition> <pristine> </pristine> </condition> <missing> <kind> <name> cache </name> <condition> <pristine> </pristine> </condition> </kind> </missing> </broken> </condition> <piled_on> </piled_on> </item> <item> <name> cache </name> <description> fully-associative </description> <adjectives> </adjectives> <condition> <pristine> </pristine> </condition> <piled_on> </piled_on> </item> </combine></command>";

            let Command::Combine(items) = parsed(input, parse_command).unwrap() else {
                panic!("expected Combine");
            };
            assert_eq!(items.len(), 2);
            assert_eq!(items[0].name, "processor");
            assert_eq!(items[1].name, "cache");
        }

        // 正常系: アイテムを使用する
        #[test]
        fn test_use() {
            let input = "<command><use> <item> <name> keypad </name> <description> labeled \"use me\" </description> <adjectives> </adjectives> <condition> <pristine> </pristine> </condition> <piled_on> </piled_on> </item> You unlock and open the door. Passing through, you find yourself on the streets of Chicago. Seeing no reason you should ever go back, you allow the door to close behind you. </use></command>";

            let Command::Use((item, message)) = parsed(input, parse_command).unwrap() else {
                panic!("expected Use");
            };
            assert_eq!(item.name, "keypad");
            assert_eq!(
                message,
                "You unlock and open the door. Passing through, you find yourself on the streets of Chicago. Seeing no reason you should ever go back, you allow the door to close behind you."
            );
        }

        // 正常系: アイテムを調べる
        #[test]
        fn test_examine() {
            let input = "<command><examine> <item> <name> manual </name> <description> <redacted/> </description> <adjectives> </adjectives> <condition> <pristine> </pristine> </condition> <piled_on> </piled_on> </item> </examine></command>";

            let Command::Examine(item) = parsed(input, parse_command).unwrap() else {
                panic!("expected Examine");
            };
            assert_eq!(item.name, "manual");
        }
    }
    mod kind {
        use super::*;

        // 正常系: <missing> 内の <kind> 群 → Vec<Kind>（複数）
        #[test]
        fn test_missing_collects_kinds() {
            let input = "<missing>\n <kind><name>transistor</name><condition><pristine></pristine></condition></kind>\n <kind><name>antenna</name><condition><pristine></pristine></condition></kind>\n </missing>";

            assert_eq!(
                parsed(input, parse_missing).unwrap(),
                vec![
                    Kind {
                        name: String::from("transistor"),
                        condition: Condition::Pristine
                    },
                    Kind {
                        name: String::from("antenna"),
                        condition: Condition::Pristine
                    },
                ]
            );
        }

        // 正常系: <kind> は name と condition を持つ → Kind に落ちる
        #[test]
        fn test_kind() {
            let input = "<kind>\n<name>antenna</name>\n<condition><pristine></pristine></condition>\n</kind>";
            assert_eq!(
                parsed(input, parse_kind).unwrap(),
                Kind::new("antenna".into(), Condition::Pristine)
            );
        }
    }
    mod condition {
        use super::*;

        #[test]
        fn test_broken() {
            let input = "<condition><broken>\n  <condition><pristine></pristine></condition>\n  <missing>\n<kind>\n<name>\nantenna</name>\n<condition>\n<pristine></pristine>\n</condition>\n</kind>\n</missing>\n</broken>\n</condition>\n";

            assert_eq!(
                parsed(input, parse_condition).unwrap(),
                Condition::Broken(Broken {
                    condition: Box::new(Condition::Pristine),
                    missing: vec![Kind::new("antenna".into(), Condition::Pristine)]
                })
            );
        }

        #[test]
        fn test_pristine() {
            let input = "\n<condition> <pristine> </pristine></condition>";

            assert_eq!(parsed(input, parse_condition).unwrap(), Condition::Pristine);
        }
    }
    mod adjectives {
        use super::*;

        // 正常系: 1 件 → 要素 1 個の Vec
        #[test]
        fn test_one_covered_adjective() {
            let input = "<adjectives><adjective>red</adjective></adjectives>";

            assert_eq!(
                parsed(input, parse_adjectives).unwrap(),
                vec![Adjective("red".to_owned())]
            );
        }

        // 正常系: 子要素が無ければ空の Vec
        #[test]
        fn test_empty_adjectives() {
            let input = "<adjectives></adjectives>";
            assert!(parsed(input, parse_adjectives).unwrap().is_empty());
        }

        // 正常系: 要素間の空白 text node は is_element で除かれ、空の Vec にする
        #[test]
        fn test_whitespace_only_is_empty() {
            let input = "\n<adjectives>\n              </adjectives>\n";
            assert!(parsed(input, parse_adjectives).unwrap().is_empty());
        }
    }
    mod adjective {
        use super::*;

        // 正常系: 前後の空白を trim
        #[test]
        fn test_trims_surrounding_whitespace() {
            let input = "<adjective>\n        green\n       </adjective>";
            assert_eq!(
                parsed(input, parse_adjective).unwrap(),
                Adjective("green".to_owned())
            );
        }
    }
    mod description {
        use super::*;

        #[test]
        fn test_undefined_inner_tag() {
            let input = "<description><undefined/></description>";
            assert!(parsed(input, parse_description).is_err(),)
        }
        #[test]
        fn test_redacted() {
            let input = "<description><redacted/></description>";
            assert_eq!(
                parsed(input, parse_description).unwrap(),
                Description::Redacted
            );
        }

        #[test]
        fn test_text() {
            let input = "<description>inner text</description>";
            let text = String::from("inner text");
            assert_eq!(
                parsed(input, parse_description).unwrap(),
                Description::Text(text)
            );
        }
    }
}
