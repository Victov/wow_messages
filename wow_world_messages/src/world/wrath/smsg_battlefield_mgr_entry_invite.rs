use std::io::{Read, Write};
use crate::wrath::Area;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/smsg_battlefield_mgr_entry_invite.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/smsg_battlefield_mgr_entry_invite.wowm#L1):
/// ```text
/// smsg SMSG_BATTLEFIELD_MGR_ENTRY_INVITE = 0x04DE {
///     u32 battle_id;
///     Area area;
///     u32 accept_time_in_seconds;
/// }
/// ```
pub struct SMSG_BATTLEFIELD_MGR_ENTRY_INVITE {
    pub battle_id: u32,
    pub area: Area,
    pub accept_time_in_seconds: u32,
}

impl crate::Message for SMSG_BATTLEFIELD_MGR_ENTRY_INVITE {
    const OPCODE: u32 = 0x04de;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // battle_id: u32
        w.write_all(&self.battle_id.to_le_bytes())?;

        // area: Area
        w.write_all(&u32::from(self.area.as_int()).to_le_bytes())?;

        // accept_time_in_seconds: u32
        w.write_all(&self.accept_time_in_seconds.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04DE, size: body_size as u32 });
        }

        // battle_id: u32
        let battle_id = crate::util::read_u32_le(&mut r)?;

        // area: Area
        let area: Area = crate::util::read_u32_le(&mut r)?.try_into()?;

        // accept_time_in_seconds: u32
        let accept_time_in_seconds = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            battle_id,
            area,
            accept_time_in_seconds,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_BATTLEFIELD_MGR_ENTRY_INVITE {}

