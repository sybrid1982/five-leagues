use character_gen::Character;
use dioxus::prelude::*;
use character_sheet::*;
use utils::RandomNumberGenerator;

mod character_sheet;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut rng = RandomNumberGenerator::new();
    let mut character = Character::new(&mut rng);
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        header {}
        div {
            class: "character-sheet",
            CharacterSheet { 
                character
            }
        }
        footer {}
    }
}