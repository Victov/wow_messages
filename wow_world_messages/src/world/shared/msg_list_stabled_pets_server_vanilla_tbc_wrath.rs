use std::io::{Read, Write};
use crate::Guid;
use crate::shared::stabled_pet_vanilla_tbc_wrath::StabledPet;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/msg_list_stabled_pets_server.wowm:14`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/msg_list_stabled_pets_server.wowm#L14):
/// ```text
/// smsg MSG_LIST_STABLED_PETS_Server = 0x026F {
///     Guid npc;
///     u8 amount_of_pets;
///     u8 stable_slots;
///     StabledPet[amount_of_pets] pets;
/// }
/// ```
pub struct MSG_LIST_STABLED_PETS_Server {
    pub npc: Guid,
    pub stable_slots: u8,
    pub pets: Vec<StabledPet>,
}

impl crate::Message for MSG_LIST_STABLED_PETS_Server {
    const OPCODE: u32 = 0x026f;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // npc: Guid
        w.write_all(&self.npc.guid().to_le_bytes())?;

        // amount_of_pets: u8
        w.write_all(&(self.pets.len() as u8).to_le_bytes())?;

        // stable_slots: u8
        w.write_all(&self.stable_slots.to_le_bytes())?;

        // pets: StabledPet[amount_of_pets]
        for i in self.pets.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(10..=69898).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x026F, size: body_size as u32 });
        }

        // npc: Guid
        let npc = Guid::read(&mut r)?;

        // amount_of_pets: u8
        let amount_of_pets = crate::util::read_u8_le(&mut r)?;

        // stable_slots: u8
        let stable_slots = crate::util::read_u8_le(&mut r)?;

        // pets: StabledPet[amount_of_pets]
        let pets = {
            let mut pets = Vec::with_capacity(amount_of_pets as usize);
            for i in 0..amount_of_pets {
                pets.push(StabledPet::read(&mut r)?);
            }
            pets
        };

        Ok(Self {
            npc,
            stable_slots,
            pets,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for MSG_LIST_STABLED_PETS_Server {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for MSG_LIST_STABLED_PETS_Server {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_LIST_STABLED_PETS_Server {}

impl MSG_LIST_STABLED_PETS_Server {
    pub(crate) fn size(&self) -> usize {
        8 // npc: Guid
        + 1 // amount_of_pets: u8
        + 1 // stable_slots: u8
        + self.pets.iter().fold(0, |acc, x| acc + x.size()) // pets: StabledPet[amount_of_pets]
    }
}

