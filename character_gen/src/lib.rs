use utils::RandomNumberGenerator;
use std::{collections::HashSet, fmt};

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum CharRace {
    HUMAN,
    FEYBLOOD,
    DUSKLING,
    PREEN,
    HALFLING,
    FERAL
}

impl CharRace {
    fn as_string(&self) -> &str {
        return match self {
            CharRace::HUMAN => "Human",
            CharRace::FEYBLOOD => "Fey-Blood",
            CharRace::DUSKLING => "Duskling",
            CharRace::PREEN => "Preen",
            CharRace::HALFLING => "Halfling",
            CharRace::FERAL => "Feral"
        }
    }

    fn get_background(&self) -> Background {
        return match self {
            /// humans can be townsfolk, noble, frontier, zealot
            CharRace::HUMAN => Background::TOWNSFOLK,
            CharRace::FEYBLOOD => Background::OUTSIDER,
            CharRace::DUSKLING => Background::OUTSIDER,
            CharRace::PREEN => Background::TOWNSFOLK,
            CharRace::HALFLING => Background::FRONTIER,
            CharRace::FERAL => Background::OUTSIDER
        }
    }

    fn get_skill_expertise(&self) -> SkillType {
        return match self {
            CharRace::HUMAN => SkillType::SPEECH,
            CharRace::FEYBLOOD => SkillType::WITS,
            CharRace::DUSKLING => SkillType::TRAVELING,
            CharRace::PREEN => SkillType::CRAFTING,
            CharRace::HALFLING => SkillType::WILDERNESS,
            CharRace::FERAL => SkillType::SCOUTING,
        }
    }
}

impl fmt::Display for CharRace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum Background {
    TOWNSFOLK,
    NOBLE,
    FRONTIER,
    ZEALOT,
    OUTSIDER,
    MYSTIC
}

impl Background {
    fn as_string(&self) -> &str {
        return match self {
            Background::FRONTIER => "Frontier",
            Background::TOWNSFOLK => "Townsfolk",
            Background::NOBLE => "Noble",
            Background::ZEALOT => "Zealot",
            Background::OUTSIDER => "Outsider",
            Background::MYSTIC => "Mystic",
        }
    }
}

impl fmt::Display for Background {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum SkillType {
    BATTLEWISE,
    CRAFTING,
    DEVOTION,
    EXPERTISE,
    LEADERSHIP,
    PATHWISE,
    SCHOLAR,
    SCOUTING,
    SPEECH,
    TRAVELING,
    WILDERNESS,
    WITS
}

impl SkillType {
    fn as_string(&self) -> &str {
        return match self {
            SkillType::BATTLEWISE => "Battlewise",
            SkillType::CRAFTING => "Crafting",
            SkillType::DEVOTION => "Devotion",
            SkillType::EXPERTISE => "Expertise",
            SkillType::LEADERSHIP => "Leadership",
            SkillType::PATHWISE => "Pathwise",
            SkillType::SCHOLAR => "Scholar",
            SkillType::SCOUTING => "Scouting",
            SkillType::SPEECH => "Speech",
            SkillType::TRAVELING => "Traveling",
            SkillType::WILDERNESS => "Wilderness",
            SkillType::WITS => "Wits",
        }
    }
}

impl fmt::Display for SkillType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

#[derive(PartialEq, Clone)]
pub struct Character {
    pub name: String,
    pub race: CharRace,
    pub background: Background,
    pub skills: HashSet<SkillType>,
    pub skill_expertise: SkillType,
    pub agility: i32,
    pub speed: (i32, i32),
    pub combat_skill: i32,
    pub toughness: i32,
    pub casting: i32,
    pub luck: i32,
    pub will: i32,
    pub xp: i32,
    pub skill_choices: i32
}

impl Character {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let new_race = generate_random_race(rng);
        let new_background = new_race.get_background();
        let skills = HashSet::new();
        let mut new_char = Character {
            name: "".to_string(),
            race: new_race,
            background: new_background,
            skills,
            skill_expertise: new_race.get_skill_expertise(),
            agility: 1,
            speed: (4, 3),
            combat_skill: 0,
            toughness: 3,
            casting: 0,
            xp: 0,
            luck: 0,
            will: 0,
            skill_choices: 0
        };

        if new_race == CharRace::PREEN {
            new_char.speed.1 += 1;
        }
        new_char
    }
}

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mystic = if self.background == Background::MYSTIC {
            &(",\nCasting: ".to_owned() + &self.casting.to_string())
        } else {
            ""
        };

        let attributes_text = format!(
            "\nATTRIBUTES\nAgility: {},\nSpeed: {}\" / +{}\"\nCombat Skill: {},\nToughness: {}{}",
            self.agility,
            self.speed.0, self.speed.1,
            self.combat_skill,
            self.toughness,
            mystic,
        );

        let points_text = format!(       
            "\nPOINTS\nExperience: {},\nLuck: {},\nWill: {}",
            self.xp,
            self.luck,
            self.will
        );
        write!(f, 
            "\nRace: {}\nBackground: {}{}\nExpertise: {}{}",
            self.race,
            self.background,
            attributes_text,
            self.skill_expertise,
            points_text
        )
    }
}

pub fn generate_random_race(rng: &mut RandomNumberGenerator) -> CharRace {
    let roll = rng.range(1..=100);
    return if roll <= 40 {
        CharRace::HUMAN
    } else if roll <= 50 {
        CharRace::FEYBLOOD
    } else if roll <= 60 {
        CharRace::DUSKLING
    } else if roll <= 75 {
        CharRace::PREEN
    } else if roll <= 90 {
        CharRace::HALFLING
    } else {
        CharRace::FERAL
    }
}

pub fn apply_background(character: &mut Character, rng: &mut RandomNumberGenerator) {
    match character.background {
        Background::FRONTIER => {
            let capabilities = rng.d20();
            match capabilities {
                1..=4 => {
                    character.agility += 1;
                },
                5..=8 => {
                    character.combat_skill += 1;
                },
                9..=12 => {
                    character.speed.0 += 2;
                },
                13..=16 => {
                    character.toughness += 1;
                },
                17 | 18 => {
                    character.agility += 1;
                    character.combat_skill += 1;    
                },
                _ => {
                    character.combat_skill += 1;
                    character.toughness += 1;    
                }
            }

            let mentality = rng.d20();
            match mentality {
                1 => {
                    if character.race == CharRace::HUMAN {
                        character.will += 2;
                    } else {
                        character.will += 1;
                        character.xp += 3;
                    }
                },
                2 | 3 => {
                    character.will += 1;
                },
                4..=16 => {
                    character.xp += 1;
                },
                17 | 18 => {
                    character.luck += 1;
                },
                _ => {
                    character.luck += 1;
                    character.will += 1;
                }
            }
        },
        Background::TOWNSFOLK => todo!(),
        Background::NOBLE => todo!(),
        Background::ZEALOT => todo!(),
        Background::OUTSIDER => todo!(),
        Background::MYSTIC => todo!(),
    }
}