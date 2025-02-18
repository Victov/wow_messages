use std::io::{Read, Write};
use crate::wrath::Faction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/faction/cmsg_set_watched_faction.wowm:7`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/faction/cmsg_set_watched_faction.wowm#L7):
/// ```text
/// cmsg CMSG_SET_WATCHED_FACTION = 0x0318 {
///     Faction faction;
/// }
/// ```
pub struct CMSG_SET_WATCHED_FACTION {
    pub faction: Faction,
}

impl crate::Message for CMSG_SET_WATCHED_FACTION {
    const OPCODE: u32 = 0x0318;

    fn size_without_header(&self) -> u32 {
        2
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // faction: Faction
        w.write_all(&u16::from(self.faction.as_int()).to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 2 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0318, size: body_size as u32 });
        }

        // faction: Faction
        let faction: Faction = crate::util::read_u16_le(&mut r)?.try_into()?;

        Ok(Self {
            faction,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_SET_WATCHED_FACTION {}

