use std::io::{Read, Write};
use crate::tbc::QuestItemReward;
use wow_world_base::shared::gold_vanilla_tbc_wrath::Gold;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_questgiver_quest_complete.wowm:16`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_questgiver_quest_complete.wowm#L16):
/// ```text
/// smsg SMSG_QUESTGIVER_QUEST_COMPLETE = 0x0191 {
///     u32 quest_id;
///     u32 unknown;
///     u32 experience_reward;
///     Gold money_reward;
///     u32 honor_reward;
///     u32 amount_of_item_rewards;
///     QuestItemReward[amount_of_item_rewards] item_rewards;
/// }
/// ```
pub struct SMSG_QUESTGIVER_QUEST_COMPLETE {
    pub quest_id: u32,
    /// cmangos/vmangos/mangoszero: set to 0x03
    ///
    pub unknown: u32,
    pub experience_reward: u32,
    pub money_reward: Gold,
    pub honor_reward: u32,
    pub item_rewards: Vec<QuestItemReward>,
}

impl crate::Message for SMSG_QUESTGIVER_QUEST_COMPLETE {
    const OPCODE: u32 = 0x0191;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // unknown: u32
        w.write_all(&self.unknown.to_le_bytes())?;

        // experience_reward: u32
        w.write_all(&self.experience_reward.to_le_bytes())?;

        // money_reward: Gold
        w.write_all(u32::from(self.money_reward.as_int()).to_le_bytes().as_slice())?;

        // honor_reward: u32
        w.write_all(&self.honor_reward.to_le_bytes())?;

        // amount_of_item_rewards: u32
        w.write_all(&(self.item_rewards.len() as u32).to_le_bytes())?;

        // item_rewards: QuestItemReward[amount_of_item_rewards]
        for i in self.item_rewards.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(24..=4294967294).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0191, size: body_size as u32 });
        }

        // quest_id: u32
        let quest_id = crate::util::read_u32_le(&mut r)?;

        // unknown: u32
        let unknown = crate::util::read_u32_le(&mut r)?;

        // experience_reward: u32
        let experience_reward = crate::util::read_u32_le(&mut r)?;

        // money_reward: Gold
        let money_reward = Gold::new(crate::util::read_u32_le(&mut r)?);

        // honor_reward: u32
        let honor_reward = crate::util::read_u32_le(&mut r)?;

        // amount_of_item_rewards: u32
        let amount_of_item_rewards = crate::util::read_u32_le(&mut r)?;

        // item_rewards: QuestItemReward[amount_of_item_rewards]
        let item_rewards = {
            let mut item_rewards = Vec::with_capacity(amount_of_item_rewards as usize);
            for i in 0..amount_of_item_rewards {
                item_rewards.push(QuestItemReward::read(&mut r)?);
            }
            item_rewards
        };

        Ok(Self {
            quest_id,
            unknown,
            experience_reward,
            money_reward,
            honor_reward,
            item_rewards,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_QUESTGIVER_QUEST_COMPLETE {}

impl SMSG_QUESTGIVER_QUEST_COMPLETE {
    pub(crate) fn size(&self) -> usize {
        4 // quest_id: u32
        + 4 // unknown: u32
        + 4 // experience_reward: u32
        + 4 // money_reward: Gold
        + 4 // honor_reward: u32
        + 4 // amount_of_item_rewards: u32
        + self.item_rewards.len() * 8 // item_rewards: QuestItemReward[amount_of_item_rewards]
    }
}

