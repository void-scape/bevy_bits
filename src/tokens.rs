use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq)]
pub enum TextCommand {
    Clear,
    Speed(f32),
    Pause(f32),
}

#[derive(Debug, Clone)]
pub struct TextSection {
    pub text: Cow<'static, str>,
    pub color: Option<TextColor>,
    pub effects: Cow<'static, [TextEffect]>,
}

impl TextSection {
    pub fn bevy_section(
        self,
        font: bevy::asset::Handle<bevy::text::Font>,
        font_size: f32,
        default_color: bevy::color::Color,
    ) -> bevy::text::TextSection {
        bevy::text::TextSection {
            value: self.text.into(),
            style: bevy::text::TextStyle {
                font_size,
                font,
                color: self.color.map(|c| c.bevy_color()).unwrap_or(default_color),
            },
        }
    }
}

impl From<&'static str> for TextSection {
    fn from(value: &'static str) -> Self {
        TextSection {
            text: Cow::Borrowed(value),
            color: None,
            effects: Vec::new().into(),
        }
    }
}

impl From<String> for TextSection {
    fn from(value: String) -> Self {
        TextSection {
            text: Cow::Owned(value),
            color: None,
            effects: Vec::new().into(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum RawText {
    Str(&'static str),
    String(String),
}

#[derive(Debug, Clone)]
pub enum Effects {
    NonConst(Vec<TextEffect>),
    Const(&'static [TextEffect]),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextEffect {
    Wave,
}

#[derive(Debug, Clone)]
pub enum TextColor {
    Red,
    Green,
    Blue,
}

impl TextColor {
    pub fn bevy_color(&self) -> bevy::color::Color {
        match self {
            Self::Red => bevy::color::Color::linear_rgb(1.0, 0.0, 0.0),
            Self::Green => bevy::color::Color::linear_rgb(0.0, 1.0, 0.0),
            Self::Blue => bevy::color::Color::linear_rgb(0.0, 0.0, 1.0),
        }
    }
}

#[derive(Debug, Clone)]
pub enum DialogueBoxToken {
    Section(TextSection),
    Command(TextCommand),
}

impl DialogueBoxToken {
    pub fn parse_command(args: Option<&str>, cmd: &str) -> Self {
        if let Some(args) = args {
            match cmd {
                "red" => Self::Section(TextSection {
                    text: args.to_owned().into(),
                    color: Some(TextColor::Red),
                    effects: Cow::Owned(Vec::new()),
                }),
                "wave" => Self::Section(TextSection {
                    text: args.to_owned().into(),
                    color: None,
                    effects: Cow::Owned(vec![TextEffect::Wave]),
                }),
                "pause" => Self::Command(TextCommand::Pause(
                    args.parse::<f32>()
                        .unwrap_or_else(|e| panic!("invalid args `{args}` for cmd `{cmd}`: {e}")),
                )),
                "speed" => Self::Command(TextCommand::Speed(
                    args.parse::<f32>()
                        .unwrap_or_else(|e| panic!("invalid args `{args}` for cmd `{cmd}`: {e}")),
                )),
                c => panic!("command `{c}` is unimplemented"),
            }
        } else {
            match cmd {
                "clear" => Self::Command(TextCommand::Clear),
                c => panic!("command `{c}` is unimplemented or requires input args"),
            }
        }
    }
}

pub trait IntoDialogueBoxToken {
    fn into_token(self) -> DialogueBoxToken;

    // fn effect(self, effect: TextEffect) -> Effect<Self>
    // where
    //     Self: Sized,
    // {
    //     Effect {
    //         token: self,
    //         effect,
    //     }
    // }
    //
    // fn color(self, color: TextColor) -> Color<Self>
    // where
    //     Self: Sized,
    // {
    //     Color { token: self, color }
    // }
}

// pub struct Effect<T> {
//     token: T,
//     effect: TextEffect,
// }
//
// impl<T: IntoDialogueBoxToken> IntoDialogueBoxToken for Effect<T> {
//     fn into_token(self) -> DialogueBoxToken {
//         let mut token = self.token.into_token();
//
//         if let DialogueBoxToken::Section(section) = &mut token {
//             section.effects.push(self.effect);
//         }
//
//         token
//     }
// }
//
// pub struct Color<T> {
//     token: T,
//     color: TextColor,
// }
//
// impl<T: IntoDialogueBoxToken> IntoDialogueBoxToken for Color<T> {
//     fn into_token(self) -> DialogueBoxToken {
//         let mut token = self.token.into_token();
//
//         if let DialogueBoxToken::Section(section) = &mut token {
//             section.color = Some(self.color);
//         }
//
//         token
//     }
// }

impl IntoDialogueBoxToken for &'static str {
    fn into_token(self) -> DialogueBoxToken {
        DialogueBoxToken::Section(TextSection::from(self))
    }
}

impl IntoDialogueBoxToken for String {
    fn into_token(self) -> DialogueBoxToken {
        self.into()
    }
}

impl IntoDialogueBoxToken for DialogueBoxToken {
    fn into_token(self) -> DialogueBoxToken {
        self
    }
}

impl From<String> for DialogueBoxToken {
    fn from(value: String) -> Self {
        DialogueBoxToken::Section(value.into())
    }
}

impl From<&'static str> for DialogueBoxToken {
    fn from(value: &'static str) -> Self {
        DialogueBoxToken::Section(value.into())
    }
}
