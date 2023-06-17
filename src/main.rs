use std::collections::HashMap;

use elden_ring_roulette::codegen::armaments::ArmamentsSchema;

fn main() {
    let armaments: HashMap<String, ArmamentsSchema> =
        serde_json::from_str(include_str!("../data/objects/armaments.json")).unwrap();

    for (k, v) in armaments {
        println!("{k}: {v:#?}");
    }
}
