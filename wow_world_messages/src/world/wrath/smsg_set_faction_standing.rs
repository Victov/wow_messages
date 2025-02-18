use std::io::{Read, Write};
use crate::wrath::FactionStanding;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/faction/smsg_set_faction_standing.wowm:26`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/faction/smsg_set_faction_standing.wowm#L26):
/// ```text
/// smsg SMSG_SET_FACTION_STANDING = 0x0124 {
///     f32 refer_a_friend_bonus;
///     Bool any_rank_increased;
///     u32 amount_of_faction_standings;
///     FactionStanding[amount_of_faction_standings] faction_standings;
/// }
/// ```
pub struct SMSG_SET_FACTION_STANDING {
    /// All emus set to 0.
    ///
    pub refer_a_friend_bonus: f32,
    /// mangostwo: display visual effect
    ///
    pub any_rank_increased: bool,
    pub faction_standings: Vec<FactionStanding>,
}

impl crate::Message for SMSG_SET_FACTION_STANDING {
    const OPCODE: u32 = 0x0124;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // refer_a_friend_bonus: f32
        w.write_all(&self.refer_a_friend_bonus.to_le_bytes())?;

        // any_rank_increased: Bool
        w.write_all(u8::from(self.any_rank_increased).to_le_bytes().as_slice())?;

        // amount_of_faction_standings: u32
        w.write_all(&(self.faction_standings.len() as u32).to_le_bytes())?;

        // faction_standings: FactionStanding[amount_of_faction_standings]
        for i in self.faction_standings.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(9..=4294967294).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0124, size: body_size as u32 });
        }

        // refer_a_friend_bonus: f32
        let refer_a_friend_bonus = crate::util::read_f32_le(&mut r)?;

        // any_rank_increased: Bool
        let any_rank_increased = crate::util::read_u8_le(&mut r)? != 0;

        // amount_of_faction_standings: u32
        let amount_of_faction_standings = crate::util::read_u32_le(&mut r)?;

        // faction_standings: FactionStanding[amount_of_faction_standings]
        let faction_standings = {
            let mut faction_standings = Vec::with_capacity(amount_of_faction_standings as usize);
            for i in 0..amount_of_faction_standings {
                faction_standings.push(FactionStanding::read(&mut r)?);
            }
            faction_standings
        };

        Ok(Self {
            refer_a_friend_bonus,
            any_rank_increased,
            faction_standings,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SET_FACTION_STANDING {}

impl SMSG_SET_FACTION_STANDING {
    pub(crate) fn size(&self) -> usize {
        4 // refer_a_friend_bonus: f32
        + 1 // any_rank_increased: Bool
        + 4 // amount_of_faction_standings: u32
        + self.faction_standings.len() * 6 // faction_standings: FactionStanding[amount_of_faction_standings]
    }
}

