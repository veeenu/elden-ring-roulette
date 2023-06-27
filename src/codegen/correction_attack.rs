// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::correction-attack.schema;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: correction-attack.schema =
// serde_json::from_str(&json).unwrap(); }

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrectionAttackSchema {
    correction: Correction,
    #[serde(rename = "override")]
    correction_attack_schema_override: Override,
    ratio: Ratio,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Correction {
    fire: AttributesCorrection,
    holy: AttributesCorrection,
    lightning: AttributesCorrection,
    magic: AttributesCorrection,
    physical: AttributesCorrection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributesCorrection {
    arcane: bool,
    dexterity: bool,
    faith: bool,
    intelligence: bool,
    strength: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Override {
    fire: Option<AttributesOverride>,
    holy: Option<AttributesOverride>,
    lightning: Option<AttributesOverride>,
    magic: Option<AttributesOverride>,
    physical: Option<AttributesOverride>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributesOverride {
    arcane: Option<f64>,
    dexterity: Option<f64>,
    faith: Option<f64>,
    intelligence: Option<f64>,
    strength: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ratio {
    fire: AttributesRatio,
    holy: AttributesRatio,
    lightning: AttributesRatio,
    magic: AttributesRatio,
    physical: AttributesRatio,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributesRatio {
    arcane: f64,
    dexterity: f64,
    faith: f64,
    intelligence: f64,
    strength: f64,
}
