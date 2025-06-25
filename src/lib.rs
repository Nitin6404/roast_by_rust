use wasm_bindgen::prelude::*;
use rand::Rng;

#[wasm_bindgen]
pub fn get_roast(target: &str) -> String {
    let insults = [
        "{} has the energy of a sleepy sloth.",
        "Hey {}, your procrastination game is elite.",
        "{} is proof that tasks don't do themselves.",
        "{} types like they're chiseling stone.",
        "{}'s to-do list is gathering more dust than a museum.",
    ];

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..insults.len());

    insults[index].replace("{}", target)
}
