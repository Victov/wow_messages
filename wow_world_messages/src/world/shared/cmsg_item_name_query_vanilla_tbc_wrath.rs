use std::io::{Read, Write};
use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/cmsg_item_name_query.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/cmsg_item_name_query.wowm#L3):
/// ```text
/// cmsg CMSG_ITEM_NAME_QUERY = 0x02C4 {
///     u32 item;
///     Guid guid;
/// }
/// ```
pub struct CMSG_ITEM_NAME_QUERY {
    pub item: u32,
    pub guid: Guid,
}

impl crate::Message for CMSG_ITEM_NAME_QUERY {
    const OPCODE: u32 = 0x02c4;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02C4, size: body_size as u32 });
        }

        // item: u32
        let item = crate::util::read_u32_le(&mut r)?;

        // guid: Guid
        let guid = Guid::read(&mut r)?;

        Ok(Self {
            item,
            guid,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_ITEM_NAME_QUERY {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_ITEM_NAME_QUERY {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_ITEM_NAME_QUERY {}

