use std::io::{Read, Write};
use crate::Guid;
use crate::vanilla::QuestCompletable;
use crate::vanilla::QuestItemRequirement;
use wow_world_base::shared::gold_vanilla_tbc_wrath::Gold;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// mangoszero/vmangos: Quests that don't require items use the `RequestItemsText` field to store the text that is shown when you talk to the quest giver while the quest is incomplete. Therefore the text should not be shown for them when the quest is complete. For quests that do require items, it is self explanatory.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_questgiver_request_item.wowm:16`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_questgiver_request_item.wowm#L16):
/// ```text
/// smsg SMSG_QUESTGIVER_REQUEST_ITEMS = 0x018B {
///     Guid npc;
///     u32 quest_id;
///     CString title;
///     CString request_items_text;
///     u32 emote_delay;
///     u32 emote;
///     Bool32 auto_finish;
///     Gold required_money;
///     u32 amount_of_required_items;
///     QuestItemRequirement[amount_of_required_items] required_items;
///     u32 unknown1;
///     QuestCompletable completable;
///     u32 flags2;
///     u32 flags3;
/// }
/// ```
pub struct SMSG_QUESTGIVER_REQUEST_ITEMS {
    pub npc: Guid,
    pub quest_id: u32,
    pub title: String,
    pub request_items_text: String,
    pub emote_delay: u32,
    pub emote: u32,
    pub auto_finish: bool,
    pub required_money: Gold,
    pub required_items: Vec<QuestItemRequirement>,
    /// cmangos/vmangos/mangoszero: All emulators set to 0x02
    ///
    pub unknown1: u32,
    pub completable: QuestCompletable,
    /// cmangos/vmangos/mangoszero: set to 0x04
    ///
    pub flags2: u32,
    /// cmangos/vmangos/mangoszero: set to 0x08
    ///
    pub flags3: u32,
}

impl crate::Message for SMSG_QUESTGIVER_REQUEST_ITEMS {
    const OPCODE: u32 = 0x018b;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // npc: Guid
        w.write_all(&self.npc.guid().to_le_bytes())?;

        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // title: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.title.as_bytes().iter().rev().next(), Some(&0_u8), "String `title` must not be null-terminated.");
        w.write_all(self.title.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // request_items_text: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.request_items_text.as_bytes().iter().rev().next(), Some(&0_u8), "String `request_items_text` must not be null-terminated.");
        w.write_all(self.request_items_text.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // emote_delay: u32
        w.write_all(&self.emote_delay.to_le_bytes())?;

        // emote: u32
        w.write_all(&self.emote.to_le_bytes())?;

        // auto_finish: Bool32
        w.write_all(u32::from(self.auto_finish).to_le_bytes().as_slice())?;

        // required_money: Gold
        w.write_all(u32::from(self.required_money.as_int()).to_le_bytes().as_slice())?;

        // amount_of_required_items: u32
        w.write_all(&(self.required_items.len() as u32).to_le_bytes())?;

        // required_items: QuestItemRequirement[amount_of_required_items]
        for i in self.required_items.iter() {
            i.write_into_vec(&mut w)?;
        }

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // completable: QuestCompletable
        w.write_all(&u32::from(self.completable.as_int()).to_le_bytes())?;

        // flags2: u32
        w.write_all(&self.flags2.to_le_bytes())?;

        // flags3: u32
        w.write_all(&self.flags3.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(50..=4294967294).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x018B, size: body_size as u32 });
        }

        // npc: Guid
        let npc = Guid::read(&mut r)?;

        // quest_id: u32
        let quest_id = crate::util::read_u32_le(&mut r)?;

        // title: CString
        let title = {
            let title = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(title)?
        };

        // request_items_text: CString
        let request_items_text = {
            let request_items_text = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(request_items_text)?
        };

        // emote_delay: u32
        let emote_delay = crate::util::read_u32_le(&mut r)?;

        // emote: u32
        let emote = crate::util::read_u32_le(&mut r)?;

        // auto_finish: Bool32
        let auto_finish = crate::util::read_u32_le(&mut r)? != 0;

        // required_money: Gold
        let required_money = Gold::new(crate::util::read_u32_le(&mut r)?);

        // amount_of_required_items: u32
        let amount_of_required_items = crate::util::read_u32_le(&mut r)?;

        // required_items: QuestItemRequirement[amount_of_required_items]
        let required_items = {
            let mut required_items = Vec::with_capacity(amount_of_required_items as usize);
            for i in 0..amount_of_required_items {
                required_items.push(QuestItemRequirement::read(&mut r)?);
            }
            required_items
        };

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(&mut r)?;

        // completable: QuestCompletable
        let completable: QuestCompletable = crate::util::read_u32_le(&mut r)?.try_into()?;

        // flags2: u32
        let flags2 = crate::util::read_u32_le(&mut r)?;

        // flags3: u32
        let flags3 = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            npc,
            quest_id,
            title,
            request_items_text,
            emote_delay,
            emote,
            auto_finish,
            required_money,
            required_items,
            unknown1,
            completable,
            flags2,
            flags3,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_QUESTGIVER_REQUEST_ITEMS {}

impl SMSG_QUESTGIVER_REQUEST_ITEMS {
    pub(crate) fn size(&self) -> usize {
        8 // npc: Guid
        + 4 // quest_id: u32
        + self.title.len() + 1 // title: CString
        + self.request_items_text.len() + 1 // request_items_text: CString
        + 4 // emote_delay: u32
        + 4 // emote: u32
        + 4 // auto_finish: Bool32
        + 4 // required_money: Gold
        + 4 // amount_of_required_items: u32
        + self.required_items.len() * 12 // required_items: QuestItemRequirement[amount_of_required_items]
        + 4 // unknown1: u32
        + 4 // completable: QuestCompletable
        + 4 // flags2: u32
        + 4 // flags3: u32
    }
}

