use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild_bank/cmsg_set_guild_bank_text.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild_bank/cmsg_set_guild_bank_text.wowm#L1):
/// ```text
/// cmsg CMSG_SET_GUILD_BANK_TEXT = 0x040A {
///     u8 tab;
///     CString text;
/// }
/// ```
pub struct CMSG_SET_GUILD_BANK_TEXT {
    pub tab: u8,
    pub text: String,
}

impl crate::Message for CMSG_SET_GUILD_BANK_TEXT {
    const OPCODE: u32 = 0x040a;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // tab: u8
        w.write_all(&self.tab.to_le_bytes())?;

        // text: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.text.as_bytes().iter().rev().next(), Some(&0_u8), "String `text` must not be null-terminated.");
        w.write_all(self.text.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(2..=257).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x040A, size: body_size as u32 });
        }

        // tab: u8
        let tab = crate::util::read_u8_le(&mut r)?;

        // text: CString
        let text = {
            let text = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(text)?
        };

        Ok(Self {
            tab,
            text,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_SET_GUILD_BANK_TEXT {}

impl CMSG_SET_GUILD_BANK_TEXT {
    pub(crate) fn size(&self) -> usize {
        1 // tab: u8
        + self.text.len() + 1 // text: CString
    }
}

