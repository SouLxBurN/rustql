use std::fmt::Display;

use juniper::GraphQLEnum;

#[derive(GraphQLEnum)]
pub enum Language {
    EN,
    ES,
    UNKNOWN,
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EN => write!(f, "EN"),
            Self::ES => write!(f, "ES"),
            Self::UNKNOWN => write!(f, "UNKNOWN"),
        }
    }
}

impl From<String> for Language {
    fn from(s: String) -> Self {
        match s.as_ref() {
            "EN" => Language::EN,
            "ES" => Language::ES,
            _ => Language::UNKNOWN
        }
    }
}

