use std::io::{Read, Write};
use crate::Guid;
use crate::vanilla::RollVote;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/cmsg_loot_roll.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/cmsg_loot_roll.wowm#L1):
/// ```text
/// cmsg CMSG_LOOT_ROLL = 0x02A0 {
///     Guid item;
///     u32 item_slot;
///     RollVote vote;
/// }
/// ```
pub struct CMSG_LOOT_ROLL {
    pub item: Guid,
    pub item_slot: u32,
    pub vote: RollVote,
}

impl crate::Message for CMSG_LOOT_ROLL {
    const OPCODE: u32 = 0x02a0;

    fn size_without_header(&self) -> u32 {
        13
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // item: Guid
        w.write_all(&self.item.guid().to_le_bytes())?;

        // item_slot: u32
        w.write_all(&self.item_slot.to_le_bytes())?;

        // vote: RollVote
        w.write_all(&u8::from(self.vote.as_int()).to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 13 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02A0, size: body_size as u32 });
        }

        // item: Guid
        let item = Guid::read(&mut r)?;

        // item_slot: u32
        let item_slot = crate::util::read_u32_le(&mut r)?;

        // vote: RollVote
        let vote: RollVote = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            item,
            item_slot,
            vote,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_LOOT_ROLL {}

