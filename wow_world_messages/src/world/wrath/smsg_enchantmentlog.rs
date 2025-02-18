use std::io::{Read, Write};
use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Some emulators have the guids as not packed.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_enchantmentlog.wowm:16`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_enchantmentlog.wowm#L16):
/// ```text
/// smsg SMSG_ENCHANTMENTLOG = 0x01D7 {
///     PackedGuid target;
///     PackedGuid caster;
///     u32 item;
///     u32 spell;
///     Bool show_affiliation;
/// }
/// ```
pub struct SMSG_ENCHANTMENTLOG {
    pub target: Guid,
    /// vmangos: message says enchant has faded if empty
    ///
    pub caster: Guid,
    pub item: u32,
    pub spell: u32,
    /// vmangos: Only used if `caster` is not 0.
    ///
    pub show_affiliation: bool,
}

impl crate::Message for SMSG_ENCHANTMENTLOG {
    const OPCODE: u32 = 0x01d7;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // target: PackedGuid
        self.target.write_packed_guid_into_vec(&mut w)?;

        // caster: PackedGuid
        self.caster.write_packed_guid_into_vec(&mut w)?;

        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // show_affiliation: Bool
        w.write_all(u8::from(self.show_affiliation).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(13..=27).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01D7, size: body_size as u32 });
        }

        // target: PackedGuid
        let target = Guid::read_packed(&mut r)?;

        // caster: PackedGuid
        let caster = Guid::read_packed(&mut r)?;

        // item: u32
        let item = crate::util::read_u32_le(&mut r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(&mut r)?;

        // show_affiliation: Bool
        let show_affiliation = crate::util::read_u8_le(&mut r)? != 0;

        Ok(Self {
            target,
            caster,
            item,
            spell,
            show_affiliation,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_ENCHANTMENTLOG {}

impl SMSG_ENCHANTMENTLOG {
    pub(crate) fn size(&self) -> usize {
        self.target.size() // target: PackedGuid
        + self.caster.size() // caster: PackedGuid
        + 4 // item: u32
        + 4 // spell: u32
        + 1 // show_affiliation: Bool
    }
}

