use std::io::{Read, Write};
use crate::Guid;
use crate::wrath::QuestPartyMessage;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/msg_quest_push_result.wowm:58`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/msg_quest_push_result.wowm#L58):
/// ```text
/// msg MSG_QUEST_PUSH_RESULT = 0x0276 {
///     Guid guid;
///     QuestPartyMessage message;
/// }
/// ```
pub struct MSG_QUEST_PUSH_RESULT {
    pub guid: Guid,
    pub message: QuestPartyMessage,
}

impl crate::Message for MSG_QUEST_PUSH_RESULT {
    const OPCODE: u32 = 0x0276;

    fn size_without_header(&self) -> u32 {
        9
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // message: QuestPartyMessage
        w.write_all(&u8::from(self.message.as_int()).to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 9 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0276, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(&mut r)?;

        // message: QuestPartyMessage
        let message: QuestPartyMessage = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            guid,
            message,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for MSG_QUEST_PUSH_RESULT {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_QUEST_PUSH_RESULT {}

