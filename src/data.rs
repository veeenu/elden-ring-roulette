use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::codegen::armaments::*;

lazy_static! {
    pub static ref ARMAMENTS: HashMap<String, ArmamentsSchema> =
        serde_json::from_str(include_str!("../data/objects/armaments.json")).unwrap();
}
