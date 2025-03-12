use character_gen::{Background, Character};
use dioxus::prelude::*;

#[component]
pub fn CharacterSheet(character: Character) -> Element {
    let mut is_editing_name = use_signal(|| false);
    let mut name = use_signal(|| character.name);
    rsx! {
        div {
            span { 
                span {
                    "Name: " 
                }
                if is_editing_name() {
                    input { 
                        value: "{name}",
                        oninput: move |event| name.set(event.value())
                    }
                } else {
                    span { "{name}" }
                }
                button {
                    onclick: move |_| { is_editing_name.set(!is_editing_name()) },
                    "Edit"
                }
            }
        }
        div {
            class: "flex-between",
            span { "Race: {character.race}" }
            span { "Background: {character.background}" }
        }
        div {
            title { "Attributes" }
            div {
                class: "flex-between",
                span { "Agility: {character.agility}" }
                span { "Speed: {character.speed.0}\" / +{character.speed.1}\"" }
                span { "Combat Skill: {character.combat_skill}" }
                span { "Toughness: {character.toughness}" }
                if character.background == Background::MYSTIC {
                    span { "Casting: {character.casting}"}
                }
            }
        }
        div {
            class: "flex-between",
            title { "Points" }
            span { "Experience Points: {character.xp}" }
            span { "Will: {character.will}" }
            span { "Luck: {character.luck}" }
        }
    }
}