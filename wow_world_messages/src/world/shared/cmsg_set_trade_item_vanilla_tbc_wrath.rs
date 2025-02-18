use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/trade/cmsg_set_trade_item.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/trade/cmsg_set_trade_item.wowm#L3):
/// ```text
/// cmsg CMSG_SET_TRADE_ITEM = 0x011D {
///     u8 trade_slot;
///     u8 bag;
///     u8 slot;
/// }
/// ```
pub struct CMSG_SET_TRADE_ITEM {
    pub trade_slot: u8,
    pub bag: u8,
    pub slot: u8,
}

impl crate::Message for CMSG_SET_TRADE_ITEM {
    const OPCODE: u32 = 0x011d;

    fn size_without_header(&self) -> u32 {
        3
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // trade_slot: u8
        w.write_all(&self.trade_slot.to_le_bytes())?;

        // bag: u8
        w.write_all(&self.bag.to_le_bytes())?;

        // slot: u8
        w.write_all(&self.slot.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 3 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x011D, size: body_size as u32 });
        }

        // trade_slot: u8
        let trade_slot = crate::util::read_u8_le(&mut r)?;

        // bag: u8
        let bag = crate::util::read_u8_le(&mut r)?;

        // slot: u8
        let slot = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            trade_slot,
            bag,
            slot,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_SET_TRADE_ITEM {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_SET_TRADE_ITEM {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_SET_TRADE_ITEM {}

