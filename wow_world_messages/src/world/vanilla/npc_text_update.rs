use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::NpcTextUpdateEmote;
use crate::world::vanilla::Language;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gossip/smsg_npc_text_update.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gossip/smsg_npc_text_update.wowm#L1):
/// ```text
/// struct NpcTextUpdate {
///     f32 probability;
///     CString[2] texts;
///     Language language;
///     NpcTextUpdateEmote[3] emotes;
/// }
/// ```
pub struct NpcTextUpdate {
    pub probability: f32,
    pub texts: [String; 2],
    pub language: Language,
    pub emotes: [NpcTextUpdateEmote; 3],
}

impl NpcTextUpdate {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // probability: f32
        w.write_all(&self.probability.to_le_bytes())?;

        // texts: CString[2]
        for i in self.texts.iter() {
            w.write_all(i.as_bytes())?;
            w.write_all(&[0])?;
        }

        // language: Language
        w.write_all(&(self.language.as_int() as u32).to_le_bytes())?;

        // emotes: NpcTextUpdateEmote[3]
        for i in self.emotes.iter() {
            i.write_into_vec(w)?;
        }

        assert_eq!(self.size() as usize, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
}

impl NpcTextUpdate {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // probability: f32
        let probability = crate::util::read_f32_le(r)?;
        // texts: CString[2]
        let mut texts = Vec::with_capacity(2);
        for i in 0..2 {
            let s = crate::util::read_c_string_to_vec(r)?;
            texts.push(String::from_utf8(s)?);
        }
        let texts = texts.try_into().unwrap();

        // language: Language
        let language: Language = crate::util::read_u32_le(r)?.try_into()?;

        // emotes: NpcTextUpdateEmote[3]
        let mut emotes = [NpcTextUpdateEmote::default(); 3];
        for i in emotes.iter_mut() {
            *i = NpcTextUpdateEmote::read(r)?;
        }

        Ok(Self {
            probability,
            texts,
            language,
            emotes,
        })
    }

}

impl NpcTextUpdate {
    pub(crate) fn size(&self) -> usize {
        4 // probability: f32
        + self.texts.iter().fold(0, |acc, x| acc + x.len() + 1) // texts: CString[2]
        + 4 // language: Language
        + 3 * 8 // emotes: NpcTextUpdateEmote[3]
    }
}

