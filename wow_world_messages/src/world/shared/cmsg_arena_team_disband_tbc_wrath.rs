use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/arena/cmsg_arena_team_disband.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/arena/cmsg_arena_team_disband.wowm#L1):
/// ```text
/// cmsg CMSG_ARENA_TEAM_DISBAND = 0x0355 {
///     u32 arena_team;
/// }
/// ```
pub struct CMSG_ARENA_TEAM_DISBAND {
    pub arena_team: u32,
}

impl crate::Message for CMSG_ARENA_TEAM_DISBAND {
    const OPCODE: u32 = 0x0355;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // arena_team: u32
        w.write_all(&self.arena_team.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0355, size: body_size as u32 });
        }

        // arena_team: u32
        let arena_team = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            arena_team,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_ARENA_TEAM_DISBAND {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_ARENA_TEAM_DISBAND {}

