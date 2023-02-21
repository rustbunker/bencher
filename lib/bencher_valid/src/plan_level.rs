#![cfg(feature = "plus")]

use derive_more::Display;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
use std::{fmt, str::FromStr};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize,
};

use crate::ValidError;

const FREE: &str = "free";
const TEAM: &str = "team";
const BENCHER_TEAM: &str = "Bencher Team";
const ENTERPRISE: &str = "enterprise";
const BENCHER_ENTERPRISE: &str = "Bencher Enterprise";

#[derive(Debug, Display, Clone, Copy, Default, Eq, PartialEq, Hash, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum PlanLevel {
    #[default]
    Free,
    Team,
    Enterprise,
}

impl FromStr for PlanLevel {
    type Err = ValidError;

    fn from_str(plan_level: &str) -> Result<Self, Self::Err> {
        if is_valid_plan_level(plan_level) {
            return Ok(match plan_level {
                FREE => Self::Free,
                TEAM | BENCHER_TEAM => Self::Team,
                ENTERPRISE | BENCHER_ENTERPRISE => Self::Enterprise,
                _ => panic!("Invalid plan_level"),
            });
        }

        Err(ValidError::PlanLevel(plan_level.into()))
    }
}

impl AsRef<str> for PlanLevel {
    fn as_ref(&self) -> &str {
        match self {
            Self::Free => FREE,
            Self::Team => TEAM,
            Self::Enterprise => ENTERPRISE,
        }
    }
}

impl From<PlanLevel> for String {
    fn from(plan_level: PlanLevel) -> Self {
        plan_level.as_ref().to_string()
    }
}

impl<'de> Deserialize<'de> for PlanLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(PlanLevelVisitor)
    }
}

struct PlanLevelVisitor;

impl<'de> Visitor<'de> for PlanLevelVisitor {
    type Value = PlanLevel;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid plan level")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        value.parse().map_err(E::custom)
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn is_valid_plan_level(plan_level: &str) -> bool {
    matches!(
        plan_level,
        FREE | TEAM | BENCHER_TEAM | ENTERPRISE | BENCHER_ENTERPRISE
    )
}

#[cfg(test)]
mod test {
    use super::is_valid_plan_level;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_plan_level() {
        assert_eq!(true, is_valid_plan_level("free"));
        assert_eq!(true, is_valid_plan_level("team"));
        assert_eq!(true, is_valid_plan_level("Bencher Team"));
        assert_eq!(true, is_valid_plan_level("enterprise"));
        assert_eq!(true, is_valid_plan_level("Bencher Enterprise"));

        assert_eq!(false, is_valid_plan_level(""));
        assert_eq!(false, is_valid_plan_level("one"));
        assert_eq!(false, is_valid_plan_level("two"));
        assert_eq!(false, is_valid_plan_level(" free"));
        assert_eq!(false, is_valid_plan_level("free "));
        assert_eq!(false, is_valid_plan_level(" free "));
    }
}
