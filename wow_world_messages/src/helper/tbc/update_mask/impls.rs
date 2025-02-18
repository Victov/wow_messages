use crate::Guid;
use std::convert::TryInto;
use super::indices::*;
use crate::tbc::{Race};
use crate::tbc::{Class};
use crate::tbc::{Gender};
use crate::tbc::{Power};
use crate::tbc::{UpdateContainer, UpdateContainerBuilder, UpdateCorpse, UpdateCorpseBuilder, UpdateDynamicObject, UpdateDynamicObjectBuilder, UpdateGameObject, UpdateGameObjectBuilder, UpdateItem, UpdateItemBuilder, UpdatePlayer, UpdatePlayerBuilder, UpdateUnit, UpdateUnitBuilder};

impl UpdateItemBuilder {
    pub fn set_object_GUID(mut self, v: Guid) -> Self {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_TYPE(mut self, v: i32) -> Self {
        self.header_set(2, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_ENTRY(mut self, v: i32) -> Self {
        self.header_set(3, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_SCALE_X(mut self, v: f32) -> Self {
        self.header_set(4, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_OWNER(mut self, v: Guid) -> Self {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_item_CONTAINED(mut self, v: Guid) -> Self {
        self.header_set(8, v.guid() as u32);
        self.header_set(9, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_item_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(10, v.guid() as u32);
        self.header_set(11, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_item_GIFTCREATOR(mut self, v: Guid) -> Self {
        self.header_set(12, v.guid() as u32);
        self.header_set(13, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_item_STACK_COUNT(mut self, v: i32) -> Self {
        self.header_set(14, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_DURATION(mut self, v: i32) -> Self {
        self.header_set(15, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_SPELL_CHARGES(mut self, v: i32) -> Self {
        self.header_set(16, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_FLAGS(mut self, v: i32) -> Self {
        self.header_set(21, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_ENCHANTMENT_1_1(mut self, v: i32) -> Self {
        self.header_set(22, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_PROPERTY_SEED(mut self, v: i32) -> Self {
        self.header_set(55, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_RANDOM_PROPERTIES_ID(mut self, v: i32) -> Self {
        self.header_set(56, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_ITEM_TEXT_ID(mut self, v: i32) -> Self {
        self.header_set(57, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_DURABILITY(mut self, v: i32) -> Self {
        self.header_set(58, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_MAXDURABILITY(mut self, v: i32) -> Self {
        self.header_set(59, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_CREATED_BY(mut self, v: Guid) -> Self {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
        self
    }

}

impl UpdateContainerBuilder {
    pub fn set_object_GUID(mut self, v: Guid) -> Self {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_TYPE(mut self, v: i32) -> Self {
        self.header_set(2, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_ENTRY(mut self, v: i32) -> Self {
        self.header_set(3, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_SCALE_X(mut self, v: f32) -> Self {
        self.header_set(4, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_OWNER(mut self, v: Guid) -> Self {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_item_CONTAINED(mut self, v: Guid) -> Self {
        self.header_set(8, v.guid() as u32);
        self.header_set(9, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_item_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(10, v.guid() as u32);
        self.header_set(11, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_item_GIFTCREATOR(mut self, v: Guid) -> Self {
        self.header_set(12, v.guid() as u32);
        self.header_set(13, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_item_STACK_COUNT(mut self, v: i32) -> Self {
        self.header_set(14, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_DURATION(mut self, v: i32) -> Self {
        self.header_set(15, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_SPELL_CHARGES(mut self, v: i32) -> Self {
        self.header_set(16, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_FLAGS(mut self, v: i32) -> Self {
        self.header_set(21, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_ENCHANTMENT_1_1(mut self, v: i32) -> Self {
        self.header_set(22, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_PROPERTY_SEED(mut self, v: i32) -> Self {
        self.header_set(55, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_RANDOM_PROPERTIES_ID(mut self, v: i32) -> Self {
        self.header_set(56, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_ITEM_TEXT_ID(mut self, v: i32) -> Self {
        self.header_set(57, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_DURABILITY(mut self, v: i32) -> Self {
        self.header_set(58, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_MAXDURABILITY(mut self, v: i32) -> Self {
        self.header_set(59, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_container_NUM_SLOTS(mut self, v: i32) -> Self {
        self.header_set(60, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_container_SLOT_1(mut self, v: Guid) -> Self {
        self.header_set(62, v.guid() as u32);
        self.header_set(63, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_CREATED_BY(mut self, v: Guid) -> Self {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
        self
    }

}

impl UpdateUnitBuilder {
    pub fn set_object_GUID(mut self, v: Guid) -> Self {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_TYPE(mut self, v: i32) -> Self {
        self.header_set(2, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_ENTRY(mut self, v: i32) -> Self {
        self.header_set(3, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_SCALE_X(mut self, v: f32) -> Self {
        self.header_set(4, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_CHARM(mut self, v: Guid) -> Self {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_SUMMON(mut self, v: Guid) -> Self {
        self.header_set(8, v.guid() as u32);
        self.header_set(9, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_CHARMEDBY(mut self, v: Guid) -> Self {
        self.header_set(10, v.guid() as u32);
        self.header_set(11, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_SUMMONEDBY(mut self, v: Guid) -> Self {
        self.header_set(12, v.guid() as u32);
        self.header_set(13, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_CREATEDBY(mut self, v: Guid) -> Self {
        self.header_set(14, v.guid() as u32);
        self.header_set(15, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_TARGET(mut self, v: Guid) -> Self {
        self.header_set(16, v.guid() as u32);
        self.header_set(17, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_PERSUADED(mut self, v: Guid) -> Self {
        self.header_set(18, v.guid() as u32);
        self.header_set(19, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_CHANNEL_OBJECT(mut self, v: Guid) -> Self {
        self.header_set(20, v.guid() as u32);
        self.header_set(21, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_HEALTH(mut self, v: i32) -> Self {
        self.header_set(22, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_POWER1(mut self, v: i32) -> Self {
        self.header_set(23, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_POWER2(mut self, v: i32) -> Self {
        self.header_set(24, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_POWER3(mut self, v: i32) -> Self {
        self.header_set(25, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_POWER4(mut self, v: i32) -> Self {
        self.header_set(26, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_POWER5(mut self, v: i32) -> Self {
        self.header_set(27, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXHEALTH(mut self, v: i32) -> Self {
        self.header_set(28, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXPOWER1(mut self, v: i32) -> Self {
        self.header_set(29, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXPOWER2(mut self, v: i32) -> Self {
        self.header_set(30, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXPOWER3(mut self, v: i32) -> Self {
        self.header_set(31, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXPOWER4(mut self, v: i32) -> Self {
        self.header_set(32, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXPOWER5(mut self, v: i32) -> Self {
        self.header_set(33, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_LEVEL(mut self, v: i32) -> Self {
        self.header_set(34, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_FACTIONTEMPLATE(mut self, v: i32) -> Self {
        self.header_set(35, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_BYTES_0(mut self, race: Race, class: Class, gender: Gender, power: Power) -> Self {
        self.header_set(36, u32::from_le_bytes([race.as_int(), class.as_int(), gender.as_int(), power.as_int()]));
        self
    }

    pub fn set_unit_VIRTUAL_ITEM_SLOT_DISPLAY(mut self, v: i32) -> Self {
        self.header_set(37, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_VIRTUAL_ITEM_INFO(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(40, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_FLAGS(mut self, v: i32) -> Self {
        self.header_set(46, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_FLAGS_2(mut self, v: i32) -> Self {
        self.header_set(47, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_AURA(mut self, v: i32) -> Self {
        self.header_set(48, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_AURAFLAGS(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(104, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_AURALEVELS(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(118, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_AURAAPPLICATIONS(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(132, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_AURASTATE(mut self, v: i32) -> Self {
        self.header_set(146, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_BASEATTACKTIME(mut self, v: i32) -> Self {
        self.header_set(147, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_RANGEDATTACKTIME(mut self, v: i32) -> Self {
        self.header_set(149, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_BOUNDINGRADIUS(mut self, v: f32) -> Self {
        self.header_set(150, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_COMBATREACH(mut self, v: f32) -> Self {
        self.header_set(151, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_DISPLAYID(mut self, v: i32) -> Self {
        self.header_set(152, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_NATIVEDISPLAYID(mut self, v: i32) -> Self {
        self.header_set(153, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MOUNTDISPLAYID(mut self, v: i32) -> Self {
        self.header_set(154, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MINDAMAGE(mut self, v: f32) -> Self {
        self.header_set(155, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXDAMAGE(mut self, v: f32) -> Self {
        self.header_set(156, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MINOFFHANDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(157, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXOFFHANDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(158, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_BYTES_1(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(159, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_PETNUMBER(mut self, v: i32) -> Self {
        self.header_set(160, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_PET_NAME_TIMESTAMP(mut self, v: i32) -> Self {
        self.header_set(161, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_PETEXPERIENCE(mut self, v: i32) -> Self {
        self.header_set(162, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_PETNEXTLEVELEXP(mut self, v: i32) -> Self {
        self.header_set(163, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_DYNAMIC_FLAGS(mut self, v: i32) -> Self {
        self.header_set(164, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_CHANNEL_SPELL(mut self, v: i32) -> Self {
        self.header_set(165, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MOD_CAST_SPEED(mut self, v: f32) -> Self {
        self.header_set(166, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_CREATED_BY_SPELL(mut self, v: i32) -> Self {
        self.header_set(167, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_NPC_FLAGS(mut self, v: i32) -> Self {
        self.header_set(168, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_NPC_EMOTESTATE(mut self, v: i32) -> Self {
        self.header_set(169, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_TRAINING_POINTS(mut self, a: u16, b: u16) -> Self {
        self.header_set(170, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_unit_STRENGTH(mut self, v: i32) -> Self {
        self.header_set(171, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_AGILITY(mut self, v: i32) -> Self {
        self.header_set(172, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_STAMINA(mut self, v: i32) -> Self {
        self.header_set(173, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_INTELLECT(mut self, v: i32) -> Self {
        self.header_set(174, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_SPIRIT(mut self, v: i32) -> Self {
        self.header_set(175, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_POSSTAT1(mut self, v: i32) -> Self {
        self.header_set(177, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_POSSTAT2(mut self, v: i32) -> Self {
        self.header_set(178, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_POSSTAT3(mut self, v: i32) -> Self {
        self.header_set(179, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_NEGSTAT1(mut self, v: i32) -> Self {
        self.header_set(182, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_NEGSTAT2(mut self, v: i32) -> Self {
        self.header_set(183, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_NEGSTAT3(mut self, v: i32) -> Self {
        self.header_set(184, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_RESISTANCES(mut self, v: i32) -> Self {
        self.header_set(186, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_BASE_MANA(mut self, v: i32) -> Self {
        self.header_set(207, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_BASE_HEALTH(mut self, v: i32) -> Self {
        self.header_set(208, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_BYTES_2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(209, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_ATTACK_POWER(mut self, v: i32) -> Self {
        self.header_set(210, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_ATTACK_POWER_MODS(mut self, a: u16, b: u16) -> Self {
        self.header_set(211, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_unit_ATTACK_POWER_MULTIPLIER(mut self, v: f32) -> Self {
        self.header_set(212, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_RANGED_ATTACK_POWER(mut self, v: i32) -> Self {
        self.header_set(213, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_RANGED_ATTACK_POWER_MODS(mut self, a: u16, b: u16) -> Self {
        self.header_set(214, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_unit_RANGED_ATTACK_POWER_MULTIPLIER(mut self, v: f32) -> Self {
        self.header_set(215, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MINRANGEDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(216, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXRANGEDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(217, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_POWER_COST_MODIFIER(mut self, v: i32) -> Self {
        self.header_set(218, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_POWER_COST_MULTIPLIER(mut self, v: f32) -> Self {
        self.header_set(225, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXHEALTHMODIFIER(mut self, v: f32) -> Self {
        self.header_set(232, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_CREATED_BY(mut self, v: Guid) -> Self {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
        self
    }

}

impl UpdatePlayerBuilder {
    pub fn set_object_GUID(mut self, v: Guid) -> Self {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_TYPE(mut self, v: i32) -> Self {
        self.header_set(2, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_ENTRY(mut self, v: i32) -> Self {
        self.header_set(3, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_SCALE_X(mut self, v: f32) -> Self {
        self.header_set(4, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_CHARM(mut self, v: Guid) -> Self {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_SUMMON(mut self, v: Guid) -> Self {
        self.header_set(8, v.guid() as u32);
        self.header_set(9, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_CHARMEDBY(mut self, v: Guid) -> Self {
        self.header_set(10, v.guid() as u32);
        self.header_set(11, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_SUMMONEDBY(mut self, v: Guid) -> Self {
        self.header_set(12, v.guid() as u32);
        self.header_set(13, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_CREATEDBY(mut self, v: Guid) -> Self {
        self.header_set(14, v.guid() as u32);
        self.header_set(15, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_TARGET(mut self, v: Guid) -> Self {
        self.header_set(16, v.guid() as u32);
        self.header_set(17, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_PERSUADED(mut self, v: Guid) -> Self {
        self.header_set(18, v.guid() as u32);
        self.header_set(19, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_CHANNEL_OBJECT(mut self, v: Guid) -> Self {
        self.header_set(20, v.guid() as u32);
        self.header_set(21, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_HEALTH(mut self, v: i32) -> Self {
        self.header_set(22, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_POWER1(mut self, v: i32) -> Self {
        self.header_set(23, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_POWER2(mut self, v: i32) -> Self {
        self.header_set(24, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_POWER3(mut self, v: i32) -> Self {
        self.header_set(25, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_POWER4(mut self, v: i32) -> Self {
        self.header_set(26, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_POWER5(mut self, v: i32) -> Self {
        self.header_set(27, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXHEALTH(mut self, v: i32) -> Self {
        self.header_set(28, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXPOWER1(mut self, v: i32) -> Self {
        self.header_set(29, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXPOWER2(mut self, v: i32) -> Self {
        self.header_set(30, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXPOWER3(mut self, v: i32) -> Self {
        self.header_set(31, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXPOWER4(mut self, v: i32) -> Self {
        self.header_set(32, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXPOWER5(mut self, v: i32) -> Self {
        self.header_set(33, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_LEVEL(mut self, v: i32) -> Self {
        self.header_set(34, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_FACTIONTEMPLATE(mut self, v: i32) -> Self {
        self.header_set(35, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_BYTES_0(mut self, race: Race, class: Class, gender: Gender, power: Power) -> Self {
        self.header_set(36, u32::from_le_bytes([race.as_int(), class.as_int(), gender.as_int(), power.as_int()]));
        self
    }

    pub fn set_unit_VIRTUAL_ITEM_SLOT_DISPLAY(mut self, v: i32) -> Self {
        self.header_set(37, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_VIRTUAL_ITEM_INFO(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(40, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_FLAGS(mut self, v: i32) -> Self {
        self.header_set(46, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_FLAGS_2(mut self, v: i32) -> Self {
        self.header_set(47, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_AURA(mut self, v: i32) -> Self {
        self.header_set(48, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_AURAFLAGS(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(104, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_AURALEVELS(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(118, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_AURAAPPLICATIONS(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(132, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_AURASTATE(mut self, v: i32) -> Self {
        self.header_set(146, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_BASEATTACKTIME(mut self, v: i32) -> Self {
        self.header_set(147, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_RANGEDATTACKTIME(mut self, v: i32) -> Self {
        self.header_set(149, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_BOUNDINGRADIUS(mut self, v: f32) -> Self {
        self.header_set(150, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_COMBATREACH(mut self, v: f32) -> Self {
        self.header_set(151, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_DISPLAYID(mut self, v: i32) -> Self {
        self.header_set(152, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_NATIVEDISPLAYID(mut self, v: i32) -> Self {
        self.header_set(153, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MOUNTDISPLAYID(mut self, v: i32) -> Self {
        self.header_set(154, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MINDAMAGE(mut self, v: f32) -> Self {
        self.header_set(155, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXDAMAGE(mut self, v: f32) -> Self {
        self.header_set(156, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MINOFFHANDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(157, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXOFFHANDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(158, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_BYTES_1(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(159, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_PETNUMBER(mut self, v: i32) -> Self {
        self.header_set(160, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_PET_NAME_TIMESTAMP(mut self, v: i32) -> Self {
        self.header_set(161, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_PETEXPERIENCE(mut self, v: i32) -> Self {
        self.header_set(162, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_PETNEXTLEVELEXP(mut self, v: i32) -> Self {
        self.header_set(163, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_DYNAMIC_FLAGS(mut self, v: i32) -> Self {
        self.header_set(164, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_CHANNEL_SPELL(mut self, v: i32) -> Self {
        self.header_set(165, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MOD_CAST_SPEED(mut self, v: f32) -> Self {
        self.header_set(166, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_CREATED_BY_SPELL(mut self, v: i32) -> Self {
        self.header_set(167, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_NPC_FLAGS(mut self, v: i32) -> Self {
        self.header_set(168, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_NPC_EMOTESTATE(mut self, v: i32) -> Self {
        self.header_set(169, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_TRAINING_POINTS(mut self, a: u16, b: u16) -> Self {
        self.header_set(170, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_unit_STRENGTH(mut self, v: i32) -> Self {
        self.header_set(171, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_AGILITY(mut self, v: i32) -> Self {
        self.header_set(172, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_STAMINA(mut self, v: i32) -> Self {
        self.header_set(173, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_INTELLECT(mut self, v: i32) -> Self {
        self.header_set(174, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_SPIRIT(mut self, v: i32) -> Self {
        self.header_set(175, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_POSSTAT0(mut self, v: i32) -> Self {
        self.header_set(176, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_POSSTAT1(mut self, v: i32) -> Self {
        self.header_set(177, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_POSSTAT2(mut self, v: i32) -> Self {
        self.header_set(178, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_POSSTAT3(mut self, v: i32) -> Self {
        self.header_set(179, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_POSSTAT4(mut self, v: i32) -> Self {
        self.header_set(180, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_NEGSTAT0(mut self, v: i32) -> Self {
        self.header_set(181, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_NEGSTAT1(mut self, v: i32) -> Self {
        self.header_set(182, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_NEGSTAT2(mut self, v: i32) -> Self {
        self.header_set(183, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_NEGSTAT3(mut self, v: i32) -> Self {
        self.header_set(184, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_NEGSTAT4(mut self, v: i32) -> Self {
        self.header_set(185, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_RESISTANCES(mut self, v: i32) -> Self {
        self.header_set(186, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_RESISTANCEBUFFMODSPOSITIVE(mut self, v: i32) -> Self {
        self.header_set(193, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_RESISTANCEBUFFMODSNEGATIVE(mut self, v: i32) -> Self {
        self.header_set(200, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_BASE_MANA(mut self, v: i32) -> Self {
        self.header_set(207, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_BASE_HEALTH(mut self, v: i32) -> Self {
        self.header_set(208, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_BYTES_2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(209, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_ATTACK_POWER(mut self, v: i32) -> Self {
        self.header_set(210, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_ATTACK_POWER_MODS(mut self, a: u16, b: u16) -> Self {
        self.header_set(211, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_unit_ATTACK_POWER_MULTIPLIER(mut self, v: f32) -> Self {
        self.header_set(212, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_RANGED_ATTACK_POWER(mut self, v: i32) -> Self {
        self.header_set(213, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_RANGED_ATTACK_POWER_MODS(mut self, a: u16, b: u16) -> Self {
        self.header_set(214, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_unit_RANGED_ATTACK_POWER_MULTIPLIER(mut self, v: f32) -> Self {
        self.header_set(215, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MINRANGEDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(216, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXRANGEDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(217, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_POWER_COST_MODIFIER(mut self, v: i32) -> Self {
        self.header_set(218, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_POWER_COST_MULTIPLIER(mut self, v: f32) -> Self {
        self.header_set(225, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXHEALTHMODIFIER(mut self, v: f32) -> Self {
        self.header_set(232, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_DUEL_ARBITER(mut self, v: Guid) -> Self {
        self.header_set(234, v.guid() as u32);
        self.header_set(235, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FLAGS(mut self, v: i32) -> Self {
        self.header_set(236, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_GUILDID(mut self, v: i32) -> Self {
        self.header_set(237, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_GUILDRANK(mut self, v: i32) -> Self {
        self.header_set(238, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_FIELD_BYTES(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(239, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_BYTES_2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(240, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_BYTES_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(241, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_DUEL_TEAM(mut self, v: i32) -> Self {
        self.header_set(242, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_GUILD_TIMESTAMP(mut self, v: i32) -> Self {
        self.header_set(243, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_1_1(mut self, v: i32) -> Self {
        self.header_set(244, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_1_2(mut self, v: i32) -> Self {
        self.header_set(245, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_1_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(246, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_1_4(mut self, v: i32) -> Self {
        self.header_set(247, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_2_1(mut self, v: i32) -> Self {
        self.header_set(248, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_2_2(mut self, v: i32) -> Self {
        self.header_set(249, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_2_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(250, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_2_4(mut self, v: i32) -> Self {
        self.header_set(251, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_3_1(mut self, v: i32) -> Self {
        self.header_set(252, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_3_2(mut self, v: i32) -> Self {
        self.header_set(253, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_3_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(254, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_3_4(mut self, v: i32) -> Self {
        self.header_set(255, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_4_1(mut self, v: i32) -> Self {
        self.header_set(256, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_4_2(mut self, v: i32) -> Self {
        self.header_set(257, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_4_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(258, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_4_4(mut self, v: i32) -> Self {
        self.header_set(259, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_5_1(mut self, v: i32) -> Self {
        self.header_set(260, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_5_2(mut self, v: i32) -> Self {
        self.header_set(261, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_5_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(262, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_5_4(mut self, v: i32) -> Self {
        self.header_set(263, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_6_1(mut self, v: i32) -> Self {
        self.header_set(264, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_6_2(mut self, v: i32) -> Self {
        self.header_set(265, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_6_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(266, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_6_4(mut self, v: i32) -> Self {
        self.header_set(267, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_7_1(mut self, v: i32) -> Self {
        self.header_set(268, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_7_2(mut self, v: i32) -> Self {
        self.header_set(269, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_7_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(270, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_7_4(mut self, v: i32) -> Self {
        self.header_set(271, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_8_1(mut self, v: i32) -> Self {
        self.header_set(272, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_8_2(mut self, v: i32) -> Self {
        self.header_set(273, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_8_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(274, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_8_4(mut self, v: i32) -> Self {
        self.header_set(275, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_9_1(mut self, v: i32) -> Self {
        self.header_set(276, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_9_2(mut self, v: i32) -> Self {
        self.header_set(277, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_9_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(278, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_9_4(mut self, v: i32) -> Self {
        self.header_set(279, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_10_1(mut self, v: i32) -> Self {
        self.header_set(280, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_10_2(mut self, v: i32) -> Self {
        self.header_set(281, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_10_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(282, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_10_4(mut self, v: i32) -> Self {
        self.header_set(283, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_11_1(mut self, v: i32) -> Self {
        self.header_set(284, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_11_2(mut self, v: i32) -> Self {
        self.header_set(285, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_11_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(286, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_11_4(mut self, v: i32) -> Self {
        self.header_set(287, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_12_1(mut self, v: i32) -> Self {
        self.header_set(288, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_12_2(mut self, v: i32) -> Self {
        self.header_set(289, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_12_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(290, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_12_4(mut self, v: i32) -> Self {
        self.header_set(291, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_13_1(mut self, v: i32) -> Self {
        self.header_set(292, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_13_2(mut self, v: i32) -> Self {
        self.header_set(293, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_13_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(294, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_13_4(mut self, v: i32) -> Self {
        self.header_set(295, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_14_1(mut self, v: i32) -> Self {
        self.header_set(296, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_14_2(mut self, v: i32) -> Self {
        self.header_set(297, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_14_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(298, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_14_4(mut self, v: i32) -> Self {
        self.header_set(299, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_15_1(mut self, v: i32) -> Self {
        self.header_set(300, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_15_2(mut self, v: i32) -> Self {
        self.header_set(301, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_15_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(302, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_15_4(mut self, v: i32) -> Self {
        self.header_set(303, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_16_1(mut self, v: i32) -> Self {
        self.header_set(304, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_16_2(mut self, v: i32) -> Self {
        self.header_set(305, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_16_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(306, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_16_4(mut self, v: i32) -> Self {
        self.header_set(307, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_17_1(mut self, v: i32) -> Self {
        self.header_set(308, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_17_2(mut self, v: i32) -> Self {
        self.header_set(309, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_17_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(310, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_17_4(mut self, v: i32) -> Self {
        self.header_set(311, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_18_1(mut self, v: i32) -> Self {
        self.header_set(312, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_18_2(mut self, v: i32) -> Self {
        self.header_set(313, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_18_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(314, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_18_4(mut self, v: i32) -> Self {
        self.header_set(315, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_19_1(mut self, v: i32) -> Self {
        self.header_set(316, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_19_2(mut self, v: i32) -> Self {
        self.header_set(317, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_19_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(318, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_19_4(mut self, v: i32) -> Self {
        self.header_set(319, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_20_1(mut self, v: i32) -> Self {
        self.header_set(320, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_20_2(mut self, v: i32) -> Self {
        self.header_set(321, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_20_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(322, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_20_4(mut self, v: i32) -> Self {
        self.header_set(323, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_21_1(mut self, v: i32) -> Self {
        self.header_set(324, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_21_2(mut self, v: i32) -> Self {
        self.header_set(325, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_21_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(326, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_21_4(mut self, v: i32) -> Self {
        self.header_set(327, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_22_1(mut self, v: i32) -> Self {
        self.header_set(328, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_22_2(mut self, v: i32) -> Self {
        self.header_set(329, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_22_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(330, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_22_4(mut self, v: i32) -> Self {
        self.header_set(331, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_23_1(mut self, v: i32) -> Self {
        self.header_set(332, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_23_2(mut self, v: i32) -> Self {
        self.header_set(333, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_23_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(334, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_23_4(mut self, v: i32) -> Self {
        self.header_set(335, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_24_1(mut self, v: i32) -> Self {
        self.header_set(336, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_24_2(mut self, v: i32) -> Self {
        self.header_set(337, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_24_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(338, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_24_4(mut self, v: i32) -> Self {
        self.header_set(339, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_25_1(mut self, v: i32) -> Self {
        self.header_set(340, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_25_2(mut self, v: i32) -> Self {
        self.header_set(341, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_25_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(342, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_25_4(mut self, v: i32) -> Self {
        self.header_set(343, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_VISIBLE_ITEM_1_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(344, v.guid() as u32);
        self.header_set(345, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_1_0(mut self, v: i32) -> Self {
        self.header_set(346, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_VISIBLE_ITEM_1_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(358, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_VISIBLE_ITEM_2_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(360, v.guid() as u32);
        self.header_set(361, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_2_0(mut self, v: i32) -> Self {
        self.header_set(362, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_VISIBLE_ITEM_2_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(374, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_VISIBLE_ITEM_3_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(376, v.guid() as u32);
        self.header_set(377, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_3_0(mut self, v: i32) -> Self {
        self.header_set(378, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_VISIBLE_ITEM_3_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(390, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_VISIBLE_ITEM_4_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(392, v.guid() as u32);
        self.header_set(393, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_4_0(mut self, v: i32) -> Self {
        self.header_set(394, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_VISIBLE_ITEM_4_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(406, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_VISIBLE_ITEM_5_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(408, v.guid() as u32);
        self.header_set(409, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_5_0(mut self, v: i32) -> Self {
        self.header_set(410, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_VISIBLE_ITEM_5_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(422, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_VISIBLE_ITEM_6_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(424, v.guid() as u32);
        self.header_set(425, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_6_0(mut self, v: i32) -> Self {
        self.header_set(426, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_VISIBLE_ITEM_6_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(438, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_VISIBLE_ITEM_7_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(440, v.guid() as u32);
        self.header_set(441, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_7_0(mut self, v: i32) -> Self {
        self.header_set(442, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_VISIBLE_ITEM_7_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(454, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_VISIBLE_ITEM_8_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(456, v.guid() as u32);
        self.header_set(457, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_8_0(mut self, v: i32) -> Self {
        self.header_set(458, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_VISIBLE_ITEM_8_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(470, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_VISIBLE_ITEM_9_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(472, v.guid() as u32);
        self.header_set(473, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_9_0(mut self, v: i32) -> Self {
        self.header_set(474, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_VISIBLE_ITEM_9_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(486, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_VISIBLE_ITEM_10_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(488, v.guid() as u32);
        self.header_set(489, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_10_0(mut self, v: i32) -> Self {
        self.header_set(490, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_VISIBLE_ITEM_10_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(502, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_VISIBLE_ITEM_11_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(504, v.guid() as u32);
        self.header_set(505, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_11_0(mut self, v: i32) -> Self {
        self.header_set(506, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_VISIBLE_ITEM_11_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(518, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_VISIBLE_ITEM_12_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(520, v.guid() as u32);
        self.header_set(521, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_12_0(mut self, v: i32) -> Self {
        self.header_set(522, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_VISIBLE_ITEM_12_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(534, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_VISIBLE_ITEM_13_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(536, v.guid() as u32);
        self.header_set(537, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_13_0(mut self, v: i32) -> Self {
        self.header_set(538, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_VISIBLE_ITEM_13_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(550, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_VISIBLE_ITEM_14_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(552, v.guid() as u32);
        self.header_set(553, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_14_0(mut self, v: i32) -> Self {
        self.header_set(554, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_VISIBLE_ITEM_14_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(566, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_VISIBLE_ITEM_15_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(568, v.guid() as u32);
        self.header_set(569, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_15_0(mut self, v: i32) -> Self {
        self.header_set(570, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_VISIBLE_ITEM_15_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(582, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_VISIBLE_ITEM_16_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(584, v.guid() as u32);
        self.header_set(585, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_16_0(mut self, v: i32) -> Self {
        self.header_set(586, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_VISIBLE_ITEM_16_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(598, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_VISIBLE_ITEM_17_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(600, v.guid() as u32);
        self.header_set(601, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_17_0(mut self, v: i32) -> Self {
        self.header_set(602, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_VISIBLE_ITEM_17_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(614, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_VISIBLE_ITEM_18_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(616, v.guid() as u32);
        self.header_set(617, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_18_0(mut self, v: i32) -> Self {
        self.header_set(618, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_VISIBLE_ITEM_18_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(630, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_VISIBLE_ITEM_19_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(632, v.guid() as u32);
        self.header_set(633, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_19_0(mut self, v: i32) -> Self {
        self.header_set(634, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_VISIBLE_ITEM_19_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(646, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_CHOSEN_TITLE(mut self, v: i32) -> Self {
        self.header_set(648, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_INV_SLOT_HEAD(mut self, v: Guid) -> Self {
        self.header_set(650, v.guid() as u32);
        self.header_set(651, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_INV_SLOT_NECK(mut self, v: Guid) -> Self {
        self.header_set(652, v.guid() as u32);
        self.header_set(653, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_INV_SLOT_SHOULDERS(mut self, v: Guid) -> Self {
        self.header_set(654, v.guid() as u32);
        self.header_set(655, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_INV_SLOT_BODY(mut self, v: Guid) -> Self {
        self.header_set(656, v.guid() as u32);
        self.header_set(657, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_INV_SLOT_CHEST(mut self, v: Guid) -> Self {
        self.header_set(658, v.guid() as u32);
        self.header_set(659, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_INV_SLOT_WAIST(mut self, v: Guid) -> Self {
        self.header_set(660, v.guid() as u32);
        self.header_set(661, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_INV_SLOT_LEGS(mut self, v: Guid) -> Self {
        self.header_set(662, v.guid() as u32);
        self.header_set(663, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_INV_SLOT_FEET(mut self, v: Guid) -> Self {
        self.header_set(664, v.guid() as u32);
        self.header_set(665, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_INV_SLOT_WRISTS(mut self, v: Guid) -> Self {
        self.header_set(666, v.guid() as u32);
        self.header_set(667, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_INV_SLOT_HANDS(mut self, v: Guid) -> Self {
        self.header_set(668, v.guid() as u32);
        self.header_set(669, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_INV_SLOT_FINGER1(mut self, v: Guid) -> Self {
        self.header_set(670, v.guid() as u32);
        self.header_set(671, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_INV_SLOT_FINGER2(mut self, v: Guid) -> Self {
        self.header_set(672, v.guid() as u32);
        self.header_set(673, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_INV_SLOT_TRINKET1(mut self, v: Guid) -> Self {
        self.header_set(674, v.guid() as u32);
        self.header_set(675, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_INV_SLOT_TRINKET2(mut self, v: Guid) -> Self {
        self.header_set(676, v.guid() as u32);
        self.header_set(677, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_INV_SLOT_BACK(mut self, v: Guid) -> Self {
        self.header_set(678, v.guid() as u32);
        self.header_set(679, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_INV_SLOT_MAIN_HAND(mut self, v: Guid) -> Self {
        self.header_set(680, v.guid() as u32);
        self.header_set(681, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_INV_SLOT_OFF_HAND(mut self, v: Guid) -> Self {
        self.header_set(682, v.guid() as u32);
        self.header_set(683, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_INV_SLOT_RANGED(mut self, v: Guid) -> Self {
        self.header_set(684, v.guid() as u32);
        self.header_set(685, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_INV_SLOT_TABARD(mut self, v: Guid) -> Self {
        self.header_set(686, v.guid() as u32);
        self.header_set(687, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_INV_SLOT_BAG1(mut self, v: Guid) -> Self {
        self.header_set(688, v.guid() as u32);
        self.header_set(689, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_INV_SLOT_BAG2(mut self, v: Guid) -> Self {
        self.header_set(690, v.guid() as u32);
        self.header_set(691, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_INV_SLOT_BAG3(mut self, v: Guid) -> Self {
        self.header_set(692, v.guid() as u32);
        self.header_set(693, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_INV_SLOT_BAG4(mut self, v: Guid) -> Self {
        self.header_set(694, v.guid() as u32);
        self.header_set(695, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_PACK_SLOT_1(mut self, v: Guid) -> Self {
        self.header_set(696, v.guid() as u32);
        self.header_set(697, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_PACK_SLOT_2(mut self, v: Guid) -> Self {
        self.header_set(698, v.guid() as u32);
        self.header_set(699, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_PACK_SLOT_3(mut self, v: Guid) -> Self {
        self.header_set(700, v.guid() as u32);
        self.header_set(701, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_PACK_SLOT_4(mut self, v: Guid) -> Self {
        self.header_set(702, v.guid() as u32);
        self.header_set(703, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_PACK_SLOT_5(mut self, v: Guid) -> Self {
        self.header_set(704, v.guid() as u32);
        self.header_set(705, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_PACK_SLOT_6(mut self, v: Guid) -> Self {
        self.header_set(706, v.guid() as u32);
        self.header_set(707, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_PACK_SLOT_7(mut self, v: Guid) -> Self {
        self.header_set(708, v.guid() as u32);
        self.header_set(709, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_PACK_SLOT_8(mut self, v: Guid) -> Self {
        self.header_set(710, v.guid() as u32);
        self.header_set(711, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_PACK_SLOT_9(mut self, v: Guid) -> Self {
        self.header_set(712, v.guid() as u32);
        self.header_set(713, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_PACK_SLOT_10(mut self, v: Guid) -> Self {
        self.header_set(714, v.guid() as u32);
        self.header_set(715, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_PACK_SLOT_11(mut self, v: Guid) -> Self {
        self.header_set(716, v.guid() as u32);
        self.header_set(717, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_PACK_SLOT_12(mut self, v: Guid) -> Self {
        self.header_set(718, v.guid() as u32);
        self.header_set(719, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_PACK_SLOT_13(mut self, v: Guid) -> Self {
        self.header_set(720, v.guid() as u32);
        self.header_set(721, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_PACK_SLOT_14(mut self, v: Guid) -> Self {
        self.header_set(722, v.guid() as u32);
        self.header_set(723, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_PACK_SLOT_15(mut self, v: Guid) -> Self {
        self.header_set(724, v.guid() as u32);
        self.header_set(725, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_PACK_SLOT_16(mut self, v: Guid) -> Self {
        self.header_set(726, v.guid() as u32);
        self.header_set(727, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_BANK_SLOT_1(mut self, v: Guid) -> Self {
        self.header_set(728, v.guid() as u32);
        self.header_set(729, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_BANK_SLOT_2(mut self, v: Guid) -> Self {
        self.header_set(730, v.guid() as u32);
        self.header_set(731, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_BANK_SLOT_3(mut self, v: Guid) -> Self {
        self.header_set(732, v.guid() as u32);
        self.header_set(733, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_BANK_SLOT_4(mut self, v: Guid) -> Self {
        self.header_set(734, v.guid() as u32);
        self.header_set(735, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_BANK_SLOT_5(mut self, v: Guid) -> Self {
        self.header_set(736, v.guid() as u32);
        self.header_set(737, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_BANK_SLOT_6(mut self, v: Guid) -> Self {
        self.header_set(738, v.guid() as u32);
        self.header_set(739, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_BANK_SLOT_7(mut self, v: Guid) -> Self {
        self.header_set(740, v.guid() as u32);
        self.header_set(741, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_BANK_SLOT_8(mut self, v: Guid) -> Self {
        self.header_set(742, v.guid() as u32);
        self.header_set(743, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_BANK_SLOT_9(mut self, v: Guid) -> Self {
        self.header_set(744, v.guid() as u32);
        self.header_set(745, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_BANK_SLOT_10(mut self, v: Guid) -> Self {
        self.header_set(746, v.guid() as u32);
        self.header_set(747, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_BANK_SLOT_11(mut self, v: Guid) -> Self {
        self.header_set(748, v.guid() as u32);
        self.header_set(749, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_BANK_SLOT_12(mut self, v: Guid) -> Self {
        self.header_set(750, v.guid() as u32);
        self.header_set(751, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_BANK_SLOT_13(mut self, v: Guid) -> Self {
        self.header_set(752, v.guid() as u32);
        self.header_set(753, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_BANK_SLOT_14(mut self, v: Guid) -> Self {
        self.header_set(754, v.guid() as u32);
        self.header_set(755, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_BANK_SLOT_15(mut self, v: Guid) -> Self {
        self.header_set(756, v.guid() as u32);
        self.header_set(757, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_BANK_SLOT_16(mut self, v: Guid) -> Self {
        self.header_set(758, v.guid() as u32);
        self.header_set(759, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_BANK_SLOT_17(mut self, v: Guid) -> Self {
        self.header_set(760, v.guid() as u32);
        self.header_set(761, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_BANK_SLOT_18(mut self, v: Guid) -> Self {
        self.header_set(762, v.guid() as u32);
        self.header_set(763, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_BANK_SLOT_19(mut self, v: Guid) -> Self {
        self.header_set(764, v.guid() as u32);
        self.header_set(765, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_BANK_SLOT_20(mut self, v: Guid) -> Self {
        self.header_set(766, v.guid() as u32);
        self.header_set(767, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_BANK_SLOT_21(mut self, v: Guid) -> Self {
        self.header_set(768, v.guid() as u32);
        self.header_set(769, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_BANK_SLOT_22(mut self, v: Guid) -> Self {
        self.header_set(770, v.guid() as u32);
        self.header_set(771, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_BANK_SLOT_23(mut self, v: Guid) -> Self {
        self.header_set(772, v.guid() as u32);
        self.header_set(773, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_BANK_SLOT_24(mut self, v: Guid) -> Self {
        self.header_set(774, v.guid() as u32);
        self.header_set(775, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_BANK_SLOT_25(mut self, v: Guid) -> Self {
        self.header_set(776, v.guid() as u32);
        self.header_set(777, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_BANK_SLOT_26(mut self, v: Guid) -> Self {
        self.header_set(778, v.guid() as u32);
        self.header_set(779, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_BANK_SLOT_27(mut self, v: Guid) -> Self {
        self.header_set(780, v.guid() as u32);
        self.header_set(781, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_BANK_SLOT_28(mut self, v: Guid) -> Self {
        self.header_set(782, v.guid() as u32);
        self.header_set(783, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_BANKBAG_SLOT_1(mut self, v: Guid) -> Self {
        self.header_set(784, v.guid() as u32);
        self.header_set(785, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_BANKBAG_SLOT_2(mut self, v: Guid) -> Self {
        self.header_set(786, v.guid() as u32);
        self.header_set(787, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_BANKBAG_SLOT_3(mut self, v: Guid) -> Self {
        self.header_set(788, v.guid() as u32);
        self.header_set(789, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_BANKBAG_SLOT_4(mut self, v: Guid) -> Self {
        self.header_set(790, v.guid() as u32);
        self.header_set(791, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_BANKBAG_SLOT_5(mut self, v: Guid) -> Self {
        self.header_set(792, v.guid() as u32);
        self.header_set(793, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_BANKBAG_SLOT_6(mut self, v: Guid) -> Self {
        self.header_set(794, v.guid() as u32);
        self.header_set(795, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_BANKBAG_SLOT_7(mut self, v: Guid) -> Self {
        self.header_set(796, v.guid() as u32);
        self.header_set(797, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VENDORBUYBACK_SLOT_1(mut self, v: Guid) -> Self {
        self.header_set(798, v.guid() as u32);
        self.header_set(799, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VENDORBUYBACK_SLOT_2(mut self, v: Guid) -> Self {
        self.header_set(800, v.guid() as u32);
        self.header_set(801, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VENDORBUYBACK_SLOT_3(mut self, v: Guid) -> Self {
        self.header_set(802, v.guid() as u32);
        self.header_set(803, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VENDORBUYBACK_SLOT_4(mut self, v: Guid) -> Self {
        self.header_set(804, v.guid() as u32);
        self.header_set(805, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VENDORBUYBACK_SLOT_5(mut self, v: Guid) -> Self {
        self.header_set(806, v.guid() as u32);
        self.header_set(807, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VENDORBUYBACK_SLOT_6(mut self, v: Guid) -> Self {
        self.header_set(808, v.guid() as u32);
        self.header_set(809, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VENDORBUYBACK_SLOT_7(mut self, v: Guid) -> Self {
        self.header_set(810, v.guid() as u32);
        self.header_set(811, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VENDORBUYBACK_SLOT_8(mut self, v: Guid) -> Self {
        self.header_set(812, v.guid() as u32);
        self.header_set(813, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VENDORBUYBACK_SLOT_9(mut self, v: Guid) -> Self {
        self.header_set(814, v.guid() as u32);
        self.header_set(815, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VENDORBUYBACK_SLOT_10(mut self, v: Guid) -> Self {
        self.header_set(816, v.guid() as u32);
        self.header_set(817, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VENDORBUYBACK_SLOT_11(mut self, v: Guid) -> Self {
        self.header_set(818, v.guid() as u32);
        self.header_set(819, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VENDORBUYBACK_SLOT_12(mut self, v: Guid) -> Self {
        self.header_set(820, v.guid() as u32);
        self.header_set(821, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_KEYRING_SLOT_1(mut self, v: Guid) -> Self {
        self.header_set(822, v.guid() as u32);
        self.header_set(823, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_KEYRING_SLOT_2(mut self, v: Guid) -> Self {
        self.header_set(824, v.guid() as u32);
        self.header_set(825, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_KEYRING_SLOT_3(mut self, v: Guid) -> Self {
        self.header_set(826, v.guid() as u32);
        self.header_set(827, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_KEYRING_SLOT_4(mut self, v: Guid) -> Self {
        self.header_set(828, v.guid() as u32);
        self.header_set(829, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_KEYRING_SLOT_5(mut self, v: Guid) -> Self {
        self.header_set(830, v.guid() as u32);
        self.header_set(831, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_KEYRING_SLOT_6(mut self, v: Guid) -> Self {
        self.header_set(832, v.guid() as u32);
        self.header_set(833, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_KEYRING_SLOT_7(mut self, v: Guid) -> Self {
        self.header_set(834, v.guid() as u32);
        self.header_set(835, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_KEYRING_SLOT_8(mut self, v: Guid) -> Self {
        self.header_set(836, v.guid() as u32);
        self.header_set(837, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_KEYRING_SLOT_9(mut self, v: Guid) -> Self {
        self.header_set(838, v.guid() as u32);
        self.header_set(839, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_KEYRING_SLOT_10(mut self, v: Guid) -> Self {
        self.header_set(840, v.guid() as u32);
        self.header_set(841, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_KEYRING_SLOT_11(mut self, v: Guid) -> Self {
        self.header_set(842, v.guid() as u32);
        self.header_set(843, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_KEYRING_SLOT_12(mut self, v: Guid) -> Self {
        self.header_set(844, v.guid() as u32);
        self.header_set(845, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_KEYRING_SLOT_13(mut self, v: Guid) -> Self {
        self.header_set(846, v.guid() as u32);
        self.header_set(847, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_KEYRING_SLOT_14(mut self, v: Guid) -> Self {
        self.header_set(848, v.guid() as u32);
        self.header_set(849, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_KEYRING_SLOT_15(mut self, v: Guid) -> Self {
        self.header_set(850, v.guid() as u32);
        self.header_set(851, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_KEYRING_SLOT_16(mut self, v: Guid) -> Self {
        self.header_set(852, v.guid() as u32);
        self.header_set(853, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_KEYRING_SLOT_17(mut self, v: Guid) -> Self {
        self.header_set(854, v.guid() as u32);
        self.header_set(855, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_KEYRING_SLOT_18(mut self, v: Guid) -> Self {
        self.header_set(856, v.guid() as u32);
        self.header_set(857, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_KEYRING_SLOT_19(mut self, v: Guid) -> Self {
        self.header_set(858, v.guid() as u32);
        self.header_set(859, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_KEYRING_SLOT_20(mut self, v: Guid) -> Self {
        self.header_set(860, v.guid() as u32);
        self.header_set(861, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_KEYRING_SLOT_21(mut self, v: Guid) -> Self {
        self.header_set(862, v.guid() as u32);
        self.header_set(863, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_KEYRING_SLOT_22(mut self, v: Guid) -> Self {
        self.header_set(864, v.guid() as u32);
        self.header_set(865, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_KEYRING_SLOT_23(mut self, v: Guid) -> Self {
        self.header_set(866, v.guid() as u32);
        self.header_set(867, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_KEYRING_SLOT_24(mut self, v: Guid) -> Self {
        self.header_set(868, v.guid() as u32);
        self.header_set(869, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_KEYRING_SLOT_25(mut self, v: Guid) -> Self {
        self.header_set(870, v.guid() as u32);
        self.header_set(871, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_KEYRING_SLOT_26(mut self, v: Guid) -> Self {
        self.header_set(872, v.guid() as u32);
        self.header_set(873, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_KEYRING_SLOT_27(mut self, v: Guid) -> Self {
        self.header_set(874, v.guid() as u32);
        self.header_set(875, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_KEYRING_SLOT_28(mut self, v: Guid) -> Self {
        self.header_set(876, v.guid() as u32);
        self.header_set(877, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_KEYRING_SLOT_29(mut self, v: Guid) -> Self {
        self.header_set(878, v.guid() as u32);
        self.header_set(879, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_KEYRING_SLOT_30(mut self, v: Guid) -> Self {
        self.header_set(880, v.guid() as u32);
        self.header_set(881, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_KEYRING_SLOT_31(mut self, v: Guid) -> Self {
        self.header_set(882, v.guid() as u32);
        self.header_set(883, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_KEYRING_SLOT_32(mut self, v: Guid) -> Self {
        self.header_set(884, v.guid() as u32);
        self.header_set(885, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VANITYPET_SLOT_1(mut self, v: Guid) -> Self {
        self.header_set(886, v.guid() as u32);
        self.header_set(887, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VANITYPET_SLOT_2(mut self, v: Guid) -> Self {
        self.header_set(888, v.guid() as u32);
        self.header_set(889, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VANITYPET_SLOT_3(mut self, v: Guid) -> Self {
        self.header_set(890, v.guid() as u32);
        self.header_set(891, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VANITYPET_SLOT_4(mut self, v: Guid) -> Self {
        self.header_set(892, v.guid() as u32);
        self.header_set(893, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VANITYPET_SLOT_5(mut self, v: Guid) -> Self {
        self.header_set(894, v.guid() as u32);
        self.header_set(895, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VANITYPET_SLOT_6(mut self, v: Guid) -> Self {
        self.header_set(896, v.guid() as u32);
        self.header_set(897, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VANITYPET_SLOT_7(mut self, v: Guid) -> Self {
        self.header_set(898, v.guid() as u32);
        self.header_set(899, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VANITYPET_SLOT_8(mut self, v: Guid) -> Self {
        self.header_set(900, v.guid() as u32);
        self.header_set(901, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VANITYPET_SLOT_9(mut self, v: Guid) -> Self {
        self.header_set(902, v.guid() as u32);
        self.header_set(903, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VANITYPET_SLOT_10(mut self, v: Guid) -> Self {
        self.header_set(904, v.guid() as u32);
        self.header_set(905, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VANITYPET_SLOT_11(mut self, v: Guid) -> Self {
        self.header_set(906, v.guid() as u32);
        self.header_set(907, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VANITYPET_SLOT_12(mut self, v: Guid) -> Self {
        self.header_set(908, v.guid() as u32);
        self.header_set(909, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VANITYPET_SLOT_13(mut self, v: Guid) -> Self {
        self.header_set(910, v.guid() as u32);
        self.header_set(911, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VANITYPET_SLOT_14(mut self, v: Guid) -> Self {
        self.header_set(912, v.guid() as u32);
        self.header_set(913, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VANITYPET_SLOT_15(mut self, v: Guid) -> Self {
        self.header_set(914, v.guid() as u32);
        self.header_set(915, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VANITYPET_SLOT_16(mut self, v: Guid) -> Self {
        self.header_set(916, v.guid() as u32);
        self.header_set(917, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VANITYPET_SLOT_17(mut self, v: Guid) -> Self {
        self.header_set(918, v.guid() as u32);
        self.header_set(919, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VANITYPET_SLOT_18(mut self, v: Guid) -> Self {
        self.header_set(920, v.guid() as u32);
        self.header_set(921, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FARSIGHT(mut self, v: Guid) -> Self {
        self.header_set(922, v.guid() as u32);
        self.header_set(923, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_KNOWN_TITLES(mut self, v: Guid) -> Self {
        self.header_set(924, v.guid() as u32);
        self.header_set(925, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_XP(mut self, v: i32) -> Self {
        self.header_set(926, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_NEXT_LEVEL_XP(mut self, v: i32) -> Self {
        self.header_set(927, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_SKILL_INFO(mut self, skill_info: crate::tbc::SkillInfo, index: SkillInfoIndex) -> Self {
        for (index, value) in skill_info.mask_values(index) {
            self.header_set(index, value);
        }
        self
    }

    pub fn set_player_CHARACTER_POINTS1(mut self, v: i32) -> Self {
        self.header_set(1312, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_CHARACTER_POINTS2(mut self, v: i32) -> Self {
        self.header_set(1313, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_TRACK_CREATURES(mut self, v: i32) -> Self {
        self.header_set(1314, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_TRACK_RESOURCES(mut self, v: i32) -> Self {
        self.header_set(1315, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_BLOCK_PERCENTAGE(mut self, v: f32) -> Self {
        self.header_set(1316, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_DODGE_PERCENTAGE(mut self, v: f32) -> Self {
        self.header_set(1317, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_PARRY_PERCENTAGE(mut self, v: f32) -> Self {
        self.header_set(1318, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_EXPERTISE(mut self, v: i32) -> Self {
        self.header_set(1319, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_OFFHAND_EXPERTISE(mut self, v: i32) -> Self {
        self.header_set(1320, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_CRIT_PERCENTAGE(mut self, v: f32) -> Self {
        self.header_set(1321, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_RANGED_CRIT_PERCENTAGE(mut self, v: f32) -> Self {
        self.header_set(1322, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_OFFHAND_CRIT_PERCENTAGE(mut self, v: f32) -> Self {
        self.header_set(1323, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_SPELL_CRIT_PERCENTAGE1(mut self, v: f32) -> Self {
        self.header_set(1324, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_SHIELD_BLOCK(mut self, v: i32) -> Self {
        self.header_set(1331, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_EXPLORED_ZONES_1(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(1332, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_REST_STATE_EXPERIENCE(mut self, v: i32) -> Self {
        self.header_set(1460, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_COINAGE(mut self, v: i32) -> Self {
        self.header_set(1461, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_MOD_DAMAGE_DONE_POS(mut self, v: i32) -> Self {
        self.header_set(1462, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_MOD_DAMAGE_DONE_NEG(mut self, v: i32) -> Self {
        self.header_set(1469, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_MOD_DAMAGE_DONE_PCT(mut self, v: i32) -> Self {
        self.header_set(1476, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_MOD_HEALING_DONE_POS(mut self, v: i32) -> Self {
        self.header_set(1483, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_MOD_TARGET_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(1484, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_MOD_TARGET_PHYSICAL_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(1485, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_FEATURES(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(1486, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_AMMO_ID(mut self, v: i32) -> Self {
        self.header_set(1487, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_SELF_RES_SPELL(mut self, v: i32) -> Self {
        self.header_set(1488, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_PVP_MEDALS(mut self, v: i32) -> Self {
        self.header_set(1489, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_BUYBACK_PRICE_1(mut self, v: i32) -> Self {
        self.header_set(1490, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_BUYBACK_TIMESTAMP_1(mut self, v: i32) -> Self {
        self.header_set(1502, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_KILLS(mut self, a: u16, b: u16) -> Self {
        self.header_set(1514, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_TODAY_CONTRIBUTION(mut self, v: i32) -> Self {
        self.header_set(1515, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_YESTERDAY_CONTRIBUTION(mut self, v: i32) -> Self {
        self.header_set(1516, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_LIFETIME_HONORABLE_KILLS(mut self, v: i32) -> Self {
        self.header_set(1517, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_BYTES2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(1518, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_WATCHED_FACTION_INDEX(mut self, v: i32) -> Self {
        self.header_set(1519, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_COMBAT_RATING_1(mut self, v: i32) -> Self {
        self.header_set(1520, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_ARENA_TEAM_INFO_1_1(mut self, v: i32) -> Self {
        self.header_set(1544, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_HONOR_CURRENCY(mut self, v: i32) -> Self {
        self.header_set(1562, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_ARENA_CURRENCY(mut self, v: i32) -> Self {
        self.header_set(1563, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_MOD_MANA_REGEN(mut self, v: f32) -> Self {
        self.header_set(1564, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_MOD_MANA_REGEN_INTERRUPT(mut self, v: f32) -> Self {
        self.header_set(1565, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_MAX_LEVEL(mut self, v: i32) -> Self {
        self.header_set(1566, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_DAILY_QUESTS_1(mut self, v: i32) -> Self {
        self.header_set(1567, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_CREATED_BY(mut self, v: Guid) -> Self {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
        self
    }

}

impl UpdateGameObjectBuilder {
    pub fn set_object_GUID(mut self, v: Guid) -> Self {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_TYPE(mut self, v: i32) -> Self {
        self.header_set(2, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_ENTRY(mut self, v: i32) -> Self {
        self.header_set(3, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_SCALE_X(mut self, v: f32) -> Self {
        self.header_set(4, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_CREATED_BY(mut self, v: Guid) -> Self {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_gameobject_DISPLAYID(mut self, v: i32) -> Self {
        self.header_set(8, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_FLAGS(mut self, v: i32) -> Self {
        self.header_set(9, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_ROTATION(mut self, v: f32) -> Self {
        self.header_set(10, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_STATE(mut self, v: i32) -> Self {
        self.header_set(14, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_POS_X(mut self, v: f32) -> Self {
        self.header_set(15, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_POS_Y(mut self, v: f32) -> Self {
        self.header_set(16, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_POS_Z(mut self, v: f32) -> Self {
        self.header_set(17, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_FACING(mut self, v: f32) -> Self {
        self.header_set(18, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_DYN_FLAGS(mut self, v: i32) -> Self {
        self.header_set(19, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_FACTION(mut self, v: i32) -> Self {
        self.header_set(20, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_TYPE_ID(mut self, v: i32) -> Self {
        self.header_set(21, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_LEVEL(mut self, v: i32) -> Self {
        self.header_set(22, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_ARTKIT(mut self, v: i32) -> Self {
        self.header_set(23, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_ANIMPROGRESS(mut self, v: i32) -> Self {
        self.header_set(24, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

}

impl UpdateDynamicObjectBuilder {
    pub fn set_object_GUID(mut self, v: Guid) -> Self {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_TYPE(mut self, v: i32) -> Self {
        self.header_set(2, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_ENTRY(mut self, v: i32) -> Self {
        self.header_set(3, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_SCALE_X(mut self, v: f32) -> Self {
        self.header_set(4, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_CREATED_BY(mut self, v: Guid) -> Self {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_dynamicobject_CASTER(mut self, v: Guid) -> Self {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_dynamicobject_BYTES(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(8, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_dynamicobject_SPELLID(mut self, v: i32) -> Self {
        self.header_set(9, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_dynamicobject_RADIUS(mut self, v: f32) -> Self {
        self.header_set(10, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_dynamicobject_POS_X(mut self, v: f32) -> Self {
        self.header_set(11, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_dynamicobject_POS_Y(mut self, v: f32) -> Self {
        self.header_set(12, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_dynamicobject_POS_Z(mut self, v: f32) -> Self {
        self.header_set(13, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_dynamicobject_FACING(mut self, v: f32) -> Self {
        self.header_set(14, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_dynamicobject_CASTTIME(mut self, v: i32) -> Self {
        self.header_set(15, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

}

impl UpdateCorpseBuilder {
    pub fn set_object_GUID(mut self, v: Guid) -> Self {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_TYPE(mut self, v: i32) -> Self {
        self.header_set(2, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_ENTRY(mut self, v: i32) -> Self {
        self.header_set(3, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_SCALE_X(mut self, v: f32) -> Self {
        self.header_set(4, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_CREATED_BY(mut self, v: Guid) -> Self {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_corpse_OWNER(mut self, v: Guid) -> Self {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_corpse_PARTY(mut self, v: Guid) -> Self {
        self.header_set(8, v.guid() as u32);
        self.header_set(9, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_corpse_FACING(mut self, v: f32) -> Self {
        self.header_set(10, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_corpse_POS_X(mut self, v: f32) -> Self {
        self.header_set(11, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_corpse_POS_Y(mut self, v: f32) -> Self {
        self.header_set(12, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_corpse_POS_Z(mut self, v: f32) -> Self {
        self.header_set(13, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_corpse_DISPLAY_ID(mut self, v: i32) -> Self {
        self.header_set(14, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_corpse_ITEM(mut self, v: i32) -> Self {
        self.header_set(15, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_corpse_BYTES_1(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(34, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_corpse_BYTES_2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(35, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_corpse_GUILD(mut self, v: i32) -> Self {
        self.header_set(36, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_corpse_FLAGS(mut self, v: i32) -> Self {
        self.header_set(37, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_corpse_DYNAMIC_FLAGS(mut self, v: i32) -> Self {
        self.header_set(38, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

}

impl UpdateItem {
    pub fn set_object_GUID(&mut self, v: Guid) {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
    }

    pub fn object_GUID(&self) -> Option<Guid> {
        let lower = self.values.get(&0);
        let upper = self.values.get(&1);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_object_TYPE(&mut self, v: i32) {
        self.header_set(2, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_TYPE(&self) -> Option<i32> {
        self.values.get(&2).map(|v| *v as i32)
    }

    pub fn set_object_ENTRY(&mut self, v: i32) {
        self.header_set(3, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_ENTRY(&self) -> Option<i32> {
        self.values.get(&3).map(|v| *v as i32)
    }

    pub fn set_object_SCALE_X(&mut self, v: f32) {
        self.header_set(4, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_SCALE_X(&self) -> Option<f32> {
        self.values.get(&4).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_item_OWNER(&mut self, v: Guid) {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
    }

    pub fn item_OWNER(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_item_CONTAINED(&mut self, v: Guid) {
        self.header_set(8, v.guid() as u32);
        self.header_set(9, (v.guid() >> 32) as u32);
    }

    pub fn item_CONTAINED(&self) -> Option<Guid> {
        let lower = self.values.get(&8);
        let upper = self.values.get(&9);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_item_CREATOR(&mut self, v: Guid) {
        self.header_set(10, v.guid() as u32);
        self.header_set(11, (v.guid() >> 32) as u32);
    }

    pub fn item_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&10);
        let upper = self.values.get(&11);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_item_GIFTCREATOR(&mut self, v: Guid) {
        self.header_set(12, v.guid() as u32);
        self.header_set(13, (v.guid() >> 32) as u32);
    }

    pub fn item_GIFTCREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&12);
        let upper = self.values.get(&13);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_item_STACK_COUNT(&mut self, v: i32) {
        self.header_set(14, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_STACK_COUNT(&self) -> Option<i32> {
        self.values.get(&14).map(|v| *v as i32)
    }

    pub fn set_item_DURATION(&mut self, v: i32) {
        self.header_set(15, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_DURATION(&self) -> Option<i32> {
        self.values.get(&15).map(|v| *v as i32)
    }

    pub fn set_item_SPELL_CHARGES(&mut self, v: i32) {
        self.header_set(16, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_SPELL_CHARGES(&self) -> Option<i32> {
        self.values.get(&16).map(|v| *v as i32)
    }

    pub fn set_item_FLAGS(&mut self, v: i32) {
        self.header_set(21, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_FLAGS(&self) -> Option<i32> {
        self.values.get(&21).map(|v| *v as i32)
    }

    pub fn set_item_ENCHANTMENT_1_1(&mut self, v: i32) {
        self.header_set(22, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_ENCHANTMENT_1_1(&self) -> Option<i32> {
        self.values.get(&22).map(|v| *v as i32)
    }

    pub fn set_item_PROPERTY_SEED(&mut self, v: i32) {
        self.header_set(55, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_PROPERTY_SEED(&self) -> Option<i32> {
        self.values.get(&55).map(|v| *v as i32)
    }

    pub fn set_item_RANDOM_PROPERTIES_ID(&mut self, v: i32) {
        self.header_set(56, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_RANDOM_PROPERTIES_ID(&self) -> Option<i32> {
        self.values.get(&56).map(|v| *v as i32)
    }

    pub fn set_item_ITEM_TEXT_ID(&mut self, v: i32) {
        self.header_set(57, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_ITEM_TEXT_ID(&self) -> Option<i32> {
        self.values.get(&57).map(|v| *v as i32)
    }

    pub fn set_item_DURABILITY(&mut self, v: i32) {
        self.header_set(58, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_DURABILITY(&self) -> Option<i32> {
        self.values.get(&58).map(|v| *v as i32)
    }

    pub fn set_item_MAXDURABILITY(&mut self, v: i32) {
        self.header_set(59, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_MAXDURABILITY(&self) -> Option<i32> {
        self.values.get(&59).map(|v| *v as i32)
    }

    pub fn set_object_CREATED_BY(&mut self, v: Guid) {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
    }

    pub fn object_CREATED_BY(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

}

impl UpdateContainer {
    pub fn set_object_GUID(&mut self, v: Guid) {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
    }

    pub fn object_GUID(&self) -> Option<Guid> {
        let lower = self.values.get(&0);
        let upper = self.values.get(&1);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_object_TYPE(&mut self, v: i32) {
        self.header_set(2, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_TYPE(&self) -> Option<i32> {
        self.values.get(&2).map(|v| *v as i32)
    }

    pub fn set_object_ENTRY(&mut self, v: i32) {
        self.header_set(3, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_ENTRY(&self) -> Option<i32> {
        self.values.get(&3).map(|v| *v as i32)
    }

    pub fn set_object_SCALE_X(&mut self, v: f32) {
        self.header_set(4, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_SCALE_X(&self) -> Option<f32> {
        self.values.get(&4).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_item_OWNER(&mut self, v: Guid) {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
    }

    pub fn item_OWNER(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_item_CONTAINED(&mut self, v: Guid) {
        self.header_set(8, v.guid() as u32);
        self.header_set(9, (v.guid() >> 32) as u32);
    }

    pub fn item_CONTAINED(&self) -> Option<Guid> {
        let lower = self.values.get(&8);
        let upper = self.values.get(&9);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_item_CREATOR(&mut self, v: Guid) {
        self.header_set(10, v.guid() as u32);
        self.header_set(11, (v.guid() >> 32) as u32);
    }

    pub fn item_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&10);
        let upper = self.values.get(&11);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_item_GIFTCREATOR(&mut self, v: Guid) {
        self.header_set(12, v.guid() as u32);
        self.header_set(13, (v.guid() >> 32) as u32);
    }

    pub fn item_GIFTCREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&12);
        let upper = self.values.get(&13);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_item_STACK_COUNT(&mut self, v: i32) {
        self.header_set(14, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_STACK_COUNT(&self) -> Option<i32> {
        self.values.get(&14).map(|v| *v as i32)
    }

    pub fn set_item_DURATION(&mut self, v: i32) {
        self.header_set(15, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_DURATION(&self) -> Option<i32> {
        self.values.get(&15).map(|v| *v as i32)
    }

    pub fn set_item_SPELL_CHARGES(&mut self, v: i32) {
        self.header_set(16, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_SPELL_CHARGES(&self) -> Option<i32> {
        self.values.get(&16).map(|v| *v as i32)
    }

    pub fn set_item_FLAGS(&mut self, v: i32) {
        self.header_set(21, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_FLAGS(&self) -> Option<i32> {
        self.values.get(&21).map(|v| *v as i32)
    }

    pub fn set_item_ENCHANTMENT_1_1(&mut self, v: i32) {
        self.header_set(22, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_ENCHANTMENT_1_1(&self) -> Option<i32> {
        self.values.get(&22).map(|v| *v as i32)
    }

    pub fn set_item_PROPERTY_SEED(&mut self, v: i32) {
        self.header_set(55, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_PROPERTY_SEED(&self) -> Option<i32> {
        self.values.get(&55).map(|v| *v as i32)
    }

    pub fn set_item_RANDOM_PROPERTIES_ID(&mut self, v: i32) {
        self.header_set(56, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_RANDOM_PROPERTIES_ID(&self) -> Option<i32> {
        self.values.get(&56).map(|v| *v as i32)
    }

    pub fn set_item_ITEM_TEXT_ID(&mut self, v: i32) {
        self.header_set(57, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_ITEM_TEXT_ID(&self) -> Option<i32> {
        self.values.get(&57).map(|v| *v as i32)
    }

    pub fn set_item_DURABILITY(&mut self, v: i32) {
        self.header_set(58, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_DURABILITY(&self) -> Option<i32> {
        self.values.get(&58).map(|v| *v as i32)
    }

    pub fn set_item_MAXDURABILITY(&mut self, v: i32) {
        self.header_set(59, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_MAXDURABILITY(&self) -> Option<i32> {
        self.values.get(&59).map(|v| *v as i32)
    }

    pub fn set_container_NUM_SLOTS(&mut self, v: i32) {
        self.header_set(60, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn container_NUM_SLOTS(&self) -> Option<i32> {
        self.values.get(&60).map(|v| *v as i32)
    }

    pub fn set_container_SLOT_1(&mut self, v: Guid) {
        self.header_set(62, v.guid() as u32);
        self.header_set(63, (v.guid() >> 32) as u32);
    }

    pub fn container_SLOT_1(&self) -> Option<Guid> {
        let lower = self.values.get(&62);
        let upper = self.values.get(&63);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_object_CREATED_BY(&mut self, v: Guid) {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
    }

    pub fn object_CREATED_BY(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

}

impl UpdateUnit {
    pub fn set_object_GUID(&mut self, v: Guid) {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
    }

    pub fn object_GUID(&self) -> Option<Guid> {
        let lower = self.values.get(&0);
        let upper = self.values.get(&1);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_object_TYPE(&mut self, v: i32) {
        self.header_set(2, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_TYPE(&self) -> Option<i32> {
        self.values.get(&2).map(|v| *v as i32)
    }

    pub fn set_object_ENTRY(&mut self, v: i32) {
        self.header_set(3, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_ENTRY(&self) -> Option<i32> {
        self.values.get(&3).map(|v| *v as i32)
    }

    pub fn set_object_SCALE_X(&mut self, v: f32) {
        self.header_set(4, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_SCALE_X(&self) -> Option<f32> {
        self.values.get(&4).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_CHARM(&mut self, v: Guid) {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
    }

    pub fn unit_CHARM(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_SUMMON(&mut self, v: Guid) {
        self.header_set(8, v.guid() as u32);
        self.header_set(9, (v.guid() >> 32) as u32);
    }

    pub fn unit_SUMMON(&self) -> Option<Guid> {
        let lower = self.values.get(&8);
        let upper = self.values.get(&9);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_CHARMEDBY(&mut self, v: Guid) {
        self.header_set(10, v.guid() as u32);
        self.header_set(11, (v.guid() >> 32) as u32);
    }

    pub fn unit_CHARMEDBY(&self) -> Option<Guid> {
        let lower = self.values.get(&10);
        let upper = self.values.get(&11);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_SUMMONEDBY(&mut self, v: Guid) {
        self.header_set(12, v.guid() as u32);
        self.header_set(13, (v.guid() >> 32) as u32);
    }

    pub fn unit_SUMMONEDBY(&self) -> Option<Guid> {
        let lower = self.values.get(&12);
        let upper = self.values.get(&13);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_CREATEDBY(&mut self, v: Guid) {
        self.header_set(14, v.guid() as u32);
        self.header_set(15, (v.guid() >> 32) as u32);
    }

    pub fn unit_CREATEDBY(&self) -> Option<Guid> {
        let lower = self.values.get(&14);
        let upper = self.values.get(&15);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_TARGET(&mut self, v: Guid) {
        self.header_set(16, v.guid() as u32);
        self.header_set(17, (v.guid() >> 32) as u32);
    }

    pub fn unit_TARGET(&self) -> Option<Guid> {
        let lower = self.values.get(&16);
        let upper = self.values.get(&17);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_PERSUADED(&mut self, v: Guid) {
        self.header_set(18, v.guid() as u32);
        self.header_set(19, (v.guid() >> 32) as u32);
    }

    pub fn unit_PERSUADED(&self) -> Option<Guid> {
        let lower = self.values.get(&18);
        let upper = self.values.get(&19);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_CHANNEL_OBJECT(&mut self, v: Guid) {
        self.header_set(20, v.guid() as u32);
        self.header_set(21, (v.guid() >> 32) as u32);
    }

    pub fn unit_CHANNEL_OBJECT(&self) -> Option<Guid> {
        let lower = self.values.get(&20);
        let upper = self.values.get(&21);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_HEALTH(&mut self, v: i32) {
        self.header_set(22, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_HEALTH(&self) -> Option<i32> {
        self.values.get(&22).map(|v| *v as i32)
    }

    pub fn set_unit_POWER1(&mut self, v: i32) {
        self.header_set(23, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_POWER1(&self) -> Option<i32> {
        self.values.get(&23).map(|v| *v as i32)
    }

    pub fn set_unit_POWER2(&mut self, v: i32) {
        self.header_set(24, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_POWER2(&self) -> Option<i32> {
        self.values.get(&24).map(|v| *v as i32)
    }

    pub fn set_unit_POWER3(&mut self, v: i32) {
        self.header_set(25, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_POWER3(&self) -> Option<i32> {
        self.values.get(&25).map(|v| *v as i32)
    }

    pub fn set_unit_POWER4(&mut self, v: i32) {
        self.header_set(26, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_POWER4(&self) -> Option<i32> {
        self.values.get(&26).map(|v| *v as i32)
    }

    pub fn set_unit_POWER5(&mut self, v: i32) {
        self.header_set(27, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_POWER5(&self) -> Option<i32> {
        self.values.get(&27).map(|v| *v as i32)
    }

    pub fn set_unit_MAXHEALTH(&mut self, v: i32) {
        self.header_set(28, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MAXHEALTH(&self) -> Option<i32> {
        self.values.get(&28).map(|v| *v as i32)
    }

    pub fn set_unit_MAXPOWER1(&mut self, v: i32) {
        self.header_set(29, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MAXPOWER1(&self) -> Option<i32> {
        self.values.get(&29).map(|v| *v as i32)
    }

    pub fn set_unit_MAXPOWER2(&mut self, v: i32) {
        self.header_set(30, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MAXPOWER2(&self) -> Option<i32> {
        self.values.get(&30).map(|v| *v as i32)
    }

    pub fn set_unit_MAXPOWER3(&mut self, v: i32) {
        self.header_set(31, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MAXPOWER3(&self) -> Option<i32> {
        self.values.get(&31).map(|v| *v as i32)
    }

    pub fn set_unit_MAXPOWER4(&mut self, v: i32) {
        self.header_set(32, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MAXPOWER4(&self) -> Option<i32> {
        self.values.get(&32).map(|v| *v as i32)
    }

    pub fn set_unit_MAXPOWER5(&mut self, v: i32) {
        self.header_set(33, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MAXPOWER5(&self) -> Option<i32> {
        self.values.get(&33).map(|v| *v as i32)
    }

    pub fn set_unit_LEVEL(&mut self, v: i32) {
        self.header_set(34, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_LEVEL(&self) -> Option<i32> {
        self.values.get(&34).map(|v| *v as i32)
    }

    pub fn set_unit_FACTIONTEMPLATE(&mut self, v: i32) {
        self.header_set(35, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_FACTIONTEMPLATE(&self) -> Option<i32> {
        self.values.get(&35).map(|v| *v as i32)
    }

    pub fn set_unit_BYTES_0(&mut self, race: Race, class: Class, gender: Gender, power: Power) {
        self.header_set(36, u32::from_le_bytes([race.as_int(), class.as_int(), gender.as_int(), power.as_int()]));
    }

    pub fn unit_BYTES_0(&self) -> Option<(Race, Class, Gender, Power)> {
        if let Some(v) = self.values.get(&36) {
            let v = v.to_le_bytes();
            let (race, class, gender, power) = (v[0], v[1], v[2], v[3]);
            Some((race.try_into().unwrap(), class.try_into().unwrap(), gender.try_into().unwrap(), power.try_into().unwrap()))
        } else {
            None
        }
    }

    pub fn set_unit_VIRTUAL_ITEM_SLOT_DISPLAY(&mut self, v: i32) {
        self.header_set(37, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_VIRTUAL_ITEM_SLOT_DISPLAY(&self) -> Option<i32> {
        self.values.get(&37).map(|v| *v as i32)
    }

    pub fn set_unit_VIRTUAL_ITEM_INFO(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(40, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn unit_VIRTUAL_ITEM_INFO(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&40) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_FLAGS(&mut self, v: i32) {
        self.header_set(46, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_FLAGS(&self) -> Option<i32> {
        self.values.get(&46).map(|v| *v as i32)
    }

    pub fn set_unit_FLAGS_2(&mut self, v: i32) {
        self.header_set(47, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_FLAGS_2(&self) -> Option<i32> {
        self.values.get(&47).map(|v| *v as i32)
    }

    pub fn set_unit_AURA(&mut self, v: i32) {
        self.header_set(48, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_AURA(&self) -> Option<i32> {
        self.values.get(&48).map(|v| *v as i32)
    }

    pub fn set_unit_AURAFLAGS(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(104, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn unit_AURAFLAGS(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&104) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_AURALEVELS(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(118, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn unit_AURALEVELS(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&118) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_AURAAPPLICATIONS(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(132, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn unit_AURAAPPLICATIONS(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&132) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_AURASTATE(&mut self, v: i32) {
        self.header_set(146, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_AURASTATE(&self) -> Option<i32> {
        self.values.get(&146).map(|v| *v as i32)
    }

    pub fn set_unit_BASEATTACKTIME(&mut self, v: i32) {
        self.header_set(147, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_BASEATTACKTIME(&self) -> Option<i32> {
        self.values.get(&147).map(|v| *v as i32)
    }

    pub fn set_unit_RANGEDATTACKTIME(&mut self, v: i32) {
        self.header_set(149, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_RANGEDATTACKTIME(&self) -> Option<i32> {
        self.values.get(&149).map(|v| *v as i32)
    }

    pub fn set_unit_BOUNDINGRADIUS(&mut self, v: f32) {
        self.header_set(150, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_BOUNDINGRADIUS(&self) -> Option<f32> {
        self.values.get(&150).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_COMBATREACH(&mut self, v: f32) {
        self.header_set(151, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_COMBATREACH(&self) -> Option<f32> {
        self.values.get(&151).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_DISPLAYID(&mut self, v: i32) {
        self.header_set(152, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_DISPLAYID(&self) -> Option<i32> {
        self.values.get(&152).map(|v| *v as i32)
    }

    pub fn set_unit_NATIVEDISPLAYID(&mut self, v: i32) {
        self.header_set(153, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_NATIVEDISPLAYID(&self) -> Option<i32> {
        self.values.get(&153).map(|v| *v as i32)
    }

    pub fn set_unit_MOUNTDISPLAYID(&mut self, v: i32) {
        self.header_set(154, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MOUNTDISPLAYID(&self) -> Option<i32> {
        self.values.get(&154).map(|v| *v as i32)
    }

    pub fn set_unit_MINDAMAGE(&mut self, v: f32) {
        self.header_set(155, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MINDAMAGE(&self) -> Option<f32> {
        self.values.get(&155).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_MAXDAMAGE(&mut self, v: f32) {
        self.header_set(156, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MAXDAMAGE(&self) -> Option<f32> {
        self.values.get(&156).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_MINOFFHANDDAMAGE(&mut self, v: f32) {
        self.header_set(157, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MINOFFHANDDAMAGE(&self) -> Option<f32> {
        self.values.get(&157).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_MAXOFFHANDDAMAGE(&mut self, v: f32) {
        self.header_set(158, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MAXOFFHANDDAMAGE(&self) -> Option<f32> {
        self.values.get(&158).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_BYTES_1(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(159, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn unit_BYTES_1(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&159) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_PETNUMBER(&mut self, v: i32) {
        self.header_set(160, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_PETNUMBER(&self) -> Option<i32> {
        self.values.get(&160).map(|v| *v as i32)
    }

    pub fn set_unit_PET_NAME_TIMESTAMP(&mut self, v: i32) {
        self.header_set(161, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_PET_NAME_TIMESTAMP(&self) -> Option<i32> {
        self.values.get(&161).map(|v| *v as i32)
    }

    pub fn set_unit_PETEXPERIENCE(&mut self, v: i32) {
        self.header_set(162, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_PETEXPERIENCE(&self) -> Option<i32> {
        self.values.get(&162).map(|v| *v as i32)
    }

    pub fn set_unit_PETNEXTLEVELEXP(&mut self, v: i32) {
        self.header_set(163, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_PETNEXTLEVELEXP(&self) -> Option<i32> {
        self.values.get(&163).map(|v| *v as i32)
    }

    pub fn set_unit_DYNAMIC_FLAGS(&mut self, v: i32) {
        self.header_set(164, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_DYNAMIC_FLAGS(&self) -> Option<i32> {
        self.values.get(&164).map(|v| *v as i32)
    }

    pub fn set_unit_CHANNEL_SPELL(&mut self, v: i32) {
        self.header_set(165, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_CHANNEL_SPELL(&self) -> Option<i32> {
        self.values.get(&165).map(|v| *v as i32)
    }

    pub fn set_unit_MOD_CAST_SPEED(&mut self, v: f32) {
        self.header_set(166, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MOD_CAST_SPEED(&self) -> Option<f32> {
        self.values.get(&166).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_CREATED_BY_SPELL(&mut self, v: i32) {
        self.header_set(167, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_CREATED_BY_SPELL(&self) -> Option<i32> {
        self.values.get(&167).map(|v| *v as i32)
    }

    pub fn set_unit_NPC_FLAGS(&mut self, v: i32) {
        self.header_set(168, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_NPC_FLAGS(&self) -> Option<i32> {
        self.values.get(&168).map(|v| *v as i32)
    }

    pub fn set_unit_NPC_EMOTESTATE(&mut self, v: i32) {
        self.header_set(169, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_NPC_EMOTESTATE(&self) -> Option<i32> {
        self.values.get(&169).map(|v| *v as i32)
    }

    pub fn set_unit_TRAINING_POINTS(&mut self, a: u16, b: u16) {
        self.header_set(170, crate::util::u16s_to_u32(a, b));
    }

    pub fn unit_TRAINING_POINTS(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&170) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_unit_STRENGTH(&mut self, v: i32) {
        self.header_set(171, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_STRENGTH(&self) -> Option<i32> {
        self.values.get(&171).map(|v| *v as i32)
    }

    pub fn set_unit_AGILITY(&mut self, v: i32) {
        self.header_set(172, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_AGILITY(&self) -> Option<i32> {
        self.values.get(&172).map(|v| *v as i32)
    }

    pub fn set_unit_STAMINA(&mut self, v: i32) {
        self.header_set(173, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_STAMINA(&self) -> Option<i32> {
        self.values.get(&173).map(|v| *v as i32)
    }

    pub fn set_unit_INTELLECT(&mut self, v: i32) {
        self.header_set(174, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_INTELLECT(&self) -> Option<i32> {
        self.values.get(&174).map(|v| *v as i32)
    }

    pub fn set_unit_SPIRIT(&mut self, v: i32) {
        self.header_set(175, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_SPIRIT(&self) -> Option<i32> {
        self.values.get(&175).map(|v| *v as i32)
    }

    pub fn set_unit_POSSTAT1(&mut self, v: i32) {
        self.header_set(177, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_POSSTAT1(&self) -> Option<i32> {
        self.values.get(&177).map(|v| *v as i32)
    }

    pub fn set_unit_POSSTAT2(&mut self, v: i32) {
        self.header_set(178, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_POSSTAT2(&self) -> Option<i32> {
        self.values.get(&178).map(|v| *v as i32)
    }

    pub fn set_unit_POSSTAT3(&mut self, v: i32) {
        self.header_set(179, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_POSSTAT3(&self) -> Option<i32> {
        self.values.get(&179).map(|v| *v as i32)
    }

    pub fn set_unit_NEGSTAT1(&mut self, v: i32) {
        self.header_set(182, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_NEGSTAT1(&self) -> Option<i32> {
        self.values.get(&182).map(|v| *v as i32)
    }

    pub fn set_unit_NEGSTAT2(&mut self, v: i32) {
        self.header_set(183, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_NEGSTAT2(&self) -> Option<i32> {
        self.values.get(&183).map(|v| *v as i32)
    }

    pub fn set_unit_NEGSTAT3(&mut self, v: i32) {
        self.header_set(184, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_NEGSTAT3(&self) -> Option<i32> {
        self.values.get(&184).map(|v| *v as i32)
    }

    pub fn set_unit_RESISTANCES(&mut self, v: i32) {
        self.header_set(186, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_RESISTANCES(&self) -> Option<i32> {
        self.values.get(&186).map(|v| *v as i32)
    }

    pub fn set_unit_BASE_MANA(&mut self, v: i32) {
        self.header_set(207, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_BASE_MANA(&self) -> Option<i32> {
        self.values.get(&207).map(|v| *v as i32)
    }

    pub fn set_unit_BASE_HEALTH(&mut self, v: i32) {
        self.header_set(208, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_BASE_HEALTH(&self) -> Option<i32> {
        self.values.get(&208).map(|v| *v as i32)
    }

    pub fn set_unit_BYTES_2(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(209, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn unit_BYTES_2(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&209) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_ATTACK_POWER(&mut self, v: i32) {
        self.header_set(210, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_ATTACK_POWER(&self) -> Option<i32> {
        self.values.get(&210).map(|v| *v as i32)
    }

    pub fn set_unit_ATTACK_POWER_MODS(&mut self, a: u16, b: u16) {
        self.header_set(211, crate::util::u16s_to_u32(a, b));
    }

    pub fn unit_ATTACK_POWER_MODS(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&211) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_unit_ATTACK_POWER_MULTIPLIER(&mut self, v: f32) {
        self.header_set(212, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_ATTACK_POWER_MULTIPLIER(&self) -> Option<f32> {
        self.values.get(&212).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_RANGED_ATTACK_POWER(&mut self, v: i32) {
        self.header_set(213, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_RANGED_ATTACK_POWER(&self) -> Option<i32> {
        self.values.get(&213).map(|v| *v as i32)
    }

    pub fn set_unit_RANGED_ATTACK_POWER_MODS(&mut self, a: u16, b: u16) {
        self.header_set(214, crate::util::u16s_to_u32(a, b));
    }

    pub fn unit_RANGED_ATTACK_POWER_MODS(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&214) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_unit_RANGED_ATTACK_POWER_MULTIPLIER(&mut self, v: f32) {
        self.header_set(215, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_RANGED_ATTACK_POWER_MULTIPLIER(&self) -> Option<f32> {
        self.values.get(&215).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_MINRANGEDDAMAGE(&mut self, v: f32) {
        self.header_set(216, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MINRANGEDDAMAGE(&self) -> Option<f32> {
        self.values.get(&216).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_MAXRANGEDDAMAGE(&mut self, v: f32) {
        self.header_set(217, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MAXRANGEDDAMAGE(&self) -> Option<f32> {
        self.values.get(&217).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_POWER_COST_MODIFIER(&mut self, v: i32) {
        self.header_set(218, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_POWER_COST_MODIFIER(&self) -> Option<i32> {
        self.values.get(&218).map(|v| *v as i32)
    }

    pub fn set_unit_POWER_COST_MULTIPLIER(&mut self, v: f32) {
        self.header_set(225, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_POWER_COST_MULTIPLIER(&self) -> Option<f32> {
        self.values.get(&225).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_MAXHEALTHMODIFIER(&mut self, v: f32) {
        self.header_set(232, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MAXHEALTHMODIFIER(&self) -> Option<f32> {
        self.values.get(&232).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_object_CREATED_BY(&mut self, v: Guid) {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
    }

    pub fn object_CREATED_BY(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

}

impl UpdatePlayer {
    pub fn set_object_GUID(&mut self, v: Guid) {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
    }

    pub fn object_GUID(&self) -> Option<Guid> {
        let lower = self.values.get(&0);
        let upper = self.values.get(&1);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_object_TYPE(&mut self, v: i32) {
        self.header_set(2, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_TYPE(&self) -> Option<i32> {
        self.values.get(&2).map(|v| *v as i32)
    }

    pub fn set_object_ENTRY(&mut self, v: i32) {
        self.header_set(3, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_ENTRY(&self) -> Option<i32> {
        self.values.get(&3).map(|v| *v as i32)
    }

    pub fn set_object_SCALE_X(&mut self, v: f32) {
        self.header_set(4, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_SCALE_X(&self) -> Option<f32> {
        self.values.get(&4).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_CHARM(&mut self, v: Guid) {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
    }

    pub fn unit_CHARM(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_SUMMON(&mut self, v: Guid) {
        self.header_set(8, v.guid() as u32);
        self.header_set(9, (v.guid() >> 32) as u32);
    }

    pub fn unit_SUMMON(&self) -> Option<Guid> {
        let lower = self.values.get(&8);
        let upper = self.values.get(&9);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_CHARMEDBY(&mut self, v: Guid) {
        self.header_set(10, v.guid() as u32);
        self.header_set(11, (v.guid() >> 32) as u32);
    }

    pub fn unit_CHARMEDBY(&self) -> Option<Guid> {
        let lower = self.values.get(&10);
        let upper = self.values.get(&11);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_SUMMONEDBY(&mut self, v: Guid) {
        self.header_set(12, v.guid() as u32);
        self.header_set(13, (v.guid() >> 32) as u32);
    }

    pub fn unit_SUMMONEDBY(&self) -> Option<Guid> {
        let lower = self.values.get(&12);
        let upper = self.values.get(&13);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_CREATEDBY(&mut self, v: Guid) {
        self.header_set(14, v.guid() as u32);
        self.header_set(15, (v.guid() >> 32) as u32);
    }

    pub fn unit_CREATEDBY(&self) -> Option<Guid> {
        let lower = self.values.get(&14);
        let upper = self.values.get(&15);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_TARGET(&mut self, v: Guid) {
        self.header_set(16, v.guid() as u32);
        self.header_set(17, (v.guid() >> 32) as u32);
    }

    pub fn unit_TARGET(&self) -> Option<Guid> {
        let lower = self.values.get(&16);
        let upper = self.values.get(&17);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_PERSUADED(&mut self, v: Guid) {
        self.header_set(18, v.guid() as u32);
        self.header_set(19, (v.guid() >> 32) as u32);
    }

    pub fn unit_PERSUADED(&self) -> Option<Guid> {
        let lower = self.values.get(&18);
        let upper = self.values.get(&19);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_CHANNEL_OBJECT(&mut self, v: Guid) {
        self.header_set(20, v.guid() as u32);
        self.header_set(21, (v.guid() >> 32) as u32);
    }

    pub fn unit_CHANNEL_OBJECT(&self) -> Option<Guid> {
        let lower = self.values.get(&20);
        let upper = self.values.get(&21);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_HEALTH(&mut self, v: i32) {
        self.header_set(22, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_HEALTH(&self) -> Option<i32> {
        self.values.get(&22).map(|v| *v as i32)
    }

    pub fn set_unit_POWER1(&mut self, v: i32) {
        self.header_set(23, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_POWER1(&self) -> Option<i32> {
        self.values.get(&23).map(|v| *v as i32)
    }

    pub fn set_unit_POWER2(&mut self, v: i32) {
        self.header_set(24, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_POWER2(&self) -> Option<i32> {
        self.values.get(&24).map(|v| *v as i32)
    }

    pub fn set_unit_POWER3(&mut self, v: i32) {
        self.header_set(25, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_POWER3(&self) -> Option<i32> {
        self.values.get(&25).map(|v| *v as i32)
    }

    pub fn set_unit_POWER4(&mut self, v: i32) {
        self.header_set(26, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_POWER4(&self) -> Option<i32> {
        self.values.get(&26).map(|v| *v as i32)
    }

    pub fn set_unit_POWER5(&mut self, v: i32) {
        self.header_set(27, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_POWER5(&self) -> Option<i32> {
        self.values.get(&27).map(|v| *v as i32)
    }

    pub fn set_unit_MAXHEALTH(&mut self, v: i32) {
        self.header_set(28, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MAXHEALTH(&self) -> Option<i32> {
        self.values.get(&28).map(|v| *v as i32)
    }

    pub fn set_unit_MAXPOWER1(&mut self, v: i32) {
        self.header_set(29, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MAXPOWER1(&self) -> Option<i32> {
        self.values.get(&29).map(|v| *v as i32)
    }

    pub fn set_unit_MAXPOWER2(&mut self, v: i32) {
        self.header_set(30, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MAXPOWER2(&self) -> Option<i32> {
        self.values.get(&30).map(|v| *v as i32)
    }

    pub fn set_unit_MAXPOWER3(&mut self, v: i32) {
        self.header_set(31, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MAXPOWER3(&self) -> Option<i32> {
        self.values.get(&31).map(|v| *v as i32)
    }

    pub fn set_unit_MAXPOWER4(&mut self, v: i32) {
        self.header_set(32, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MAXPOWER4(&self) -> Option<i32> {
        self.values.get(&32).map(|v| *v as i32)
    }

    pub fn set_unit_MAXPOWER5(&mut self, v: i32) {
        self.header_set(33, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MAXPOWER5(&self) -> Option<i32> {
        self.values.get(&33).map(|v| *v as i32)
    }

    pub fn set_unit_LEVEL(&mut self, v: i32) {
        self.header_set(34, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_LEVEL(&self) -> Option<i32> {
        self.values.get(&34).map(|v| *v as i32)
    }

    pub fn set_unit_FACTIONTEMPLATE(&mut self, v: i32) {
        self.header_set(35, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_FACTIONTEMPLATE(&self) -> Option<i32> {
        self.values.get(&35).map(|v| *v as i32)
    }

    pub fn set_unit_BYTES_0(&mut self, race: Race, class: Class, gender: Gender, power: Power) {
        self.header_set(36, u32::from_le_bytes([race.as_int(), class.as_int(), gender.as_int(), power.as_int()]));
    }

    pub fn unit_BYTES_0(&self) -> Option<(Race, Class, Gender, Power)> {
        if let Some(v) = self.values.get(&36) {
            let v = v.to_le_bytes();
            let (race, class, gender, power) = (v[0], v[1], v[2], v[3]);
            Some((race.try_into().unwrap(), class.try_into().unwrap(), gender.try_into().unwrap(), power.try_into().unwrap()))
        } else {
            None
        }
    }

    pub fn set_unit_VIRTUAL_ITEM_SLOT_DISPLAY(&mut self, v: i32) {
        self.header_set(37, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_VIRTUAL_ITEM_SLOT_DISPLAY(&self) -> Option<i32> {
        self.values.get(&37).map(|v| *v as i32)
    }

    pub fn set_unit_VIRTUAL_ITEM_INFO(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(40, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn unit_VIRTUAL_ITEM_INFO(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&40) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_FLAGS(&mut self, v: i32) {
        self.header_set(46, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_FLAGS(&self) -> Option<i32> {
        self.values.get(&46).map(|v| *v as i32)
    }

    pub fn set_unit_FLAGS_2(&mut self, v: i32) {
        self.header_set(47, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_FLAGS_2(&self) -> Option<i32> {
        self.values.get(&47).map(|v| *v as i32)
    }

    pub fn set_unit_AURA(&mut self, v: i32) {
        self.header_set(48, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_AURA(&self) -> Option<i32> {
        self.values.get(&48).map(|v| *v as i32)
    }

    pub fn set_unit_AURAFLAGS(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(104, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn unit_AURAFLAGS(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&104) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_AURALEVELS(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(118, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn unit_AURALEVELS(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&118) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_AURAAPPLICATIONS(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(132, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn unit_AURAAPPLICATIONS(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&132) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_AURASTATE(&mut self, v: i32) {
        self.header_set(146, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_AURASTATE(&self) -> Option<i32> {
        self.values.get(&146).map(|v| *v as i32)
    }

    pub fn set_unit_BASEATTACKTIME(&mut self, v: i32) {
        self.header_set(147, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_BASEATTACKTIME(&self) -> Option<i32> {
        self.values.get(&147).map(|v| *v as i32)
    }

    pub fn set_unit_RANGEDATTACKTIME(&mut self, v: i32) {
        self.header_set(149, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_RANGEDATTACKTIME(&self) -> Option<i32> {
        self.values.get(&149).map(|v| *v as i32)
    }

    pub fn set_unit_BOUNDINGRADIUS(&mut self, v: f32) {
        self.header_set(150, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_BOUNDINGRADIUS(&self) -> Option<f32> {
        self.values.get(&150).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_COMBATREACH(&mut self, v: f32) {
        self.header_set(151, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_COMBATREACH(&self) -> Option<f32> {
        self.values.get(&151).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_DISPLAYID(&mut self, v: i32) {
        self.header_set(152, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_DISPLAYID(&self) -> Option<i32> {
        self.values.get(&152).map(|v| *v as i32)
    }

    pub fn set_unit_NATIVEDISPLAYID(&mut self, v: i32) {
        self.header_set(153, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_NATIVEDISPLAYID(&self) -> Option<i32> {
        self.values.get(&153).map(|v| *v as i32)
    }

    pub fn set_unit_MOUNTDISPLAYID(&mut self, v: i32) {
        self.header_set(154, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MOUNTDISPLAYID(&self) -> Option<i32> {
        self.values.get(&154).map(|v| *v as i32)
    }

    pub fn set_unit_MINDAMAGE(&mut self, v: f32) {
        self.header_set(155, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MINDAMAGE(&self) -> Option<f32> {
        self.values.get(&155).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_MAXDAMAGE(&mut self, v: f32) {
        self.header_set(156, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MAXDAMAGE(&self) -> Option<f32> {
        self.values.get(&156).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_MINOFFHANDDAMAGE(&mut self, v: f32) {
        self.header_set(157, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MINOFFHANDDAMAGE(&self) -> Option<f32> {
        self.values.get(&157).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_MAXOFFHANDDAMAGE(&mut self, v: f32) {
        self.header_set(158, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MAXOFFHANDDAMAGE(&self) -> Option<f32> {
        self.values.get(&158).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_BYTES_1(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(159, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn unit_BYTES_1(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&159) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_PETNUMBER(&mut self, v: i32) {
        self.header_set(160, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_PETNUMBER(&self) -> Option<i32> {
        self.values.get(&160).map(|v| *v as i32)
    }

    pub fn set_unit_PET_NAME_TIMESTAMP(&mut self, v: i32) {
        self.header_set(161, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_PET_NAME_TIMESTAMP(&self) -> Option<i32> {
        self.values.get(&161).map(|v| *v as i32)
    }

    pub fn set_unit_PETEXPERIENCE(&mut self, v: i32) {
        self.header_set(162, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_PETEXPERIENCE(&self) -> Option<i32> {
        self.values.get(&162).map(|v| *v as i32)
    }

    pub fn set_unit_PETNEXTLEVELEXP(&mut self, v: i32) {
        self.header_set(163, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_PETNEXTLEVELEXP(&self) -> Option<i32> {
        self.values.get(&163).map(|v| *v as i32)
    }

    pub fn set_unit_DYNAMIC_FLAGS(&mut self, v: i32) {
        self.header_set(164, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_DYNAMIC_FLAGS(&self) -> Option<i32> {
        self.values.get(&164).map(|v| *v as i32)
    }

    pub fn set_unit_CHANNEL_SPELL(&mut self, v: i32) {
        self.header_set(165, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_CHANNEL_SPELL(&self) -> Option<i32> {
        self.values.get(&165).map(|v| *v as i32)
    }

    pub fn set_unit_MOD_CAST_SPEED(&mut self, v: f32) {
        self.header_set(166, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MOD_CAST_SPEED(&self) -> Option<f32> {
        self.values.get(&166).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_CREATED_BY_SPELL(&mut self, v: i32) {
        self.header_set(167, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_CREATED_BY_SPELL(&self) -> Option<i32> {
        self.values.get(&167).map(|v| *v as i32)
    }

    pub fn set_unit_NPC_FLAGS(&mut self, v: i32) {
        self.header_set(168, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_NPC_FLAGS(&self) -> Option<i32> {
        self.values.get(&168).map(|v| *v as i32)
    }

    pub fn set_unit_NPC_EMOTESTATE(&mut self, v: i32) {
        self.header_set(169, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_NPC_EMOTESTATE(&self) -> Option<i32> {
        self.values.get(&169).map(|v| *v as i32)
    }

    pub fn set_unit_TRAINING_POINTS(&mut self, a: u16, b: u16) {
        self.header_set(170, crate::util::u16s_to_u32(a, b));
    }

    pub fn unit_TRAINING_POINTS(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&170) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_unit_STRENGTH(&mut self, v: i32) {
        self.header_set(171, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_STRENGTH(&self) -> Option<i32> {
        self.values.get(&171).map(|v| *v as i32)
    }

    pub fn set_unit_AGILITY(&mut self, v: i32) {
        self.header_set(172, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_AGILITY(&self) -> Option<i32> {
        self.values.get(&172).map(|v| *v as i32)
    }

    pub fn set_unit_STAMINA(&mut self, v: i32) {
        self.header_set(173, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_STAMINA(&self) -> Option<i32> {
        self.values.get(&173).map(|v| *v as i32)
    }

    pub fn set_unit_INTELLECT(&mut self, v: i32) {
        self.header_set(174, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_INTELLECT(&self) -> Option<i32> {
        self.values.get(&174).map(|v| *v as i32)
    }

    pub fn set_unit_SPIRIT(&mut self, v: i32) {
        self.header_set(175, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_SPIRIT(&self) -> Option<i32> {
        self.values.get(&175).map(|v| *v as i32)
    }

    pub fn set_player_POSSTAT0(&mut self, v: i32) {
        self.header_set(176, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_POSSTAT0(&self) -> Option<i32> {
        self.values.get(&176).map(|v| *v as i32)
    }

    pub fn set_unit_POSSTAT1(&mut self, v: i32) {
        self.header_set(177, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_POSSTAT1(&self) -> Option<i32> {
        self.values.get(&177).map(|v| *v as i32)
    }

    pub fn set_unit_POSSTAT2(&mut self, v: i32) {
        self.header_set(178, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_POSSTAT2(&self) -> Option<i32> {
        self.values.get(&178).map(|v| *v as i32)
    }

    pub fn set_unit_POSSTAT3(&mut self, v: i32) {
        self.header_set(179, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_POSSTAT3(&self) -> Option<i32> {
        self.values.get(&179).map(|v| *v as i32)
    }

    pub fn set_player_POSSTAT4(&mut self, v: i32) {
        self.header_set(180, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_POSSTAT4(&self) -> Option<i32> {
        self.values.get(&180).map(|v| *v as i32)
    }

    pub fn set_player_NEGSTAT0(&mut self, v: i32) {
        self.header_set(181, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_NEGSTAT0(&self) -> Option<i32> {
        self.values.get(&181).map(|v| *v as i32)
    }

    pub fn set_unit_NEGSTAT1(&mut self, v: i32) {
        self.header_set(182, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_NEGSTAT1(&self) -> Option<i32> {
        self.values.get(&182).map(|v| *v as i32)
    }

    pub fn set_unit_NEGSTAT2(&mut self, v: i32) {
        self.header_set(183, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_NEGSTAT2(&self) -> Option<i32> {
        self.values.get(&183).map(|v| *v as i32)
    }

    pub fn set_unit_NEGSTAT3(&mut self, v: i32) {
        self.header_set(184, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_NEGSTAT3(&self) -> Option<i32> {
        self.values.get(&184).map(|v| *v as i32)
    }

    pub fn set_player_NEGSTAT4(&mut self, v: i32) {
        self.header_set(185, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_NEGSTAT4(&self) -> Option<i32> {
        self.values.get(&185).map(|v| *v as i32)
    }

    pub fn set_unit_RESISTANCES(&mut self, v: i32) {
        self.header_set(186, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_RESISTANCES(&self) -> Option<i32> {
        self.values.get(&186).map(|v| *v as i32)
    }

    pub fn set_player_RESISTANCEBUFFMODSPOSITIVE(&mut self, v: i32) {
        self.header_set(193, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_RESISTANCEBUFFMODSPOSITIVE(&self) -> Option<i32> {
        self.values.get(&193).map(|v| *v as i32)
    }

    pub fn set_player_RESISTANCEBUFFMODSNEGATIVE(&mut self, v: i32) {
        self.header_set(200, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_RESISTANCEBUFFMODSNEGATIVE(&self) -> Option<i32> {
        self.values.get(&200).map(|v| *v as i32)
    }

    pub fn set_unit_BASE_MANA(&mut self, v: i32) {
        self.header_set(207, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_BASE_MANA(&self) -> Option<i32> {
        self.values.get(&207).map(|v| *v as i32)
    }

    pub fn set_unit_BASE_HEALTH(&mut self, v: i32) {
        self.header_set(208, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_BASE_HEALTH(&self) -> Option<i32> {
        self.values.get(&208).map(|v| *v as i32)
    }

    pub fn set_unit_BYTES_2(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(209, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn unit_BYTES_2(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&209) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_ATTACK_POWER(&mut self, v: i32) {
        self.header_set(210, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_ATTACK_POWER(&self) -> Option<i32> {
        self.values.get(&210).map(|v| *v as i32)
    }

    pub fn set_unit_ATTACK_POWER_MODS(&mut self, a: u16, b: u16) {
        self.header_set(211, crate::util::u16s_to_u32(a, b));
    }

    pub fn unit_ATTACK_POWER_MODS(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&211) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_unit_ATTACK_POWER_MULTIPLIER(&mut self, v: f32) {
        self.header_set(212, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_ATTACK_POWER_MULTIPLIER(&self) -> Option<f32> {
        self.values.get(&212).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_RANGED_ATTACK_POWER(&mut self, v: i32) {
        self.header_set(213, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_RANGED_ATTACK_POWER(&self) -> Option<i32> {
        self.values.get(&213).map(|v| *v as i32)
    }

    pub fn set_unit_RANGED_ATTACK_POWER_MODS(&mut self, a: u16, b: u16) {
        self.header_set(214, crate::util::u16s_to_u32(a, b));
    }

    pub fn unit_RANGED_ATTACK_POWER_MODS(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&214) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_unit_RANGED_ATTACK_POWER_MULTIPLIER(&mut self, v: f32) {
        self.header_set(215, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_RANGED_ATTACK_POWER_MULTIPLIER(&self) -> Option<f32> {
        self.values.get(&215).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_MINRANGEDDAMAGE(&mut self, v: f32) {
        self.header_set(216, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MINRANGEDDAMAGE(&self) -> Option<f32> {
        self.values.get(&216).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_MAXRANGEDDAMAGE(&mut self, v: f32) {
        self.header_set(217, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MAXRANGEDDAMAGE(&self) -> Option<f32> {
        self.values.get(&217).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_POWER_COST_MODIFIER(&mut self, v: i32) {
        self.header_set(218, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_POWER_COST_MODIFIER(&self) -> Option<i32> {
        self.values.get(&218).map(|v| *v as i32)
    }

    pub fn set_unit_POWER_COST_MULTIPLIER(&mut self, v: f32) {
        self.header_set(225, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_POWER_COST_MULTIPLIER(&self) -> Option<f32> {
        self.values.get(&225).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_MAXHEALTHMODIFIER(&mut self, v: f32) {
        self.header_set(232, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MAXHEALTHMODIFIER(&self) -> Option<f32> {
        self.values.get(&232).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_DUEL_ARBITER(&mut self, v: Guid) {
        self.header_set(234, v.guid() as u32);
        self.header_set(235, (v.guid() >> 32) as u32);
    }

    pub fn player_DUEL_ARBITER(&self) -> Option<Guid> {
        let lower = self.values.get(&234);
        let upper = self.values.get(&235);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FLAGS(&mut self, v: i32) {
        self.header_set(236, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_FLAGS(&self) -> Option<i32> {
        self.values.get(&236).map(|v| *v as i32)
    }

    pub fn set_player_GUILDID(&mut self, v: i32) {
        self.header_set(237, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_GUILDID(&self) -> Option<i32> {
        self.values.get(&237).map(|v| *v as i32)
    }

    pub fn set_player_GUILDRANK(&mut self, v: i32) {
        self.header_set(238, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_GUILDRANK(&self) -> Option<i32> {
        self.values.get(&238).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_BYTES(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(239, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_FIELD_BYTES(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&239) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_BYTES_2(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(240, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_BYTES_2(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&240) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_BYTES_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(241, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_BYTES_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&241) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_DUEL_TEAM(&mut self, v: i32) {
        self.header_set(242, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_DUEL_TEAM(&self) -> Option<i32> {
        self.values.get(&242).map(|v| *v as i32)
    }

    pub fn set_player_GUILD_TIMESTAMP(&mut self, v: i32) {
        self.header_set(243, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_GUILD_TIMESTAMP(&self) -> Option<i32> {
        self.values.get(&243).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_1_1(&mut self, v: i32) {
        self.header_set(244, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_1_1(&self) -> Option<i32> {
        self.values.get(&244).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_1_2(&mut self, v: i32) {
        self.header_set(245, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_1_2(&self) -> Option<i32> {
        self.values.get(&245).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_1_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(246, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_QUEST_LOG_1_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&246) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_1_4(&mut self, v: i32) {
        self.header_set(247, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_1_4(&self) -> Option<i32> {
        self.values.get(&247).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_2_1(&mut self, v: i32) {
        self.header_set(248, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_2_1(&self) -> Option<i32> {
        self.values.get(&248).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_2_2(&mut self, v: i32) {
        self.header_set(249, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_2_2(&self) -> Option<i32> {
        self.values.get(&249).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_2_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(250, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_QUEST_LOG_2_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&250) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_2_4(&mut self, v: i32) {
        self.header_set(251, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_2_4(&self) -> Option<i32> {
        self.values.get(&251).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_3_1(&mut self, v: i32) {
        self.header_set(252, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_3_1(&self) -> Option<i32> {
        self.values.get(&252).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_3_2(&mut self, v: i32) {
        self.header_set(253, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_3_2(&self) -> Option<i32> {
        self.values.get(&253).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_3_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(254, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_QUEST_LOG_3_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&254) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_3_4(&mut self, v: i32) {
        self.header_set(255, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_3_4(&self) -> Option<i32> {
        self.values.get(&255).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_4_1(&mut self, v: i32) {
        self.header_set(256, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_4_1(&self) -> Option<i32> {
        self.values.get(&256).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_4_2(&mut self, v: i32) {
        self.header_set(257, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_4_2(&self) -> Option<i32> {
        self.values.get(&257).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_4_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(258, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_QUEST_LOG_4_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&258) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_4_4(&mut self, v: i32) {
        self.header_set(259, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_4_4(&self) -> Option<i32> {
        self.values.get(&259).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_5_1(&mut self, v: i32) {
        self.header_set(260, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_5_1(&self) -> Option<i32> {
        self.values.get(&260).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_5_2(&mut self, v: i32) {
        self.header_set(261, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_5_2(&self) -> Option<i32> {
        self.values.get(&261).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_5_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(262, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_QUEST_LOG_5_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&262) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_5_4(&mut self, v: i32) {
        self.header_set(263, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_5_4(&self) -> Option<i32> {
        self.values.get(&263).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_6_1(&mut self, v: i32) {
        self.header_set(264, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_6_1(&self) -> Option<i32> {
        self.values.get(&264).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_6_2(&mut self, v: i32) {
        self.header_set(265, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_6_2(&self) -> Option<i32> {
        self.values.get(&265).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_6_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(266, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_QUEST_LOG_6_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&266) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_6_4(&mut self, v: i32) {
        self.header_set(267, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_6_4(&self) -> Option<i32> {
        self.values.get(&267).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_7_1(&mut self, v: i32) {
        self.header_set(268, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_7_1(&self) -> Option<i32> {
        self.values.get(&268).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_7_2(&mut self, v: i32) {
        self.header_set(269, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_7_2(&self) -> Option<i32> {
        self.values.get(&269).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_7_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(270, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_QUEST_LOG_7_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&270) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_7_4(&mut self, v: i32) {
        self.header_set(271, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_7_4(&self) -> Option<i32> {
        self.values.get(&271).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_8_1(&mut self, v: i32) {
        self.header_set(272, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_8_1(&self) -> Option<i32> {
        self.values.get(&272).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_8_2(&mut self, v: i32) {
        self.header_set(273, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_8_2(&self) -> Option<i32> {
        self.values.get(&273).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_8_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(274, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_QUEST_LOG_8_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&274) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_8_4(&mut self, v: i32) {
        self.header_set(275, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_8_4(&self) -> Option<i32> {
        self.values.get(&275).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_9_1(&mut self, v: i32) {
        self.header_set(276, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_9_1(&self) -> Option<i32> {
        self.values.get(&276).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_9_2(&mut self, v: i32) {
        self.header_set(277, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_9_2(&self) -> Option<i32> {
        self.values.get(&277).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_9_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(278, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_QUEST_LOG_9_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&278) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_9_4(&mut self, v: i32) {
        self.header_set(279, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_9_4(&self) -> Option<i32> {
        self.values.get(&279).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_10_1(&mut self, v: i32) {
        self.header_set(280, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_10_1(&self) -> Option<i32> {
        self.values.get(&280).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_10_2(&mut self, v: i32) {
        self.header_set(281, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_10_2(&self) -> Option<i32> {
        self.values.get(&281).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_10_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(282, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_QUEST_LOG_10_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&282) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_10_4(&mut self, v: i32) {
        self.header_set(283, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_10_4(&self) -> Option<i32> {
        self.values.get(&283).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_11_1(&mut self, v: i32) {
        self.header_set(284, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_11_1(&self) -> Option<i32> {
        self.values.get(&284).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_11_2(&mut self, v: i32) {
        self.header_set(285, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_11_2(&self) -> Option<i32> {
        self.values.get(&285).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_11_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(286, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_QUEST_LOG_11_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&286) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_11_4(&mut self, v: i32) {
        self.header_set(287, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_11_4(&self) -> Option<i32> {
        self.values.get(&287).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_12_1(&mut self, v: i32) {
        self.header_set(288, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_12_1(&self) -> Option<i32> {
        self.values.get(&288).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_12_2(&mut self, v: i32) {
        self.header_set(289, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_12_2(&self) -> Option<i32> {
        self.values.get(&289).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_12_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(290, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_QUEST_LOG_12_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&290) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_12_4(&mut self, v: i32) {
        self.header_set(291, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_12_4(&self) -> Option<i32> {
        self.values.get(&291).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_13_1(&mut self, v: i32) {
        self.header_set(292, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_13_1(&self) -> Option<i32> {
        self.values.get(&292).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_13_2(&mut self, v: i32) {
        self.header_set(293, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_13_2(&self) -> Option<i32> {
        self.values.get(&293).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_13_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(294, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_QUEST_LOG_13_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&294) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_13_4(&mut self, v: i32) {
        self.header_set(295, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_13_4(&self) -> Option<i32> {
        self.values.get(&295).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_14_1(&mut self, v: i32) {
        self.header_set(296, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_14_1(&self) -> Option<i32> {
        self.values.get(&296).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_14_2(&mut self, v: i32) {
        self.header_set(297, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_14_2(&self) -> Option<i32> {
        self.values.get(&297).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_14_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(298, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_QUEST_LOG_14_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&298) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_14_4(&mut self, v: i32) {
        self.header_set(299, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_14_4(&self) -> Option<i32> {
        self.values.get(&299).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_15_1(&mut self, v: i32) {
        self.header_set(300, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_15_1(&self) -> Option<i32> {
        self.values.get(&300).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_15_2(&mut self, v: i32) {
        self.header_set(301, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_15_2(&self) -> Option<i32> {
        self.values.get(&301).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_15_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(302, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_QUEST_LOG_15_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&302) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_15_4(&mut self, v: i32) {
        self.header_set(303, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_15_4(&self) -> Option<i32> {
        self.values.get(&303).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_16_1(&mut self, v: i32) {
        self.header_set(304, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_16_1(&self) -> Option<i32> {
        self.values.get(&304).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_16_2(&mut self, v: i32) {
        self.header_set(305, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_16_2(&self) -> Option<i32> {
        self.values.get(&305).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_16_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(306, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_QUEST_LOG_16_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&306) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_16_4(&mut self, v: i32) {
        self.header_set(307, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_16_4(&self) -> Option<i32> {
        self.values.get(&307).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_17_1(&mut self, v: i32) {
        self.header_set(308, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_17_1(&self) -> Option<i32> {
        self.values.get(&308).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_17_2(&mut self, v: i32) {
        self.header_set(309, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_17_2(&self) -> Option<i32> {
        self.values.get(&309).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_17_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(310, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_QUEST_LOG_17_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&310) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_17_4(&mut self, v: i32) {
        self.header_set(311, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_17_4(&self) -> Option<i32> {
        self.values.get(&311).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_18_1(&mut self, v: i32) {
        self.header_set(312, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_18_1(&self) -> Option<i32> {
        self.values.get(&312).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_18_2(&mut self, v: i32) {
        self.header_set(313, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_18_2(&self) -> Option<i32> {
        self.values.get(&313).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_18_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(314, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_QUEST_LOG_18_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&314) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_18_4(&mut self, v: i32) {
        self.header_set(315, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_18_4(&self) -> Option<i32> {
        self.values.get(&315).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_19_1(&mut self, v: i32) {
        self.header_set(316, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_19_1(&self) -> Option<i32> {
        self.values.get(&316).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_19_2(&mut self, v: i32) {
        self.header_set(317, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_19_2(&self) -> Option<i32> {
        self.values.get(&317).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_19_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(318, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_QUEST_LOG_19_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&318) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_19_4(&mut self, v: i32) {
        self.header_set(319, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_19_4(&self) -> Option<i32> {
        self.values.get(&319).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_20_1(&mut self, v: i32) {
        self.header_set(320, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_20_1(&self) -> Option<i32> {
        self.values.get(&320).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_20_2(&mut self, v: i32) {
        self.header_set(321, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_20_2(&self) -> Option<i32> {
        self.values.get(&321).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_20_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(322, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_QUEST_LOG_20_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&322) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_20_4(&mut self, v: i32) {
        self.header_set(323, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_20_4(&self) -> Option<i32> {
        self.values.get(&323).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_21_1(&mut self, v: i32) {
        self.header_set(324, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_21_1(&self) -> Option<i32> {
        self.values.get(&324).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_21_2(&mut self, v: i32) {
        self.header_set(325, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_21_2(&self) -> Option<i32> {
        self.values.get(&325).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_21_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(326, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_QUEST_LOG_21_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&326) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_21_4(&mut self, v: i32) {
        self.header_set(327, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_21_4(&self) -> Option<i32> {
        self.values.get(&327).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_22_1(&mut self, v: i32) {
        self.header_set(328, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_22_1(&self) -> Option<i32> {
        self.values.get(&328).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_22_2(&mut self, v: i32) {
        self.header_set(329, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_22_2(&self) -> Option<i32> {
        self.values.get(&329).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_22_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(330, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_QUEST_LOG_22_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&330) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_22_4(&mut self, v: i32) {
        self.header_set(331, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_22_4(&self) -> Option<i32> {
        self.values.get(&331).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_23_1(&mut self, v: i32) {
        self.header_set(332, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_23_1(&self) -> Option<i32> {
        self.values.get(&332).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_23_2(&mut self, v: i32) {
        self.header_set(333, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_23_2(&self) -> Option<i32> {
        self.values.get(&333).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_23_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(334, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_QUEST_LOG_23_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&334) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_23_4(&mut self, v: i32) {
        self.header_set(335, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_23_4(&self) -> Option<i32> {
        self.values.get(&335).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_24_1(&mut self, v: i32) {
        self.header_set(336, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_24_1(&self) -> Option<i32> {
        self.values.get(&336).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_24_2(&mut self, v: i32) {
        self.header_set(337, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_24_2(&self) -> Option<i32> {
        self.values.get(&337).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_24_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(338, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_QUEST_LOG_24_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&338) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_24_4(&mut self, v: i32) {
        self.header_set(339, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_24_4(&self) -> Option<i32> {
        self.values.get(&339).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_25_1(&mut self, v: i32) {
        self.header_set(340, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_25_1(&self) -> Option<i32> {
        self.values.get(&340).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_25_2(&mut self, v: i32) {
        self.header_set(341, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_25_2(&self) -> Option<i32> {
        self.values.get(&341).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_25_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(342, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_QUEST_LOG_25_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&342) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_25_4(&mut self, v: i32) {
        self.header_set(343, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_25_4(&self) -> Option<i32> {
        self.values.get(&343).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_1_CREATOR(&mut self, v: Guid) {
        self.header_set(344, v.guid() as u32);
        self.header_set(345, (v.guid() >> 32) as u32);
    }

    pub fn player_VISIBLE_ITEM_1_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&344);
        let upper = self.values.get(&345);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VISIBLE_ITEM_1_0(&mut self, v: i32) {
        self.header_set(346, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_VISIBLE_ITEM_1_0(&self) -> Option<i32> {
        self.values.get(&346).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_1_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(358, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_VISIBLE_ITEM_1_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&358) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_2_CREATOR(&mut self, v: Guid) {
        self.header_set(360, v.guid() as u32);
        self.header_set(361, (v.guid() >> 32) as u32);
    }

    pub fn player_VISIBLE_ITEM_2_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&360);
        let upper = self.values.get(&361);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VISIBLE_ITEM_2_0(&mut self, v: i32) {
        self.header_set(362, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_VISIBLE_ITEM_2_0(&self) -> Option<i32> {
        self.values.get(&362).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_2_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(374, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_VISIBLE_ITEM_2_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&374) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_3_CREATOR(&mut self, v: Guid) {
        self.header_set(376, v.guid() as u32);
        self.header_set(377, (v.guid() >> 32) as u32);
    }

    pub fn player_VISIBLE_ITEM_3_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&376);
        let upper = self.values.get(&377);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VISIBLE_ITEM_3_0(&mut self, v: i32) {
        self.header_set(378, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_VISIBLE_ITEM_3_0(&self) -> Option<i32> {
        self.values.get(&378).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_3_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(390, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_VISIBLE_ITEM_3_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&390) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_4_CREATOR(&mut self, v: Guid) {
        self.header_set(392, v.guid() as u32);
        self.header_set(393, (v.guid() >> 32) as u32);
    }

    pub fn player_VISIBLE_ITEM_4_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&392);
        let upper = self.values.get(&393);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VISIBLE_ITEM_4_0(&mut self, v: i32) {
        self.header_set(394, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_VISIBLE_ITEM_4_0(&self) -> Option<i32> {
        self.values.get(&394).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_4_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(406, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_VISIBLE_ITEM_4_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&406) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_5_CREATOR(&mut self, v: Guid) {
        self.header_set(408, v.guid() as u32);
        self.header_set(409, (v.guid() >> 32) as u32);
    }

    pub fn player_VISIBLE_ITEM_5_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&408);
        let upper = self.values.get(&409);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VISIBLE_ITEM_5_0(&mut self, v: i32) {
        self.header_set(410, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_VISIBLE_ITEM_5_0(&self) -> Option<i32> {
        self.values.get(&410).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_5_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(422, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_VISIBLE_ITEM_5_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&422) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_6_CREATOR(&mut self, v: Guid) {
        self.header_set(424, v.guid() as u32);
        self.header_set(425, (v.guid() >> 32) as u32);
    }

    pub fn player_VISIBLE_ITEM_6_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&424);
        let upper = self.values.get(&425);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VISIBLE_ITEM_6_0(&mut self, v: i32) {
        self.header_set(426, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_VISIBLE_ITEM_6_0(&self) -> Option<i32> {
        self.values.get(&426).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_6_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(438, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_VISIBLE_ITEM_6_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&438) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_7_CREATOR(&mut self, v: Guid) {
        self.header_set(440, v.guid() as u32);
        self.header_set(441, (v.guid() >> 32) as u32);
    }

    pub fn player_VISIBLE_ITEM_7_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&440);
        let upper = self.values.get(&441);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VISIBLE_ITEM_7_0(&mut self, v: i32) {
        self.header_set(442, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_VISIBLE_ITEM_7_0(&self) -> Option<i32> {
        self.values.get(&442).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_7_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(454, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_VISIBLE_ITEM_7_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&454) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_8_CREATOR(&mut self, v: Guid) {
        self.header_set(456, v.guid() as u32);
        self.header_set(457, (v.guid() >> 32) as u32);
    }

    pub fn player_VISIBLE_ITEM_8_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&456);
        let upper = self.values.get(&457);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VISIBLE_ITEM_8_0(&mut self, v: i32) {
        self.header_set(458, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_VISIBLE_ITEM_8_0(&self) -> Option<i32> {
        self.values.get(&458).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_8_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(470, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_VISIBLE_ITEM_8_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&470) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_9_CREATOR(&mut self, v: Guid) {
        self.header_set(472, v.guid() as u32);
        self.header_set(473, (v.guid() >> 32) as u32);
    }

    pub fn player_VISIBLE_ITEM_9_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&472);
        let upper = self.values.get(&473);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VISIBLE_ITEM_9_0(&mut self, v: i32) {
        self.header_set(474, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_VISIBLE_ITEM_9_0(&self) -> Option<i32> {
        self.values.get(&474).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_9_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(486, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_VISIBLE_ITEM_9_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&486) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_10_CREATOR(&mut self, v: Guid) {
        self.header_set(488, v.guid() as u32);
        self.header_set(489, (v.guid() >> 32) as u32);
    }

    pub fn player_VISIBLE_ITEM_10_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&488);
        let upper = self.values.get(&489);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VISIBLE_ITEM_10_0(&mut self, v: i32) {
        self.header_set(490, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_VISIBLE_ITEM_10_0(&self) -> Option<i32> {
        self.values.get(&490).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_10_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(502, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_VISIBLE_ITEM_10_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&502) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_11_CREATOR(&mut self, v: Guid) {
        self.header_set(504, v.guid() as u32);
        self.header_set(505, (v.guid() >> 32) as u32);
    }

    pub fn player_VISIBLE_ITEM_11_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&504);
        let upper = self.values.get(&505);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VISIBLE_ITEM_11_0(&mut self, v: i32) {
        self.header_set(506, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_VISIBLE_ITEM_11_0(&self) -> Option<i32> {
        self.values.get(&506).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_11_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(518, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_VISIBLE_ITEM_11_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&518) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_12_CREATOR(&mut self, v: Guid) {
        self.header_set(520, v.guid() as u32);
        self.header_set(521, (v.guid() >> 32) as u32);
    }

    pub fn player_VISIBLE_ITEM_12_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&520);
        let upper = self.values.get(&521);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VISIBLE_ITEM_12_0(&mut self, v: i32) {
        self.header_set(522, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_VISIBLE_ITEM_12_0(&self) -> Option<i32> {
        self.values.get(&522).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_12_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(534, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_VISIBLE_ITEM_12_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&534) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_13_CREATOR(&mut self, v: Guid) {
        self.header_set(536, v.guid() as u32);
        self.header_set(537, (v.guid() >> 32) as u32);
    }

    pub fn player_VISIBLE_ITEM_13_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&536);
        let upper = self.values.get(&537);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VISIBLE_ITEM_13_0(&mut self, v: i32) {
        self.header_set(538, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_VISIBLE_ITEM_13_0(&self) -> Option<i32> {
        self.values.get(&538).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_13_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(550, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_VISIBLE_ITEM_13_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&550) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_14_CREATOR(&mut self, v: Guid) {
        self.header_set(552, v.guid() as u32);
        self.header_set(553, (v.guid() >> 32) as u32);
    }

    pub fn player_VISIBLE_ITEM_14_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&552);
        let upper = self.values.get(&553);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VISIBLE_ITEM_14_0(&mut self, v: i32) {
        self.header_set(554, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_VISIBLE_ITEM_14_0(&self) -> Option<i32> {
        self.values.get(&554).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_14_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(566, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_VISIBLE_ITEM_14_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&566) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_15_CREATOR(&mut self, v: Guid) {
        self.header_set(568, v.guid() as u32);
        self.header_set(569, (v.guid() >> 32) as u32);
    }

    pub fn player_VISIBLE_ITEM_15_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&568);
        let upper = self.values.get(&569);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VISIBLE_ITEM_15_0(&mut self, v: i32) {
        self.header_set(570, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_VISIBLE_ITEM_15_0(&self) -> Option<i32> {
        self.values.get(&570).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_15_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(582, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_VISIBLE_ITEM_15_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&582) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_16_CREATOR(&mut self, v: Guid) {
        self.header_set(584, v.guid() as u32);
        self.header_set(585, (v.guid() >> 32) as u32);
    }

    pub fn player_VISIBLE_ITEM_16_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&584);
        let upper = self.values.get(&585);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VISIBLE_ITEM_16_0(&mut self, v: i32) {
        self.header_set(586, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_VISIBLE_ITEM_16_0(&self) -> Option<i32> {
        self.values.get(&586).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_16_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(598, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_VISIBLE_ITEM_16_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&598) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_17_CREATOR(&mut self, v: Guid) {
        self.header_set(600, v.guid() as u32);
        self.header_set(601, (v.guid() >> 32) as u32);
    }

    pub fn player_VISIBLE_ITEM_17_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&600);
        let upper = self.values.get(&601);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VISIBLE_ITEM_17_0(&mut self, v: i32) {
        self.header_set(602, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_VISIBLE_ITEM_17_0(&self) -> Option<i32> {
        self.values.get(&602).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_17_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(614, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_VISIBLE_ITEM_17_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&614) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_18_CREATOR(&mut self, v: Guid) {
        self.header_set(616, v.guid() as u32);
        self.header_set(617, (v.guid() >> 32) as u32);
    }

    pub fn player_VISIBLE_ITEM_18_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&616);
        let upper = self.values.get(&617);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VISIBLE_ITEM_18_0(&mut self, v: i32) {
        self.header_set(618, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_VISIBLE_ITEM_18_0(&self) -> Option<i32> {
        self.values.get(&618).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_18_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(630, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_VISIBLE_ITEM_18_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&630) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_19_CREATOR(&mut self, v: Guid) {
        self.header_set(632, v.guid() as u32);
        self.header_set(633, (v.guid() >> 32) as u32);
    }

    pub fn player_VISIBLE_ITEM_19_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&632);
        let upper = self.values.get(&633);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VISIBLE_ITEM_19_0(&mut self, v: i32) {
        self.header_set(634, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_VISIBLE_ITEM_19_0(&self) -> Option<i32> {
        self.values.get(&634).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_19_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(646, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_VISIBLE_ITEM_19_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&646) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_CHOSEN_TITLE(&mut self, v: i32) {
        self.header_set(648, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_CHOSEN_TITLE(&self) -> Option<i32> {
        self.values.get(&648).map(|v| *v as i32)
    }

    pub fn set_player_INV_SLOT_HEAD(&mut self, v: Guid) {
        self.header_set(650, v.guid() as u32);
        self.header_set(651, (v.guid() >> 32) as u32);
    }

    pub fn player_INV_SLOT_HEAD(&self) -> Option<Guid> {
        let lower = self.values.get(&650);
        let upper = self.values.get(&651);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_INV_SLOT_NECK(&mut self, v: Guid) {
        self.header_set(652, v.guid() as u32);
        self.header_set(653, (v.guid() >> 32) as u32);
    }

    pub fn player_INV_SLOT_NECK(&self) -> Option<Guid> {
        let lower = self.values.get(&652);
        let upper = self.values.get(&653);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_INV_SLOT_SHOULDERS(&mut self, v: Guid) {
        self.header_set(654, v.guid() as u32);
        self.header_set(655, (v.guid() >> 32) as u32);
    }

    pub fn player_INV_SLOT_SHOULDERS(&self) -> Option<Guid> {
        let lower = self.values.get(&654);
        let upper = self.values.get(&655);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_INV_SLOT_BODY(&mut self, v: Guid) {
        self.header_set(656, v.guid() as u32);
        self.header_set(657, (v.guid() >> 32) as u32);
    }

    pub fn player_INV_SLOT_BODY(&self) -> Option<Guid> {
        let lower = self.values.get(&656);
        let upper = self.values.get(&657);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_INV_SLOT_CHEST(&mut self, v: Guid) {
        self.header_set(658, v.guid() as u32);
        self.header_set(659, (v.guid() >> 32) as u32);
    }

    pub fn player_INV_SLOT_CHEST(&self) -> Option<Guid> {
        let lower = self.values.get(&658);
        let upper = self.values.get(&659);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_INV_SLOT_WAIST(&mut self, v: Guid) {
        self.header_set(660, v.guid() as u32);
        self.header_set(661, (v.guid() >> 32) as u32);
    }

    pub fn player_INV_SLOT_WAIST(&self) -> Option<Guid> {
        let lower = self.values.get(&660);
        let upper = self.values.get(&661);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_INV_SLOT_LEGS(&mut self, v: Guid) {
        self.header_set(662, v.guid() as u32);
        self.header_set(663, (v.guid() >> 32) as u32);
    }

    pub fn player_INV_SLOT_LEGS(&self) -> Option<Guid> {
        let lower = self.values.get(&662);
        let upper = self.values.get(&663);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_INV_SLOT_FEET(&mut self, v: Guid) {
        self.header_set(664, v.guid() as u32);
        self.header_set(665, (v.guid() >> 32) as u32);
    }

    pub fn player_INV_SLOT_FEET(&self) -> Option<Guid> {
        let lower = self.values.get(&664);
        let upper = self.values.get(&665);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_INV_SLOT_WRISTS(&mut self, v: Guid) {
        self.header_set(666, v.guid() as u32);
        self.header_set(667, (v.guid() >> 32) as u32);
    }

    pub fn player_INV_SLOT_WRISTS(&self) -> Option<Guid> {
        let lower = self.values.get(&666);
        let upper = self.values.get(&667);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_INV_SLOT_HANDS(&mut self, v: Guid) {
        self.header_set(668, v.guid() as u32);
        self.header_set(669, (v.guid() >> 32) as u32);
    }

    pub fn player_INV_SLOT_HANDS(&self) -> Option<Guid> {
        let lower = self.values.get(&668);
        let upper = self.values.get(&669);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_INV_SLOT_FINGER1(&mut self, v: Guid) {
        self.header_set(670, v.guid() as u32);
        self.header_set(671, (v.guid() >> 32) as u32);
    }

    pub fn player_INV_SLOT_FINGER1(&self) -> Option<Guid> {
        let lower = self.values.get(&670);
        let upper = self.values.get(&671);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_INV_SLOT_FINGER2(&mut self, v: Guid) {
        self.header_set(672, v.guid() as u32);
        self.header_set(673, (v.guid() >> 32) as u32);
    }

    pub fn player_INV_SLOT_FINGER2(&self) -> Option<Guid> {
        let lower = self.values.get(&672);
        let upper = self.values.get(&673);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_INV_SLOT_TRINKET1(&mut self, v: Guid) {
        self.header_set(674, v.guid() as u32);
        self.header_set(675, (v.guid() >> 32) as u32);
    }

    pub fn player_INV_SLOT_TRINKET1(&self) -> Option<Guid> {
        let lower = self.values.get(&674);
        let upper = self.values.get(&675);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_INV_SLOT_TRINKET2(&mut self, v: Guid) {
        self.header_set(676, v.guid() as u32);
        self.header_set(677, (v.guid() >> 32) as u32);
    }

    pub fn player_INV_SLOT_TRINKET2(&self) -> Option<Guid> {
        let lower = self.values.get(&676);
        let upper = self.values.get(&677);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_INV_SLOT_BACK(&mut self, v: Guid) {
        self.header_set(678, v.guid() as u32);
        self.header_set(679, (v.guid() >> 32) as u32);
    }

    pub fn player_INV_SLOT_BACK(&self) -> Option<Guid> {
        let lower = self.values.get(&678);
        let upper = self.values.get(&679);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_INV_SLOT_MAIN_HAND(&mut self, v: Guid) {
        self.header_set(680, v.guid() as u32);
        self.header_set(681, (v.guid() >> 32) as u32);
    }

    pub fn player_INV_SLOT_MAIN_HAND(&self) -> Option<Guid> {
        let lower = self.values.get(&680);
        let upper = self.values.get(&681);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_INV_SLOT_OFF_HAND(&mut self, v: Guid) {
        self.header_set(682, v.guid() as u32);
        self.header_set(683, (v.guid() >> 32) as u32);
    }

    pub fn player_INV_SLOT_OFF_HAND(&self) -> Option<Guid> {
        let lower = self.values.get(&682);
        let upper = self.values.get(&683);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_INV_SLOT_RANGED(&mut self, v: Guid) {
        self.header_set(684, v.guid() as u32);
        self.header_set(685, (v.guid() >> 32) as u32);
    }

    pub fn player_INV_SLOT_RANGED(&self) -> Option<Guid> {
        let lower = self.values.get(&684);
        let upper = self.values.get(&685);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_INV_SLOT_TABARD(&mut self, v: Guid) {
        self.header_set(686, v.guid() as u32);
        self.header_set(687, (v.guid() >> 32) as u32);
    }

    pub fn player_INV_SLOT_TABARD(&self) -> Option<Guid> {
        let lower = self.values.get(&686);
        let upper = self.values.get(&687);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_INV_SLOT_BAG1(&mut self, v: Guid) {
        self.header_set(688, v.guid() as u32);
        self.header_set(689, (v.guid() >> 32) as u32);
    }

    pub fn player_INV_SLOT_BAG1(&self) -> Option<Guid> {
        let lower = self.values.get(&688);
        let upper = self.values.get(&689);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_INV_SLOT_BAG2(&mut self, v: Guid) {
        self.header_set(690, v.guid() as u32);
        self.header_set(691, (v.guid() >> 32) as u32);
    }

    pub fn player_INV_SLOT_BAG2(&self) -> Option<Guid> {
        let lower = self.values.get(&690);
        let upper = self.values.get(&691);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_INV_SLOT_BAG3(&mut self, v: Guid) {
        self.header_set(692, v.guid() as u32);
        self.header_set(693, (v.guid() >> 32) as u32);
    }

    pub fn player_INV_SLOT_BAG3(&self) -> Option<Guid> {
        let lower = self.values.get(&692);
        let upper = self.values.get(&693);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_INV_SLOT_BAG4(&mut self, v: Guid) {
        self.header_set(694, v.guid() as u32);
        self.header_set(695, (v.guid() >> 32) as u32);
    }

    pub fn player_INV_SLOT_BAG4(&self) -> Option<Guid> {
        let lower = self.values.get(&694);
        let upper = self.values.get(&695);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_PACK_SLOT_1(&mut self, v: Guid) {
        self.header_set(696, v.guid() as u32);
        self.header_set(697, (v.guid() >> 32) as u32);
    }

    pub fn player_PACK_SLOT_1(&self) -> Option<Guid> {
        let lower = self.values.get(&696);
        let upper = self.values.get(&697);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_PACK_SLOT_2(&mut self, v: Guid) {
        self.header_set(698, v.guid() as u32);
        self.header_set(699, (v.guid() >> 32) as u32);
    }

    pub fn player_PACK_SLOT_2(&self) -> Option<Guid> {
        let lower = self.values.get(&698);
        let upper = self.values.get(&699);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_PACK_SLOT_3(&mut self, v: Guid) {
        self.header_set(700, v.guid() as u32);
        self.header_set(701, (v.guid() >> 32) as u32);
    }

    pub fn player_PACK_SLOT_3(&self) -> Option<Guid> {
        let lower = self.values.get(&700);
        let upper = self.values.get(&701);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_PACK_SLOT_4(&mut self, v: Guid) {
        self.header_set(702, v.guid() as u32);
        self.header_set(703, (v.guid() >> 32) as u32);
    }

    pub fn player_PACK_SLOT_4(&self) -> Option<Guid> {
        let lower = self.values.get(&702);
        let upper = self.values.get(&703);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_PACK_SLOT_5(&mut self, v: Guid) {
        self.header_set(704, v.guid() as u32);
        self.header_set(705, (v.guid() >> 32) as u32);
    }

    pub fn player_PACK_SLOT_5(&self) -> Option<Guid> {
        let lower = self.values.get(&704);
        let upper = self.values.get(&705);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_PACK_SLOT_6(&mut self, v: Guid) {
        self.header_set(706, v.guid() as u32);
        self.header_set(707, (v.guid() >> 32) as u32);
    }

    pub fn player_PACK_SLOT_6(&self) -> Option<Guid> {
        let lower = self.values.get(&706);
        let upper = self.values.get(&707);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_PACK_SLOT_7(&mut self, v: Guid) {
        self.header_set(708, v.guid() as u32);
        self.header_set(709, (v.guid() >> 32) as u32);
    }

    pub fn player_PACK_SLOT_7(&self) -> Option<Guid> {
        let lower = self.values.get(&708);
        let upper = self.values.get(&709);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_PACK_SLOT_8(&mut self, v: Guid) {
        self.header_set(710, v.guid() as u32);
        self.header_set(711, (v.guid() >> 32) as u32);
    }

    pub fn player_PACK_SLOT_8(&self) -> Option<Guid> {
        let lower = self.values.get(&710);
        let upper = self.values.get(&711);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_PACK_SLOT_9(&mut self, v: Guid) {
        self.header_set(712, v.guid() as u32);
        self.header_set(713, (v.guid() >> 32) as u32);
    }

    pub fn player_PACK_SLOT_9(&self) -> Option<Guid> {
        let lower = self.values.get(&712);
        let upper = self.values.get(&713);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_PACK_SLOT_10(&mut self, v: Guid) {
        self.header_set(714, v.guid() as u32);
        self.header_set(715, (v.guid() >> 32) as u32);
    }

    pub fn player_PACK_SLOT_10(&self) -> Option<Guid> {
        let lower = self.values.get(&714);
        let upper = self.values.get(&715);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_PACK_SLOT_11(&mut self, v: Guid) {
        self.header_set(716, v.guid() as u32);
        self.header_set(717, (v.guid() >> 32) as u32);
    }

    pub fn player_PACK_SLOT_11(&self) -> Option<Guid> {
        let lower = self.values.get(&716);
        let upper = self.values.get(&717);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_PACK_SLOT_12(&mut self, v: Guid) {
        self.header_set(718, v.guid() as u32);
        self.header_set(719, (v.guid() >> 32) as u32);
    }

    pub fn player_PACK_SLOT_12(&self) -> Option<Guid> {
        let lower = self.values.get(&718);
        let upper = self.values.get(&719);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_PACK_SLOT_13(&mut self, v: Guid) {
        self.header_set(720, v.guid() as u32);
        self.header_set(721, (v.guid() >> 32) as u32);
    }

    pub fn player_PACK_SLOT_13(&self) -> Option<Guid> {
        let lower = self.values.get(&720);
        let upper = self.values.get(&721);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_PACK_SLOT_14(&mut self, v: Guid) {
        self.header_set(722, v.guid() as u32);
        self.header_set(723, (v.guid() >> 32) as u32);
    }

    pub fn player_PACK_SLOT_14(&self) -> Option<Guid> {
        let lower = self.values.get(&722);
        let upper = self.values.get(&723);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_PACK_SLOT_15(&mut self, v: Guid) {
        self.header_set(724, v.guid() as u32);
        self.header_set(725, (v.guid() >> 32) as u32);
    }

    pub fn player_PACK_SLOT_15(&self) -> Option<Guid> {
        let lower = self.values.get(&724);
        let upper = self.values.get(&725);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_PACK_SLOT_16(&mut self, v: Guid) {
        self.header_set(726, v.guid() as u32);
        self.header_set(727, (v.guid() >> 32) as u32);
    }

    pub fn player_PACK_SLOT_16(&self) -> Option<Guid> {
        let lower = self.values.get(&726);
        let upper = self.values.get(&727);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_BANK_SLOT_1(&mut self, v: Guid) {
        self.header_set(728, v.guid() as u32);
        self.header_set(729, (v.guid() >> 32) as u32);
    }

    pub fn player_BANK_SLOT_1(&self) -> Option<Guid> {
        let lower = self.values.get(&728);
        let upper = self.values.get(&729);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_BANK_SLOT_2(&mut self, v: Guid) {
        self.header_set(730, v.guid() as u32);
        self.header_set(731, (v.guid() >> 32) as u32);
    }

    pub fn player_BANK_SLOT_2(&self) -> Option<Guid> {
        let lower = self.values.get(&730);
        let upper = self.values.get(&731);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_BANK_SLOT_3(&mut self, v: Guid) {
        self.header_set(732, v.guid() as u32);
        self.header_set(733, (v.guid() >> 32) as u32);
    }

    pub fn player_BANK_SLOT_3(&self) -> Option<Guid> {
        let lower = self.values.get(&732);
        let upper = self.values.get(&733);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_BANK_SLOT_4(&mut self, v: Guid) {
        self.header_set(734, v.guid() as u32);
        self.header_set(735, (v.guid() >> 32) as u32);
    }

    pub fn player_BANK_SLOT_4(&self) -> Option<Guid> {
        let lower = self.values.get(&734);
        let upper = self.values.get(&735);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_BANK_SLOT_5(&mut self, v: Guid) {
        self.header_set(736, v.guid() as u32);
        self.header_set(737, (v.guid() >> 32) as u32);
    }

    pub fn player_BANK_SLOT_5(&self) -> Option<Guid> {
        let lower = self.values.get(&736);
        let upper = self.values.get(&737);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_BANK_SLOT_6(&mut self, v: Guid) {
        self.header_set(738, v.guid() as u32);
        self.header_set(739, (v.guid() >> 32) as u32);
    }

    pub fn player_BANK_SLOT_6(&self) -> Option<Guid> {
        let lower = self.values.get(&738);
        let upper = self.values.get(&739);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_BANK_SLOT_7(&mut self, v: Guid) {
        self.header_set(740, v.guid() as u32);
        self.header_set(741, (v.guid() >> 32) as u32);
    }

    pub fn player_BANK_SLOT_7(&self) -> Option<Guid> {
        let lower = self.values.get(&740);
        let upper = self.values.get(&741);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_BANK_SLOT_8(&mut self, v: Guid) {
        self.header_set(742, v.guid() as u32);
        self.header_set(743, (v.guid() >> 32) as u32);
    }

    pub fn player_BANK_SLOT_8(&self) -> Option<Guid> {
        let lower = self.values.get(&742);
        let upper = self.values.get(&743);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_BANK_SLOT_9(&mut self, v: Guid) {
        self.header_set(744, v.guid() as u32);
        self.header_set(745, (v.guid() >> 32) as u32);
    }

    pub fn player_BANK_SLOT_9(&self) -> Option<Guid> {
        let lower = self.values.get(&744);
        let upper = self.values.get(&745);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_BANK_SLOT_10(&mut self, v: Guid) {
        self.header_set(746, v.guid() as u32);
        self.header_set(747, (v.guid() >> 32) as u32);
    }

    pub fn player_BANK_SLOT_10(&self) -> Option<Guid> {
        let lower = self.values.get(&746);
        let upper = self.values.get(&747);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_BANK_SLOT_11(&mut self, v: Guid) {
        self.header_set(748, v.guid() as u32);
        self.header_set(749, (v.guid() >> 32) as u32);
    }

    pub fn player_BANK_SLOT_11(&self) -> Option<Guid> {
        let lower = self.values.get(&748);
        let upper = self.values.get(&749);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_BANK_SLOT_12(&mut self, v: Guid) {
        self.header_set(750, v.guid() as u32);
        self.header_set(751, (v.guid() >> 32) as u32);
    }

    pub fn player_BANK_SLOT_12(&self) -> Option<Guid> {
        let lower = self.values.get(&750);
        let upper = self.values.get(&751);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_BANK_SLOT_13(&mut self, v: Guid) {
        self.header_set(752, v.guid() as u32);
        self.header_set(753, (v.guid() >> 32) as u32);
    }

    pub fn player_BANK_SLOT_13(&self) -> Option<Guid> {
        let lower = self.values.get(&752);
        let upper = self.values.get(&753);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_BANK_SLOT_14(&mut self, v: Guid) {
        self.header_set(754, v.guid() as u32);
        self.header_set(755, (v.guid() >> 32) as u32);
    }

    pub fn player_BANK_SLOT_14(&self) -> Option<Guid> {
        let lower = self.values.get(&754);
        let upper = self.values.get(&755);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_BANK_SLOT_15(&mut self, v: Guid) {
        self.header_set(756, v.guid() as u32);
        self.header_set(757, (v.guid() >> 32) as u32);
    }

    pub fn player_BANK_SLOT_15(&self) -> Option<Guid> {
        let lower = self.values.get(&756);
        let upper = self.values.get(&757);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_BANK_SLOT_16(&mut self, v: Guid) {
        self.header_set(758, v.guid() as u32);
        self.header_set(759, (v.guid() >> 32) as u32);
    }

    pub fn player_BANK_SLOT_16(&self) -> Option<Guid> {
        let lower = self.values.get(&758);
        let upper = self.values.get(&759);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_BANK_SLOT_17(&mut self, v: Guid) {
        self.header_set(760, v.guid() as u32);
        self.header_set(761, (v.guid() >> 32) as u32);
    }

    pub fn player_BANK_SLOT_17(&self) -> Option<Guid> {
        let lower = self.values.get(&760);
        let upper = self.values.get(&761);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_BANK_SLOT_18(&mut self, v: Guid) {
        self.header_set(762, v.guid() as u32);
        self.header_set(763, (v.guid() >> 32) as u32);
    }

    pub fn player_BANK_SLOT_18(&self) -> Option<Guid> {
        let lower = self.values.get(&762);
        let upper = self.values.get(&763);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_BANK_SLOT_19(&mut self, v: Guid) {
        self.header_set(764, v.guid() as u32);
        self.header_set(765, (v.guid() >> 32) as u32);
    }

    pub fn player_BANK_SLOT_19(&self) -> Option<Guid> {
        let lower = self.values.get(&764);
        let upper = self.values.get(&765);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_BANK_SLOT_20(&mut self, v: Guid) {
        self.header_set(766, v.guid() as u32);
        self.header_set(767, (v.guid() >> 32) as u32);
    }

    pub fn player_BANK_SLOT_20(&self) -> Option<Guid> {
        let lower = self.values.get(&766);
        let upper = self.values.get(&767);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_BANK_SLOT_21(&mut self, v: Guid) {
        self.header_set(768, v.guid() as u32);
        self.header_set(769, (v.guid() >> 32) as u32);
    }

    pub fn player_BANK_SLOT_21(&self) -> Option<Guid> {
        let lower = self.values.get(&768);
        let upper = self.values.get(&769);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_BANK_SLOT_22(&mut self, v: Guid) {
        self.header_set(770, v.guid() as u32);
        self.header_set(771, (v.guid() >> 32) as u32);
    }

    pub fn player_BANK_SLOT_22(&self) -> Option<Guid> {
        let lower = self.values.get(&770);
        let upper = self.values.get(&771);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_BANK_SLOT_23(&mut self, v: Guid) {
        self.header_set(772, v.guid() as u32);
        self.header_set(773, (v.guid() >> 32) as u32);
    }

    pub fn player_BANK_SLOT_23(&self) -> Option<Guid> {
        let lower = self.values.get(&772);
        let upper = self.values.get(&773);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_BANK_SLOT_24(&mut self, v: Guid) {
        self.header_set(774, v.guid() as u32);
        self.header_set(775, (v.guid() >> 32) as u32);
    }

    pub fn player_BANK_SLOT_24(&self) -> Option<Guid> {
        let lower = self.values.get(&774);
        let upper = self.values.get(&775);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_BANK_SLOT_25(&mut self, v: Guid) {
        self.header_set(776, v.guid() as u32);
        self.header_set(777, (v.guid() >> 32) as u32);
    }

    pub fn player_BANK_SLOT_25(&self) -> Option<Guid> {
        let lower = self.values.get(&776);
        let upper = self.values.get(&777);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_BANK_SLOT_26(&mut self, v: Guid) {
        self.header_set(778, v.guid() as u32);
        self.header_set(779, (v.guid() >> 32) as u32);
    }

    pub fn player_BANK_SLOT_26(&self) -> Option<Guid> {
        let lower = self.values.get(&778);
        let upper = self.values.get(&779);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_BANK_SLOT_27(&mut self, v: Guid) {
        self.header_set(780, v.guid() as u32);
        self.header_set(781, (v.guid() >> 32) as u32);
    }

    pub fn player_BANK_SLOT_27(&self) -> Option<Guid> {
        let lower = self.values.get(&780);
        let upper = self.values.get(&781);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_BANK_SLOT_28(&mut self, v: Guid) {
        self.header_set(782, v.guid() as u32);
        self.header_set(783, (v.guid() >> 32) as u32);
    }

    pub fn player_BANK_SLOT_28(&self) -> Option<Guid> {
        let lower = self.values.get(&782);
        let upper = self.values.get(&783);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_BANKBAG_SLOT_1(&mut self, v: Guid) {
        self.header_set(784, v.guid() as u32);
        self.header_set(785, (v.guid() >> 32) as u32);
    }

    pub fn player_BANKBAG_SLOT_1(&self) -> Option<Guid> {
        let lower = self.values.get(&784);
        let upper = self.values.get(&785);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_BANKBAG_SLOT_2(&mut self, v: Guid) {
        self.header_set(786, v.guid() as u32);
        self.header_set(787, (v.guid() >> 32) as u32);
    }

    pub fn player_BANKBAG_SLOT_2(&self) -> Option<Guid> {
        let lower = self.values.get(&786);
        let upper = self.values.get(&787);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_BANKBAG_SLOT_3(&mut self, v: Guid) {
        self.header_set(788, v.guid() as u32);
        self.header_set(789, (v.guid() >> 32) as u32);
    }

    pub fn player_BANKBAG_SLOT_3(&self) -> Option<Guid> {
        let lower = self.values.get(&788);
        let upper = self.values.get(&789);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_BANKBAG_SLOT_4(&mut self, v: Guid) {
        self.header_set(790, v.guid() as u32);
        self.header_set(791, (v.guid() >> 32) as u32);
    }

    pub fn player_BANKBAG_SLOT_4(&self) -> Option<Guid> {
        let lower = self.values.get(&790);
        let upper = self.values.get(&791);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_BANKBAG_SLOT_5(&mut self, v: Guid) {
        self.header_set(792, v.guid() as u32);
        self.header_set(793, (v.guid() >> 32) as u32);
    }

    pub fn player_BANKBAG_SLOT_5(&self) -> Option<Guid> {
        let lower = self.values.get(&792);
        let upper = self.values.get(&793);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_BANKBAG_SLOT_6(&mut self, v: Guid) {
        self.header_set(794, v.guid() as u32);
        self.header_set(795, (v.guid() >> 32) as u32);
    }

    pub fn player_BANKBAG_SLOT_6(&self) -> Option<Guid> {
        let lower = self.values.get(&794);
        let upper = self.values.get(&795);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_BANKBAG_SLOT_7(&mut self, v: Guid) {
        self.header_set(796, v.guid() as u32);
        self.header_set(797, (v.guid() >> 32) as u32);
    }

    pub fn player_BANKBAG_SLOT_7(&self) -> Option<Guid> {
        let lower = self.values.get(&796);
        let upper = self.values.get(&797);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VENDORBUYBACK_SLOT_1(&mut self, v: Guid) {
        self.header_set(798, v.guid() as u32);
        self.header_set(799, (v.guid() >> 32) as u32);
    }

    pub fn player_VENDORBUYBACK_SLOT_1(&self) -> Option<Guid> {
        let lower = self.values.get(&798);
        let upper = self.values.get(&799);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VENDORBUYBACK_SLOT_2(&mut self, v: Guid) {
        self.header_set(800, v.guid() as u32);
        self.header_set(801, (v.guid() >> 32) as u32);
    }

    pub fn player_VENDORBUYBACK_SLOT_2(&self) -> Option<Guid> {
        let lower = self.values.get(&800);
        let upper = self.values.get(&801);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VENDORBUYBACK_SLOT_3(&mut self, v: Guid) {
        self.header_set(802, v.guid() as u32);
        self.header_set(803, (v.guid() >> 32) as u32);
    }

    pub fn player_VENDORBUYBACK_SLOT_3(&self) -> Option<Guid> {
        let lower = self.values.get(&802);
        let upper = self.values.get(&803);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VENDORBUYBACK_SLOT_4(&mut self, v: Guid) {
        self.header_set(804, v.guid() as u32);
        self.header_set(805, (v.guid() >> 32) as u32);
    }

    pub fn player_VENDORBUYBACK_SLOT_4(&self) -> Option<Guid> {
        let lower = self.values.get(&804);
        let upper = self.values.get(&805);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VENDORBUYBACK_SLOT_5(&mut self, v: Guid) {
        self.header_set(806, v.guid() as u32);
        self.header_set(807, (v.guid() >> 32) as u32);
    }

    pub fn player_VENDORBUYBACK_SLOT_5(&self) -> Option<Guid> {
        let lower = self.values.get(&806);
        let upper = self.values.get(&807);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VENDORBUYBACK_SLOT_6(&mut self, v: Guid) {
        self.header_set(808, v.guid() as u32);
        self.header_set(809, (v.guid() >> 32) as u32);
    }

    pub fn player_VENDORBUYBACK_SLOT_6(&self) -> Option<Guid> {
        let lower = self.values.get(&808);
        let upper = self.values.get(&809);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VENDORBUYBACK_SLOT_7(&mut self, v: Guid) {
        self.header_set(810, v.guid() as u32);
        self.header_set(811, (v.guid() >> 32) as u32);
    }

    pub fn player_VENDORBUYBACK_SLOT_7(&self) -> Option<Guid> {
        let lower = self.values.get(&810);
        let upper = self.values.get(&811);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VENDORBUYBACK_SLOT_8(&mut self, v: Guid) {
        self.header_set(812, v.guid() as u32);
        self.header_set(813, (v.guid() >> 32) as u32);
    }

    pub fn player_VENDORBUYBACK_SLOT_8(&self) -> Option<Guid> {
        let lower = self.values.get(&812);
        let upper = self.values.get(&813);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VENDORBUYBACK_SLOT_9(&mut self, v: Guid) {
        self.header_set(814, v.guid() as u32);
        self.header_set(815, (v.guid() >> 32) as u32);
    }

    pub fn player_VENDORBUYBACK_SLOT_9(&self) -> Option<Guid> {
        let lower = self.values.get(&814);
        let upper = self.values.get(&815);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VENDORBUYBACK_SLOT_10(&mut self, v: Guid) {
        self.header_set(816, v.guid() as u32);
        self.header_set(817, (v.guid() >> 32) as u32);
    }

    pub fn player_VENDORBUYBACK_SLOT_10(&self) -> Option<Guid> {
        let lower = self.values.get(&816);
        let upper = self.values.get(&817);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VENDORBUYBACK_SLOT_11(&mut self, v: Guid) {
        self.header_set(818, v.guid() as u32);
        self.header_set(819, (v.guid() >> 32) as u32);
    }

    pub fn player_VENDORBUYBACK_SLOT_11(&self) -> Option<Guid> {
        let lower = self.values.get(&818);
        let upper = self.values.get(&819);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VENDORBUYBACK_SLOT_12(&mut self, v: Guid) {
        self.header_set(820, v.guid() as u32);
        self.header_set(821, (v.guid() >> 32) as u32);
    }

    pub fn player_VENDORBUYBACK_SLOT_12(&self) -> Option<Guid> {
        let lower = self.values.get(&820);
        let upper = self.values.get(&821);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_KEYRING_SLOT_1(&mut self, v: Guid) {
        self.header_set(822, v.guid() as u32);
        self.header_set(823, (v.guid() >> 32) as u32);
    }

    pub fn player_KEYRING_SLOT_1(&self) -> Option<Guid> {
        let lower = self.values.get(&822);
        let upper = self.values.get(&823);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_KEYRING_SLOT_2(&mut self, v: Guid) {
        self.header_set(824, v.guid() as u32);
        self.header_set(825, (v.guid() >> 32) as u32);
    }

    pub fn player_KEYRING_SLOT_2(&self) -> Option<Guid> {
        let lower = self.values.get(&824);
        let upper = self.values.get(&825);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_KEYRING_SLOT_3(&mut self, v: Guid) {
        self.header_set(826, v.guid() as u32);
        self.header_set(827, (v.guid() >> 32) as u32);
    }

    pub fn player_KEYRING_SLOT_3(&self) -> Option<Guid> {
        let lower = self.values.get(&826);
        let upper = self.values.get(&827);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_KEYRING_SLOT_4(&mut self, v: Guid) {
        self.header_set(828, v.guid() as u32);
        self.header_set(829, (v.guid() >> 32) as u32);
    }

    pub fn player_KEYRING_SLOT_4(&self) -> Option<Guid> {
        let lower = self.values.get(&828);
        let upper = self.values.get(&829);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_KEYRING_SLOT_5(&mut self, v: Guid) {
        self.header_set(830, v.guid() as u32);
        self.header_set(831, (v.guid() >> 32) as u32);
    }

    pub fn player_KEYRING_SLOT_5(&self) -> Option<Guid> {
        let lower = self.values.get(&830);
        let upper = self.values.get(&831);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_KEYRING_SLOT_6(&mut self, v: Guid) {
        self.header_set(832, v.guid() as u32);
        self.header_set(833, (v.guid() >> 32) as u32);
    }

    pub fn player_KEYRING_SLOT_6(&self) -> Option<Guid> {
        let lower = self.values.get(&832);
        let upper = self.values.get(&833);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_KEYRING_SLOT_7(&mut self, v: Guid) {
        self.header_set(834, v.guid() as u32);
        self.header_set(835, (v.guid() >> 32) as u32);
    }

    pub fn player_KEYRING_SLOT_7(&self) -> Option<Guid> {
        let lower = self.values.get(&834);
        let upper = self.values.get(&835);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_KEYRING_SLOT_8(&mut self, v: Guid) {
        self.header_set(836, v.guid() as u32);
        self.header_set(837, (v.guid() >> 32) as u32);
    }

    pub fn player_KEYRING_SLOT_8(&self) -> Option<Guid> {
        let lower = self.values.get(&836);
        let upper = self.values.get(&837);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_KEYRING_SLOT_9(&mut self, v: Guid) {
        self.header_set(838, v.guid() as u32);
        self.header_set(839, (v.guid() >> 32) as u32);
    }

    pub fn player_KEYRING_SLOT_9(&self) -> Option<Guid> {
        let lower = self.values.get(&838);
        let upper = self.values.get(&839);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_KEYRING_SLOT_10(&mut self, v: Guid) {
        self.header_set(840, v.guid() as u32);
        self.header_set(841, (v.guid() >> 32) as u32);
    }

    pub fn player_KEYRING_SLOT_10(&self) -> Option<Guid> {
        let lower = self.values.get(&840);
        let upper = self.values.get(&841);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_KEYRING_SLOT_11(&mut self, v: Guid) {
        self.header_set(842, v.guid() as u32);
        self.header_set(843, (v.guid() >> 32) as u32);
    }

    pub fn player_KEYRING_SLOT_11(&self) -> Option<Guid> {
        let lower = self.values.get(&842);
        let upper = self.values.get(&843);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_KEYRING_SLOT_12(&mut self, v: Guid) {
        self.header_set(844, v.guid() as u32);
        self.header_set(845, (v.guid() >> 32) as u32);
    }

    pub fn player_KEYRING_SLOT_12(&self) -> Option<Guid> {
        let lower = self.values.get(&844);
        let upper = self.values.get(&845);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_KEYRING_SLOT_13(&mut self, v: Guid) {
        self.header_set(846, v.guid() as u32);
        self.header_set(847, (v.guid() >> 32) as u32);
    }

    pub fn player_KEYRING_SLOT_13(&self) -> Option<Guid> {
        let lower = self.values.get(&846);
        let upper = self.values.get(&847);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_KEYRING_SLOT_14(&mut self, v: Guid) {
        self.header_set(848, v.guid() as u32);
        self.header_set(849, (v.guid() >> 32) as u32);
    }

    pub fn player_KEYRING_SLOT_14(&self) -> Option<Guid> {
        let lower = self.values.get(&848);
        let upper = self.values.get(&849);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_KEYRING_SLOT_15(&mut self, v: Guid) {
        self.header_set(850, v.guid() as u32);
        self.header_set(851, (v.guid() >> 32) as u32);
    }

    pub fn player_KEYRING_SLOT_15(&self) -> Option<Guid> {
        let lower = self.values.get(&850);
        let upper = self.values.get(&851);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_KEYRING_SLOT_16(&mut self, v: Guid) {
        self.header_set(852, v.guid() as u32);
        self.header_set(853, (v.guid() >> 32) as u32);
    }

    pub fn player_KEYRING_SLOT_16(&self) -> Option<Guid> {
        let lower = self.values.get(&852);
        let upper = self.values.get(&853);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_KEYRING_SLOT_17(&mut self, v: Guid) {
        self.header_set(854, v.guid() as u32);
        self.header_set(855, (v.guid() >> 32) as u32);
    }

    pub fn player_KEYRING_SLOT_17(&self) -> Option<Guid> {
        let lower = self.values.get(&854);
        let upper = self.values.get(&855);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_KEYRING_SLOT_18(&mut self, v: Guid) {
        self.header_set(856, v.guid() as u32);
        self.header_set(857, (v.guid() >> 32) as u32);
    }

    pub fn player_KEYRING_SLOT_18(&self) -> Option<Guid> {
        let lower = self.values.get(&856);
        let upper = self.values.get(&857);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_KEYRING_SLOT_19(&mut self, v: Guid) {
        self.header_set(858, v.guid() as u32);
        self.header_set(859, (v.guid() >> 32) as u32);
    }

    pub fn player_KEYRING_SLOT_19(&self) -> Option<Guid> {
        let lower = self.values.get(&858);
        let upper = self.values.get(&859);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_KEYRING_SLOT_20(&mut self, v: Guid) {
        self.header_set(860, v.guid() as u32);
        self.header_set(861, (v.guid() >> 32) as u32);
    }

    pub fn player_KEYRING_SLOT_20(&self) -> Option<Guid> {
        let lower = self.values.get(&860);
        let upper = self.values.get(&861);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_KEYRING_SLOT_21(&mut self, v: Guid) {
        self.header_set(862, v.guid() as u32);
        self.header_set(863, (v.guid() >> 32) as u32);
    }

    pub fn player_KEYRING_SLOT_21(&self) -> Option<Guid> {
        let lower = self.values.get(&862);
        let upper = self.values.get(&863);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_KEYRING_SLOT_22(&mut self, v: Guid) {
        self.header_set(864, v.guid() as u32);
        self.header_set(865, (v.guid() >> 32) as u32);
    }

    pub fn player_KEYRING_SLOT_22(&self) -> Option<Guid> {
        let lower = self.values.get(&864);
        let upper = self.values.get(&865);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_KEYRING_SLOT_23(&mut self, v: Guid) {
        self.header_set(866, v.guid() as u32);
        self.header_set(867, (v.guid() >> 32) as u32);
    }

    pub fn player_KEYRING_SLOT_23(&self) -> Option<Guid> {
        let lower = self.values.get(&866);
        let upper = self.values.get(&867);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_KEYRING_SLOT_24(&mut self, v: Guid) {
        self.header_set(868, v.guid() as u32);
        self.header_set(869, (v.guid() >> 32) as u32);
    }

    pub fn player_KEYRING_SLOT_24(&self) -> Option<Guid> {
        let lower = self.values.get(&868);
        let upper = self.values.get(&869);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_KEYRING_SLOT_25(&mut self, v: Guid) {
        self.header_set(870, v.guid() as u32);
        self.header_set(871, (v.guid() >> 32) as u32);
    }

    pub fn player_KEYRING_SLOT_25(&self) -> Option<Guid> {
        let lower = self.values.get(&870);
        let upper = self.values.get(&871);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_KEYRING_SLOT_26(&mut self, v: Guid) {
        self.header_set(872, v.guid() as u32);
        self.header_set(873, (v.guid() >> 32) as u32);
    }

    pub fn player_KEYRING_SLOT_26(&self) -> Option<Guid> {
        let lower = self.values.get(&872);
        let upper = self.values.get(&873);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_KEYRING_SLOT_27(&mut self, v: Guid) {
        self.header_set(874, v.guid() as u32);
        self.header_set(875, (v.guid() >> 32) as u32);
    }

    pub fn player_KEYRING_SLOT_27(&self) -> Option<Guid> {
        let lower = self.values.get(&874);
        let upper = self.values.get(&875);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_KEYRING_SLOT_28(&mut self, v: Guid) {
        self.header_set(876, v.guid() as u32);
        self.header_set(877, (v.guid() >> 32) as u32);
    }

    pub fn player_KEYRING_SLOT_28(&self) -> Option<Guid> {
        let lower = self.values.get(&876);
        let upper = self.values.get(&877);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_KEYRING_SLOT_29(&mut self, v: Guid) {
        self.header_set(878, v.guid() as u32);
        self.header_set(879, (v.guid() >> 32) as u32);
    }

    pub fn player_KEYRING_SLOT_29(&self) -> Option<Guid> {
        let lower = self.values.get(&878);
        let upper = self.values.get(&879);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_KEYRING_SLOT_30(&mut self, v: Guid) {
        self.header_set(880, v.guid() as u32);
        self.header_set(881, (v.guid() >> 32) as u32);
    }

    pub fn player_KEYRING_SLOT_30(&self) -> Option<Guid> {
        let lower = self.values.get(&880);
        let upper = self.values.get(&881);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_KEYRING_SLOT_31(&mut self, v: Guid) {
        self.header_set(882, v.guid() as u32);
        self.header_set(883, (v.guid() >> 32) as u32);
    }

    pub fn player_KEYRING_SLOT_31(&self) -> Option<Guid> {
        let lower = self.values.get(&882);
        let upper = self.values.get(&883);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_KEYRING_SLOT_32(&mut self, v: Guid) {
        self.header_set(884, v.guid() as u32);
        self.header_set(885, (v.guid() >> 32) as u32);
    }

    pub fn player_KEYRING_SLOT_32(&self) -> Option<Guid> {
        let lower = self.values.get(&884);
        let upper = self.values.get(&885);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VANITYPET_SLOT_1(&mut self, v: Guid) {
        self.header_set(886, v.guid() as u32);
        self.header_set(887, (v.guid() >> 32) as u32);
    }

    pub fn player_VANITYPET_SLOT_1(&self) -> Option<Guid> {
        let lower = self.values.get(&886);
        let upper = self.values.get(&887);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VANITYPET_SLOT_2(&mut self, v: Guid) {
        self.header_set(888, v.guid() as u32);
        self.header_set(889, (v.guid() >> 32) as u32);
    }

    pub fn player_VANITYPET_SLOT_2(&self) -> Option<Guid> {
        let lower = self.values.get(&888);
        let upper = self.values.get(&889);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VANITYPET_SLOT_3(&mut self, v: Guid) {
        self.header_set(890, v.guid() as u32);
        self.header_set(891, (v.guid() >> 32) as u32);
    }

    pub fn player_VANITYPET_SLOT_3(&self) -> Option<Guid> {
        let lower = self.values.get(&890);
        let upper = self.values.get(&891);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VANITYPET_SLOT_4(&mut self, v: Guid) {
        self.header_set(892, v.guid() as u32);
        self.header_set(893, (v.guid() >> 32) as u32);
    }

    pub fn player_VANITYPET_SLOT_4(&self) -> Option<Guid> {
        let lower = self.values.get(&892);
        let upper = self.values.get(&893);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VANITYPET_SLOT_5(&mut self, v: Guid) {
        self.header_set(894, v.guid() as u32);
        self.header_set(895, (v.guid() >> 32) as u32);
    }

    pub fn player_VANITYPET_SLOT_5(&self) -> Option<Guid> {
        let lower = self.values.get(&894);
        let upper = self.values.get(&895);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VANITYPET_SLOT_6(&mut self, v: Guid) {
        self.header_set(896, v.guid() as u32);
        self.header_set(897, (v.guid() >> 32) as u32);
    }

    pub fn player_VANITYPET_SLOT_6(&self) -> Option<Guid> {
        let lower = self.values.get(&896);
        let upper = self.values.get(&897);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VANITYPET_SLOT_7(&mut self, v: Guid) {
        self.header_set(898, v.guid() as u32);
        self.header_set(899, (v.guid() >> 32) as u32);
    }

    pub fn player_VANITYPET_SLOT_7(&self) -> Option<Guid> {
        let lower = self.values.get(&898);
        let upper = self.values.get(&899);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VANITYPET_SLOT_8(&mut self, v: Guid) {
        self.header_set(900, v.guid() as u32);
        self.header_set(901, (v.guid() >> 32) as u32);
    }

    pub fn player_VANITYPET_SLOT_8(&self) -> Option<Guid> {
        let lower = self.values.get(&900);
        let upper = self.values.get(&901);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VANITYPET_SLOT_9(&mut self, v: Guid) {
        self.header_set(902, v.guid() as u32);
        self.header_set(903, (v.guid() >> 32) as u32);
    }

    pub fn player_VANITYPET_SLOT_9(&self) -> Option<Guid> {
        let lower = self.values.get(&902);
        let upper = self.values.get(&903);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VANITYPET_SLOT_10(&mut self, v: Guid) {
        self.header_set(904, v.guid() as u32);
        self.header_set(905, (v.guid() >> 32) as u32);
    }

    pub fn player_VANITYPET_SLOT_10(&self) -> Option<Guid> {
        let lower = self.values.get(&904);
        let upper = self.values.get(&905);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VANITYPET_SLOT_11(&mut self, v: Guid) {
        self.header_set(906, v.guid() as u32);
        self.header_set(907, (v.guid() >> 32) as u32);
    }

    pub fn player_VANITYPET_SLOT_11(&self) -> Option<Guid> {
        let lower = self.values.get(&906);
        let upper = self.values.get(&907);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VANITYPET_SLOT_12(&mut self, v: Guid) {
        self.header_set(908, v.guid() as u32);
        self.header_set(909, (v.guid() >> 32) as u32);
    }

    pub fn player_VANITYPET_SLOT_12(&self) -> Option<Guid> {
        let lower = self.values.get(&908);
        let upper = self.values.get(&909);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VANITYPET_SLOT_13(&mut self, v: Guid) {
        self.header_set(910, v.guid() as u32);
        self.header_set(911, (v.guid() >> 32) as u32);
    }

    pub fn player_VANITYPET_SLOT_13(&self) -> Option<Guid> {
        let lower = self.values.get(&910);
        let upper = self.values.get(&911);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VANITYPET_SLOT_14(&mut self, v: Guid) {
        self.header_set(912, v.guid() as u32);
        self.header_set(913, (v.guid() >> 32) as u32);
    }

    pub fn player_VANITYPET_SLOT_14(&self) -> Option<Guid> {
        let lower = self.values.get(&912);
        let upper = self.values.get(&913);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VANITYPET_SLOT_15(&mut self, v: Guid) {
        self.header_set(914, v.guid() as u32);
        self.header_set(915, (v.guid() >> 32) as u32);
    }

    pub fn player_VANITYPET_SLOT_15(&self) -> Option<Guid> {
        let lower = self.values.get(&914);
        let upper = self.values.get(&915);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VANITYPET_SLOT_16(&mut self, v: Guid) {
        self.header_set(916, v.guid() as u32);
        self.header_set(917, (v.guid() >> 32) as u32);
    }

    pub fn player_VANITYPET_SLOT_16(&self) -> Option<Guid> {
        let lower = self.values.get(&916);
        let upper = self.values.get(&917);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VANITYPET_SLOT_17(&mut self, v: Guid) {
        self.header_set(918, v.guid() as u32);
        self.header_set(919, (v.guid() >> 32) as u32);
    }

    pub fn player_VANITYPET_SLOT_17(&self) -> Option<Guid> {
        let lower = self.values.get(&918);
        let upper = self.values.get(&919);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VANITYPET_SLOT_18(&mut self, v: Guid) {
        self.header_set(920, v.guid() as u32);
        self.header_set(921, (v.guid() >> 32) as u32);
    }

    pub fn player_VANITYPET_SLOT_18(&self) -> Option<Guid> {
        let lower = self.values.get(&920);
        let upper = self.values.get(&921);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FARSIGHT(&mut self, v: Guid) {
        self.header_set(922, v.guid() as u32);
        self.header_set(923, (v.guid() >> 32) as u32);
    }

    pub fn player_FARSIGHT(&self) -> Option<Guid> {
        let lower = self.values.get(&922);
        let upper = self.values.get(&923);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_KNOWN_TITLES(&mut self, v: Guid) {
        self.header_set(924, v.guid() as u32);
        self.header_set(925, (v.guid() >> 32) as u32);
    }

    pub fn player_KNOWN_TITLES(&self) -> Option<Guid> {
        let lower = self.values.get(&924);
        let upper = self.values.get(&925);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_XP(&mut self, v: i32) {
        self.header_set(926, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_XP(&self) -> Option<i32> {
        self.values.get(&926).map(|v| *v as i32)
    }

    pub fn set_player_NEXT_LEVEL_XP(&mut self, v: i32) {
        self.header_set(927, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_NEXT_LEVEL_XP(&self) -> Option<i32> {
        self.values.get(&927).map(|v| *v as i32)
    }

    pub fn set_player_SKILL_INFO(&mut self, skill_info: crate::tbc::SkillInfo, index: SkillInfoIndex) {
        for (index, value) in skill_info.mask_values(index) {
            self.header_set(index, value);
        }
    }

    pub fn player_SKILL_INFO(&self, index: SkillInfoIndex) -> Option<crate::tbc::SkillInfo> {
        crate::tbc::SkillInfo::from_range(self.values.range(index.first()..=index.last()))
    }

    pub fn set_player_CHARACTER_POINTS1(&mut self, v: i32) {
        self.header_set(1312, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_CHARACTER_POINTS1(&self) -> Option<i32> {
        self.values.get(&1312).map(|v| *v as i32)
    }

    pub fn set_player_CHARACTER_POINTS2(&mut self, v: i32) {
        self.header_set(1313, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_CHARACTER_POINTS2(&self) -> Option<i32> {
        self.values.get(&1313).map(|v| *v as i32)
    }

    pub fn set_player_TRACK_CREATURES(&mut self, v: i32) {
        self.header_set(1314, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_TRACK_CREATURES(&self) -> Option<i32> {
        self.values.get(&1314).map(|v| *v as i32)
    }

    pub fn set_player_TRACK_RESOURCES(&mut self, v: i32) {
        self.header_set(1315, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_TRACK_RESOURCES(&self) -> Option<i32> {
        self.values.get(&1315).map(|v| *v as i32)
    }

    pub fn set_player_BLOCK_PERCENTAGE(&mut self, v: f32) {
        self.header_set(1316, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_BLOCK_PERCENTAGE(&self) -> Option<f32> {
        self.values.get(&1316).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_DODGE_PERCENTAGE(&mut self, v: f32) {
        self.header_set(1317, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_DODGE_PERCENTAGE(&self) -> Option<f32> {
        self.values.get(&1317).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_PARRY_PERCENTAGE(&mut self, v: f32) {
        self.header_set(1318, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_PARRY_PERCENTAGE(&self) -> Option<f32> {
        self.values.get(&1318).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_EXPERTISE(&mut self, v: i32) {
        self.header_set(1319, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_EXPERTISE(&self) -> Option<i32> {
        self.values.get(&1319).map(|v| *v as i32)
    }

    pub fn set_player_OFFHAND_EXPERTISE(&mut self, v: i32) {
        self.header_set(1320, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_OFFHAND_EXPERTISE(&self) -> Option<i32> {
        self.values.get(&1320).map(|v| *v as i32)
    }

    pub fn set_player_CRIT_PERCENTAGE(&mut self, v: f32) {
        self.header_set(1321, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_CRIT_PERCENTAGE(&self) -> Option<f32> {
        self.values.get(&1321).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_RANGED_CRIT_PERCENTAGE(&mut self, v: f32) {
        self.header_set(1322, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_RANGED_CRIT_PERCENTAGE(&self) -> Option<f32> {
        self.values.get(&1322).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_OFFHAND_CRIT_PERCENTAGE(&mut self, v: f32) {
        self.header_set(1323, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_OFFHAND_CRIT_PERCENTAGE(&self) -> Option<f32> {
        self.values.get(&1323).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_SPELL_CRIT_PERCENTAGE1(&mut self, v: f32) {
        self.header_set(1324, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_SPELL_CRIT_PERCENTAGE1(&self) -> Option<f32> {
        self.values.get(&1324).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_SHIELD_BLOCK(&mut self, v: i32) {
        self.header_set(1331, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_SHIELD_BLOCK(&self) -> Option<i32> {
        self.values.get(&1331).map(|v| *v as i32)
    }

    pub fn set_player_EXPLORED_ZONES_1(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(1332, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_EXPLORED_ZONES_1(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&1332) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_REST_STATE_EXPERIENCE(&mut self, v: i32) {
        self.header_set(1460, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_REST_STATE_EXPERIENCE(&self) -> Option<i32> {
        self.values.get(&1460).map(|v| *v as i32)
    }

    pub fn set_player_COINAGE(&mut self, v: i32) {
        self.header_set(1461, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_COINAGE(&self) -> Option<i32> {
        self.values.get(&1461).map(|v| *v as i32)
    }

    pub fn set_player_MOD_DAMAGE_DONE_POS(&mut self, v: i32) {
        self.header_set(1462, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_MOD_DAMAGE_DONE_POS(&self) -> Option<i32> {
        self.values.get(&1462).map(|v| *v as i32)
    }

    pub fn set_player_MOD_DAMAGE_DONE_NEG(&mut self, v: i32) {
        self.header_set(1469, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_MOD_DAMAGE_DONE_NEG(&self) -> Option<i32> {
        self.values.get(&1469).map(|v| *v as i32)
    }

    pub fn set_player_MOD_DAMAGE_DONE_PCT(&mut self, v: i32) {
        self.header_set(1476, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_MOD_DAMAGE_DONE_PCT(&self) -> Option<i32> {
        self.values.get(&1476).map(|v| *v as i32)
    }

    pub fn set_player_MOD_HEALING_DONE_POS(&mut self, v: i32) {
        self.header_set(1483, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_MOD_HEALING_DONE_POS(&self) -> Option<i32> {
        self.values.get(&1483).map(|v| *v as i32)
    }

    pub fn set_player_MOD_TARGET_RESISTANCE(&mut self, v: i32) {
        self.header_set(1484, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_MOD_TARGET_RESISTANCE(&self) -> Option<i32> {
        self.values.get(&1484).map(|v| *v as i32)
    }

    pub fn set_player_MOD_TARGET_PHYSICAL_RESISTANCE(&mut self, v: i32) {
        self.header_set(1485, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_MOD_TARGET_PHYSICAL_RESISTANCE(&self) -> Option<i32> {
        self.values.get(&1485).map(|v| *v as i32)
    }

    pub fn set_player_FEATURES(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(1486, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_FEATURES(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&1486) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_AMMO_ID(&mut self, v: i32) {
        self.header_set(1487, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_AMMO_ID(&self) -> Option<i32> {
        self.values.get(&1487).map(|v| *v as i32)
    }

    pub fn set_player_SELF_RES_SPELL(&mut self, v: i32) {
        self.header_set(1488, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_SELF_RES_SPELL(&self) -> Option<i32> {
        self.values.get(&1488).map(|v| *v as i32)
    }

    pub fn set_player_PVP_MEDALS(&mut self, v: i32) {
        self.header_set(1489, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_PVP_MEDALS(&self) -> Option<i32> {
        self.values.get(&1489).map(|v| *v as i32)
    }

    pub fn set_player_BUYBACK_PRICE_1(&mut self, v: i32) {
        self.header_set(1490, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_BUYBACK_PRICE_1(&self) -> Option<i32> {
        self.values.get(&1490).map(|v| *v as i32)
    }

    pub fn set_player_BUYBACK_TIMESTAMP_1(&mut self, v: i32) {
        self.header_set(1502, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_BUYBACK_TIMESTAMP_1(&self) -> Option<i32> {
        self.values.get(&1502).map(|v| *v as i32)
    }

    pub fn set_player_KILLS(&mut self, a: u16, b: u16) {
        self.header_set(1514, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_KILLS(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&1514) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_TODAY_CONTRIBUTION(&mut self, v: i32) {
        self.header_set(1515, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_TODAY_CONTRIBUTION(&self) -> Option<i32> {
        self.values.get(&1515).map(|v| *v as i32)
    }

    pub fn set_player_YESTERDAY_CONTRIBUTION(&mut self, v: i32) {
        self.header_set(1516, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_YESTERDAY_CONTRIBUTION(&self) -> Option<i32> {
        self.values.get(&1516).map(|v| *v as i32)
    }

    pub fn set_player_LIFETIME_HONORABLE_KILLS(&mut self, v: i32) {
        self.header_set(1517, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_LIFETIME_HONORABLE_KILLS(&self) -> Option<i32> {
        self.values.get(&1517).map(|v| *v as i32)
    }

    pub fn set_player_BYTES2(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(1518, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_BYTES2(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&1518) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_WATCHED_FACTION_INDEX(&mut self, v: i32) {
        self.header_set(1519, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_WATCHED_FACTION_INDEX(&self) -> Option<i32> {
        self.values.get(&1519).map(|v| *v as i32)
    }

    pub fn set_player_COMBAT_RATING_1(&mut self, v: i32) {
        self.header_set(1520, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_COMBAT_RATING_1(&self) -> Option<i32> {
        self.values.get(&1520).map(|v| *v as i32)
    }

    pub fn set_player_ARENA_TEAM_INFO_1_1(&mut self, v: i32) {
        self.header_set(1544, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_ARENA_TEAM_INFO_1_1(&self) -> Option<i32> {
        self.values.get(&1544).map(|v| *v as i32)
    }

    pub fn set_player_HONOR_CURRENCY(&mut self, v: i32) {
        self.header_set(1562, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_HONOR_CURRENCY(&self) -> Option<i32> {
        self.values.get(&1562).map(|v| *v as i32)
    }

    pub fn set_player_ARENA_CURRENCY(&mut self, v: i32) {
        self.header_set(1563, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_ARENA_CURRENCY(&self) -> Option<i32> {
        self.values.get(&1563).map(|v| *v as i32)
    }

    pub fn set_player_MOD_MANA_REGEN(&mut self, v: f32) {
        self.header_set(1564, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_MOD_MANA_REGEN(&self) -> Option<f32> {
        self.values.get(&1564).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_MOD_MANA_REGEN_INTERRUPT(&mut self, v: f32) {
        self.header_set(1565, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_MOD_MANA_REGEN_INTERRUPT(&self) -> Option<f32> {
        self.values.get(&1565).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_MAX_LEVEL(&mut self, v: i32) {
        self.header_set(1566, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_MAX_LEVEL(&self) -> Option<i32> {
        self.values.get(&1566).map(|v| *v as i32)
    }

    pub fn set_player_DAILY_QUESTS_1(&mut self, v: i32) {
        self.header_set(1567, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_DAILY_QUESTS_1(&self) -> Option<i32> {
        self.values.get(&1567).map(|v| *v as i32)
    }

    pub fn set_object_CREATED_BY(&mut self, v: Guid) {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
    }

    pub fn object_CREATED_BY(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

}

impl UpdateGameObject {
    pub fn set_object_GUID(&mut self, v: Guid) {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
    }

    pub fn object_GUID(&self) -> Option<Guid> {
        let lower = self.values.get(&0);
        let upper = self.values.get(&1);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_object_TYPE(&mut self, v: i32) {
        self.header_set(2, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_TYPE(&self) -> Option<i32> {
        self.values.get(&2).map(|v| *v as i32)
    }

    pub fn set_object_ENTRY(&mut self, v: i32) {
        self.header_set(3, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_ENTRY(&self) -> Option<i32> {
        self.values.get(&3).map(|v| *v as i32)
    }

    pub fn set_object_SCALE_X(&mut self, v: f32) {
        self.header_set(4, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_SCALE_X(&self) -> Option<f32> {
        self.values.get(&4).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_object_CREATED_BY(&mut self, v: Guid) {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
    }

    pub fn object_CREATED_BY(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_gameobject_DISPLAYID(&mut self, v: i32) {
        self.header_set(8, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_DISPLAYID(&self) -> Option<i32> {
        self.values.get(&8).map(|v| *v as i32)
    }

    pub fn set_gameobject_FLAGS(&mut self, v: i32) {
        self.header_set(9, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_FLAGS(&self) -> Option<i32> {
        self.values.get(&9).map(|v| *v as i32)
    }

    pub fn set_gameobject_ROTATION(&mut self, v: f32) {
        self.header_set(10, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_ROTATION(&self) -> Option<f32> {
        self.values.get(&10).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_gameobject_STATE(&mut self, v: i32) {
        self.header_set(14, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_STATE(&self) -> Option<i32> {
        self.values.get(&14).map(|v| *v as i32)
    }

    pub fn set_gameobject_POS_X(&mut self, v: f32) {
        self.header_set(15, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_POS_X(&self) -> Option<f32> {
        self.values.get(&15).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_gameobject_POS_Y(&mut self, v: f32) {
        self.header_set(16, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_POS_Y(&self) -> Option<f32> {
        self.values.get(&16).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_gameobject_POS_Z(&mut self, v: f32) {
        self.header_set(17, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_POS_Z(&self) -> Option<f32> {
        self.values.get(&17).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_gameobject_FACING(&mut self, v: f32) {
        self.header_set(18, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_FACING(&self) -> Option<f32> {
        self.values.get(&18).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_gameobject_DYN_FLAGS(&mut self, v: i32) {
        self.header_set(19, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_DYN_FLAGS(&self) -> Option<i32> {
        self.values.get(&19).map(|v| *v as i32)
    }

    pub fn set_gameobject_FACTION(&mut self, v: i32) {
        self.header_set(20, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_FACTION(&self) -> Option<i32> {
        self.values.get(&20).map(|v| *v as i32)
    }

    pub fn set_gameobject_TYPE_ID(&mut self, v: i32) {
        self.header_set(21, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_TYPE_ID(&self) -> Option<i32> {
        self.values.get(&21).map(|v| *v as i32)
    }

    pub fn set_gameobject_LEVEL(&mut self, v: i32) {
        self.header_set(22, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_LEVEL(&self) -> Option<i32> {
        self.values.get(&22).map(|v| *v as i32)
    }

    pub fn set_gameobject_ARTKIT(&mut self, v: i32) {
        self.header_set(23, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_ARTKIT(&self) -> Option<i32> {
        self.values.get(&23).map(|v| *v as i32)
    }

    pub fn set_gameobject_ANIMPROGRESS(&mut self, v: i32) {
        self.header_set(24, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_ANIMPROGRESS(&self) -> Option<i32> {
        self.values.get(&24).map(|v| *v as i32)
    }

}

impl UpdateDynamicObject {
    pub fn set_object_GUID(&mut self, v: Guid) {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
    }

    pub fn object_GUID(&self) -> Option<Guid> {
        let lower = self.values.get(&0);
        let upper = self.values.get(&1);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_object_TYPE(&mut self, v: i32) {
        self.header_set(2, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_TYPE(&self) -> Option<i32> {
        self.values.get(&2).map(|v| *v as i32)
    }

    pub fn set_object_ENTRY(&mut self, v: i32) {
        self.header_set(3, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_ENTRY(&self) -> Option<i32> {
        self.values.get(&3).map(|v| *v as i32)
    }

    pub fn set_object_SCALE_X(&mut self, v: f32) {
        self.header_set(4, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_SCALE_X(&self) -> Option<f32> {
        self.values.get(&4).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_object_CREATED_BY(&mut self, v: Guid) {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
    }

    pub fn object_CREATED_BY(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_dynamicobject_CASTER(&mut self, v: Guid) {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
    }

    pub fn dynamicobject_CASTER(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_dynamicobject_BYTES(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(8, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn dynamicobject_BYTES(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&8) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_dynamicobject_SPELLID(&mut self, v: i32) {
        self.header_set(9, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn dynamicobject_SPELLID(&self) -> Option<i32> {
        self.values.get(&9).map(|v| *v as i32)
    }

    pub fn set_dynamicobject_RADIUS(&mut self, v: f32) {
        self.header_set(10, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn dynamicobject_RADIUS(&self) -> Option<f32> {
        self.values.get(&10).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_dynamicobject_POS_X(&mut self, v: f32) {
        self.header_set(11, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn dynamicobject_POS_X(&self) -> Option<f32> {
        self.values.get(&11).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_dynamicobject_POS_Y(&mut self, v: f32) {
        self.header_set(12, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn dynamicobject_POS_Y(&self) -> Option<f32> {
        self.values.get(&12).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_dynamicobject_POS_Z(&mut self, v: f32) {
        self.header_set(13, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn dynamicobject_POS_Z(&self) -> Option<f32> {
        self.values.get(&13).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_dynamicobject_FACING(&mut self, v: f32) {
        self.header_set(14, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn dynamicobject_FACING(&self) -> Option<f32> {
        self.values.get(&14).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_dynamicobject_CASTTIME(&mut self, v: i32) {
        self.header_set(15, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn dynamicobject_CASTTIME(&self) -> Option<i32> {
        self.values.get(&15).map(|v| *v as i32)
    }

}

impl UpdateCorpse {
    pub fn set_object_GUID(&mut self, v: Guid) {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
    }

    pub fn object_GUID(&self) -> Option<Guid> {
        let lower = self.values.get(&0);
        let upper = self.values.get(&1);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_object_TYPE(&mut self, v: i32) {
        self.header_set(2, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_TYPE(&self) -> Option<i32> {
        self.values.get(&2).map(|v| *v as i32)
    }

    pub fn set_object_ENTRY(&mut self, v: i32) {
        self.header_set(3, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_ENTRY(&self) -> Option<i32> {
        self.values.get(&3).map(|v| *v as i32)
    }

    pub fn set_object_SCALE_X(&mut self, v: f32) {
        self.header_set(4, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_SCALE_X(&self) -> Option<f32> {
        self.values.get(&4).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_object_CREATED_BY(&mut self, v: Guid) {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
    }

    pub fn object_CREATED_BY(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_corpse_OWNER(&mut self, v: Guid) {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
    }

    pub fn corpse_OWNER(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_corpse_PARTY(&mut self, v: Guid) {
        self.header_set(8, v.guid() as u32);
        self.header_set(9, (v.guid() >> 32) as u32);
    }

    pub fn corpse_PARTY(&self) -> Option<Guid> {
        let lower = self.values.get(&8);
        let upper = self.values.get(&9);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_corpse_FACING(&mut self, v: f32) {
        self.header_set(10, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn corpse_FACING(&self) -> Option<f32> {
        self.values.get(&10).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_corpse_POS_X(&mut self, v: f32) {
        self.header_set(11, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn corpse_POS_X(&self) -> Option<f32> {
        self.values.get(&11).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_corpse_POS_Y(&mut self, v: f32) {
        self.header_set(12, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn corpse_POS_Y(&self) -> Option<f32> {
        self.values.get(&12).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_corpse_POS_Z(&mut self, v: f32) {
        self.header_set(13, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn corpse_POS_Z(&self) -> Option<f32> {
        self.values.get(&13).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_corpse_DISPLAY_ID(&mut self, v: i32) {
        self.header_set(14, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn corpse_DISPLAY_ID(&self) -> Option<i32> {
        self.values.get(&14).map(|v| *v as i32)
    }

    pub fn set_corpse_ITEM(&mut self, v: i32) {
        self.header_set(15, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn corpse_ITEM(&self) -> Option<i32> {
        self.values.get(&15).map(|v| *v as i32)
    }

    pub fn set_corpse_BYTES_1(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(34, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn corpse_BYTES_1(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&34) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_corpse_BYTES_2(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(35, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn corpse_BYTES_2(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&35) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_corpse_GUILD(&mut self, v: i32) {
        self.header_set(36, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn corpse_GUILD(&self) -> Option<i32> {
        self.values.get(&36).map(|v| *v as i32)
    }

    pub fn set_corpse_FLAGS(&mut self, v: i32) {
        self.header_set(37, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn corpse_FLAGS(&self) -> Option<i32> {
        self.values.get(&37).map(|v| *v as i32)
    }

    pub fn set_corpse_DYNAMIC_FLAGS(&mut self, v: i32) {
        self.header_set(38, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn corpse_DYNAMIC_FLAGS(&self) -> Option<i32> {
        self.values.get(&38).map(|v| *v as i32)
    }

}

