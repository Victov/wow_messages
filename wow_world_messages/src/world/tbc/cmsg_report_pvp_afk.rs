use std::io::{Read, Write};
use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/cmsg_report_pvp_afk.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/cmsg_report_pvp_afk.wowm#L1):
/// ```text
/// cmsg CMSG_REPORT_PVP_AFK = 0x03E3 {
///     Guid player;
/// }
/// ```
pub struct CMSG_REPORT_PVP_AFK {
    pub player: Guid,
}

impl crate::Message for CMSG_REPORT_PVP_AFK {
    const OPCODE: u32 = 0x03e3;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // player: Guid
        w.write_all(&self.player.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03E3, size: body_size as u32 });
        }

        // player: Guid
        let player = Guid::read(&mut r)?;

        Ok(Self {
            player,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_REPORT_PVP_AFK {}

