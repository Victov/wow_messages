use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/cmsg_guild_create.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/cmsg_guild_create.wowm#L3):
/// ```text
/// cmsg CMSG_GUILD_CREATE = 0x0081 {
///     CString guild_name;
/// }
/// ```
pub struct CMSG_GUILD_CREATE {
    pub guild_name: String,
}

impl crate::Message for CMSG_GUILD_CREATE {
    const OPCODE: u32 = 0x0081;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guild_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.guild_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `guild_name` must not be null-terminated.");
        w.write_all(self.guild_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(1..=256).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0081, size: body_size as u32 });
        }

        // guild_name: CString
        let guild_name = {
            let guild_name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(guild_name)?
        };

        Ok(Self {
            guild_name,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_GUILD_CREATE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_GUILD_CREATE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_GUILD_CREATE {}

impl CMSG_GUILD_CREATE {
    pub(crate) fn size(&self) -> usize {
        self.guild_name.len() + 1 // guild_name: CString
    }
}

