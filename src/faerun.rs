use std::fmt::{self, Display};

/// Trait to handle cycling through enum options in the UI (Left/Right arrows)
pub trait Cycle {
    fn next(&mut self);
    fn prev(&mut self);
}

/*
 * ====================================
 * RACE & SUBRACES
 * ====================================
 */

#[derive(Debug, Clone, Copy)]
pub enum Race {
    Human(HumanSubrace),
    Elf(ElfSubrace),
    Drow(DrowSubrace),
    HalfElf, // No subrace
    Dwarf(DwarfSubrace),
    Halfling(HalflingSubrace),
    Gnome(GnomeSubrace),
    HalfOrc, // No subrace
    Tiefling(TieflingSubrace),
    Gith(GithSubrace),
    DragonBorn(DragonbornSubrace),
}

impl Race {
    pub const ALL_RACES: [&str; 11] = [
        "Human",
        "Elf",
        "Drow",
        "Half-Elf",
        "Dwarf",
        "Halfling",
        "Gnome",
        "Half-Orc",
        "Tiefling",
        "Gith",
        "Dragonborn",
    ];

    pub fn default_list() -> [Self; 11] {
        [
            Self::Human(HumanSubrace::Standard),
            Self::Elf(ElfSubrace::High),
            Self::Drow(DrowSubrace::LolthSworn),
            Self::HalfElf,
            Self::Dwarf(DwarfSubrace::Mountain),
            Self::Halfling(HalflingSubrace::Lightfoot),
            Self::Gnome(GnomeSubrace::Forest),
            Self::HalfOrc,
            Self::Tiefling(TieflingSubrace::Asmodeus),
            Self::Gith(GithSubrace::Githyanki),
            Self::DragonBorn(DragonbornSubrace::Gold),
        ]
    }

    pub fn has_subrace(&self) -> bool {
        !matches!(self, Self::HalfOrc | Self::HalfElf)
    }

    /// Helper to cycle the *inner* subrace if it exists
    pub fn cycle_sub_next(&mut self) {
        match self {
            Self::Human(s) => s.next(),
            Self::Elf(s) => s.next(),
            Self::Drow(s) => s.next(),
            Self::Dwarf(s) => s.next(),
            Self::Halfling(s) => s.next(),
            Self::Gnome(s) => s.next(),
            Self::Tiefling(s) => s.next(),
            Self::Gith(s) => s.next(),
            Self::DragonBorn(s) => s.next(),
            _ => {} // Races without subraces do nothing
        }
    }

    pub fn cycle_sub_prev(&mut self) {
        match self {
            Self::Human(s) => s.prev(),
            Self::Elf(s) => s.prev(),
            Self::Drow(s) => s.prev(),
            Self::Dwarf(s) => s.prev(),
            Self::Halfling(s) => s.prev(),
            Self::Gnome(s) => s.prev(),
            Self::Tiefling(s) => s.prev(),
            Self::Gith(s) => s.prev(),
            Self::DragonBorn(s) => s.prev(),
            _ => {}
        }
    }
}

impl Display for Race {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Human(s) => write!(f, "Human ({:?})", s),
            Self::Elf(s) => write!(f, "Elf ({:?})", s),
            Self::Drow(s) => write!(f, "Drow ({:?})", s),
            Self::HalfElf => write!(f, "Half-Elf"),
            Self::Dwarf(s) => write!(f, "Dwarf ({:?})", s),
            Self::Halfling(s) => write!(f, "Halfling ({:?})", s),
            Self::Gnome(s) => write!(f, "Gnome ({:?})", s),
            Self::HalfOrc => write!(f, "Half-Orc"),
            Self::Tiefling(s) => write!(f, "Tiefling ({:?})", s),
            Self::Gith(s) => write!(f, "Gith ({:?})", s),
            Self::DragonBorn(s) => write!(f, "Dragonborn ({:?})", s),
        }
    }
}

// --- Subrace Definitions & Cycle Impls ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HumanSubrace {
    Standard,
    Variant,
}
impl Cycle for HumanSubrace {
    fn next(&mut self) {
        *self = match self {
            Self::Standard => Self::Variant,
            Self::Variant => Self::Standard,
        }
    }
    fn prev(&mut self) {
        self.next()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElfSubrace {
    High,
    Wood,
}
impl Cycle for ElfSubrace {
    fn next(&mut self) {
        *self = match self {
            Self::High => Self::Wood,
            Self::Wood => Self::High,
        }
    }
    fn prev(&mut self) {
        self.next()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrowSubrace {
    LolthSworn,
    Seldarine,
}
impl Cycle for DrowSubrace {
    fn next(&mut self) {
        *self = match self {
            Self::LolthSworn => Self::Seldarine,
            Self::Seldarine => Self::LolthSworn,
        }
    }
    fn prev(&mut self) {
        self.next()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DwarfSubrace {
    Mountain,
    Hill,
    Duergar,
}
impl Cycle for DwarfSubrace {
    fn next(&mut self) {
        *self = match self {
            Self::Mountain => Self::Hill,
            Self::Hill => Self::Duergar,
            Self::Duergar => Self::Mountain,
        }
    }
    fn prev(&mut self) {
        *self = match self {
            Self::Mountain => Self::Duergar,
            Self::Hill => Self::Mountain,
            Self::Duergar => Self::Hill,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HalflingSubrace {
    Lightfoot,
    Stout,
}
impl Cycle for HalflingSubrace {
    fn next(&mut self) {
        *self = match self {
            Self::Lightfoot => Self::Stout,
            Self::Stout => Self::Lightfoot,
        }
    }
    fn prev(&mut self) {
        self.next()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GnomeSubrace {
    Forest,
    Rock,
    Deep,
}
impl Cycle for GnomeSubrace {
    fn next(&mut self) {
        *self = match self {
            Self::Forest => Self::Rock,
            Self::Rock => Self::Deep,
            Self::Deep => Self::Forest,
        }
    }
    fn prev(&mut self) {
        *self = match self {
            Self::Forest => Self::Deep,
            Self::Rock => Self::Forest,
            Self::Deep => Self::Rock,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TieflingSubrace {
    Asmodeus,
    Zariel,
    Mephistopheles,
}
impl Cycle for TieflingSubrace {
    fn next(&mut self) {
        *self = match self {
            Self::Asmodeus => Self::Zariel,
            Self::Zariel => Self::Mephistopheles,
            Self::Mephistopheles => Self::Asmodeus,
        }
    }
    fn prev(&mut self) {
        *self = match self {
            Self::Asmodeus => Self::Mephistopheles,
            Self::Zariel => Self::Asmodeus,
            Self::Mephistopheles => Self::Zariel,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GithSubrace {
    Githyanki,
    Githzerai,
}
impl Cycle for GithSubrace {
    fn next(&mut self) {
        *self = match self {
            Self::Githyanki => Self::Githzerai,
            Self::Githzerai => Self::Githyanki,
        }
    }
    fn prev(&mut self) {
        self.next()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DragonbornSubrace {
    Black,
    Blue,
    Brass,
    Bronze,
    Copper,
    Gold,
    Green,
    Red,
    Silver,
    White,
}
impl Cycle for DragonbornSubrace {
    fn next(&mut self) {
        *self = match self {
            Self::Black => Self::Blue,
            Self::Blue => Self::Brass,
            Self::Brass => Self::Bronze,
            Self::Bronze => Self::Copper,
            Self::Copper => Self::Gold,
            Self::Gold => Self::Green,
            Self::Green => Self::Red,
            Self::Red => Self::Silver,
            Self::Silver => Self::White,
            Self::White => Self::Black,
        }
    }
    fn prev(&mut self) {
        *self = match self {
            Self::Black => Self::White,
            Self::Blue => Self::Black,
            Self::Brass => Self::Blue,
            Self::Bronze => Self::Brass,
            Self::Copper => Self::Bronze,
            Self::Gold => Self::Copper,
            Self::Green => Self::Gold,
            Self::Red => Self::Green,
            Self::Silver => Self::Red,
            Self::White => Self::Silver,
        }
    }
}

/*
 * ====================================
 * CLASS & SUBCLASSES
 * ====================================
 */

#[derive(Debug, Clone, Copy)]
pub enum Class {
    Barbarian(BarbarianSubclass),
    Bard(BardSubclass),
    Wizard(WizardSubclass),
    Sorcerer(SorcererSubclass),
    Ranger(RangerSubclass),
    Fighter(FighterSubclass),
    Monk(MonkSubclass),
    Rogue(RogueSubclass),
    Druid(DruidSubclass),
    Cleric(ClericSubclass),
    Warlock(WarlockSubclass),
    Paladin(PaladinSubclass),
}

impl Class {
    pub const ALL_CLASSES: [&'static str; 12] = [
        "Barbarian",
        "Bard",
        "Wizard",
        "Sorcerer",
        "Ranger",
        "Fighter",
        "Monk",
        "Rogue",
        "Druid",
        "Cleric",
        "Warlock",
        "Paladin",
    ];

    pub fn default_list() -> [Self; 12] {
        [
            Self::Barbarian(BarbarianSubclass::Berserker),
            Self::Bard(BardSubclass::Lore),
            Self::Wizard(WizardSubclass::Evocation),
            Self::Sorcerer(SorcererSubclass::Storm),
            Self::Ranger(RangerSubclass::Hunter),
            Self::Fighter(FighterSubclass::BattleMaster),
            Self::Monk(MonkSubclass::OpenHand),
            Self::Rogue(RogueSubclass::Assassin),
            Self::Druid(DruidSubclass::Land),
            Self::Cleric(ClericSubclass::Trickery),
            Self::Warlock(WarlockSubclass::Fiend),
            Self::Paladin(PaladinSubclass::Devotion),
        ]
    }

    pub fn cycle_sub_next(&mut self) {
        match self {
            Self::Barbarian(s) => s.next(),
            Self::Bard(s) => s.next(),
            Self::Wizard(s) => s.next(),
            Self::Sorcerer(s) => s.next(),
            Self::Ranger(s) => s.next(),
            Self::Fighter(s) => s.next(),
            Self::Monk(s) => s.next(),
            Self::Rogue(s) => s.next(),
            Self::Druid(s) => s.next(),
            Self::Cleric(s) => s.next(),
            Self::Warlock(s) => s.next(),
            Self::Paladin(s) => s.next(),
        }
    }

    pub fn cycle_sub_prev(&mut self) {
        match self {
            Self::Barbarian(s) => s.prev(),
            Self::Bard(s) => s.prev(),
            Self::Wizard(s) => s.prev(),
            Self::Sorcerer(s) => s.prev(),
            Self::Ranger(s) => s.prev(),
            Self::Fighter(s) => s.prev(),
            Self::Monk(s) => s.prev(),
            Self::Rogue(s) => s.prev(),
            Self::Druid(s) => s.prev(),
            Self::Cleric(s) => s.prev(),
            Self::Warlock(s) => s.prev(),
            Self::Paladin(s) => s.prev(),
        }
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Barbarian(s) => write!(f, "Barbarian ({:?})", s),
            Self::Bard(s) => write!(f, "Bard ({:?})", s),
            Self::Wizard(s) => write!(f, "Wizard ({:?})", s),
            Self::Sorcerer(s) => write!(f, "Sorcerer ({:?})", s),
            Self::Ranger(s) => write!(f, "Ranger ({:?})", s),
            Self::Fighter(s) => write!(f, "Fighter ({:?})", s),
            Self::Monk(s) => write!(f, "Monk ({:?})", s),
            Self::Rogue(s) => write!(f, "Rogue ({:?})", s),
            Self::Druid(s) => write!(f, "Druid ({:?})", s),
            Self::Cleric(s) => write!(f, "Cleric ({:?})", s),
            Self::Warlock(s) => write!(f, "Warlock ({:?})", s),
            Self::Paladin(s) => write!(f, "Paladin ({:?})", s),
        }
    }
}

// --- Subclass Definitions & Cycle Impls ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BarbarianSubclass {
    Berserker,
    TotemWarrior,
    Ancestral,
    Storm,
    Zealot,
}
impl Cycle for BarbarianSubclass {
    fn next(&mut self) {
        *self = match self {
            Self::Berserker => Self::TotemWarrior,
            Self::TotemWarrior => Self::Ancestral,
            Self::Ancestral => Self::Storm,
            Self::Storm => Self::Zealot,
            Self::Zealot => Self::Berserker,
        }
    }
    fn prev(&mut self) {
        *self = match self {
            Self::Berserker => Self::Zealot,
            Self::TotemWarrior => Self::Berserker,
            Self::Ancestral => Self::TotemWarrior,
            Self::Storm => Self::Ancestral,
            Self::Zealot => Self::Storm,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BardSubclass {
    Lore,
    Valor,
    Glamour,
    Swords,
    Whispers,
}
impl Cycle for BardSubclass {
    fn next(&mut self) {
        *self = match self {
            Self::Lore => Self::Valor,
            Self::Valor => Self::Glamour,
            Self::Glamour => Self::Swords,
            Self::Swords => Self::Whispers,
            Self::Whispers => Self::Lore,
        }
    }
    fn prev(&mut self) {
        *self = match self {
            Self::Lore => Self::Whispers,
            Self::Valor => Self::Lore,
            Self::Glamour => Self::Valor,
            Self::Swords => Self::Glamour,
            Self::Whispers => Self::Swords,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WizardSubclass {
    Abjuration,
    Conjuration,
    Divination,
    Enchantment,
    Evocation,
    Illusion,
    Necromancy,
    Transmutation,
}
impl Cycle for WizardSubclass {
    fn next(&mut self) {
        *self = match self {
            Self::Abjuration => Self::Conjuration,
            Self::Conjuration => Self::Divination,
            Self::Divination => Self::Enchantment,
            Self::Enchantment => Self::Evocation,
            Self::Evocation => Self::Illusion,
            Self::Illusion => Self::Necromancy,
            Self::Necromancy => Self::Transmutation,
            Self::Transmutation => Self::Abjuration,
        }
    }
    fn prev(&mut self) {
        *self = match self {
            Self::Abjuration => Self::Transmutation,
            Self::Conjuration => Self::Abjuration,
            Self::Divination => Self::Conjuration,
            Self::Enchantment => Self::Divination,
            Self::Evocation => Self::Enchantment,
            Self::Illusion => Self::Evocation,
            Self::Necromancy => Self::Illusion,
            Self::Transmutation => Self::Necromancy,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SorcererSubclass {
    DraconicBloodline,
    WildMagic,
    Divine,
    Shadow,
    Storm,
}
impl Cycle for SorcererSubclass {
    fn next(&mut self) {
        *self = match self {
            Self::DraconicBloodline => Self::WildMagic,
            Self::WildMagic => Self::Divine,
            Self::Divine => Self::Shadow,
            Self::Shadow => Self::Storm,
            Self::Storm => Self::DraconicBloodline,
        }
    }
    fn prev(&mut self) {
        *self = match self {
            Self::DraconicBloodline => Self::Storm,
            Self::WildMagic => Self::DraconicBloodline,
            Self::Divine => Self::WildMagic,
            Self::Shadow => Self::Divine,
            Self::Storm => Self::Shadow,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RangerSubclass {
    BeastMaster,
    Hunter,
    GloomStalker,
    HorizonWalker,
    MonsterSlayer,
}
impl Cycle for RangerSubclass {
    fn next(&mut self) {
        *self = match self {
            Self::BeastMaster => Self::Hunter,
            Self::Hunter => Self::GloomStalker,
            Self::GloomStalker => Self::HorizonWalker,
            Self::HorizonWalker => Self::MonsterSlayer,
            Self::MonsterSlayer => Self::BeastMaster,
        }
    }
    fn prev(&mut self) {
        *self = match self {
            Self::BeastMaster => Self::MonsterSlayer,
            Self::Hunter => Self::BeastMaster,
            Self::GloomStalker => Self::Hunter,
            Self::HorizonWalker => Self::GloomStalker,
            Self::MonsterSlayer => Self::HorizonWalker,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FighterSubclass {
    Champion,
    BattleMaster,
    EldritchKnight,
    Samurai,
    Cavalier,
}
impl Cycle for FighterSubclass {
    fn next(&mut self) {
        *self = match self {
            Self::Champion => Self::BattleMaster,
            Self::BattleMaster => Self::EldritchKnight,
            Self::EldritchKnight => Self::Samurai,
            Self::Samurai => Self::Cavalier,
            Self::Cavalier => Self::Champion,
        }
    }
    fn prev(&mut self) {
        *self = match self {
            Self::Champion => Self::Cavalier,
            Self::BattleMaster => Self::Champion,
            Self::EldritchKnight => Self::BattleMaster,
            Self::Samurai => Self::EldritchKnight,
            Self::Cavalier => Self::Samurai,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonkSubclass {
    OpenHand,
    Shadow,
    FourElements,
    Drunken,
    Kensei,
}
impl Cycle for MonkSubclass {
    fn next(&mut self) {
        *self = match self {
            Self::OpenHand => Self::Shadow,
            Self::Shadow => Self::FourElements,
            Self::FourElements => Self::Drunken,
            Self::Drunken => Self::Kensei,
            Self::Kensei => Self::OpenHand,
        }
    }
    fn prev(&mut self) {
        *self = match self {
            Self::OpenHand => Self::Kensei,
            Self::Shadow => Self::OpenHand,
            Self::FourElements => Self::Shadow,
            Self::Drunken => Self::FourElements,
            Self::Kensei => Self::Drunken,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RogueSubclass {
    Thief,
    Assassin,
    ArcaneTrickster,
    Inquisitive,
    Scout,
}
impl Cycle for RogueSubclass {
    fn next(&mut self) {
        *self = match self {
            Self::Thief => Self::Assassin,
            Self::Assassin => Self::ArcaneTrickster,
            Self::ArcaneTrickster => Self::Inquisitive,
            Self::Inquisitive => Self::Scout,
            Self::Scout => Self::Thief,
        }
    }
    fn prev(&mut self) {
        *self = match self {
            Self::Thief => Self::Scout,
            Self::Assassin => Self::Thief,
            Self::ArcaneTrickster => Self::Assassin,
            Self::Inquisitive => Self::ArcaneTrickster,
            Self::Scout => Self::Inquisitive,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DruidSubclass {
    Land,
    Moon,
    Dreams,
    Shepherd,
    Spores,
}
impl Cycle for DruidSubclass {
    fn next(&mut self) {
        *self = match self {
            Self::Land => Self::Moon,
            Self::Moon => Self::Dreams,
            Self::Dreams => Self::Shepherd,
            Self::Shepherd => Self::Spores,
            Self::Spores => Self::Land,
        }
    }
    fn prev(&mut self) {
        *self = match self {
            Self::Land => Self::Spores,
            Self::Moon => Self::Land,
            Self::Dreams => Self::Moon,
            Self::Shepherd => Self::Dreams,
            Self::Spores => Self::Shepherd,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClericSubclass {
    Life,
    Light,
    Knowledge,
    Nature,
    Tempest,
    Trickery,
    War,
}
impl Cycle for ClericSubclass {
    fn next(&mut self) {
        *self = match self {
            Self::Life => Self::Light,
            Self::Light => Self::Knowledge,
            Self::Knowledge => Self::Nature,
            Self::Nature => Self::Tempest,
            Self::Tempest => Self::Trickery,
            Self::Trickery => Self::War,
            Self::War => Self::Life,
        }
    }
    fn prev(&mut self) {
        *self = match self {
            Self::Life => Self::War,
            Self::Light => Self::Life,
            Self::Knowledge => Self::Light,
            Self::Nature => Self::Knowledge,
            Self::Tempest => Self::Nature,
            Self::Trickery => Self::Tempest,
            Self::War => Self::Trickery,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WarlockSubclass {
    Archfey,
    Fiend,
    GreatOldOne,
    Celestial,
    Hexblade,
}
impl Cycle for WarlockSubclass {
    fn next(&mut self) {
        *self = match self {
            Self::Archfey => Self::Fiend,
            Self::Fiend => Self::GreatOldOne,
            Self::GreatOldOne => Self::Celestial,
            Self::Celestial => Self::Hexblade,
            Self::Hexblade => Self::Archfey,
        }
    }
    fn prev(&mut self) {
        *self = match self {
            Self::Archfey => Self::Hexblade,
            Self::Fiend => Self::Archfey,
            Self::GreatOldOne => Self::Fiend,
            Self::Celestial => Self::GreatOldOne,
            Self::Hexblade => Self::Celestial,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaladinSubclass {
    Devotion,
    Ancients,
    Vengeance,
    Conquest,
    Redemption,
}
impl Cycle for PaladinSubclass {
    fn next(&mut self) {
        *self = match self {
            Self::Devotion => Self::Ancients,
            Self::Ancients => Self::Vengeance,
            Self::Vengeance => Self::Conquest,
            Self::Conquest => Self::Redemption,
            Self::Redemption => Self::Devotion,
        }
    }
    fn prev(&mut self) {
        *self = match self {
            Self::Devotion => Self::Redemption,
            Self::Ancients => Self::Devotion,
            Self::Vengeance => Self::Ancients,
            Self::Conquest => Self::Vengeance,
            Self::Redemption => Self::Conquest,
        }
    }
}

/*
 * ====================================
 * STATS
 * ====================================
 */

#[derive(Debug, Clone, Copy)]
pub struct Stats {
    pub str: u8,
    pub dex: u8,
    pub con: u8,
    pub int: u8,
    pub wis: u8,
    pub cha: u8,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            str: 10,
            dex: 10,
            con: 10,
            int: 10,
            wis: 10,
            cha: 10,
        }
    }

    /// Returns the stats as a list for the UI to iterate over
    pub fn list(&self) -> [(&'static str, u8); 6] {
        [
            ("str", self.str),
            ("dex", self.dex),
            ("con", self.con),
            ("int", self.int),
            ("wis", self.wis),
            ("cha", self.cha),
        ]
    }

    fn calculate_mod(score: u8) -> i8 {
        ((score as i16 - 10) / 2) as i8
    }

    pub fn str_mod(&self) -> i8 {
        Self::calculate_mod(self.str)
    }
    pub fn dex_mod(&self) -> i8 {
        Self::calculate_mod(self.dex)
    }
    pub fn con_mod(&self) -> i8 {
        Self::calculate_mod(self.con)
    }
    pub fn int_mod(&self) -> i8 {
        Self::calculate_mod(self.int)
    }
    pub fn wis_mod(&self) -> i8 {
        Self::calculate_mod(self.wis)
    }
    pub fn cha_mod(&self) -> i8 {
        Self::calculate_mod(self.cha)
    }
}

impl Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "STR:{:2} DEX:{:2} CON:{:2} INT:{:2} WIS:{:2} CHA:{:2}",
            self.str, self.dex, self.con, self.int, self.wis, self.cha
        )
    }
}

impl From<[(&str, u8); 6]> for Stats {
    fn from(value: [(&str, u8); 6]) -> Self {
        let mut stat = Stats::new();
        for (name, val) in value {
            match name {
                "str" => stat.str = val,
                "dex" => stat.dex = val,
                "con" => stat.con = val,
                "int" => stat.int = val,
                "wis" => stat.wis = val,
                "cha" => stat.cha = val,
                _ => {}
            }
        }
        stat
    }
}

/*
 * ====================================
 * SKILLS
 * ====================================
 */

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProficiencyLevel {
    None,       // [ ]
    Half,       // [h]
    Proficient, // [P]
    Expertise,  // [E]
}

impl Cycle for ProficiencyLevel {
    fn next(&mut self) {
        *self = match self {
            Self::None => Self::Proficient,
            Self::Proficient => Self::Expertise,
            Self::Expertise => Self::Half,
            Self::Half => Self::None,
        }
    }

    fn prev(&mut self) {
        *self = match self {
            Self::None => Self::Half,
            Self::Half => Self::Expertise,
            Self::Expertise => Self::Proficient,
            Self::Proficient => Self::None,
        }
    }
}

impl ProficiencyLevel {
    pub fn symbol(&self) -> &str {
        match self {
            Self::None => "[ ]",
            Self::Half => "[h]",
            Self::Proficient => "[P]",
            Self::Expertise => "[E]",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Skill {
    pub proficiency: ProficiencyLevel,
    pub misc_bonus: i8,
}

impl Skill {
    pub fn new(level: ProficiencyLevel) -> Self {
        Self {
            proficiency: level,
            misc_bonus: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Skills {
    pub acrobatics: Skill,
    pub animal_handling: Skill,
    pub arcana: Skill,
    pub athletics: Skill,
    pub deception: Skill,
    pub history: Skill,
    pub insight: Skill,
    pub intimidation: Skill,
    pub investigation: Skill,
    pub medicine: Skill,
    pub nature: Skill,
    pub perception: Skill,
    pub performance: Skill,
    pub persuasion: Skill,
    pub religion: Skill,
    pub sleight_of_hand: Skill,
    pub stealth: Skill,
    pub survival: Skill,
}

impl Skills {
    pub const ALL_NAMES: [&'static str; 18] = [
        "Acrobatics",
        "Animal Handling",
        "Arcana",
        "Athletics",
        "Deception",
        "History",
        "Insight",
        "Intimidation",
        "Investigation",
        "Medicine",
        "Nature",
        "Perception",
        "Performance",
        "Persuasion",
        "Religion",
        "Sleight of Hand",
        "Stealth",
        "Survival",
    ];
}

impl From<[Skill; 18]> for Skills {
    fn from(skills: [Skill; 18]) -> Self {
        Self {
            acrobatics: skills[0],
            animal_handling: skills[1],
            arcana: skills[2],
            athletics: skills[3],
            deception: skills[4],
            history: skills[5],
            insight: skills[6],
            intimidation: skills[7],
            investigation: skills[8],
            medicine: skills[9],
            nature: skills[10],
            perception: skills[11],
            performance: skills[12],
            persuasion: skills[13],
            religion: skills[14],
            sleight_of_hand: skills[15],
            stealth: skills[16],
            survival: skills[17],
        }
    }
}

/*
 * ====================================
 * CHARACTER
 * ====================================
 */

pub struct Character {
    pub lvl: u8,
    pub name: String,
    pub nickname: String,
    pub ac: u8,
    pub race: Race,
    pub class: Class,
    pub stats: Stats,
    pub skills: Skills,
    pub items_notes: String,
    pub notes: String,
}

impl ToString for Character {
    fn to_string(&self) -> String {
        format!(
            r"
Name: {}
Nickname: {}
Level: {}
CA: {} 
Race: {}
Class: {}

Abilities: 
{}
Modifiers:
str: {:+}, dex: {:+}, con: {:+}, int: {:+}, wis: {:+}, cha: {:+}

Items Notes: 
{}
          
Notes:
{}
",
            self.name,
            self.nickname,
            self.lvl,
            self.ac,
            self.race,
            self.class,
            self.stats,
            self.stats.str_mod(),
            self.stats.dex_mod(),
            self.stats.con_mod(),
            self.stats.int_mod(),
            self.stats.wis_mod(),
            self.stats.cha_mod(),
            self.items_notes,
            self.notes
        )
    }
}
