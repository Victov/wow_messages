use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/cmsg_group_raid_convert.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_group_raid_convert.wowm#L3):
/// ```text
/// cmsg CMSG_GROUP_RAID_CONVERT = 0x028E {
/// }
/// ```
pub struct CMSG_GROUP_RAID_CONVERT {
}

impl crate::Message for CMSG_GROUP_RAID_CONVERT {
    const OPCODE: u32 = 0x028e;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x028E, size: body_size as u32 });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_GROUP_RAID_CONVERT {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_GROUP_RAID_CONVERT {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_GROUP_RAID_CONVERT {}

