use elden_ring_roulette::data::ARMAMENTS;

fn main() {
    for (k, v) in &*ARMAMENTS {
        println!("{k}: {v:#?}");
    }
}
