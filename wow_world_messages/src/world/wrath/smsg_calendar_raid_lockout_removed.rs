use std::io::{Read, Write};
use crate::Guid;
use crate::wrath::Map;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/smsg_calendar_raid_lockout_removed.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/smsg_calendar_raid_lockout_removed.wowm#L1):
/// ```text
/// smsg SMSG_CALENDAR_RAID_LOCKOUT_REMOVED = 0x043F {
///     Map map;
///     u32 difficulty;
///     u32 remaining_time;
///     Guid instance_id;
/// }
/// ```
pub struct SMSG_CALENDAR_RAID_LOCKOUT_REMOVED {
    pub map: Map,
    pub difficulty: u32,
    pub remaining_time: u32,
    pub instance_id: Guid,
}

impl crate::Message for SMSG_CALENDAR_RAID_LOCKOUT_REMOVED {
    const OPCODE: u32 = 0x043f;

    fn size_without_header(&self) -> u32 {
        20
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // map: Map
        w.write_all(&u32::from(self.map.as_int()).to_le_bytes())?;

        // difficulty: u32
        w.write_all(&self.difficulty.to_le_bytes())?;

        // remaining_time: u32
        w.write_all(&self.remaining_time.to_le_bytes())?;

        // instance_id: Guid
        w.write_all(&self.instance_id.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 20 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x043F, size: body_size as u32 });
        }

        // map: Map
        let map: Map = crate::util::read_u32_le(&mut r)?.try_into()?;

        // difficulty: u32
        let difficulty = crate::util::read_u32_le(&mut r)?;

        // remaining_time: u32
        let remaining_time = crate::util::read_u32_le(&mut r)?;

        // instance_id: Guid
        let instance_id = Guid::read(&mut r)?;

        Ok(Self {
            map,
            difficulty,
            remaining_time,
            instance_id,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CALENDAR_RAID_LOCKOUT_REMOVED {}

