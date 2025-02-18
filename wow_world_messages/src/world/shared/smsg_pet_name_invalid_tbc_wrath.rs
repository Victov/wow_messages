use std::io::{Read, Write};
use wow_world_base::shared::declined_pet_name_included_tbc_wrath::DeclinedPetNameIncluded;
use wow_world_base::shared::pet_name_invalid_reason_tbc_wrath::PetNameInvalidReason;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Some emulators have this with fields, but it has been verified to be empty on 1.12 through reverse engineering.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/smsg_pet_name_invalid.wowm:31`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/smsg_pet_name_invalid.wowm#L31):
/// ```text
/// smsg SMSG_PET_NAME_INVALID = 0x0178 {
///     (u32)PetNameInvalidReason reason;
///     CString name;
///     DeclinedPetNameIncluded included;
///     if (included == INCLUDED) {
///         CString[5] declined_names;
///     }
/// }
/// ```
pub struct SMSG_PET_NAME_INVALID {
    pub reason: PetNameInvalidReason,
    pub name: String,
    pub included: SMSG_PET_NAME_INVALID_DeclinedPetNameIncluded,
}

impl crate::Message for SMSG_PET_NAME_INVALID {
    const OPCODE: u32 = 0x0178;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // reason: PetNameInvalidReason
        w.write_all(&u32::from(self.reason.as_int()).to_le_bytes())?;

        // name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.name.as_bytes().iter().rev().next(), Some(&0_u8), "String `name` must not be null-terminated.");
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // included: DeclinedPetNameIncluded
        w.write_all(&u8::from(self.included.as_int()).to_le_bytes())?;

        match &self.included {
            SMSG_PET_NAME_INVALID_DeclinedPetNameIncluded::NotIncluded => {}
            SMSG_PET_NAME_INVALID_DeclinedPetNameIncluded::Included {
                declined_names,
            } => {
                // declined_names: CString[5]
                for i in declined_names.iter() {
                    w.write_all(i.as_bytes())?;
                    w.write_all(&[0])?;
                }

            }
        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(6..=1541).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0178, size: body_size as u32 });
        }

        // reason: PetNameInvalidReason
        let reason: PetNameInvalidReason = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        // name: CString
        let name = {
            let name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(name)?
        };

        // included: DeclinedPetNameIncluded
        let included: DeclinedPetNameIncluded = crate::util::read_u8_le(&mut r)?.try_into()?;

        let included_if = match included {
            DeclinedPetNameIncluded::NotIncluded => SMSG_PET_NAME_INVALID_DeclinedPetNameIncluded::NotIncluded,
            DeclinedPetNameIncluded::Included => {
                // declined_names: CString[5]
                let declined_names = {
                    let mut declined_names = [(); 5].map(|_| String::default());
                    for i in declined_names.iter_mut() {
                        let s = crate::util::read_c_string_to_vec(&mut r)?;
                        *i = String::from_utf8(s)?;
                    }
                    declined_names
                };

                SMSG_PET_NAME_INVALID_DeclinedPetNameIncluded::Included {
                    declined_names,
                }
            }
        };

        Ok(Self {
            reason,
            name,
            included: included_if,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_PET_NAME_INVALID {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_PET_NAME_INVALID {}

impl SMSG_PET_NAME_INVALID {
    pub(crate) fn size(&self) -> usize {
        4 // reason: PetNameInvalidReason
        + self.name.len() + 1 // name: CString
        + self.included.size() // included: SMSG_PET_NAME_INVALID_DeclinedPetNameIncluded
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_PET_NAME_INVALID_DeclinedPetNameIncluded {
    NotIncluded,
    Included {
        declined_names: [String; 5],
    },
}

impl Default for SMSG_PET_NAME_INVALID_DeclinedPetNameIncluded {
    fn default() -> Self {
        // First enumerator without any fields
        Self::NotIncluded
    }
}

impl SMSG_PET_NAME_INVALID_DeclinedPetNameIncluded {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NotIncluded => 0,
            Self::Included { .. } => 1,
        }
    }

}

impl SMSG_PET_NAME_INVALID_DeclinedPetNameIncluded {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::NotIncluded => {
                1
            }
            Self::Included {
                declined_names,
            } => {
                1
                + declined_names.iter().fold(0, |acc, x| acc + x.len() + 1) // declined_names: CString[5]
            }
        }
    }
}

