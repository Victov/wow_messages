#[allow(clippy::missing_panics_doc)]
mod impls;
mod indices;

pub use impls::*;
pub use indices::*;

use crate::helper::update_mask_common;
use crate::helper::update_mask_common::{
    skill_info, update_item, update_mask, update_mask_size, CONTAINER, CORPSE, DYNAMICOBJECT,
    GAMEOBJECT, ITEM, PLAYER, UNIT,
};
use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::io;
use std::io::Read;

update_item!(UpdateItem, UpdateItemBuilder, ITEM);
update_item!(UpdateContainer, UpdateContainerBuilder, ITEM | CONTAINER);
update_item!(UpdateUnit, UpdateUnitBuilder, UNIT);
update_item!(UpdatePlayer, UpdatePlayerBuilder, UNIT | PLAYER);
update_item!(UpdateGameObject, UpdateGameObjectBuilder, GAMEOBJECT);
update_item!(
    UpdateDynamicObject,
    UpdateDynamicObjectBuilder,
    DYNAMICOBJECT
);
update_item!(UpdateCorpse, UpdateCorpseBuilder, CORPSE);

update_mask!();

skill_info!(wow_world_base::vanilla::Skill, indices::SkillInfoIndex);

#[cfg(test)]
mod test {
    use crate::vanilla::update_mask::indices::SkillInfoIndex;
    use crate::vanilla::update_mask::UpdatePlayer;
    use crate::vanilla::{Class, Gender, Power, Race};
    use crate::vanilla::{SkillInfo, UpdateMask};
    use crate::Guid;
    use wow_world_base::vanilla::Skill;

    #[test]
    fn most_minimal_example() {
        let b = [
            2_u8, // Amount of u32 mask blocks that will follow
            // Mask blocks
            7, 0, 64, 0, 16, 0, 0, 0, // End of mask blocks
            4, 0, 0, 0, 0, 0, 0, 0, // OBJECT_FIELD_GUID (4) (notice unpacked u64)
            25, 0, 0,
            0, // OBJECT_FIELD_TYPE (16 | 8 | 1) (TYPE_PLAYER | TYPE_UNIT | TYPE_OBJECT)
            100, 0, 0, 0, // UNIT_FIELD_HEALTH (100)
            1, // UNIT_FIELD_BYTES[0] // Race (Human)
            1, // UNIT_FIELD_BYTES[1] // Class (Warrior)
            1, // UNIT_FIELD_BYTES[2] // Gender (Female)
            1, // UNIT_FIELD_BYTES[3] // Power (Rage)
        ];

        let update_mask = UpdatePlayer::builder()
            .set_object_GUID(Guid::new(4))
            .set_unit_BYTES_0(Race::Human, Class::Warrior, Gender::Female, Power::Rage)
            .set_unit_HEALTH(100)
            .finalize();
        let update_mask = UpdateMask::Player(update_mask);

        assert_eq!(b.len(), update_mask.size());
        let mut v = Vec::with_capacity(update_mask.size());
        update_mask.write_into_vec(&mut v).unwrap();
        assert_eq!(b.as_slice(), v.as_slice());
    }

    #[test]
    fn skill_info_in_out() {
        let b = [
            23_u8, // Amount of u32 mask blocks that will follow
            // Mask blocks
            7, 0, 64, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 192, 1, 0, 4, 0, 0, 0, 0, 0, 0,
            0, // OBJECT_FIELD_GUID (4) (notice unpacked u64)
            25, 0, 0,
            0, // OBJECT_FIELD_TYPE (16 | 8 | 1) (TYPE_PLAYER | TYPE_UNIT | TYPE_OBJECT)
            100, 0, 0, 0, // UNIT_FIELD_HEALTH (100)
            1, // UNIT_FIELD_BYTES[0] // Race (Human)
            1, // UNIT_FIELD_BYTES[1] // Class (Warrior)
            1, // UNIT_FIELD_BYTES[2] // Gender (Female)
            1, // UNIT_FIELD_BYTES[3] // Power (Rage)
            98, 0, 5, 0, // PLAYER_SKILL_INFO_1_1
            44, 1, 46, 1, // PLAYER_SKILL_INFO_1_2
            10, 0, 7, 0, // PLAYER_SKILL_INFO_1_3
        ];

        let skill_info = SkillInfo::new(Skill::LanguageCommon, 5, 300, 302, 7, 10);

        let update_mask = UpdateMask::Player(
            UpdatePlayer::builder()
                .set_object_GUID(Guid::new(4))
                .set_unit_BYTES_0(Race::Human, Class::Warrior, Gender::Female, Power::Rage)
                .set_unit_HEALTH(100)
                .set_player_SKILL_INFO(skill_info, SkillInfoIndex::Index0)
                .finalize(),
        );

        assert_eq!(b.len(), update_mask.size());

        let mut v = Vec::with_capacity(update_mask.size());
        update_mask.write_into_vec(&mut v).unwrap();
        assert_eq!(b.as_slice(), v.as_slice());
        let update_mask = UpdateMask::read(&mut b.as_slice()).unwrap();
        match update_mask {
            UpdateMask::Player(p) => {
                assert_eq!(
                    p.player_SKILL_INFO(SkillInfoIndex::Index0).unwrap(),
                    skill_info
                );
                assert_eq!(p.player_SKILL_INFO(SkillInfoIndex::Index1), None)
            }
            _ => panic!(),
        }
    }

    #[test]
    fn reset_header_add_one() {
        let b = [
            5_u8, // Amount of u32 mask blocks that will follow
            // Mask blocks
            3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            // End of mask blocks
            4, 0, 0, 0, 0, 0, 0, 0, // OBJECT_FIELD_GUID (4) (notice unpacked u64)
        ];

        let mut update_mask = UpdatePlayer::builder()
            .set_object_GUID(Guid::new(4))
            .set_unit_BYTES_0(Race::Human, Class::Warrior, Gender::Female, Power::Rage)
            .set_object_SCALE_X(1.0)
            .set_unit_HEALTH(100)
            .set_unit_MAXHEALTH(100)
            .set_unit_LEVEL(1)
            .set_unit_FACTIONTEMPLATE(1)
            .set_unit_DISPLAYID(50)
            .set_unit_NATIVEDISPLAYID(50)
            .finalize();
        update_mask.dirty_reset();

        update_mask.set_object_GUID(4.into());

        let update_mask = UpdateMask::Player(update_mask);
        assert_eq!(b.len(), update_mask.size());

        let mut v = Vec::with_capacity(update_mask.size());
        update_mask.write_into_vec(&mut v).unwrap();
        assert_eq!(b.as_slice(), v.as_slice());
    }

    #[test]
    fn reset_header() {
        let b = [
            5_u8, // Amount of u32 mask blocks that will follow
            // Mask blocks
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0,
            // End of mask blocks
            // No value blocks
        ];

        let mut update_mask = UpdatePlayer::builder()
            .set_object_GUID(Guid::new(4))
            .set_unit_BYTES_0(Race::Human, Class::Warrior, Gender::Female, Power::Rage)
            .set_object_SCALE_X(1.0)
            .set_unit_HEALTH(100)
            .set_unit_MAXHEALTH(100)
            .set_unit_LEVEL(1)
            .set_unit_FACTIONTEMPLATE(1)
            .set_unit_DISPLAYID(50)
            .set_unit_NATIVEDISPLAYID(50)
            .finalize();

        update_mask.dirty_reset();
        let update_mask = UpdateMask::Player(update_mask);

        assert_eq!(b.len(), update_mask.size());
        let mut v = Vec::with_capacity(update_mask.size());
        update_mask.write_into_vec(&mut v).unwrap();
        assert_eq!(b.as_slice(), v.as_slice());
    }

    #[test]
    fn small_example() {
        let b = [
            5_u8, // Amount of u32 mask blocks that will follow
            // Mask blocks
            23, 0, 64, 16, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0,
            // End of mask blocks
            4, 0, 0, 0, 0, 0, 0, 0, // OBJECT_FIELD_GUID (4) (notice unpacked u64)
            25, 0, 0,
            0, // OBJECT_FIELD_TYPE (16 | 8 | 1) (TYPE_PLAYER | TYPE_UNIT | TYPE_OBJECT)
            0, 0, 128, 63, // Scale (1.0)
            100, 0, 0, 0, // UNIT_FIELD_HEALTH (100)
            100, 0, 0, 0, // UNIT_FIELD_MAXHEALTH (100)
            1, 0, 0, 0, // UNIT_FIELD_LEVEL (1)
            1, 0, 0, 0, // UNIT_FIELD_FACTIONTEMPLATE (1)
            1, // UNIT_FIELD_BYTES[0] // Race (Human)
            1, // UNIT_FIELD_BYTES[1] // Class (Warrior)
            1, // UNIT_FIELD_BYTES[2] // Gender (Female)
            1, // UNIT_FIELD_BYTES[3] // Power (Rage)
            50, 0, 0, 0, // UNIT_FIELD_DISPLAYD (50, Human Female)
            50, 0, 0, 0, // UNIT_FIELD_NATIVEDISPLAYID (50, Human Female)
        ];

        let update_mask = UpdatePlayer::builder()
            .set_object_GUID(Guid::new(4))
            .set_unit_BYTES_0(Race::Human, Class::Warrior, Gender::Female, Power::Rage)
            .set_object_SCALE_X(1.0)
            .set_unit_HEALTH(100)
            .set_unit_MAXHEALTH(100)
            .set_unit_LEVEL(1)
            .set_unit_FACTIONTEMPLATE(1)
            .set_unit_DISPLAYID(50)
            .set_unit_NATIVEDISPLAYID(50)
            .finalize();
        let update_mask = UpdateMask::Player(update_mask);

        assert_eq!(b.len(), update_mask.size());
        let mut v = Vec::with_capacity(update_mask.size());
        update_mask.write_into_vec(&mut v).unwrap();
        assert_eq!(b.as_slice(), v.as_slice());
    }

    #[test]
    fn reset_dirty_and_mark_full_dirty() {
        let b = [
            5_u8, // Amount of u32 mask blocks that will follow
            // Mask blocks
            23, 0, 64, 16, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0,
            // End of mask blocks
            4, 0, 0, 0, 0, 0, 0, 0, // OBJECT_FIELD_GUID (4) (notice unpacked u64)
            25, 0, 0,
            0, // OBJECT_FIELD_TYPE (16 | 8 | 1) (TYPE_PLAYER | TYPE_UNIT | TYPE_OBJECT)
            0, 0, 128, 63, // Scale (1.0)
            100, 0, 0, 0, // UNIT_FIELD_HEALTH (100)
            100, 0, 0, 0, // UNIT_FIELD_MAXHEALTH (100)
            1, 0, 0, 0, // UNIT_FIELD_LEVEL (1)
            1, 0, 0, 0, // UNIT_FIELD_FACTIONTEMPLATE (1)
            1, // UNIT_FIELD_BYTES[0] // Race (Human)
            1, // UNIT_FIELD_BYTES[1] // Class (Warrior)
            1, // UNIT_FIELD_BYTES[2] // Gender (Female)
            1, // UNIT_FIELD_BYTES[3] // Power (Rage)
            50, 0, 0, 0, // UNIT_FIELD_DISPLAYD (50, Human Female)
            50, 0, 0, 0, // UNIT_FIELD_NATIVEDISPLAYID (50, Human Female)
        ];

        let mut update_mask = UpdatePlayer::builder()
            .set_object_GUID(Guid::new(4))
            .set_unit_BYTES_0(Race::Human, Class::Warrior, Gender::Female, Power::Rage)
            .set_object_SCALE_X(1.0)
            .set_unit_HEALTH(100)
            .set_unit_MAXHEALTH(100)
            .set_unit_LEVEL(1)
            .set_unit_FACTIONTEMPLATE(1)
            .set_unit_DISPLAYID(50)
            .set_unit_NATIVEDISPLAYID(50)
            .finalize();
        //Fully clear the dirty mask
        update_mask.dirty_reset();
        assert!(!update_mask.has_any_dirty_fields());

        //Mark completely dirty
        update_mask.mark_fully_dirty();
        assert!(update_mask.has_any_dirty_fields());

        let update_mask = UpdateMask::Player(update_mask);

        assert_eq!(b.len(), update_mask.size());
        let mut v = Vec::with_capacity(update_mask.size());
        update_mask.write_into_vec(&mut v).unwrap();

        //Output should match all previously written fields
        assert_eq!(b.as_slice(), v.as_slice());
    }
}
