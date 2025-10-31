use std::{fmt::Display, str::FromStr};

use bevy::asset::uuid::Uuid;

#[derive(Clone, Debug)]
pub enum Identifier {
    Uuid(Uuid),
    Id(String),
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = match self {
            Self::Uuid(uuid) => uuid.to_string(),
            Self::Id(id) => id.to_string(),
        };

        write!(f, "{x}")
    }
}

impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl PartialEq<Uuid> for Identifier {
    fn eq(&self, other: &Uuid) -> bool {
        match self {
            Self::Uuid(uuid) => uuid == other,
            Self::Id(id) => id.to_string() == other.to_string(),
        }
    }
}

impl PartialEq<str> for Identifier {
    fn eq(&self, other: &str) -> bool {
        match self {
            Self::Uuid(uuid) => uuid.to_string() == other,
            Self::Id(id) => id == other,
        }
    }
}

impl Into<Uuid> for Identifier {
    fn into(self) -> Uuid {
        match self {
            Self::Uuid(uuid) => uuid,
            Self::Id(id) => Uuid::from_str(&id).unwrap(),
        }
    }
}

impl Into<String> for Identifier {
    fn into(self) -> String {
        match self {
            Self::Uuid(uuid) => uuid.to_string(),
            Self::Id(id) => id,
        }
    }
}
