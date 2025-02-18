use std::io::{Read, Write};
use crate::Guid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild_bank/cmsg_guild_bank_update_tab.wowm:10`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild_bank/cmsg_guild_bank_update_tab.wowm#L10):
/// ```text
/// cmsg CMSG_GUILD_BANK_UPDATE_TAB = 0x03EB {
///     Guid bank;
///     u8 tab;
///     CString name;
///     CString icon;
/// }
/// ```
pub struct CMSG_GUILD_BANK_UPDATE_TAB {
    pub bank: Guid,
    pub tab: u8,
    pub name: String,
    pub icon: String,
}

impl crate::Message for CMSG_GUILD_BANK_UPDATE_TAB {
    const OPCODE: u32 = 0x03eb;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // bank: Guid
        w.write_all(&self.bank.guid().to_le_bytes())?;

        // tab: u8
        w.write_all(&self.tab.to_le_bytes())?;

        // name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.name.as_bytes().iter().rev().next(), Some(&0_u8), "String `name` must not be null-terminated.");
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // icon: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.icon.as_bytes().iter().rev().next(), Some(&0_u8), "String `icon` must not be null-terminated.");
        w.write_all(self.icon.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(11..=521).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03EB, size: body_size as u32 });
        }

        // bank: Guid
        let bank = Guid::read(&mut r)?;

        // tab: u8
        let tab = crate::util::read_u8_le(&mut r)?;

        // name: CString
        let name = {
            let name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(name)?
        };

        // icon: CString
        let icon = {
            let icon = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(icon)?
        };

        Ok(Self {
            bank,
            tab,
            name,
            icon,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_GUILD_BANK_UPDATE_TAB {}

impl CMSG_GUILD_BANK_UPDATE_TAB {
    pub(crate) fn size(&self) -> usize {
        8 // bank: Guid
        + 1 // tab: u8
        + self.name.len() + 1 // name: CString
        + self.icon.len() + 1 // icon: CString
    }
}

