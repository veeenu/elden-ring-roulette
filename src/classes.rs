use std::ops::Deref;

#[derive(Default, Clone)]
pub struct Character {
    pub level: u8,
    pub vigor: u8,
    pub mind: u8,
    pub endurance: u8,
    pub strength: u8,
    pub dexterity: u8,
    pub intelligence: u8,
    pub faith: u8,
    pub arcane: u8,
}

pub struct Class(Character);

impl Deref for Class {
    type Target = Character;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&Class> for Character {
    fn from(value: &Class) -> Self {
        value.0.clone()
    }
}

pub enum Classes {
    Hero,
    Bandit,
    Astrologer,
    Warrior,
    Prisoner,
    Confessor,
    Wretch,
    Vagabond,
    Prophet,
    Samurai,
}

impl Deref for Classes {
    type Target = Class;

    fn deref(&self) -> &Self::Target {
        match self {
            Classes::Hero => &CLASS_HERO,
            Classes::Bandit => &CLASS_BANDIT,
            Classes::Astrologer => &CLASS_ASTROLOGER,
            Classes::Warrior => &CLASS_WARRIOR,
            Classes::Prisoner => &CLASS_PRISONER,
            Classes::Confessor => &CLASS_CONFESSOR,
            Classes::Wretch => &CLASS_WRETCH,
            Classes::Vagabond => &CLASS_VAGABOND,
            Classes::Prophet => &CLASS_PROPHET,
            Classes::Samurai => &CLASS_SAMURAI,
        }
    }
}

const CLASS_HERO: Class = Class(Character {
    level: 7,
    vigor: 14,
    mind: 9,
    endurance: 12,
    strength: 16,
    dexterity: 9,
    intelligence: 7,
    faith: 8,
    arcane: 11,
});

const CLASS_BANDIT: Class = Class(Character {
    level: 5,
    vigor: 10,
    mind: 11,
    endurance: 10,
    strength: 9,
    dexterity: 13,
    intelligence: 9,
    faith: 8,
    arcane: 14,
});

const CLASS_ASTROLOGER: Class = Class(Character {
    level: 6,
    vigor: 9,
    mind: 15,
    endurance: 9,
    strength: 8,
    dexterity: 12,
    intelligence: 16,
    faith: 7,
    arcane: 9,
});

const CLASS_WARRIOR: Class = Class(Character {
    level: 8,
    vigor: 11,
    mind: 12,
    endurance: 11,
    strength: 10,
    dexterity: 16,
    intelligence: 10,
    faith: 8,
    arcane: 9,
});

const CLASS_PRISONER: Class = Class(Character {
    level: 9,
    vigor: 11,
    mind: 12,
    endurance: 11,
    strength: 11,
    dexterity: 14,
    intelligence: 14,
    faith: 6,
    arcane: 9,
});

const CLASS_CONFESSOR: Class = Class(Character {
    level: 10,
    vigor: 10,
    mind: 13,
    endurance: 10,
    strength: 12,
    dexterity: 12,
    intelligence: 9,
    faith: 14,
    arcane: 9,
});

const CLASS_WRETCH: Class = Class(Character {
    level: 1,
    vigor: 10,
    mind: 10,
    endurance: 10,
    strength: 10,
    dexterity: 10,
    intelligence: 10,
    faith: 10,
    arcane: 10,
});

const CLASS_VAGABOND: Class = Class(Character {
    level: 9,
    vigor: 15,
    mind: 10,
    endurance: 11,
    strength: 14,
    dexterity: 13,
    intelligence: 9,
    faith: 9,
    arcane: 7,
});

const CLASS_PROPHET: Class = Class(Character {
    level: 7,
    vigor: 10,
    mind: 14,
    endurance: 8,
    strength: 11,
    dexterity: 10,
    intelligence: 7,
    faith: 16,
    arcane: 10,
});

const CLASS_SAMURAI: Class = Class(Character {
    level: 9,
    vigor: 12,
    mind: 11,
    endurance: 13,
    strength: 12,
    dexterity: 15,
    intelligence: 9,
    faith: 8,
    arcane: 8,
});
