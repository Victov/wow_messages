use std::io::{Read, Write};
use crate::Guid;
use wow_world_base::shared::new_item_chat_alert_vanilla_tbc_wrath::NewItemChatAlert;
use wow_world_base::shared::new_item_creation_type_vanilla_tbc_wrath::NewItemCreationType;
use wow_world_base::shared::new_item_source_vanilla_tbc_wrath::NewItemSource;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_item_push_result.wowm:72`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_item_push_result.wowm#L72):
/// ```text
/// smsg SMSG_ITEM_PUSH_RESULT = 0x0166 {
///     Guid guid;
///     NewItemSource source;
///     NewItemCreationType creation_type;
///     NewItemChatAlert alert_chat;
///     u8 bag_slot;
///     u32 item_slot;
///     u32 item;
///     u32 item_suffix_factor;
///     u32 item_random_property_id;
///     u32 item_count;
///     u32 item_count_in_inventory;
/// }
/// ```
pub struct SMSG_ITEM_PUSH_RESULT {
    pub guid: Guid,
    pub source: NewItemSource,
    pub creation_type: NewItemCreationType,
    pub alert_chat: NewItemChatAlert,
    pub bag_slot: u8,
    /// mangoszero: item slot, but when added to stack: 0xFFFFFFFF
    ///
    pub item_slot: u32,
    pub item: u32,
    /// mangoszero: SuffixFactor
    ///
    pub item_suffix_factor: u32,
    /// mangoszero: random item property id
    ///
    pub item_random_property_id: u32,
    pub item_count: u32,
    pub item_count_in_inventory: u32,
}

impl crate::Message for SMSG_ITEM_PUSH_RESULT {
    const OPCODE: u32 = 0x0166;

    fn size_without_header(&self) -> u32 {
        45
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // source: NewItemSource
        w.write_all(&u32::from(self.source.as_int()).to_le_bytes())?;

        // creation_type: NewItemCreationType
        w.write_all(&u32::from(self.creation_type.as_int()).to_le_bytes())?;

        // alert_chat: NewItemChatAlert
        w.write_all(&u32::from(self.alert_chat.as_int()).to_le_bytes())?;

        // bag_slot: u8
        w.write_all(&self.bag_slot.to_le_bytes())?;

        // item_slot: u32
        w.write_all(&self.item_slot.to_le_bytes())?;

        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        // item_suffix_factor: u32
        w.write_all(&self.item_suffix_factor.to_le_bytes())?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes())?;

        // item_count: u32
        w.write_all(&self.item_count.to_le_bytes())?;

        // item_count_in_inventory: u32
        w.write_all(&self.item_count_in_inventory.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 45 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0166, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(&mut r)?;

        // source: NewItemSource
        let source: NewItemSource = crate::util::read_u32_le(&mut r)?.try_into()?;

        // creation_type: NewItemCreationType
        let creation_type: NewItemCreationType = crate::util::read_u32_le(&mut r)?.try_into()?;

        // alert_chat: NewItemChatAlert
        let alert_chat: NewItemChatAlert = crate::util::read_u32_le(&mut r)?.try_into()?;

        // bag_slot: u8
        let bag_slot = crate::util::read_u8_le(&mut r)?;

        // item_slot: u32
        let item_slot = crate::util::read_u32_le(&mut r)?;

        // item: u32
        let item = crate::util::read_u32_le(&mut r)?;

        // item_suffix_factor: u32
        let item_suffix_factor = crate::util::read_u32_le(&mut r)?;

        // item_random_property_id: u32
        let item_random_property_id = crate::util::read_u32_le(&mut r)?;

        // item_count: u32
        let item_count = crate::util::read_u32_le(&mut r)?;

        // item_count_in_inventory: u32
        let item_count_in_inventory = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            guid,
            source,
            creation_type,
            alert_chat,
            bag_slot,
            item_slot,
            item,
            item_suffix_factor,
            item_random_property_id,
            item_count,
            item_count_in_inventory,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_ITEM_PUSH_RESULT {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_ITEM_PUSH_RESULT {}

