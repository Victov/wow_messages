use crate::extended::vanilla::vanilla_race_class_match;
use crate::vanilla::RaceClass;

vanilla_race_class_match!(starter_spells, &'static [u32]);

// AUTOGENERATED_START
const HUMAN_WARRIOR: &[u32] = &[
    78, // Heroic Strike
    81, // Dodge
    107, // Block
    196, // One-Handed Axes
    198, // One-Handed Maces
    201, // One-Handed Swords
    203, // Unarmed
    204, // Defense
    522, // SPELLDEFENSE (DND)
    668, // Language Common
    2382, // Generic
    2457, // Battle Stance
    2479, // Honorless Target
    3050, // Detect
    3365, // Opening
    5301, // Defensive State (DND)
    6233, // Closing
    6246, // Closing
    6247, // Opening
    6477, // Opening
    6478, // Opening
    6603, // Attack
    7266, // Duel
    7267, // Grovel
    7355, // Stuck
    8386, // Attacking
    8737, // Mail
    9077, // Leather
    9078, // Cloth
    9116, // Shield
    9125, // Generic
    20597, // Sword Specialization
    20598, // The Human Spirit
    20599, // Diplomacy
    20600, // Perception
    20864, // Mace Specialization
    21651, // Opening
    21652, // Closing
    22027, // Remove Insignia
    22810, // Opening - No Text
];
const HUMAN_PALADIN: &[u32] = &[
    81, // Dodge
    107, // Block
    198, // One-Handed Maces
    199, // Two-Handed Maces
    203, // Unarmed
    204, // Defense
    522, // SPELLDEFENSE (DND)
    635, // Holy Light
    668, // Language Common
    2382, // Generic
    2479, // Honorless Target
    3050, // Detect
    3365, // Opening
    6233, // Closing
    6246, // Closing
    6247, // Opening
    6477, // Opening
    6478, // Opening
    6603, // Attack
    7266, // Duel
    7267, // Grovel
    7355, // Stuck
    8386, // Attacking
    8737, // Mail
    9077, // Leather
    9078, // Cloth
    9116, // Shield
    9125, // Generic
    20154, // Seal of Righteousness
    20597, // Sword Specialization
    20598, // The Human Spirit
    20599, // Diplomacy
    20600, // Perception
    20864, // Mace Specialization
    21651, // Opening
    21652, // Closing
    22027, // Remove Insignia
    22810, // Opening - No Text
    27762, // Libram
];
const HUMAN_ROGUE: &[u32] = &[
    81, // Dodge
    203, // Unarmed
    204, // Defense
    522, // SPELLDEFENSE (DND)
    668, // Language Common
    1180, // Daggers
    1752, // Sinister Strike
    2098, // Eviscerate
    2382, // Generic
    2479, // Honorless Target
    2567, // Thrown
    2764, // Throw
    3050, // Detect
    3365, // Opening
    6233, // Closing
    6246, // Closing
    6247, // Opening
    6477, // Opening
    6478, // Opening
    6603, // Attack
    7266, // Duel
    7267, // Grovel
    7355, // Stuck
    8386, // Attacking
    9077, // Leather
    9078, // Cloth
    9125, // Generic
    16092, // Defensive State (DND)
    20597, // Sword Specialization
    20598, // The Human Spirit
    20599, // Diplomacy
    20600, // Perception
    20864, // Mace Specialization
    21184, // Rogue Passive (DND)
    21651, // Opening
    21652, // Closing
    22027, // Remove Insignia
    22810, // Opening - No Text
];
const HUMAN_PRIEST: &[u32] = &[
    81, // Dodge
    198, // One-Handed Maces
    203, // Unarmed
    204, // Defense
    522, // SPELLDEFENSE (DND)
    585, // Smite
    668, // Language Common
    2050, // Lesser Heal
    2382, // Generic
    2479, // Honorless Target
    3050, // Detect
    3365, // Opening
    5009, // Wands
    5019, // Shoot
    6233, // Closing
    6246, // Closing
    6247, // Opening
    6477, // Opening
    6478, // Opening
    6603, // Attack
    7266, // Duel
    7267, // Grovel
    7355, // Stuck
    8386, // Attacking
    9078, // Cloth
    9125, // Generic
    20597, // Sword Specialization
    20598, // The Human Spirit
    20599, // Diplomacy
    20600, // Perception
    20864, // Mace Specialization
    21651, // Opening
    21652, // Closing
    22027, // Remove Insignia
    22810, // Opening - No Text
];
const HUMAN_MAGE: &[u32] = &[
    81, // Dodge
    133, // Fireball
    168, // Frost Armor
    203, // Unarmed
    204, // Defense
    227, // Staves
    522, // SPELLDEFENSE (DND)
    668, // Language Common
    2382, // Generic
    2479, // Honorless Target
    3050, // Detect
    3365, // Opening
    5009, // Wands
    5019, // Shoot
    6233, // Closing
    6246, // Closing
    6247, // Opening
    6477, // Opening
    6478, // Opening
    6603, // Attack
    7266, // Duel
    7267, // Grovel
    7355, // Stuck
    8386, // Attacking
    9078, // Cloth
    9125, // Generic
    20597, // Sword Specialization
    20598, // The Human Spirit
    20599, // Diplomacy
    20600, // Perception
    20864, // Mace Specialization
    21651, // Opening
    21652, // Closing
    22027, // Remove Insignia
    22810, // Opening - No Text
];
const HUMAN_WARLOCK: &[u32] = &[
    81, // Dodge
    203, // Unarmed
    204, // Defense
    522, // SPELLDEFENSE (DND)
    668, // Language Common
    686, // Shadow Bolt
    687, // Demon Skin
    1180, // Daggers
    2382, // Generic
    2479, // Honorless Target
    3050, // Detect
    3365, // Opening
    5009, // Wands
    5019, // Shoot
    6233, // Closing
    6246, // Closing
    6247, // Opening
    6477, // Opening
    6478, // Opening
    6603, // Attack
    7266, // Duel
    7267, // Grovel
    7355, // Stuck
    8386, // Attacking
    9078, // Cloth
    9125, // Generic
    20597, // Sword Specialization
    20598, // The Human Spirit
    20599, // Diplomacy
    20600, // Perception
    20864, // Mace Specialization
    21651, // Opening
    21652, // Closing
    22027, // Remove Insignia
    22810, // Opening - No Text
];
const ORC_WARRIOR: &[u32] = &[
    78, // Heroic Strike
    81, // Dodge
    107, // Block
    196, // One-Handed Axes
    197, // Two-Handed Axes
    201, // One-Handed Swords
    203, // Unarmed
    204, // Defense
    522, // SPELLDEFENSE (DND)
    669, // Language Orcish
    2382, // Generic
    2457, // Battle Stance
    2479, // Honorless Target
    3050, // Detect
    3365, // Opening
    5301, // Defensive State (DND)
    6233, // Closing
    6246, // Closing
    6247, // Opening
    6477, // Opening
    6478, // Opening
    6603, // Attack
    7266, // Duel
    7267, // Grovel
    7355, // Stuck
    8386, // Attacking
    8737, // Mail
    9077, // Leather
    9078, // Cloth
    9116, // Shield
    9125, // Generic
    20572, // Blood Fury
    20573, // Hardiness
    20574, // Axe Specialization
    21563, // Command
    21651, // Opening
    21652, // Closing
    22027, // Remove Insignia
    22810, // Opening - No Text
];
const ORC_HUNTER: &[u32] = &[
    75, // Auto Shot
    81, // Dodge
    196, // One-Handed Axes
    203, // Unarmed
    204, // Defense
    264, // Bows
    522, // SPELLDEFENSE (DND)
    669, // Language Orcish
    2382, // Generic
    2479, // Honorless Target
    2973, // Raptor Strike
    3050, // Detect
    3365, // Opening
    6233, // Closing
    6246, // Closing
    6247, // Opening
    6477, // Opening
    6478, // Opening
    6603, // Attack
    7266, // Duel
    7267, // Grovel
    7355, // Stuck
    8386, // Attacking
    9077, // Leather
    9078, // Cloth
    9125, // Generic
    13358, // Defensive State (DND)
    20572, // Blood Fury
    20573, // Hardiness
    20574, // Axe Specialization
    20576, // Command
    21651, // Opening
    21652, // Closing
    22027, // Remove Insignia
    22810, // Opening - No Text
    24949, // Defensive State 2 (DND)
];
const ORC_ROGUE: &[u32] = &[
    81, // Dodge
    203, // Unarmed
    204, // Defense
    522, // SPELLDEFENSE (DND)
    669, // Language Orcish
    1180, // Daggers
    1752, // Sinister Strike
    2098, // Eviscerate
    2382, // Generic
    2479, // Honorless Target
    2567, // Thrown
    2764, // Throw
    3050, // Detect
    3365, // Opening
    6233, // Closing
    6246, // Closing
    6247, // Opening
    6477, // Opening
    6478, // Opening
    6603, // Attack
    7266, // Duel
    7267, // Grovel
    7355, // Stuck
    8386, // Attacking
    9077, // Leather
    9078, // Cloth
    9125, // Generic
    16092, // Defensive State (DND)
    20572, // Blood Fury
    20573, // Hardiness
    20574, // Axe Specialization
    21184, // Rogue Passive (DND)
    21563, // Command
    21651, // Opening
    21652, // Closing
    22027, // Remove Insignia
    22810, // Opening - No Text
];
const ORC_SHAMAN: &[u32] = &[
    81, // Dodge
    107, // Block
    198, // One-Handed Maces
    203, // Unarmed
    204, // Defense
    227, // Staves
    331, // Healing Wave
    403, // Lightning Bolt
    522, // SPELLDEFENSE (DND)
    669, // Language Orcish
    2382, // Generic
    2479, // Honorless Target
    3050, // Detect
    3365, // Opening
    6233, // Closing
    6246, // Closing
    6247, // Opening
    6477, // Opening
    6478, // Opening
    6603, // Attack
    7266, // Duel
    7267, // Grovel
    7355, // Stuck
    8386, // Attacking
    9077, // Leather
    9078, // Cloth
    9116, // Shield
    9125, // Generic
    20572, // Blood Fury
    20573, // Hardiness
    20574, // Axe Specialization
    21563, // Command
    21651, // Opening
    21652, // Closing
    22027, // Remove Insignia
    22810, // Opening - No Text
    27763, // Totem
];
const ORC_WARLOCK: &[u32] = &[
    81, // Dodge
    203, // Unarmed
    204, // Defense
    522, // SPELLDEFENSE (DND)
    669, // Language Orcish
    686, // Shadow Bolt
    687, // Demon Skin
    1180, // Daggers
    2382, // Generic
    2479, // Honorless Target
    3050, // Detect
    3365, // Opening
    5009, // Wands
    5019, // Shoot
    6233, // Closing
    6246, // Closing
    6247, // Opening
    6477, // Opening
    6478, // Opening
    6603, // Attack
    7266, // Duel
    7267, // Grovel
    7355, // Stuck
    8386, // Attacking
    9078, // Cloth
    9125, // Generic
    20572, // Blood Fury
    20573, // Hardiness
    20574, // Axe Specialization
    20575, // Command
    21651, // Opening
    21652, // Closing
    22027, // Remove Insignia
    22810, // Opening - No Text
];
const DWARF_WARRIOR: &[u32] = &[
    78, // Heroic Strike
    81, // Dodge
    107, // Block
    196, // One-Handed Axes
    197, // Two-Handed Axes
    198, // One-Handed Maces
    203, // Unarmed
    204, // Defense
    522, // SPELLDEFENSE (DND)
    668, // Language Common
    672, // Language Dwarven
    2382, // Generic
    2457, // Battle Stance
    2479, // Honorless Target
    2481, // Find Treasure
    3050, // Detect
    3365, // Opening
    5301, // Defensive State (DND)
    6233, // Closing
    6246, // Closing
    6247, // Opening
    6477, // Opening
    6478, // Opening
    6603, // Attack
    7266, // Duel
    7267, // Grovel
    7355, // Stuck
    8386, // Attacking
    8737, // Mail
    9077, // Leather
    9078, // Cloth
    9116, // Shield
    9125, // Generic
    20594, // Stoneform
    20595, // Gun Specialization
    20596, // Frost Resistance
    21651, // Opening
    21652, // Closing
    22027, // Remove Insignia
    22810, // Opening - No Text
];
const DWARF_PALADIN: &[u32] = &[
    81, // Dodge
    107, // Block
    198, // One-Handed Maces
    199, // Two-Handed Maces
    203, // Unarmed
    204, // Defense
    522, // SPELLDEFENSE (DND)
    635, // Holy Light
    668, // Language Common
    672, // Language Dwarven
    2382, // Generic
    2479, // Honorless Target
    2481, // Find Treasure
    3050, // Detect
    3365, // Opening
    6233, // Closing
    6246, // Closing
    6247, // Opening
    6477, // Opening
    6478, // Opening
    6603, // Attack
    7266, // Duel
    7267, // Grovel
    7355, // Stuck
    8386, // Attacking
    8737, // Mail
    9077, // Leather
    9078, // Cloth
    9116, // Shield
    9125, // Generic
    20154, // Seal of Righteousness
    20594, // Stoneform
    20595, // Gun Specialization
    20596, // Frost Resistance
    21651, // Opening
    21652, // Closing
    22027, // Remove Insignia
    22810, // Opening - No Text
    27762, // Libram
];
const DWARF_HUNTER: &[u32] = &[
    75, // Auto Shot
    81, // Dodge
    196, // One-Handed Axes
    203, // Unarmed
    204, // Defense
    266, // Guns
    522, // SPELLDEFENSE (DND)
    668, // Language Common
    672, // Language Dwarven
    2382, // Generic
    2479, // Honorless Target
    2481, // Find Treasure
    2973, // Raptor Strike
    3050, // Detect
    3365, // Opening
    6233, // Closing
    6246, // Closing
    6247, // Opening
    6477, // Opening
    6478, // Opening
    6603, // Attack
    7266, // Duel
    7267, // Grovel
    7355, // Stuck
    8386, // Attacking
    9077, // Leather
    9078, // Cloth
    9125, // Generic
    13358, // Defensive State (DND)
    20594, // Stoneform
    20595, // Gun Specialization
    20596, // Frost Resistance
    21651, // Opening
    21652, // Closing
    22027, // Remove Insignia
    22810, // Opening - No Text
    24949, // Defensive State 2 (DND)
];
const DWARF_ROGUE: &[u32] = &[
    81, // Dodge
    203, // Unarmed
    204, // Defense
    522, // SPELLDEFENSE (DND)
    668, // Language Common
    672, // Language Dwarven
    1180, // Daggers
    1752, // Sinister Strike
    2098, // Eviscerate
    2382, // Generic
    2479, // Honorless Target
    2481, // Find Treasure
    2567, // Thrown
    2764, // Throw
    3050, // Detect
    3365, // Opening
    6233, // Closing
    6246, // Closing
    6247, // Opening
    6477, // Opening
    6478, // Opening
    6603, // Attack
    7266, // Duel
    7267, // Grovel
    7355, // Stuck
    8386, // Attacking
    9077, // Leather
    9078, // Cloth
    9125, // Generic
    16092, // Defensive State (DND)
    20594, // Stoneform
    20595, // Gun Specialization
    20596, // Frost Resistance
    21184, // Rogue Passive (DND)
    21651, // Opening
    21652, // Closing
    22027, // Remove Insignia
    22810, // Opening - No Text
];
const DWARF_PRIEST: &[u32] = &[
    81, // Dodge
    198, // One-Handed Maces
    203, // Unarmed
    204, // Defense
    522, // SPELLDEFENSE (DND)
    585, // Smite
    668, // Language Common
    672, // Language Dwarven
    2050, // Lesser Heal
    2382, // Generic
    2479, // Honorless Target
    2481, // Find Treasure
    3050, // Detect
    3365, // Opening
    5009, // Wands
    5019, // Shoot
    6233, // Closing
    6246, // Closing
    6247, // Opening
    6477, // Opening
    6478, // Opening
    6603, // Attack
    7266, // Duel
    7267, // Grovel
    7355, // Stuck
    8386, // Attacking
    9078, // Cloth
    9125, // Generic
    20594, // Stoneform
    20595, // Gun Specialization
    20596, // Frost Resistance
    21651, // Opening
    21652, // Closing
    22027, // Remove Insignia
    22810, // Opening - No Text
];
const NIGHT_ELF_WARRIOR: &[u32] = &[
    78, // Heroic Strike
    81, // Dodge
    107, // Block
    198, // One-Handed Maces
    201, // One-Handed Swords
    203, // Unarmed
    204, // Defense
    522, // SPELLDEFENSE (DND)
    668, // Language Common
    671, // Language Darnassian
    1180, // Daggers
    2382, // Generic
    2457, // Battle Stance
    2479, // Honorless Target
    3050, // Detect
    3365, // Opening
    5301, // Defensive State (DND)
    6233, // Closing
    6246, // Closing
    6247, // Opening
    6477, // Opening
    6478, // Opening
    6603, // Attack
    7266, // Duel
    7267, // Grovel
    7355, // Stuck
    8386, // Attacking
    8737, // Mail
    9077, // Leather
    9078, // Cloth
    9116, // Shield
    9125, // Generic
    20580, // Shadowmeld
    20582, // Quickness
    20583, // Nature Resistance
    20585, // Wisp Spirit
    21009, // Shadowmeld Passive
    21651, // Opening
    21652, // Closing
    22027, // Remove Insignia
    22810, // Opening - No Text
];
const NIGHT_ELF_HUNTER: &[u32] = &[
    75, // Auto Shot
    81, // Dodge
    203, // Unarmed
    204, // Defense
    264, // Bows
    522, // SPELLDEFENSE (DND)
    668, // Language Common
    671, // Language Darnassian
    1180, // Daggers
    2382, // Generic
    2479, // Honorless Target
    2973, // Raptor Strike
    3050, // Detect
    3365, // Opening
    6233, // Closing
    6246, // Closing
    6247, // Opening
    6477, // Opening
    6478, // Opening
    6603, // Attack
    7266, // Duel
    7267, // Grovel
    7355, // Stuck
    8386, // Attacking
    9077, // Leather
    9078, // Cloth
    9125, // Generic
    13358, // Defensive State (DND)
    20580, // Shadowmeld
    20582, // Quickness
    20583, // Nature Resistance
    20585, // Wisp Spirit
    21009, // Shadowmeld Passive
    21651, // Opening
    21652, // Closing
    22027, // Remove Insignia
    22810, // Opening - No Text
    24949, // Defensive State 2 (DND)
];
const NIGHT_ELF_ROGUE: &[u32] = &[
    81, // Dodge
    203, // Unarmed
    204, // Defense
    522, // SPELLDEFENSE (DND)
    668, // Language Common
    671, // Language Darnassian
    1180, // Daggers
    1752, // Sinister Strike
    2098, // Eviscerate
    2382, // Generic
    2479, // Honorless Target
    2567, // Thrown
    2764, // Throw
    3050, // Detect
    3365, // Opening
    6233, // Closing
    6246, // Closing
    6247, // Opening
    6477, // Opening
    6478, // Opening
    6603, // Attack
    7266, // Duel
    7267, // Grovel
    7355, // Stuck
    8386, // Attacking
    9077, // Leather
    9078, // Cloth
    9125, // Generic
    16092, // Defensive State (DND)
    20580, // Shadowmeld
    20582, // Quickness
    20583, // Nature Resistance
    20585, // Wisp Spirit
    21009, // Shadowmeld Passive
    21184, // Rogue Passive (DND)
    21651, // Opening
    21652, // Closing
    22027, // Remove Insignia
    22810, // Opening - No Text
];
const NIGHT_ELF_PRIEST: &[u32] = &[
    81, // Dodge
    198, // One-Handed Maces
    203, // Unarmed
    204, // Defense
    522, // SPELLDEFENSE (DND)
    585, // Smite
    668, // Language Common
    671, // Language Darnassian
    2050, // Lesser Heal
    2382, // Generic
    2479, // Honorless Target
    3050, // Detect
    3365, // Opening
    5009, // Wands
    5019, // Shoot
    6233, // Closing
    6246, // Closing
    6247, // Opening
    6477, // Opening
    6478, // Opening
    6603, // Attack
    7266, // Duel
    7267, // Grovel
    7355, // Stuck
    8386, // Attacking
    9078, // Cloth
    9125, // Generic
    20580, // Shadowmeld
    20582, // Quickness
    20583, // Nature Resistance
    20585, // Wisp Spirit
    21009, // Shadowmeld Passive
    21651, // Opening
    21652, // Closing
    22027, // Remove Insignia
    22810, // Opening - No Text
];
const NIGHT_ELF_DRUID: &[u32] = &[
    81, // Dodge
    203, // Unarmed
    204, // Defense
    227, // Staves
    522, // SPELLDEFENSE (DND)
    668, // Language Common
    671, // Language Darnassian
    1180, // Daggers
    2382, // Generic
    2479, // Honorless Target
    3050, // Detect
    3365, // Opening
    5176, // Wrath
    5185, // Healing Touch
    6233, // Closing
    6246, // Closing
    6247, // Opening
    6477, // Opening
    6478, // Opening
    6603, // Attack
    7266, // Duel
    7267, // Grovel
    7355, // Stuck
    8386, // Attacking
    9077, // Leather
    9078, // Cloth
    9125, // Generic
    20580, // Shadowmeld
    20582, // Quickness
    20583, // Nature Resistance
    20585, // Wisp Spirit
    21009, // Shadowmeld Passive
    21651, // Opening
    21652, // Closing
    22027, // Remove Insignia
    22810, // Opening - No Text
    27764, // Fetish
];
const UNDEAD_WARRIOR: &[u32] = &[
    78, // Heroic Strike
    81, // Dodge
    107, // Block
    201, // One-Handed Swords
    202, // Two-Handed Swords
    203, // Unarmed
    204, // Defense
    522, // SPELLDEFENSE (DND)
    669, // Language Orcish
    1180, // Daggers
    2382, // Generic
    2457, // Battle Stance
    2479, // Honorless Target
    3050, // Detect
    3365, // Opening
    5227, // Underwater Breathing
    5301, // Defensive State (DND)
    6233, // Closing
    6246, // Closing
    6247, // Opening
    6477, // Opening
    6478, // Opening
    6603, // Attack
    7266, // Duel
    7267, // Grovel
    7355, // Stuck
    7744, // Will of the Forsaken
    8386, // Attacking
    8737, // Mail
    9077, // Leather
    9078, // Cloth
    9116, // Shield
    9125, // Generic
    17737, // Language Gutterspeak
    20577, // Cannibalize
    20579, // Shadow Resistance
    21651, // Opening
    21652, // Closing
    22027, // Remove Insignia
    22810, // Opening - No Text
];
const UNDEAD_ROGUE: &[u32] = &[
    81, // Dodge
    203, // Unarmed
    204, // Defense
    522, // SPELLDEFENSE (DND)
    669, // Language Orcish
    1180, // Daggers
    1752, // Sinister Strike
    2098, // Eviscerate
    2382, // Generic
    2479, // Honorless Target
    2567, // Thrown
    2764, // Throw
    3050, // Detect
    3365, // Opening
    5227, // Underwater Breathing
    6233, // Closing
    6246, // Closing
    6247, // Opening
    6477, // Opening
    6478, // Opening
    6603, // Attack
    7266, // Duel
    7267, // Grovel
    7355, // Stuck
    7744, // Will of the Forsaken
    8386, // Attacking
    9077, // Leather
    9078, // Cloth
    9125, // Generic
    16092, // Defensive State (DND)
    17737, // Language Gutterspeak
    20577, // Cannibalize
    20579, // Shadow Resistance
    21184, // Rogue Passive (DND)
    21651, // Opening
    21652, // Closing
    22027, // Remove Insignia
    22810, // Opening - No Text
];
const UNDEAD_PRIEST: &[u32] = &[
    81, // Dodge
    198, // One-Handed Maces
    203, // Unarmed
    204, // Defense
    522, // SPELLDEFENSE (DND)
    585, // Smite
    669, // Language Orcish
    2050, // Lesser Heal
    2382, // Generic
    2479, // Honorless Target
    3050, // Detect
    3365, // Opening
    5009, // Wands
    5019, // Shoot
    5227, // Underwater Breathing
    6233, // Closing
    6246, // Closing
    6247, // Opening
    6477, // Opening
    6478, // Opening
    6603, // Attack
    7266, // Duel
    7267, // Grovel
    7355, // Stuck
    7744, // Will of the Forsaken
    8386, // Attacking
    9078, // Cloth
    9125, // Generic
    17737, // Language Gutterspeak
    20577, // Cannibalize
    20579, // Shadow Resistance
    21651, // Opening
    21652, // Closing
    22027, // Remove Insignia
    22810, // Opening - No Text
];
const UNDEAD_MAGE: &[u32] = &[
    81, // Dodge
    133, // Fireball
    168, // Frost Armor
    203, // Unarmed
    204, // Defense
    227, // Staves
    522, // SPELLDEFENSE (DND)
    669, // Language Orcish
    2382, // Generic
    2479, // Honorless Target
    3050, // Detect
    3365, // Opening
    5009, // Wands
    5019, // Shoot
    5227, // Underwater Breathing
    6233, // Closing
    6246, // Closing
    6247, // Opening
    6477, // Opening
    6478, // Opening
    6603, // Attack
    7266, // Duel
    7267, // Grovel
    7355, // Stuck
    7744, // Will of the Forsaken
    8386, // Attacking
    9078, // Cloth
    9125, // Generic
    17737, // Language Gutterspeak
    20577, // Cannibalize
    20579, // Shadow Resistance
    21651, // Opening
    21652, // Closing
    22027, // Remove Insignia
    22810, // Opening - No Text
];
const UNDEAD_WARLOCK: &[u32] = &[
    81, // Dodge
    203, // Unarmed
    204, // Defense
    522, // SPELLDEFENSE (DND)
    669, // Language Orcish
    686, // Shadow Bolt
    687, // Demon Skin
    1180, // Daggers
    2382, // Generic
    2479, // Honorless Target
    3050, // Detect
    3365, // Opening
    5009, // Wands
    5019, // Shoot
    5227, // Underwater Breathing
    6233, // Closing
    6246, // Closing
    6247, // Opening
    6477, // Opening
    6478, // Opening
    6603, // Attack
    7266, // Duel
    7267, // Grovel
    7355, // Stuck
    7744, // Will of the Forsaken
    8386, // Attacking
    9078, // Cloth
    9125, // Generic
    17737, // Language Gutterspeak
    20577, // Cannibalize
    20579, // Shadow Resistance
    21651, // Opening
    21652, // Closing
    22027, // Remove Insignia
    22810, // Opening - No Text
];
const TAUREN_WARRIOR: &[u32] = &[
    78, // Heroic Strike
    81, // Dodge
    107, // Block
    196, // One-Handed Axes
    198, // One-Handed Maces
    199, // Two-Handed Maces
    203, // Unarmed
    204, // Defense
    522, // SPELLDEFENSE (DND)
    669, // Language Orcish
    670, // Language Taurahe
    2382, // Generic
    2457, // Battle Stance
    2479, // Honorless Target
    3050, // Detect
    3365, // Opening
    5301, // Defensive State (DND)
    6233, // Closing
    6246, // Closing
    6247, // Opening
    6477, // Opening
    6478, // Opening
    6603, // Attack
    7266, // Duel
    7267, // Grovel
    7355, // Stuck
    8386, // Attacking
    8737, // Mail
    9077, // Leather
    9078, // Cloth
    9116, // Shield
    9125, // Generic
    20549, // War Stomp
    20550, // Endurance
    20551, // Nature Resistance
    20552, // Cultivation
    21651, // Opening
    21652, // Closing
    22027, // Remove Insignia
    22810, // Opening - No Text
];
const TAUREN_HUNTER: &[u32] = &[
    75, // Auto Shot
    81, // Dodge
    196, // One-Handed Axes
    203, // Unarmed
    204, // Defense
    266, // Guns
    522, // SPELLDEFENSE (DND)
    669, // Language Orcish
    670, // Language Taurahe
    2382, // Generic
    2479, // Honorless Target
    2973, // Raptor Strike
    3050, // Detect
    3365, // Opening
    6233, // Closing
    6246, // Closing
    6247, // Opening
    6477, // Opening
    6478, // Opening
    6603, // Attack
    7266, // Duel
    7267, // Grovel
    7355, // Stuck
    8386, // Attacking
    9077, // Leather
    9078, // Cloth
    9125, // Generic
    13358, // Defensive State (DND)
    20549, // War Stomp
    20550, // Endurance
    20551, // Nature Resistance
    20552, // Cultivation
    21651, // Opening
    21652, // Closing
    22027, // Remove Insignia
    22810, // Opening - No Text
    24949, // Defensive State 2 (DND)
];
const TAUREN_SHAMAN: &[u32] = &[
    81, // Dodge
    107, // Block
    198, // One-Handed Maces
    203, // Unarmed
    204, // Defense
    227, // Staves
    331, // Healing Wave
    403, // Lightning Bolt
    522, // SPELLDEFENSE (DND)
    669, // Language Orcish
    670, // Language Taurahe
    2382, // Generic
    2479, // Honorless Target
    3050, // Detect
    3365, // Opening
    6233, // Closing
    6246, // Closing
    6247, // Opening
    6477, // Opening
    6478, // Opening
    6603, // Attack
    7266, // Duel
    7267, // Grovel
    7355, // Stuck
    8386, // Attacking
    9077, // Leather
    9078, // Cloth
    9116, // Shield
    9125, // Generic
    20549, // War Stomp
    20550, // Endurance
    20551, // Nature Resistance
    20552, // Cultivation
    21651, // Opening
    21652, // Closing
    22027, // Remove Insignia
    22810, // Opening - No Text
    27763, // Totem
];
const TAUREN_DRUID: &[u32] = &[
    81, // Dodge
    198, // One-Handed Maces
    203, // Unarmed
    204, // Defense
    227, // Staves
    522, // SPELLDEFENSE (DND)
    669, // Language Orcish
    670, // Language Taurahe
    2382, // Generic
    2479, // Honorless Target
    3050, // Detect
    3365, // Opening
    5176, // Wrath
    5185, // Healing Touch
    6233, // Closing
    6246, // Closing
    6247, // Opening
    6477, // Opening
    6478, // Opening
    6603, // Attack
    7266, // Duel
    7267, // Grovel
    7355, // Stuck
    8386, // Attacking
    9077, // Leather
    9078, // Cloth
    9125, // Generic
    20549, // War Stomp
    20550, // Endurance
    20551, // Nature Resistance
    20552, // Cultivation
    21651, // Opening
    21652, // Closing
    22027, // Remove Insignia
    22810, // Opening - No Text
    27764, // Fetish
];
const GNOME_WARRIOR: &[u32] = &[
    78, // Heroic Strike
    81, // Dodge
    107, // Block
    198, // One-Handed Maces
    201, // One-Handed Swords
    203, // Unarmed
    204, // Defense
    522, // SPELLDEFENSE (DND)
    668, // Language Common
    1180, // Daggers
    2382, // Generic
    2457, // Battle Stance
    2479, // Honorless Target
    3050, // Detect
    3365, // Opening
    5301, // Defensive State (DND)
    6233, // Closing
    6246, // Closing
    6247, // Opening
    6477, // Opening
    6478, // Opening
    6603, // Attack
    7266, // Duel
    7267, // Grovel
    7340, // Language Gnomish
    7355, // Stuck
    8386, // Attacking
    8737, // Mail
    9077, // Leather
    9078, // Cloth
    9116, // Shield
    9125, // Generic
    20589, // Escape Artist
    20591, // Expansive Mind
    20592, // Arcane Resistance
    20593, // Engineering Specialization
    21651, // Opening
    21652, // Closing
    22027, // Remove Insignia
    22810, // Opening - No Text
];
const GNOME_ROGUE: &[u32] = &[
    81, // Dodge
    203, // Unarmed
    204, // Defense
    522, // SPELLDEFENSE (DND)
    668, // Language Common
    1180, // Daggers
    1752, // Sinister Strike
    2098, // Eviscerate
    2382, // Generic
    2479, // Honorless Target
    2567, // Thrown
    2764, // Throw
    3050, // Detect
    3365, // Opening
    6233, // Closing
    6246, // Closing
    6247, // Opening
    6477, // Opening
    6478, // Opening
    6603, // Attack
    7266, // Duel
    7267, // Grovel
    7340, // Language Gnomish
    7355, // Stuck
    8386, // Attacking
    9077, // Leather
    9078, // Cloth
    9125, // Generic
    16092, // Defensive State (DND)
    20589, // Escape Artist
    20591, // Expansive Mind
    20592, // Arcane Resistance
    20593, // Engineering Specialization
    21184, // Rogue Passive (DND)
    21651, // Opening
    21652, // Closing
    22027, // Remove Insignia
    22810, // Opening - No Text
];
const GNOME_MAGE: &[u32] = &[
    81, // Dodge
    133, // Fireball
    168, // Frost Armor
    203, // Unarmed
    204, // Defense
    227, // Staves
    522, // SPELLDEFENSE (DND)
    668, // Language Common
    2382, // Generic
    2479, // Honorless Target
    3050, // Detect
    3365, // Opening
    5009, // Wands
    5019, // Shoot
    6233, // Closing
    6246, // Closing
    6247, // Opening
    6477, // Opening
    6478, // Opening
    6603, // Attack
    7266, // Duel
    7267, // Grovel
    7340, // Language Gnomish
    7355, // Stuck
    8386, // Attacking
    9078, // Cloth
    9125, // Generic
    20589, // Escape Artist
    20591, // Expansive Mind
    20592, // Arcane Resistance
    20593, // Engineering Specialization
    21651, // Opening
    21652, // Closing
    22027, // Remove Insignia
    22810, // Opening - No Text
];
const GNOME_WARLOCK: &[u32] = &[
    81, // Dodge
    203, // Unarmed
    204, // Defense
    522, // SPELLDEFENSE (DND)
    668, // Language Common
    686, // Shadow Bolt
    687, // Demon Skin
    1180, // Daggers
    2382, // Generic
    2479, // Honorless Target
    3050, // Detect
    3365, // Opening
    5009, // Wands
    5019, // Shoot
    6233, // Closing
    6246, // Closing
    6247, // Opening
    6477, // Opening
    6478, // Opening
    6603, // Attack
    7266, // Duel
    7267, // Grovel
    7340, // Language Gnomish
    7355, // Stuck
    8386, // Attacking
    9078, // Cloth
    9125, // Generic
    20589, // Escape Artist
    20591, // Expansive Mind
    20592, // Arcane Resistance
    20593, // Engineering Specialization
    21651, // Opening
    21652, // Closing
    22027, // Remove Insignia
    22810, // Opening - No Text
];
const TROLL_WARRIOR: &[u32] = &[
    78, // Heroic Strike
    81, // Dodge
    107, // Block
    196, // One-Handed Axes
    203, // Unarmed
    204, // Defense
    522, // SPELLDEFENSE (DND)
    669, // Language Orcish
    1180, // Daggers
    2382, // Generic
    2457, // Battle Stance
    2479, // Honorless Target
    2567, // Thrown
    2764, // Throw
    3050, // Detect
    3365, // Opening
    5301, // Defensive State (DND)
    6233, // Closing
    6246, // Closing
    6247, // Opening
    6477, // Opening
    6478, // Opening
    6603, // Attack
    7266, // Duel
    7267, // Grovel
    7341, // Language Troll
    7355, // Stuck
    8386, // Attacking
    8737, // Mail
    9077, // Leather
    9078, // Cloth
    9116, // Shield
    9125, // Generic
    20555, // Regeneration
    20557, // Beast Slaying
    20558, // Throwing Specialization
    21651, // Opening
    21652, // Closing
    22027, // Remove Insignia
    22810, // Opening - No Text
    26290, // Bow Specialization
    26296, // Berserking
];
const TROLL_HUNTER: &[u32] = &[
    75, // Auto Shot
    81, // Dodge
    196, // One-Handed Axes
    203, // Unarmed
    204, // Defense
    264, // Bows
    522, // SPELLDEFENSE (DND)
    669, // Language Orcish
    2382, // Generic
    2479, // Honorless Target
    2973, // Raptor Strike
    3050, // Detect
    3365, // Opening
    6233, // Closing
    6246, // Closing
    6247, // Opening
    6477, // Opening
    6478, // Opening
    6603, // Attack
    7266, // Duel
    7267, // Grovel
    7341, // Language Troll
    7355, // Stuck
    8386, // Attacking
    9077, // Leather
    9078, // Cloth
    9125, // Generic
    13358, // Defensive State (DND)
    20554, // Berserking
    20555, // Regeneration
    20557, // Beast Slaying
    20558, // Throwing Specialization
    21651, // Opening
    21652, // Closing
    22027, // Remove Insignia
    22810, // Opening - No Text
    24949, // Defensive State 2 (DND)
    26290, // Bow Specialization
];
const TROLL_ROGUE: &[u32] = &[
    81, // Dodge
    203, // Unarmed
    204, // Defense
    522, // SPELLDEFENSE (DND)
    669, // Language Orcish
    1180, // Daggers
    1752, // Sinister Strike
    2098, // Eviscerate
    2382, // Generic
    2479, // Honorless Target
    2567, // Thrown
    2764, // Throw
    3050, // Detect
    3365, // Opening
    6233, // Closing
    6246, // Closing
    6247, // Opening
    6477, // Opening
    6478, // Opening
    6603, // Attack
    7266, // Duel
    7267, // Grovel
    7341, // Language Troll
    7355, // Stuck
    8386, // Attacking
    9077, // Leather
    9078, // Cloth
    9125, // Generic
    16092, // Defensive State (DND)
    20555, // Regeneration
    20557, // Beast Slaying
    20558, // Throwing Specialization
    21184, // Rogue Passive (DND)
    21651, // Opening
    21652, // Closing
    22027, // Remove Insignia
    22810, // Opening - No Text
    26290, // Bow Specialization
    26297, // Berserking
];
const TROLL_PRIEST: &[u32] = &[
    81, // Dodge
    198, // One-Handed Maces
    203, // Unarmed
    204, // Defense
    522, // SPELLDEFENSE (DND)
    585, // Smite
    669, // Language Orcish
    2050, // Lesser Heal
    2382, // Generic
    2479, // Honorless Target
    3050, // Detect
    3365, // Opening
    5009, // Wands
    5019, // Shoot
    6233, // Closing
    6246, // Closing
    6247, // Opening
    6477, // Opening
    6478, // Opening
    6603, // Attack
    7266, // Duel
    7267, // Grovel
    7341, // Language Troll
    7355, // Stuck
    8386, // Attacking
    9078, // Cloth
    9125, // Generic
    20554, // Berserking
    20555, // Regeneration
    20557, // Beast Slaying
    20558, // Throwing Specialization
    21651, // Opening
    21652, // Closing
    22027, // Remove Insignia
    22810, // Opening - No Text
    26290, // Bow Specialization
];
const TROLL_SHAMAN: &[u32] = &[
    81, // Dodge
    107, // Block
    198, // One-Handed Maces
    203, // Unarmed
    204, // Defense
    227, // Staves
    331, // Healing Wave
    403, // Lightning Bolt
    522, // SPELLDEFENSE (DND)
    669, // Language Orcish
    2382, // Generic
    2479, // Honorless Target
    3050, // Detect
    3365, // Opening
    6233, // Closing
    6246, // Closing
    6247, // Opening
    6477, // Opening
    6478, // Opening
    6603, // Attack
    7266, // Duel
    7267, // Grovel
    7341, // Language Troll
    7355, // Stuck
    8386, // Attacking
    9077, // Leather
    9078, // Cloth
    9116, // Shield
    9125, // Generic
    20554, // Berserking
    20555, // Regeneration
    20557, // Beast Slaying
    20558, // Throwing Specialization
    21651, // Opening
    21652, // Closing
    22027, // Remove Insignia
    22810, // Opening - No Text
    26290, // Bow Specialization
    27763, // Totem
];
const TROLL_MAGE: &[u32] = &[
    81, // Dodge
    133, // Fireball
    168, // Frost Armor
    203, // Unarmed
    204, // Defense
    227, // Staves
    522, // SPELLDEFENSE (DND)
    669, // Language Orcish
    2382, // Generic
    2479, // Honorless Target
    3050, // Detect
    3365, // Opening
    5009, // Wands
    5019, // Shoot
    6233, // Closing
    6246, // Closing
    6247, // Opening
    6477, // Opening
    6478, // Opening
    6603, // Attack
    7266, // Duel
    7267, // Grovel
    7341, // Language Troll
    7355, // Stuck
    8386, // Attacking
    9078, // Cloth
    9125, // Generic
    20554, // Berserking
    20555, // Regeneration
    20557, // Beast Slaying
    20558, // Throwing Specialization
    21651, // Opening
    21652, // Closing
    22027, // Remove Insignia
    22810, // Opening - No Text
    26290, // Bow Specialization
];
// AUTOGENERATED_END
