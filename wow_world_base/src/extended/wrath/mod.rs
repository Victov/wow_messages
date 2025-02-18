use crate::wrath::{Class, Map, Race, RaceClass};

mod actions;
mod base_stats;
pub mod character_features;
mod class;
mod creature_family;
pub mod exp;
pub mod item;
mod item_quality;
mod item_set;
mod items;
mod map;
pub mod position;
mod skill;
mod skills;
mod spells;
pub mod stats;
pub mod trigger;

pub use actions::*;
pub use base_stats::*;
pub use class::*;
pub use creature_family::*;
pub use item_quality::*;
pub use item_set::*;
pub use items::*;
pub use map::*;
pub use skill::*;
pub use skills::*;
pub use spells::*;

macro_rules! wrath_race_class_match {
    ($function:ident, $ret_type:ty) => {
        impl RaceClass {
            pub const fn $function(&self) -> $ret_type {
                match self {
                    RaceClass::DwarfHunter => DWARF_HUNTER,
                    RaceClass::DwarfPaladin => DWARF_PALADIN,
                    RaceClass::DwarfPriest => DWARF_PRIEST,
                    RaceClass::DwarfRogue => DWARF_ROGUE,
                    RaceClass::DwarfWarrior => DWARF_WARRIOR,
                    RaceClass::GnomeMage => GNOME_MAGE,
                    RaceClass::GnomeRogue => GNOME_ROGUE,
                    RaceClass::GnomeWarlock => GNOME_WARLOCK,
                    RaceClass::GnomeWarrior => GNOME_WARRIOR,
                    RaceClass::HumanMage => HUMAN_MAGE,
                    RaceClass::HumanPaladin => HUMAN_PALADIN,
                    RaceClass::HumanPriest => HUMAN_PRIEST,
                    RaceClass::HumanRogue => HUMAN_ROGUE,
                    RaceClass::HumanWarlock => HUMAN_WARLOCK,
                    RaceClass::HumanWarrior => HUMAN_WARRIOR,
                    RaceClass::NightElfDruid => NIGHT_ELF_DRUID,
                    RaceClass::NightElfHunter => NIGHT_ELF_HUNTER,
                    RaceClass::NightElfPriest => NIGHT_ELF_PRIEST,
                    RaceClass::NightElfRogue => NIGHT_ELF_ROGUE,
                    RaceClass::NightElfWarrior => NIGHT_ELF_WARRIOR,
                    RaceClass::OrcHunter => ORC_HUNTER,
                    RaceClass::OrcRogue => ORC_ROGUE,
                    RaceClass::OrcShaman => ORC_SHAMAN,
                    RaceClass::OrcWarlock => ORC_WARLOCK,
                    RaceClass::OrcWarrior => ORC_WARRIOR,
                    RaceClass::TaurenDruid => TAUREN_DRUID,
                    RaceClass::TaurenHunter => TAUREN_HUNTER,
                    RaceClass::TaurenShaman => TAUREN_SHAMAN,
                    RaceClass::TaurenWarrior => TAUREN_WARRIOR,
                    RaceClass::TrollHunter => TROLL_HUNTER,
                    RaceClass::TrollMage => TROLL_MAGE,
                    RaceClass::TrollPriest => TROLL_PRIEST,
                    RaceClass::TrollRogue => TROLL_ROGUE,
                    RaceClass::TrollShaman => TROLL_SHAMAN,
                    RaceClass::TrollWarrior => TROLL_WARRIOR,
                    RaceClass::UndeadMage => UNDEAD_MAGE,
                    RaceClass::UndeadPriest => UNDEAD_PRIEST,
                    RaceClass::UndeadRogue => UNDEAD_ROGUE,
                    RaceClass::UndeadWarlock => UNDEAD_WARLOCK,
                    RaceClass::UndeadWarrior => UNDEAD_WARRIOR,
                    RaceClass::DraeneiWarrior => DRAENEI_WARRIOR,
                    RaceClass::DraeneiPaladin => DRAENEI_PALADIN,
                    RaceClass::DraeneiHunter => DRAENEI_HUNTER,
                    RaceClass::DraeneiPriest => DRAENEI_PRIEST,
                    RaceClass::DraeneiShaman => DRAENEI_SHAMAN,
                    RaceClass::DraeneiMage => DRAENEI_MAGE,
                    RaceClass::BloodElfPaladin => BLOOD_ELF_PALADIN,
                    RaceClass::BloodElfHunter => BLOOD_ELF_HUNTER,
                    RaceClass::BloodElfRogue => BLOOD_ELF_ROGUE,
                    RaceClass::BloodElfPriest => BLOOD_ELF_PRIEST,
                    RaceClass::BloodElfMage => BLOOD_ELF_MAGE,
                    RaceClass::BloodElfWarlock => BLOOD_ELF_WARLOCK,
                    RaceClass::HumanDeathKnight => HUMAN_DEATH_KNIGHT,
                    RaceClass::OrcDeathKnight => ORC_DEATH_KNIGHT,
                    RaceClass::DwarfDeathKnight => DWARF_DEATH_KNIGHT,
                    RaceClass::NightElfDeathKnight => NIGHT_ELF_DEATH_KNIGHT,
                    RaceClass::UndeadDeathKnight => UNDEAD_DEATH_KNIGHT,
                    RaceClass::TaurenDeathKnight => TAUREN_DEATH_KNIGHT,
                    RaceClass::GnomeDeathKnight => GNOME_DEATH_KNIGHT,
                    RaceClass::TrollDeathKnight => TROLL_DEATH_KNIGHT,
                    RaceClass::BloodElfDeathKnight => BLOOD_ELF_DEATH_KNIGHT,
                    RaceClass::DraeneiDeathKnight => DRAENEI_DEATH_KNIGHT,
                }
            }
        }
    };
}
pub(crate) use wrath_race_class_match;
