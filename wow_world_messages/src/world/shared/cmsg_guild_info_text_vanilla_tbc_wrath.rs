use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/cmsg_guild_info_text.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/cmsg_guild_info_text.wowm#L3):
/// ```text
/// cmsg CMSG_GUILD_INFO_TEXT = 0x02FC {
///     CString guild_info;
/// }
/// ```
pub struct CMSG_GUILD_INFO_TEXT {
    pub guild_info: String,
}

impl crate::Message for CMSG_GUILD_INFO_TEXT {
    const OPCODE: u32 = 0x02fc;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guild_info: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.guild_info.as_bytes().iter().rev().next(), Some(&0_u8), "String `guild_info` must not be null-terminated.");
        w.write_all(self.guild_info.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(1..=256).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02FC, size: body_size as u32 });
        }

        // guild_info: CString
        let guild_info = {
            let guild_info = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(guild_info)?
        };

        Ok(Self {
            guild_info,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_GUILD_INFO_TEXT {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_GUILD_INFO_TEXT {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_GUILD_INFO_TEXT {}

impl CMSG_GUILD_INFO_TEXT {
    pub(crate) fn size(&self) -> usize {
        self.guild_info.len() + 1 // guild_info: CString
    }
}

