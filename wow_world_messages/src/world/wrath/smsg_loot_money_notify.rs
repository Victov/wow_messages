use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/smsg_loot_money_notify.wowm:7`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/smsg_loot_money_notify.wowm#L7):
/// ```text
/// smsg SMSG_LOOT_MONEY_NOTIFY = 0x0163 {
///     u32 amount;
///     Bool alone;
/// }
/// ```
pub struct SMSG_LOOT_MONEY_NOTIFY {
    pub amount: u32,
    /// Controls the text displayed in chat. False is 'Your share is...' and true is 'You loot...'
    ///
    pub alone: bool,
}

impl crate::Message for SMSG_LOOT_MONEY_NOTIFY {
    const OPCODE: u32 = 0x0163;

    fn size_without_header(&self) -> u32 {
        5
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // amount: u32
        w.write_all(&self.amount.to_le_bytes())?;

        // alone: Bool
        w.write_all(u8::from(self.alone).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 5 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0163, size: body_size as u32 });
        }

        // amount: u32
        let amount = crate::util::read_u32_le(&mut r)?;

        // alone: Bool
        let alone = crate::util::read_u8_le(&mut r)? != 0;

        Ok(Self {
            amount,
            alone,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_LOOT_MONEY_NOTIFY {}

