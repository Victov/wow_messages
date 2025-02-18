use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/cmsg_item_text_query.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/cmsg_item_text_query.wowm#L1):
/// ```text
/// cmsg CMSG_ITEM_TEXT_QUERY = 0x0243 {
///     u32 item_text_id;
///     u32 mail_id;
///     u32 unknown1;
/// }
/// ```
pub struct CMSG_ITEM_TEXT_QUERY {
    pub item_text_id: u32,
    /// vmangos/cmangos/mangoszero: this value can be item id in bag, but it is also mail id
    ///
    pub mail_id: u32,
    /// vmangos/cmangos/mangoszero: maybe something like state - 0x70000000
    ///
    pub unknown1: u32,
}

impl crate::Message for CMSG_ITEM_TEXT_QUERY {
    const OPCODE: u32 = 0x0243;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // item_text_id: u32
        w.write_all(&self.item_text_id.to_le_bytes())?;

        // mail_id: u32
        w.write_all(&self.mail_id.to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0243, size: body_size as u32 });
        }

        // item_text_id: u32
        let item_text_id = crate::util::read_u32_le(&mut r)?;

        // mail_id: u32
        let mail_id = crate::util::read_u32_le(&mut r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            item_text_id,
            mail_id,
            unknown1,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_ITEM_TEXT_QUERY {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_ITEM_TEXT_QUERY {}

