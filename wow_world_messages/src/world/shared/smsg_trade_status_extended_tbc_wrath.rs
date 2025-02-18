use std::io::{Read, Write};
use crate::shared::trade_slot_tbc_wrath::TradeSlot;
use wow_world_base::shared::gold_vanilla_tbc_wrath::Gold;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/trade/smsg_trade_status_extended.wowm:67`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/trade/smsg_trade_status_extended.wowm#L67):
/// ```text
/// smsg SMSG_TRADE_STATUS_EXTENDED = 0x0121 {
///     Bool self_player;
///     u32 trade_id;
///     u32 trade_slot_count1;
///     u32 trade_slot_count2;
///     Gold money_in_trade;
///     u32 spell_on_lowest_slot;
///     TradeSlot[7] trade_slots;
/// }
/// ```
pub struct SMSG_TRADE_STATUS_EXTENDED {
    /// cmangos/vmangos/mangoszero: send trader or own trade windows state (last need for proper show spell apply to non-trade slot)
    ///
    pub self_player: bool,
    /// added in 2.4.0, this value must be equal to value from TRADE_STATUS_OPEN_WINDOW status packet (different value for different players to block multiple trades?)
    ///
    pub trade_id: u32,
    /// cmangos/vmangos/mangoszero: sets to 7
    /// cmangos/vmangos/mangoszero: trade slots count/number?, = next field in most cases
    ///
    pub trade_slot_count1: u32,
    /// cmangos/vmangos/mangoszero: sets to 7
    /// cmangos/vmangos/mangoszero: trade slots count/number?, = prev field in most cases
    ///
    pub trade_slot_count2: u32,
    pub money_in_trade: Gold,
    pub spell_on_lowest_slot: u32,
    /// vmangos/cmangos/mangoszero: All set to same as trade_slot_count* (7), unsure which determines how big this is. Unused slots are 0.
    ///
    pub trade_slots: [TradeSlot; 7],
}

impl crate::Message for SMSG_TRADE_STATUS_EXTENDED {
    const OPCODE: u32 = 0x0121;

    fn size_without_header(&self) -> u32 {
        532
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // self_player: Bool
        w.write_all(u8::from(self.self_player).to_le_bytes().as_slice())?;

        // trade_id: u32
        w.write_all(&self.trade_id.to_le_bytes())?;

        // trade_slot_count1: u32
        w.write_all(&self.trade_slot_count1.to_le_bytes())?;

        // trade_slot_count2: u32
        w.write_all(&self.trade_slot_count2.to_le_bytes())?;

        // money_in_trade: Gold
        w.write_all(u32::from(self.money_in_trade.as_int()).to_le_bytes().as_slice())?;

        // spell_on_lowest_slot: u32
        w.write_all(&self.spell_on_lowest_slot.to_le_bytes())?;

        // trade_slots: TradeSlot[7]
        for i in self.trade_slots.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 532 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0121, size: body_size as u32 });
        }

        // self_player: Bool
        let self_player = crate::util::read_u8_le(&mut r)? != 0;

        // trade_id: u32
        let trade_id = crate::util::read_u32_le(&mut r)?;

        // trade_slot_count1: u32
        let trade_slot_count1 = crate::util::read_u32_le(&mut r)?;

        // trade_slot_count2: u32
        let trade_slot_count2 = crate::util::read_u32_le(&mut r)?;

        // money_in_trade: Gold
        let money_in_trade = Gold::new(crate::util::read_u32_le(&mut r)?);

        // spell_on_lowest_slot: u32
        let spell_on_lowest_slot = crate::util::read_u32_le(&mut r)?;

        // trade_slots: TradeSlot[7]
        let trade_slots = {
            let mut trade_slots = [TradeSlot::default(); 7];
            for i in trade_slots.iter_mut() {
                *i = TradeSlot::read(&mut r)?;
            }
            trade_slots
        };

        Ok(Self {
            self_player,
            trade_id,
            trade_slot_count1,
            trade_slot_count2,
            money_in_trade,
            spell_on_lowest_slot,
            trade_slots,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_TRADE_STATUS_EXTENDED {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_TRADE_STATUS_EXTENDED {}

