use crate::data_parsing::*;
use logos::{Lexer, Logos};
use lsp_types::*;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Logos)]
pub enum Token {
    #[error]
    Error,
    #[token("Show")]
    Show,
    #[token("Hide")]
    Hide,
    #[token("Continue")]
    Continue,
    #[token("#", ignore_comments)]
    Hash,
    #[regex(" | |", logos::skip)]
    Skip,
    #[token("\n")]
    EndLine,

    // Conditions
    #[token("AreaLevel")]
    AreaLevel,
    #[token("ItemLevel")]
    ItemLevel,
    #[token("DropLevel")]
    DropLevel,
    #[token("Quality")]
    Quality,
    #[token("Rarity")]
    Rarity,
    #[token("Class")]
    Class,
    #[token("BaseType")]
    BaseType,
    #[token("Prophecy")]
    Prophecy,
    #[token("LinkedSockets")]
    LinkedSockets,
    #[token("SocketGroup")]
    SocketGroup,
    #[token("Sockets")]
    Sockets,
    #[token("Height")]
    Height,
    #[token("Width")]
    Width,
    #[token("HasExplicitMod")]
    HasExplicitMod,
    #[token("AnyEnchantment")]
    AnyEnchantment,
    #[token("HasEnchantment")]
    HasEnchantment,
    #[token("StackSize")]
    StackSize,
    #[token("GemLevel")]
    GemLevel,
    #[token("Identified")]
    Identified,
    #[token("Corrupted")]
    Corrupted,
    #[token("CorruptedMods")]
    CorruptedMods,
    #[token("Mirrored")]
    Mirrored,
    #[token("ElderItem")]
    ElderItem,
    #[token("ShaperItem")]
    ShaperItem,
    #[token("HasInfluence")]
    HasInfluence,
    #[token("FracturedItem")]
    FracturedItem,
    #[token("SynthesisedItem")]
    SynthesisedItem,
    #[token("ShapedMap")]
    ShapedMap,
    #[token("MapTier")]
    MapTier,

    // Actions
    #[token("SetBorderColor")]
    SetBorderColor,
    #[token("SetTextColor")]
    SetTextColor,
    #[token("SetBackgroundColor")]
    SetBackgroundColor,
    #[token("SetFontSize")]
    SetFontSize,
    #[token("PlayAlertSound")]
    PlayAlertSound,
    #[token("PlayAlertSoundPositional")]
    PlayAlertSoundPositional,
    #[token("DisableDropSound")]
    DisableDropSound,
    #[token("CustomAlertSound")]
    CustomAlertSound,
    #[token("MinimapIcon")]
    MinimapIcon,
    #[token("PlayEffect")]
    PlayEffect,

    // Values
    #[regex("[0-9]+", |s| s.slice().to_string())]
    Numbers(String),
    #[regex("\"([^\"]*)\"", |s| s.slice().to_string())]
    Quotes(String),
    #[regex("true|false|True|False", |s| s.slice().to_string())]
    Boolean(String),
    #[regex("[a-zA-Z]+", |s| s.slice().to_string())]
    Text(String),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum KeywordType {
    Conditions,
    Actions,
    Block,
    Operations,
    Values(String),
}
impl KeywordType {
    pub fn token_list(&self) -> Vec<Token> {
        match self {
            KeywordType::Conditions => vec![
                Token::AreaLevel,
                Token::ItemLevel,
                Token::DropLevel,
                Token::Quality,
                Token::Rarity,
                Token::Class,
                Token::BaseType,
                Token::Prophecy,
                Token::LinkedSockets,
                Token::SocketGroup,
                Token::Sockets,
                Token::Height,
                Token::Width,
                Token::HasExplicitMod,
                Token::AnyEnchantment,
                Token::HasEnchantment,
                Token::StackSize,
                Token::GemLevel,
                Token::Identified,
                Token::Corrupted,
                Token::CorruptedMods,
                Token::Mirrored,
                Token::ElderItem,
                Token::ShaperItem,
                Token::HasInfluence,
                Token::FracturedItem,
                Token::SynthesisedItem,
                Token::ShapedMap,
                Token::MapTier,
            ],
            KeywordType::Actions => vec![
                Token::SetBorderColor,
                Token::SetTextColor,
                Token::SetBackgroundColor,
                Token::SetFontSize,
                Token::PlayAlertSound,
                Token::PlayAlertSoundPositional,
                Token::DisableDropSound,
                Token::CustomAlertSound,
                Token::MinimapIcon,
                Token::PlayEffect,
            ],
            KeywordType::Block => vec![Token::Show, Token::Hide, Token::Continue],
            KeywordType::Operations => vec![Token::Error],
            KeywordType::Values(_) => vec![
                Token::Numbers(String::new()),
                Token::Quotes(String::new()),
                Token::Boolean(String::new()),
                Token::Text(String::new()),
            ],
        }
    }
}

impl Token {
    pub fn create_completion_item(&self) -> CompletionItem {
        CompletionItem {
            label: format!("{:?}", self),
            kind: {
                if let Some(keyword) = self.keyword_type() {
                    match keyword {
                        KeywordType::Conditions => Some(CompletionItemKind::Property),
                        KeywordType::Actions => Some(CompletionItemKind::Method),
                        KeywordType::Block => Some(CompletionItemKind::Constant),
                        KeywordType::Operations => Some(CompletionItemKind::Operator),
                        KeywordType::Values(_) => Some(CompletionItemKind::Variable),
                    }
                } else {
                    None
                }
            },
            detail: Some(self.description()),
            ..CompletionItem::default()
        }
    }

    pub fn valid_values(&self, poe_data: PoeData) -> Option<Vec<Record>> {
        match self {
            Token::HasExplicitMod => Some(poe_data.mods),
            Token::Class => Some(poe_data.classes),
            Token::BaseType => Some(poe_data.bases),
            Token::PlayEffect => Some(vec![
                Record {name: Some("Red".to_string()), ..Default::default()},
                Record {name: Some("Green".to_string()), ..Default::default()},
                Record {name: Some("Blue".to_string()), ..Default::default()},
                Record {name: Some("Brown".to_string()), ..Default::default()},
                Record {name: Some("White".to_string()), ..Default::default()},
                Record {name: Some("Yellow".to_string()), ..Default::default()},
                Record {name: Some("Cyan".to_string()), ..Default::default()},
                Record {name: Some("Grey".to_string()), ..Default::default()},
                Record {name: Some("Orange".to_string()), ..Default::default()},
                Record {name: Some("Pink".to_string()), ..Default::default()},
                Record {name: Some("Purple".to_string()), ..Default::default()},
                ]),
            Token::MinimapIcon => Some(vec![
                Record {name: Some("Red".to_string()), ..Default::default()},
                Record {name: Some("Green".to_string()), ..Default::default()},
                Record {name: Some("Blue".to_string()), ..Default::default()},
                Record {name: Some("Brown".to_string()), ..Default::default()},
                Record {name: Some("White".to_string()), ..Default::default()},
                Record {name: Some("Yellow".to_string()), ..Default::default()},
                Record {name: Some("Cyan".to_string()), ..Default::default()},
                Record {name: Some("Grey".to_string()), ..Default::default()},
                Record {name: Some("Orange".to_string()), ..Default::default()},
                Record {name: Some("Pink".to_string()), ..Default::default()},
                Record {name: Some("Purple".to_string()), ..Default::default()},
                Record {name: Some("Circle".to_string()), ..Default::default()},
                Record {name: Some("Diamond".to_string()), ..Default::default()},
                Record {name: Some("Hexagon".to_string()), ..Default::default()},
                Record {name: Some("Square".to_string()), ..Default::default()},
                Record {name: Some("Star".to_string()), ..Default::default()},
                Record {name: Some("Triangle".to_string()), ..Default::default()},
                Record {name: Some("Cross".to_string()), ..Default::default()},
                Record {name: Some("Moon".to_string()), ..Default::default()},
                Record {name: Some("Raindrop".to_string()), ..Default::default()},
                Record {name: Some("Kite".to_string()), ..Default::default()},
                Record {name: Some("Pentagon".to_string()), ..Default::default()},
                Record {name: Some("UpsideDownHouse".to_string()), ..Default::default()},
                ]),
            Token::Rarity => Some(vec![
                Record {name: Some("Normal".to_string()), ..Default::default()},
                Record {name: Some("Magic".to_string()), ..Default::default()},
                Record {name: Some("Rare".to_string()), ..Default::default()},
                Record {name: Some("Unique".to_string()), ..Default::default()},
            ]),
            Token::HasInfluence => Some(vec![
                Record {name: Some("Shaper".to_string()), ..Default::default()},
                Record {name: Some("Elder".to_string()), ..Default::default()},
                Record {name: Some("Crusader".to_string()), ..Default::default()},
                Record {name: Some("Hunter".to_string()), ..Default::default()},
                Record {name: Some("Redeemer".to_string()), ..Default::default()},
                Record {name: Some("Warlord".to_string()), ..Default::default()},
            ]),
            _ => None,
        }
    }

    pub fn keyword_type(&self) -> Option<KeywordType> {
        match self {
            Token::Error => None,
            Token::Hash => None,
            Token::Skip => None,
            Token::EndLine => None,
            // blocks
            Token::Show => Some(KeywordType::Block),
            Token::Hide => Some(KeywordType::Block),
            Token::Continue => Some(KeywordType::Block),
            //contisitons
            Token::AreaLevel => Some(KeywordType::Conditions),
            Token::ItemLevel => Some(KeywordType::Conditions),
            Token::DropLevel => Some(KeywordType::Conditions),
            Token::Quality => Some(KeywordType::Conditions),
            Token::Rarity => Some(KeywordType::Conditions),
            Token::Class => Some(KeywordType::Conditions),
            Token::BaseType => Some(KeywordType::Conditions),
            Token::Prophecy => Some(KeywordType::Conditions),
            Token::LinkedSockets => Some(KeywordType::Conditions),
            Token::SocketGroup => Some(KeywordType::Conditions),
            Token::Sockets => Some(KeywordType::Conditions),
            Token::Height => Some(KeywordType::Conditions),
            Token::Width => Some(KeywordType::Conditions),
            Token::HasExplicitMod => Some(KeywordType::Conditions),
            Token::AnyEnchantment => Some(KeywordType::Conditions),
            Token::HasEnchantment => Some(KeywordType::Conditions),
            Token::StackSize => Some(KeywordType::Conditions),
            Token::GemLevel => Some(KeywordType::Conditions),
            Token::Identified => Some(KeywordType::Conditions),
            Token::Corrupted => Some(KeywordType::Conditions),
            Token::CorruptedMods => Some(KeywordType::Conditions),
            Token::Mirrored => Some(KeywordType::Conditions),
            Token::ElderItem => Some(KeywordType::Conditions),
            Token::ShaperItem => Some(KeywordType::Conditions),
            Token::HasInfluence => Some(KeywordType::Conditions),
            Token::FracturedItem => Some(KeywordType::Conditions),
            Token::SynthesisedItem => Some(KeywordType::Conditions),
            Token::ShapedMap => Some(KeywordType::Conditions),
            Token::MapTier => Some(KeywordType::Conditions),
            //actions
            Token::SetBorderColor => Some(KeywordType::Actions),
            Token::SetTextColor => Some(KeywordType::Actions),
            Token::SetBackgroundColor => Some(KeywordType::Actions),
            Token::SetFontSize => Some(KeywordType::Actions),
            Token::PlayAlertSound => Some(KeywordType::Actions),
            Token::PlayAlertSoundPositional => Some(KeywordType::Actions),
            Token::DisableDropSound => Some(KeywordType::Actions),
            Token::CustomAlertSound => Some(KeywordType::Actions),
            Token::MinimapIcon => Some(KeywordType::Actions),
            Token::PlayEffect => Some(KeywordType::Actions),
            // values
            Token::Numbers(s) => Some(KeywordType::Values(s.to_owned())),
            Token::Quotes(s) => Some(KeywordType::Values(s.to_owned())),
            Token::Boolean(s) => Some(KeywordType::Values(s.to_owned())),
            Token::Text(s) => Some(KeywordType::Values(s.to_owned())),
        }
    }
}

impl Default for Token {
    fn default() -> Self {
        Token::Error
    }
}

#[derive(PartialEq, Debug, Clone, Default)]
pub struct Filter {
    pub vec: Vec<FilterBlock>,
}
impl Filter {
    pub fn search_bytes(&self, byte: usize) -> Option<Token> {
        for block in self.vec.iter() {
            if block.bspan.start <= byte && block.bspan.end >= byte {
                return block.block.clone();
            } else {
                for line in block.keywords.iter() {
                    if line.span.start <= byte && line.span.end >= byte {
                        return Some(line.token.clone());
                    } else {
                        for value in line.value.iter() {
                            if value.span.start <= byte && value.span.end >= byte {
                                return Some(value.token.clone());
                            }
                        }
                    }
                }
            }
        }
        None
    }
    pub fn search_block(&self, byte: usize) -> Option<&FilterBlock> {
        for block in self.vec.iter().rev() {
            if block.bspan.start <= byte {
                return Some(block);
            }
        }
        None
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct FilterBlock {
    pub block: Option<Token>,
    pub keywords: Vec<TokenAndSpan>,
    // pub tspan: std::ops::Range<usize>,
    pub bspan: std::ops::Range<usize>,
}
impl FilterBlock {
    pub fn search_keyword(&self, byte: usize) -> Option<Token> {
        for keyword in self.keywords.iter().rev() {
            if keyword.span.start <= byte {
                return Some(keyword.token.clone());
            }
        }
        None
    }
}
impl Default for FilterBlock {
    fn default() -> FilterBlock {
        FilterBlock {
            block: None,
            keywords: vec![],
            bspan: 0..1,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct TokenAndSpan {
    pub token: Token,
    // pub tspan: std::ops::Range<usize>,
    pub span: std::ops::Range<usize>,
    pub value: Vec<ValueAndSpan>,
}
#[derive(PartialEq, Debug, Clone)]
pub struct ValueAndSpan {
    pub token: Token,
    pub span: std::ops::Range<usize>,
    pub value: String,
}

fn match_filter(
    filter: &mut Filter,
    token: Token,
    span: std::ops::Range<usize>,
    block: &mut FilterBlock,
) {
    if let Some(key) = token.keyword_type() {
        match key {
            KeywordType::Block => {
                new_block(filter, token, span.clone(), block);
            }
            KeywordType::Conditions => add_keyword(token, span, block),
            KeywordType::Actions => add_keyword(token, span, block),
            KeywordType::Operations => {}
            KeywordType::Values(s) => {
                add_values(token, span.clone(), block, s);
            }
        }
    }
}

pub fn parse(filter_file: &str) -> Filter {
    let mut filter = Filter::default();
    let mut block = FilterBlock::default();
    let mut lex = Token::lexer(filter_file).spanned();
    while let Some((token, span)) = lex.next() {
        match_filter(&mut filter, token.clone(), span.clone(), &mut block);
    }
    filter.vec.push(block.clone());
    filter
}

fn new_block(
    filter: &mut Filter,
    token: Token,
    span: std::ops::Range<usize>,
    block: &mut FilterBlock,
) {
    if let Some(_) = &block.block {
        filter.vec.push(block.clone());
    }
    block.block = Some(token.clone());
    block.keywords = vec![];
    block.bspan = span
}

fn add_keyword(token: Token, span: std::ops::Range<usize>, block: &mut FilterBlock) {
    block.keywords.push(TokenAndSpan {
        token: token.clone(),
        span: span,
        value: vec![],
    })
}

fn add_values(token: Token, span: std::ops::Range<usize>, block: &mut FilterBlock, string: String) {
    if let Some(last_key) = block.keywords.last_mut() {
        last_key.value.push(ValueAndSpan {
            token: token.clone(),
            span: span,
            value: string,
        });
    };
}

pub fn ignore_comments(lex: &mut Lexer<Token>) {
    if lex.slice() == "#" {
        loop {
            match lex.next() {
                Some(Token::EndLine) => break,
                _ => {}
            }
        }
    }
}

enum Arguments {
    Operator,
    Value,
    ValidNumeric,
    ValidRarity,
    ValidClass,
    ValidQuality,
    ValidItem,
    ValidProphecy,
    ValidLinks,
    ValidLSockets,
    ValidSockets,
    ValidHeight,
    ValidWidth,
    Boolean,
    Enchant,
    StackSize,
    GemLevel,
    CorruptedMods,
    HasInfluence,
    MapTier,
    RGB,
    FontSize,
    AlertSound,
    CustomSound,
    PlayEffect,
    MinimapIcon,
}

impl Arguments {
    fn description(&self) -> String {
        match self {
            Arguments::Operator => String::from("The following Operators can be used with numeric conditions. Note that for equal operations there is no operator required.  \n
    <   Less than
    <=  Less than or equal to
    >   Greater than
    >=  Greater or equal to
    =   Equal to"),
            Arguments::Value => String::new(),
            Arguments::ValidNumeric => String::from("Valid Values: Numeric Level (0-100)"),
            Arguments::ValidClass => String::from("Valid Values: Full or partial item class name"),
            Arguments::ValidRarity => String::from("Valid Values: Normal, Magic, Rare, Unique"),
            Arguments::ValidQuality => String::from("Valid Values: Numeric Quality (0-20)"),
            Arguments::ValidItem => String::from("Valid Values: Full or partial item name"),
            Arguments::ValidProphecy => String::from("Valid Values: Full or partial prophecy name"),
            Arguments::ValidLinks => String::from("Valid Values: Numeric Number of Links (0-6)"),
            Arguments::ValidLSockets => String::from("Valid Values: Numeric Number of Linked Sockets (2-6) followed by R, G, B, D, A, W"),
            Arguments::ValidSockets => String::from("Valid Values: Numeric Number of Sockets (0-6) followed by R, G, B, D, A, W"),
            Arguments::ValidHeight => String::from("Valid Values: Numeric number of slots (1-4)"),
            Arguments::ValidWidth => String::from("Valid Values: Numeric number of slots (1-2)"),
            Arguments::Boolean => String::from("Valid Values: True or False"),
            Arguments::Enchant => String::from("Valid Values: Full or partial name of enchantment"),
            Arguments::StackSize => String::from("Valid Values: Numeric number of slots (1-n)"),
            Arguments::GemLevel => String::from("Valid Values: Numeric number of slots (1-21)"),
            Arguments::CorruptedMods => String::from("Valid Values: Numeric number of corrupted mods (0-n)."),
            Arguments::HasInfluence => String::from("Valid Values: Shaper, Elder, Crusader, Hunter, Redeemer, Warlord"),
            Arguments::MapTier => String::from("Valid Values: Numeric Tier (1-17)"),
            Arguments::RGB => String::from("Valid Values: 0-255"),
            Arguments::FontSize => String::from("Valid Values: 18-45 (default: 32)"),
            Arguments::CustomSound => String::from("Valid Values: local path to the file (with quotation marks):  \n - \"None\" (disable)  \n - [reative path]  \n - [absolute path]"),
            Arguments::AlertSound => String::from("Valid Values:  \nDisable: None  \nId: [1-16] Ingame Sounds  \nVolume: [0-300]"),
            Arguments::PlayEffect => String::from("Valid Values:  \nDisable: None  \nColor: Red, Green, Blue, Brown, White, Yellow, Cyan, Grey, Orange, Pink, Purple  \nBeamVisualization [Temp]: Empty (no value, instantly) or Temp (temporary)"),
            Arguments::MinimapIcon => String::from("Valid Values:\n\nDisable: -1  \nSize: 0 (large), 1 (medium), 2 (small)  \nColor: Red, Green, Blue, Brown, White, Yellow, Cyan, Grey, Orange, Pink, Purple  \nShape: Circle, Diamond, Hexagon, Square, Star, Triangle, Cross, Moon, Raindrop, Kite, Pentagon, UpsideDownHouse")
        }
    }
}

impl Token {
    pub fn description(&self) -> String {
        match self {
            Token::Error => String::from("Unrecognized Token"),
            Token::Show => String::from("# Show\n\nIf all conditions are matched, show the item and do any actions specified."),
            Token::Hide => String::from("# Hide\n\nIf all conditions are matched, hide the item and do any actions specified."),
            Token::Continue => String::from("# Continue\n\nContinues block to other match.\n\nThis is a special flag that indicates that the filter rule matching should not stop when an item matches this block.\n\nNote that if an item matches a Hide block that Continues, then later matches a Show block, it will use the most recently matched Show or Hide flag, and thus show. If an item matches and Continues and then never matches any further blocks, it will also show or hide based on the most recently matched block."),
            Token::Hash => String::from("Comment"),
            Token::Skip => String::from("Skipped Token"),
            Token::EndLine => String::from("Endline"),
            Token::AreaLevel => format!("# AreaLevel [Operator] <Value>\n\nFilters for items dropped in a particular [Monster level](https://pathofexile.gamepedia.com/Monster_level) of the current area. This is probably the most relevant of the filters, as it allows enabling/disabling filters dynamically depending on leveling.  \n  \n{}\n\n{}", Arguments::Operator.description(), Arguments::ValidNumeric.description()),
            Token::ItemLevel => format!("# ItemLevel [Operator] <Level>\n\nThe [item level](https://pathofexile.gamepedia.com/Item_level) the item was generated at.\n\n{}\n\n{}", Arguments::Operator.description(), Arguments::ValidNumeric.description()),
            Token::DropLevel => format!("# DropLevel [Operator] <Level>\n\nThe level that the item starts dropping at.\n\n{}\n\n{}", Arguments::Operator.description(), Arguments::ValidNumeric.description()),
            Token::Quality => format!("# Quality [Operator] <Quality> \n\nThe amount of [quality](https://pathofexile.gamepedia.com/Quality) on the item.\n\n{}\n\n{}", Arguments::Operator.description(), Arguments::ValidQuality.description()),
            Token::Rarity => format!("# Rarity [Operator] <Rarity>\n\n[Rarity](https://pathofexile.gamepedia.com/Rarity) of the item.\n\n{}\n\n{}", Arguments::Operator.description(), Arguments::ValidRarity.description()),
            Token::Class => format!("# Class <Class>\n\nThe [item class](https://pathofexile.gamepedia.com/Item_class#Item_classes). Specifying part of a class name is allowed and will match any classes with that text in the name. So for example \"One Hand\" will match both \"One Hand Sword\" and \"One Hand Axe\"\n\n{}", Arguments::ValidClass.description()),
            Token::BaseType => format!("# BaseType <Type>\n\nThe base type of the item. Specifying a part of a base type name is allowed and will match any of the base types with that text in the name.\n\n{}", Arguments::ValidItem.description()),
            Token::Prophecy => format!("# Prophecy <Type>\n\nThe prophecy name. Specifying a part of a prophecy name is allowed and will match any of the prophecies with that text in the name. Prophecies have the Class type \"Stackable Currency\".\n\n{}", Arguments::ValidProphecy.description()),
            Token::LinkedSockets => format!("# LinkedSockets [Operator] <Links>\n\nThe size of the largest group of [linked sockets](https://pathofexile.gamepedia.com/Sockets) that the item has.\n\n{}\n\n{}", Arguments::Operator.description(), Arguments::ValidLinks.description()),
            Token::SocketGroup => format!("# SocketGroup [Operator] <GroupSyntax>\n\nSupports a list of groups that each one represents linked sockets containing a specific set of colors, at least one group must be matched for the condition to pass.\n\nEach group is composed by an optional number and a sequence of letters. The number specifies the longest link which contains the following color sequence described by the letters. Each letter is short-hand for the colour ([R]ed, [G]reen, [B]lue, [W]hite) or Special ones ([D]elve Socket, [A]byss Socket). For example, 5RRG will match any group that contains two red sockets linked with a green socket in a 5-link group. Delve and Abyss cannot be in the same group as any other, as they cannot be linked.\n\nIf a comparison operator is used, it will apply to the numeric portion, so a \">= 5GGG\" will match a 5 or more linked group with 3 green sockets.\n\nSocketGroup with A and D socket has no effect. For example \"SocketGroup RGBA\" or \"SocketGroup DD\". As Abyss and Delve sockets are never linked.\n\n{}\n\n{}", Arguments::Operator.description(), Arguments::ValidLSockets.description()),
            Token::Sockets => format!("# Sockets [Operator] <GroupSyntax>\n\nDoes the exact same thing as SocketGroup but does not require the sockets to be linked. So the same example \">= 5GGG\" will match 5 or more sockets not necessarily linked, with at least 3 green sockets anywhere.\n\nUnlike SocketGroup, this condition does allow for mixing and using Delve and Abyss sockets, for example, a [Resonator](https://pathofexile.gamepedia.com/Delve_Stackable_Socketable_Currency) with 3 sockets would be \"DDD\".\n\n{}\n\n{}", Arguments::Operator.description(), Arguments::ValidSockets.description()),
            Token::Height => format!("# Height [Operator] <Value>\n\nhe number of slots the item takes on the Y-axis (verical axis), i.e. the height of the item.\n\n{}\n\n{}",Arguments::Operator.description(), Arguments::ValidHeight.description()),
            Token::Width => format!("# Width [Operator] <Value>\n\nThe number of slots the item takes on the X-axis (horizontal axis), i.e. the width of the item.\n\n{}\n\n{}",Arguments::Operator.description(),Arguments::ValidWidth.description()),
            Token::HasExplicitMod => format!("# HasExplicitMod <Value>\n\nFilter by mods on an item by name. For example: [HasExplicitMod \"Tyrannical\" ] (Tyrannical=Local Physical Damage 155 to 169%)\n\n{}",Arguments::ValidItem.description()),
            Token::AnyEnchantment => format!("# AnyEnchantment <Boolean>\n\nIf an item has any enchantment from the Labyrinth.\n\n{}", Arguments::Boolean.description()),
            Token::HasEnchantment => format!("# HasEnchantment <Value>\n\nFilter by enchantments\n\n{}", Arguments::Enchant.description()),
            Token::StackSize => format!("# StackSize [Operator] <Value>\n\nCurrency stack size\n\n{}\n\n{}",Arguments::Operator.description(), Arguments::StackSize.description()),
            Token::GemLevel => format!("# GemLevel [Operator] <Value>\n\nGem Level\n\n{}\n\n{}", Arguments::Operator.description(), Arguments::GemLevel.description()),
            Token::Identified => format!("# Identified <Boolean>\n\nIf an item is identified or not.\n\n{}",Arguments::Boolean.description()),
            Token::Corrupted => format!("# Corrupted <Boolean>\n\nIf an item is [corrupted](https://pathofexile.gamepedia.com/Corrupted) or not.\n\n{}", Arguments::Boolean.description()),
            Token::CorruptedMods => format!("# CorruptedMods [Operator] <Value>\n\nHow many corrupted mods are present.\n\n{}\n\n{}", Arguments::Operator.description(),Arguments::CorruptedMods.description()),
            Token::Mirrored => format!("# Mirrored <Boolean>\n\nIf the item is a [Mirrored item](https://pathofexile.gamepedia.com/Mirrored) or not. Does not drop normally, except when opening a [Strongbox](https://pathofexile.gamepedia.com/Strongbox) with the \"Contains Mirrored Items\", or via the Prophecy [Kalandra's Craft](https://pathofexile.gamepedia.com/Kalandra%27s_Craft).\n\n{}", Arguments::Boolean.description()),
            Token::ElderItem => format!("# ElderItem <Boolean>\n\nIf an item is an [Elder item](https://pathofexile.gamepedia.com/Elder_item) or not.\n\n{}", Arguments::Boolean.description()),
            Token::ShaperItem => format!("# ShaperItem <Boolean>\n\nIf an item is a [Shaper item](https://pathofexile.gamepedia.com/Shaper_item) or not.\n\n{}", Arguments::Boolean.description()),
            Token::HasInfluence => format!("# HasInfluence <Type>\n\nIf an item has [Influence](https://pathofexile.gamepedia.com/Influenced_item) of a certain type. Note that this also affects [Maps](https://pathofexile.gamepedia.com/Map) that are influenced.\n\n{}", Arguments::HasInfluence.description()),
            Token::FracturedItem => format!("# FracturedItem <Boolean>\n\nIf an item is [fractured](https://pathofexile.gamepedia.com/Fractured_item) or not\n\n{}", Arguments::Boolean.description()),
            Token::SynthesisedItem => format!("# SynthesisedItem <Boolean>\n\nIf an item is [synthesised](https://pathofexile.gamepedia.com/Synthesised_item) or not\n\n{}", Arguments::Boolean.description()),
            Token::ShapedMap => format!("# ShapedMap <Boolean>\n\nIf the map is [shaped](https://pathofexile.gamepedia.com/Shaped) or not.\n\n{}", Arguments::Boolean.description()),
            Token::MapTier => format!("# MapTier [Operator] <Value>\n\nThe [map tier](https://pathofexile.gamepedia.com/Map#Tiers) of the [map](https://pathofexile.gamepedia.com/Map).\n\n{}\n\n{}", Arguments::Operator.description(), Arguments::MapTier.description()),
            Token::SetBorderColor => format!("# SetBorderColor  \n## <Red> <Green> <Blue> [Alpha]\n\nSets the border colour of the item box in RGB values from 0-255 with optional Alpha (opacity) value of 0-255\n\n{}", Arguments::RGB.description()),
            Token::SetTextColor => format!("# SetTextColor  \n## <Red> <Green> <Blue> [Alpha]\n\nSets the text colour of the item box in RGB values from 0-255 with optional Alpha (opacity) value of 0-255\n\n{}", Arguments::RGB.description()),
            Token::SetBackgroundColor => format!("# SetBackgroundColor  \n## <Red> <Green> <Blue> [Alpha]\n\nSets the colour of the item box in RGB values from 0-255 with optional Alpha (opacity) value of 0-255\n\n{}",Arguments::RGB.description()),
            Token::SetFontSize => format!("# SetFontSize <FontSize>\n\nSets the font-size of item text.\n\n{}",Arguments::FontSize.description()),
            Token::PlayAlertSound => format!("# PlayAlertSound <Id> [Volume]\n\nPlays the specified Alert Sound with optional volume when dropped. Only one sound can be played at a time.\n\n{}", Arguments::AlertSound.description()),
            Token::PlayAlertSoundPositional => format!("# PlayAlertSoundPositional <Id> [Volume]\n\nWork as PlayAlertSound with Sound Volume relative to distance where Item dropped. Could be usable with low Tier Items to smooth Sounds.\n\n{}", Arguments::AlertSound.description()),
            Token::DisableDropSound => format!("# DisableDropSound\n\nDisable the drop sound (undocumented feature)."),
            Token::CustomAlertSound => format!("# CustomAlertSound  \n## <FileName | FileFullPath>\n\nPlays the specified custom sound when a specified item drops. (almost all of the common file extensions should be supported)\n\n{}", Arguments::CustomSound.description()),
            Token::PlayEffect => format!("# PlayEffect <Color> [Temp]\n\nDisplays a coloured beam of light above an item highlighted by an item filter.\n\nUse the Temp parameter to have a beam only appear as the item drops.\n\nOtherwise, it will be permanently visible.\n\n{}", Arguments::PlayEffect.description()),
            Token::MinimapIcon => format!("# MinimapIcon  \n## <Size> <Color> <Shape>\n\nDisplays an icon on the minimap for specified items.\n\n{}", Arguments::MinimapIcon.description()),
            Token::Numbers(_) => format!("# Numeric\n\nLike: 20. No quotation marks.\n\nValid Values: -1,[0-9]"),
            Token::Quotes(_) | Token::Text(_) => format!("# String\n\nText with quotation marks, but not in all conditions, as example is Class or BaseType where values could be strings separated by space only\n\nValid Values: [a-zA-Z]"),
            Token::Boolean(_) => format!("# Boolean\n\nTrue or False"),
        }
    }
    // pub fn small_description(&self) -> String {
    //     match self {
    //         Token::Error => String::from("Unrecognized Token"),
    //         Token::Show => String::from("# Show\n\nIf all conditions are matched, show the item and do any actions specified."),
    //         Token::Hide => String::from("# Hide\n\nIf all conditions are matched, hide the item and do any actions specified."),
    //         Token::Continue => String::from("# Continue\n\nContinues block to other match.\n\nThis is a special flag that indicates that the filter rule matching should not stop when an item matches this block.\n\nNote that if an item matches a Hide block that Continues, then later matches a Show block, it will use the most recently matched Show or Hide flag, and thus show. If an item matches and Continues and then never matches any further blocks, it will also show or hide based on the most recently matched block."),
    //         Token::Hash => String::from("Comment"),
    //         Token::Skip => String::from("Skipped Token"),
    //         Token::EndLine => String::from("Endline"),
    //         Token::AreaLevel => format!("# AreaLevel [Operator] <Value>\n\nFilters for items dropped in a particular [Monster level](https://pathofexile.gamepedia.com/Monster_level) of the current area. This is probably the most relevant of the filters, as it allows enabling/disabling filters dynamically depending on leveling.  \n  \n{}\n\n{}", Arguments::Operator.description(), Arguments::ValidNumeric.description()),
    //         Token::ItemLevel => format!("# ItemLevel [Operator] <Level>\n\nThe [item level](https://pathofexile.gamepedia.com/Item_level) the item was generated at.\n\n{}\n\n{}", Arguments::Operator.description(), Arguments::ValidNumeric.description()),
    //         Token::DropLevel => format!("# DropLevel [Operator] <Level>\n\nThe level that the item starts dropping at.\n\n{}\n\n{}", Arguments::Operator.description(), Arguments::ValidNumeric.description()),
    //         Token::Quality => format!("# Quality [Operator] <Quality> \n\nThe amount of [quality](https://pathofexile.gamepedia.com/Quality) on the item.\n\n{}\n\n{}", Arguments::Operator.description(), Arguments::ValidQuality.description()),
    //         Token::Rarity => format!("# Rarity [Operator] <Rarity>\n\n[Rarity](https://pathofexile.gamepedia.com/Rarity) of the item.\n\n{}\n\n{}", Arguments::Operator.description(), Arguments::ValidRarity.description()),
    //         Token::Class => format!("# Class <Class>\n\nThe [item class](https://pathofexile.gamepedia.com/Item_class#Item_classes). Specifying part of a class name is allowed and will match any classes with that text in the name. So for example \"One Hand\" will match both \"One Hand Sword\" and \"One Hand Axe\"\n\n{}", Arguments::ValidClass.description()),
    //         Token::BaseType => format!("# BaseType <Type>\n\nThe base type of the item. Specifying a part of a base type name is allowed and will match any of the base types with that text in the name.\n\n{}", Arguments::ValidItem.description()),
    //         Token::Prophecy => format!("# Prophecy <Type>\n\nThe prophecy name. Specifying a part of a prophecy name is allowed and will match any of the prophecies with that text in the name. Prophecies have the Class type \"Stackable Currency\".\n\n{}", Arguments::ValidProphecy.description()),
    //         Token::LinkedSockets => format!("# LinkedSockets [Operator] <Links>\n\nThe size of the largest group of [linked sockets](https://pathofexile.gamepedia.com/Sockets) that the item has.\n\n{}\n\n{}", Arguments::Operator.description(), Arguments::ValidLinks.description()),
    //         Token::SocketGroup => format!("# SocketGroup [Operator] <GroupSyntax>\n\nSupports a list of groups that each one represents linked sockets containing a specific set of colors, at least one group must be matched for the condition to pass.\n\nEach group is composed by an optional number and a sequence of letters. The number specifies the longest link which contains the following color sequence described by the letters. Each letter is short-hand for the colour ([R]ed, [G]reen, [B]lue, [W]hite) or Special ones ([D]elve Socket, [A]byss Socket). For example, 5RRG will match any group that contains two red sockets linked with a green socket in a 5-link group. Delve and Abyss cannot be in the same group as any other, as they cannot be linked.\n\nIf a comparison operator is used, it will apply to the numeric portion, so a \">= 5GGG\" will match a 5 or more linked group with 3 green sockets.\n\nSocketGroup with A and D socket has no effect. For example \"SocketGroup RGBA\" or \"SocketGroup DD\". As Abyss and Delve sockets are never linked.\n\n{}\n\n{}", Arguments::Operator.description(), Arguments::ValidLSockets.description()),
    //         Token::Sockets => format!("# Sockets [Operator] <GroupSyntax>\n\nDoes the exact same thing as SocketGroup but does not require the sockets to be linked. So the same example \">= 5GGG\" will match 5 or more sockets not necessarily linked, with at least 3 green sockets anywhere.\n\nUnlike SocketGroup, this condition does allow for mixing and using Delve and Abyss sockets, for example, a [Resonator](https://pathofexile.gamepedia.com/Delve_Stackable_Socketable_Currency) with 3 sockets would be \"DDD\".\n\n{}\n\n{}", Arguments::Operator.description(), Arguments::ValidSockets.description()),
    //         Token::Height => format!("# Height [Operator] <Value>\n\nhe number of slots the item takes on the Y-axis (verical axis), i.e. the height of the item.\n\n{}\n\n{}",Arguments::Operator.description(), Arguments::ValidHeight.description()),
    //         Token::Width => format!("# Width [Operator] <Value>\n\nThe number of slots the item takes on the X-axis (horizontal axis), i.e. the width of the item.\n\n{}\n\n{}",Arguments::Operator.description(),Arguments::ValidWidth.description()),
    //         Token::HasExplicitMod => format!("# HasExplicitMod <Value>\n\nFilter by mods on an item by name. For example: [HasExplicitMod \"Tyrannical\" ] (Tyrannical=Local Physical Damage 155 to 169%)\n\n{}",Arguments::ValidItem.description()),
    //         Token::AnyEnchantment => format!("# AnyEnchantment <Boolean>\n\nIf an item has any enchantment from the Labyrinth.\n\n{}", Arguments::Boolean.description()),
    //         Token::HasEnchantment => format!("# HasEnchantment <Value>\n\nFilter by enchantments\n\n{}", Arguments::Enchant.description()),
    //         Token::StackSize => format!("# StackSize [Operator] <Value>\n\nCurrency stack size\n\n{}\n\n{}",Arguments::Operator.description(), Arguments::StackSize.description()),
    //         Token::GemLevel => format!("# GemLevel [Operator] <Value>\n\nGem Level\n\n{}\n\n{}", Arguments::Operator.description(), Arguments::GemLevel.description()),
    //         Token::Identified => format!("# Identified <Boolean>\n\nIf an item is identified or not.\n\n{}",Arguments::Boolean.description()),
    //         Token::Corrupted => format!("# Corrupted <Boolean>\n\nIf an item is [corrupted](https://pathofexile.gamepedia.com/Corrupted) or not.\n\n{}", Arguments::Boolean.description()),
    //         Token::CorruptedMods => format!("# CorruptedMods [Operator] <Value>\n\nHow many corrupted mods are present.\n\n{}\n\n{}", Arguments::Operator.description(),Arguments::CorruptedMods.description()),
    //         Token::Mirrored => format!("# Mirrored <Boolean>\n\nIf the item is a [Mirrored item](https://pathofexile.gamepedia.com/Mirrored) or not. Does not drop normally, except when opening a [Strongbox](https://pathofexile.gamepedia.com/Strongbox) with the \"Contains Mirrored Items\", or via the Prophecy [Kalandra's Craft](https://pathofexile.gamepedia.com/Kalandra%27s_Craft).\n\n{}", Arguments::Boolean.description()),
    //         Token::ElderItem => format!("# ElderItem <Boolean>\n\nIf an item is an [Elder item](https://pathofexile.gamepedia.com/Elder_item) or not.\n\n{}", Arguments::Boolean.description()),
    //         Token::ShaperItem => format!("# ShaperItem <Boolean>\n\nIf an item is a [Shaper item](https://pathofexile.gamepedia.com/Shaper_item) or not.\n\n{}", Arguments::Boolean.description()),
    //         Token::HasInfluence => format!("# HasInfluence <Type>\n\nIf an item has [Influence](https://pathofexile.gamepedia.com/Influenced_item) of a certain type. Note that this also affects [Maps](https://pathofexile.gamepedia.com/Map) that are influenced.\n\n{}", Arguments::HasInfluence.description()),
    //         Token::FracturedItem => format!("# FracturedItem <Boolean>\n\nIf an item is [fractured](https://pathofexile.gamepedia.com/Fractured_item) or not\n\n{}", Arguments::Boolean.description()),
    //         Token::SynthesisedItem => format!("# SynthesisedItem <Boolean>\n\nIf an item is [synthesised](https://pathofexile.gamepedia.com/Synthesised_item) or not\n\n{}", Arguments::Boolean.description()),
    //         Token::ShapedMap => format!("# ShapedMap <Boolean>\n\nIf the map is [shaped](https://pathofexile.gamepedia.com/Shaped) or not.\n\n{}", Arguments::Boolean.description()),
    //         Token::MapTier => format!("# MapTier [Operator] <Value>\n\nThe [map tier](https://pathofexile.gamepedia.com/Map#Tiers) of the [map](https://pathofexile.gamepedia.com/Map).\n\n{}\n\n{}", Arguments::Operator.description(), Arguments::MapTier.description()),
    //         Token::SetBorderColor => format!("# SetBorderColor  \n## <Red> <Green> <Blue> [Alpha]\n\nSets the border colour of the item box in RGB values from 0-255 with optional Alpha (opacity) value of 0-255\n\n{}", Arguments::RGB.description()),
    //         Token::SetTextColor => format!("# SetTextColor  \n## <Red> <Green> <Blue> [Alpha]\n\nSets the text colour of the item box in RGB values from 0-255 with optional Alpha (opacity) value of 0-255\n\n{}", Arguments::RGB.description()),
    //         Token::SetBackgroundColor => format!("# SetBackgroundColor  \n## <Red> <Green> <Blue> [Alpha]\n\nSets the colour of the item box in RGB values from 0-255 with optional Alpha (opacity) value of 0-255\n\n{}",Arguments::RGB.description()),
    //         Token::SetFontSize => format!("# SetFontSize <FontSize>\n\nSets the font-size of item text.\n\n{}",Arguments::FontSize.description()),
    //         Token::PlayAlertSound => format!("# PlayAlertSound <Id> [Volume]\n\nPlays the specified Alert Sound with optional volume when dropped. Only one sound can be played at a time.\n\n{}", Arguments::AlertSound.description()),
    //         Token::PlayAlertSoundPositional => format!("# PlayAlertSoundPositional <Id> [Volume]\n\nWork as PlayAlertSound with Sound Volume relative to distance where Item dropped. Could be usable with low Tier Items to smooth Sounds.\n\n{}", Arguments::AlertSound.description()),
    //         Token::DisableDropSound => format!("# DisableDropSound\n\nDisable the drop sound (undocumented feature)."),
    //         Token::CustomAlertSound => format!("# CustomAlertSound  \n## <FileName | FileFullPath>\n\nPlays the specified custom sound when a specified item drops. (almost all of the common file extensions should be supported)\n\n{}", Arguments::CustomSound.description()),
    //         Token::PlayEffect => format!("# PlayEffect <Color> [Temp]\n\nDisplays a coloured beam of light above an item highlighted by an item filter.\n\nUse the Temp parameter to have a beam only appear as the item drops.\n\nOtherwise, it will be permanently visible.\n\n{}", Arguments::PlayEffect.description()),
    //         Token::MinimapIcon => format!("# MinimapIcon  \n## <Size> <Color> <Shape>\n\nDisplays an icon on the minimap for specified items.\n\n{}", Arguments::MinimapIcon.description()),
    //         Token::Numbers(_) => format!("# Numeric\n\nLike: 20. No quotation marks.\n\nValid Values: -1,[0-9]"),
    //         Token::Quotes(_) | Token::Text(_) => format!("# String\n\nText with quotation marks, but not in all conditions, as example is Class or BaseType where values could be strings separated by space only\n\nValid Values: [a-zA-Z]"),
    //         Token::Boolean(_) => format!("# Boolean\n\nTrue or False"),
    //     }
    // }
}
