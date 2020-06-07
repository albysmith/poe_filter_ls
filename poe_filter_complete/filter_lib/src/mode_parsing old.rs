use logos::Lexer;
use logos::Logos;
// use logos_derive::Logos;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Logos)]
pub enum Block {
    #[error]
    Error,
    #[token("Show")]
    Show,
    #[token("Hide")]
    Hide,
    #[token("Continue")]
    Continue,
    #[token("#")]
    Hash,
    #[regex(" | |", logos::skip)]
    Skip,
}
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Logos)]
pub enum Outer {
    #[error]
    Error,
    #[token("#", ignore_outer_comments)]
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
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Logos)]
pub enum Inner {
    #[error]
    Error,
    #[regex("[0-9]+")]
    Numbers,
    #[regex("\"([^\"]*)\"")]
    Quotes,
    #[token("\n")]
    EndLine,
    #[regex("true|false|True|False")]
    Boolean,
    #[regex("[a-zA-Z]+")]
    Text,
    #[regex(" ", logos::skip)]
    Skip,
    #[token("#", ignore_inner_comments)]
    Hash,
}

pub enum Modes<'source> {
    Block(Lexer<'source, Block>),
    Outer(Lexer<'source, Outer>),
    Inner(Lexer<'source, Inner>),
}

impl<'source> Modes<'source> {
    pub fn new(s: &'source str) -> Self {
        Self::Block(Block::lexer(s))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Tokens {
    InnerToken(Inner),
    OuterToken(Outer),
    BlockToken(Block),
}

pub struct ModeBridge<'source> {
    pub mode: Modes<'source>,
}

// Clones as we switch between modes
impl<'source> Iterator for ModeBridge<'source> {
    type Item = Tokens;
    fn next(&mut self) -> Option<Self::Item> {
        use Tokens::*;
        match &mut self.mode {
            Modes::Inner(inner) => {
                let result = inner.next();
                if Some(Inner::EndLine) == result {
                    self.mode = Modes::Outer(inner.to_owned().morph());
                }
                result.map(InnerToken)
            }
            Modes::Outer(outer) => {
                let result = outer.next();
                if let Some(x) = result {
                    match x {
                        Outer::Error => {}
                        Outer::Skip => {}
                        Outer::Hash => {}
                        _ => {
                            self.mode = Modes::Inner(outer.to_owned().morph());
                        }
                    }
                }
                result.map(OuterToken)
            }
            Modes::Block(block) => {
                let result = block.next();
                if Some(Block::Show) == result
                    || Some(Block::Hide) == result
                    || Some(Block::Continue) == result
                {
                    self.mode = Modes::Outer(block.to_owned().morph());
                }
                result.map(BlockToken)
            }
        }
    }
}

pub fn ignore_outer_comments(lex: &mut Lexer<Outer>) {
    if lex.slice() == "#" {
        loop {
            // let result = lex.next();
            match lex.next() {
                Some(Outer::EndLine) => break,
                _ => lex.bump(lex.slice().len()),
            }
        }
    }
}
pub fn ignore_inner_comments(lex: &mut Lexer<Inner>) {
    if lex.slice() == "#" {
        loop {
            // let result = lex.next();
            match lex.next() {
                Some(Inner::EndLine) => break,
                _ => {}
            }
        }
    }
}
