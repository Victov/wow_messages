use std::collections::BTreeMap;
use std::io::Read;

pub const OBJECT: u32 = 0x0001;
pub const ITEM: u32 = 0x0002;
pub const CONTAINER: u32 = 0x0004;
pub const UNIT: u32 = 0x0008;
pub const PLAYER: u32 = 0x0010;
pub const GAMEOBJECT: u32 = 0x0020;
pub const DYNAMICOBJECT: u32 = 0x0040;
pub const CORPSE: u32 = 0x0080;

pub(crate) fn update_mask_size(dirty_mask: &[u32], header: &[u32]) -> usize {
    let amount_of_blocks = dirty_mask.len() * 4;
    let amount_of_values = dirty_mask
        .iter()
        .zip(header)
        .fold(0, |acc, x| acc + (x.0 & x.1).count_ones())
        * 4;

    1 + amount_of_blocks + amount_of_values as usize
}

pub(crate) fn array_set(array: &mut Vec<u32>, bit: u16) {
    let index = bit / 32;
    let offset = bit % 32;

    if index >= array.len() as u16 {
        let extras = index - array.len() as u16;
        for _ in 0..=extras {
            array.push(0);
        }
    }

    array[index as usize] |= 1 << offset;
}

pub(crate) fn array_reset(array: &mut [u32]) {
    for item in array {
        *item = 0;
    }
}

pub(crate) fn array_fill_ones(array: &mut [u32]) {
    for item in array {
        *item = 0xFFFFFFFF;
    }
}

pub(crate) fn has_array_bit_set(array: &[u32], bit: u16) -> bool {
    let index = bit / 32;
    let offset = bit % 32;
    let item = array[index as usize];

    item & (1 << offset) > 0
}

pub(crate) fn has_any_bit_set(array: &[u32]) -> bool {
    array.iter().any(|h| *h != 0)
}

pub(crate) fn write_into_vec(
    mut v: impl std::io::Write,
    header: &[u32],
    dirty_mask: &[u32],
    values: &BTreeMap<u16, u32>,
) -> Result<(), std::io::Error> {
    assert_eq!(header.len(), dirty_mask.len());

    v.write_all(&[header.len() as u8])?;

    for (h, d) in header.iter().zip(dirty_mask) {
        let masked = h & d;
        v.write_all(masked.to_le_bytes().as_slice())?;
    }

    for (&index, value) in values.iter() {
        if has_array_bit_set(header, index) && has_array_bit_set(dirty_mask, index) {
            v.write_all(&value.to_le_bytes())?;
        }
    }

    Ok(())
}

pub(crate) fn read_inner(
    r: &mut impl Read,
) -> Result<(Vec<u32>, BTreeMap<u16, u32>), std::io::Error> {
    let amount_of_blocks = crate::util::read_u8_le(r)?;

    let mut header = Vec::new();
    for _ in 0..amount_of_blocks {
        header.push(crate::util::read_u32_le(r)?);
    }

    let mut values = BTreeMap::new();
    for (index, block) in header.iter().enumerate() {
        for bit in 0..32 {
            if (block & 1 << bit) != 0 {
                values.insert(index as u16 * 32 + bit, crate::util::read_u32_le(r)?);
            }
        }
    }

    Ok((header, values))
}

#[allow(unused)]
macro_rules! update_item {
    ($name:ident, $builder_name:ident, $type_value:expr) => {
        #[derive(Debug, Hash, Clone, Default, PartialEq, Eq, PartialOrd)]
        pub struct $builder_name {
            header: Vec<u32>,
            values: BTreeMap<u16, u32>,
        }

        impl $builder_name {
            pub fn finalize(self) -> $name {
                $name::from_inners(self.header, self.values)
            }

            pub(crate) fn header_set(&mut self, bit: u16, value: u32) {
                self.values.insert(bit.into(), value);
                $crate::helper::update_mask_common::array_set(&mut self.header, bit);
            }

            pub fn new() -> Self {
                const OBJECT_FIELD_TYPE: u16 = 2;

                let mut header = vec![];
                let mut values = BTreeMap::new();

                $crate::helper::update_mask_common::array_set(&mut header, OBJECT_FIELD_TYPE);
                values.insert(
                    OBJECT_FIELD_TYPE,
                    $crate::helper::update_mask_common::OBJECT | $type_value,
                );

                Self { header, values }
            }
        }

        #[derive(Debug, Hash, Clone, Default, PartialEq, Eq, PartialOrd)]
        pub struct $name {
            header: Vec<u32>,
            dirty_mask: Vec<u32>,
            values: BTreeMap<u16, u32>,
        }

        impl $name {
            pub fn new() -> Self {
                $builder_name::new().finalize()
            }

            pub fn builder() -> $builder_name {
                $builder_name::new()
            }

            fn from_inners(header: Vec<u32>, values: BTreeMap<u16, u32>) -> Self {
                Self {
                    header: header.clone(),
                    dirty_mask: header,
                    values,
                }
            }

            pub(crate) fn header_set(&mut self, bit: u16, value: u32) {
                self.values.insert(bit.into(), value);
                $crate::helper::update_mask_common::array_set(&mut self.header, bit);
                //Any modification to the header also means we set it dirty
                $crate::helper::update_mask_common::array_set(&mut self.dirty_mask, bit);
            }

            pub fn dirty_reset(&mut self) {
                $crate::helper::update_mask_common::array_reset(&mut self.dirty_mask);
            }

            pub fn mark_fully_dirty(&mut self) {
                $crate::helper::update_mask_common::array_fill_ones(&mut self.dirty_mask);
            }

            pub fn has_any_dirty_fields(&self) -> bool {
                $crate::helper::update_mask_common::has_any_bit_set(&self.dirty_mask)
            }

            pub fn is_bit_dirty(&self, bit: u16) -> bool {
                $crate::helper::update_mask_common::has_array_bit_set(&self.dirty_mask, bit)
            }

            pub(crate) fn write_into_vec(
                &self,
                v: impl std::io::Write,
            ) -> Result<(), std::io::Error> {
                $crate::helper::update_mask_common::write_into_vec(
                    v,
                    &self.header,
                    &self.dirty_mask,
                    &self.values,
                )
            }
        }
    };
}

#[allow(unused)]
macro_rules! update_mask {
    () => {
        #[derive(Debug, Hash, Clone, PartialEq, Eq, PartialOrd)]
        pub enum UpdateMask {
            Item(UpdateItem),
            Container(UpdateContainer),
            Unit(UpdateUnit),
            Player(UpdatePlayer),
            GameObject(UpdateGameObject),
            DynamicObject(UpdateDynamicObject),
            Corpse(UpdateCorpse),
        }

        impl Default for UpdateMask {
            fn default() -> Self {
                Self::Item(Default::default())
            }
        }

        impl UpdateMask {
            pub(crate) fn read(r: &mut impl Read) -> Result<Self, io::Error> {
                let (header, values) = update_mask_common::read_inner(r)?;

                let ty = match values.get(&2) {
                    None => {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "Missing object TYPE",
                        ))
                    }
                    Some(ty) => *ty,
                };

                Ok(if (ty & CONTAINER) != 0 {
                    Self::Container(UpdateContainer::from_inners(header, values))
                } else if (ty & ITEM) != 0 {
                    Self::Item(UpdateItem::from_inners(header, values))
                } else if (ty & PLAYER) != 0 {
                    Self::Player(UpdatePlayer::from_inners(header, values))
                } else if (ty & UNIT) != 0 {
                    Self::Unit(UpdateUnit::from_inners(header, values))
                } else if (ty & GAMEOBJECT) != 0 {
                    Self::GameObject(UpdateGameObject::from_inners(header, values))
                } else if (ty & DYNAMICOBJECT) != 0 {
                    Self::DynamicObject(UpdateDynamicObject::from_inners(header, values))
                } else if (ty & CORPSE) != 0 {
                    Self::Corpse(UpdateCorpse::from_inners(header, values))
                } else {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Object type not valid",
                    ));
                })
            }

            pub(crate) fn write_into_vec(
                &self,
                v: impl std::io::Write,
            ) -> Result<(), std::io::Error> {
                match self {
                    UpdateMask::Item(i) => i.write_into_vec(v),
                    UpdateMask::Container(i) => i.write_into_vec(v),
                    UpdateMask::Unit(i) => i.write_into_vec(v),
                    UpdateMask::Player(i) => i.write_into_vec(v),
                    UpdateMask::GameObject(i) => i.write_into_vec(v),
                    UpdateMask::DynamicObject(i) => i.write_into_vec(v),
                    UpdateMask::Corpse(i) => i.write_into_vec(v),
                }
            }

            pub(crate) fn size(&self) -> usize {
                match self {
                    UpdateMask::Item(i) => update_mask_size(&i.dirty_mask, &i.header),
                    UpdateMask::Container(i) => update_mask_size(&i.dirty_mask, &i.header),
                    UpdateMask::Unit(i) => update_mask_size(&i.dirty_mask, &i.header),
                    UpdateMask::Player(i) => update_mask_size(&i.dirty_mask, &i.header),
                    UpdateMask::GameObject(i) => update_mask_size(&i.dirty_mask, &i.header),
                    UpdateMask::DynamicObject(i) => update_mask_size(&i.dirty_mask, &i.header),
                    UpdateMask::Corpse(i) => update_mask_size(&i.dirty_mask, &i.header),
                }
            }
        }
    };
}

#[allow(unused)]
pub(crate) use update_item;
#[allow(unused)]
pub(crate) use update_mask;

#[allow(unused)]
macro_rules! skill_info {
    ($skill:ty, $index:ty) => {
        #[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
        pub struct SkillInfo {
            pub skill: $skill,
            pub skill_step: u16,
            pub minimum: u16,
            pub maximum: u16,
            pub permanent_bonus: u16,
            pub temporary_bonus: u16,
        }

        impl SkillInfo {
            pub const fn new(
                skill: $skill,
                skill_step: u16,
                minimum: u16,
                maximum: u16,
                permanent_bonus: u16,
                temporary_bonus: u16,
            ) -> Self {
                Self {
                    skill,
                    skill_step,
                    minimum,
                    maximum,
                    permanent_bonus,
                    temporary_bonus,
                }
            }

            pub(crate) const fn mask_values(&self, index: $index) -> [(u16, u32); 3] {
                let offset = index.offset();

                [
                    (
                        offset,
                        $crate::util::u16s_to_u32(self.skill_step, self.skill.as_int()),
                    ),
                    (
                        offset + 1,
                        $crate::util::u16s_to_u32(self.maximum, self.minimum),
                    ),
                    (
                        offset + 2,
                        $crate::util::u16s_to_u32(self.permanent_bonus, self.temporary_bonus),
                    ),
                ]
            }

            pub(crate) fn from_range<'a>(
                mut range: impl Iterator<Item = (&'a u16, &'a u32)>,
            ) -> Option<Self> {
                let (_, first) = range.next()?;
                let (_, second) = range.next()?;
                let (_, third) = range.next()?;

                let (skill_step, skill) = $crate::util::u32_to_u16s(*first);
                let (maximum, minimum) = $crate::util::u32_to_u16s(*second);
                let (permanent_bonus, temporary_bonus) = $crate::util::u32_to_u16s(*third);
                let skill = match <$skill>::try_from(skill) {
                    Ok(v) => v,
                    Err(_) => return None,
                };

                Some(Self {
                    skill,
                    skill_step,
                    minimum,
                    maximum,
                    permanent_bonus,
                    temporary_bonus,
                })
            }
        }
    };
}
#[allow(unused)]
pub(crate) use skill_info;
