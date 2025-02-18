use std::io::{Read, Write};
use crate::Guid;
use crate::wrath::Map;
use crate::wrath::RollFlags;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/smsg_loot_start_roll.wowm:23`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/smsg_loot_start_roll.wowm#L23):
/// ```text
/// smsg SMSG_LOOT_START_ROLL = 0x02A1 {
///     Guid creature;
///     Map map;
///     u32 loot_slot;
///     u32 item;
///     u32 item_random_suffix;
///     u32 item_random_property_id;
///     u32 countdown_time_in_milliseconds;
///     RollFlags flags;
/// }
/// ```
pub struct SMSG_LOOT_START_ROLL {
    pub creature: Guid,
    pub map: Map,
    pub loot_slot: u32,
    pub item: u32,
    /// vmangos/mangoszero: not used ?
    ///
    pub item_random_suffix: u32,
    pub item_random_property_id: u32,
    pub countdown_time_in_milliseconds: u32,
    pub flags: RollFlags,
}

impl crate::Message for SMSG_LOOT_START_ROLL {
    const OPCODE: u32 = 0x02a1;

    fn size_without_header(&self) -> u32 {
        33
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // creature: Guid
        w.write_all(&self.creature.guid().to_le_bytes())?;

        // map: Map
        w.write_all(&u32::from(self.map.as_int()).to_le_bytes())?;

        // loot_slot: u32
        w.write_all(&self.loot_slot.to_le_bytes())?;

        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        // item_random_suffix: u32
        w.write_all(&self.item_random_suffix.to_le_bytes())?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes())?;

        // countdown_time_in_milliseconds: u32
        w.write_all(&self.countdown_time_in_milliseconds.to_le_bytes())?;

        // flags: RollFlags
        w.write_all(&u8::from(self.flags.as_int()).to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 33 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02A1, size: body_size as u32 });
        }

        // creature: Guid
        let creature = Guid::read(&mut r)?;

        // map: Map
        let map: Map = crate::util::read_u32_le(&mut r)?.try_into()?;

        // loot_slot: u32
        let loot_slot = crate::util::read_u32_le(&mut r)?;

        // item: u32
        let item = crate::util::read_u32_le(&mut r)?;

        // item_random_suffix: u32
        let item_random_suffix = crate::util::read_u32_le(&mut r)?;

        // item_random_property_id: u32
        let item_random_property_id = crate::util::read_u32_le(&mut r)?;

        // countdown_time_in_milliseconds: u32
        let countdown_time_in_milliseconds = crate::util::read_u32_le(&mut r)?;

        // flags: RollFlags
        let flags = RollFlags::new(crate::util::read_u8_le(&mut r)?);

        Ok(Self {
            creature,
            map,
            loot_slot,
            item,
            item_random_suffix,
            item_random_property_id,
            countdown_time_in_milliseconds,
            flags,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_LOOT_START_ROLL {}

