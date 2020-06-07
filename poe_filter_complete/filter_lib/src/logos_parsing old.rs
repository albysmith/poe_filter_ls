use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[error]
    Error,
    #[token("Show")]
    Show,
    #[token("Hide")]
    Hide,
    #[token("Continue")]
    Continue,
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
    // Numbers
    #[regex("[0-9]+")]
    Numbers,
    #[regex("\"([^\"]*)\"")]
    Quotes,
}

#[derive(PartialEq, Debug, Clone)]
pub struct FilterBlock {
    pub block: Option<TokenAndSpan>,
    pub hasexplicitmod: Option<TokenAndSpan>,
}
#[derive(PartialEq, Debug, Clone)]

pub struct TokenAndSpan {
    pub token: Token,
    pub span: std::ops::Range<usize>,
    pub value: Option<Token>,
}
pub const TESTFILTER: &str = include_str!("test_filters/filter.filter");

pub fn parse() -> Vec<FilterBlock> {
    let filter_file = include_str!("test_filters/filter.filter");
    let mut vec: Vec<FilterBlock> = vec![];
    let mut block = FilterBlock {
        block: None,
        hasexplicitmod: None,
    };
    let mut lex = Token::lexer(filter_file).spanned();

    let thing = lex
        .map(|x| match x.0 {
            Token::Error => {}
            Token::Show => {
                vec.push(block.clone());
                block.block = Some(TokenAndSpan {
                    token: x.0,
                    span: x.1,
                    value: None,
                });
            }
            Token::Hide => {
                vec.push(block.clone());
                block.block = Some(TokenAndSpan {
                    token: x.0,
                    span: x.1,
                    value: None,
                });
            }
            Token::Continue => {
                vec.push(block.clone());
                block.block = Some(TokenAndSpan {
                    token: x.0,
                    span: x.1,
                    value: None,
                });
            }
            Token::AreaLevel => {}
            Token::ItemLevel => {}
            Token::DropLevel => {}
            Token::Quality => {}
            Token::Rarity => {}
            Token::Class => {}
            Token::BaseType => {}
            Token::Prophecy => {}
            Token::LinkedSockets => {}
            Token::SocketGroup => {}
            Token::Sockets => {}
            Token::Height => {}
            Token::Width => {}
            Token::HasExplicitMod => {
                block.hasexplicitmod = Some(TokenAndSpan {
                    token: x.0,
                    span: x.1,
                    value: None,
                })
            }
            Token::AnyEnchantment => {}
            Token::HasEnchantment => {}
            Token::StackSize => {}
            Token::GemLevel => {}
            Token::Identified => {}
            Token::Corrupted => {}
            Token::CorruptedMods => {}
            Token::Mirrored => {}
            Token::ElderItem => {}
            Token::ShaperItem => {}
            Token::HasInfluence => {}
            Token::FracturedItem => {}
            Token::SynthesisedItem => {}
            Token::ShapedMap => {}
            Token::MapTier => {}
            Token::SetBorderColor => {}
            Token::SetTextColor => {}
            Token::SetBackgroundColor => {}
            Token::SetFontSize => {}
            Token::PlayAlertSound => {}
            Token::PlayAlertSoundPositional => {}
            Token::DisableDropSound => {}
            Token::CustomAlertSound => {}
            Token::MinimapIcon => {}
            Token::PlayEffect => {}
            Token::Numbers => {}
            Token::Quotes => {}
        })
        .collect::<Vec<_>>();
    vec
}
