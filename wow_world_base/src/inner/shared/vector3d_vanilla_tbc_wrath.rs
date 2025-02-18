use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/vector.wowm:4`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/vector.wowm#L4):
/// ```text
/// struct Vector3d {
///     f32 x;
///     f32 y;
///     f32 z;
/// }
/// ```
pub struct Vector3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3d {
    pub fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // x: f32
        w.write_all(&self.x.to_le_bytes())?;

        // y: f32
        w.write_all(&self.y.to_le_bytes())?;

        // z: f32
        w.write_all(&self.z.to_le_bytes())?;

        Ok(())
    }
}

impl Vector3d {
    pub fn read<R: std::io::Read>(mut r: R) -> std::result::Result<Self, std::io::Error> {
        // x: f32
        let x = crate::util::read_f32_le(&mut r)?;

        // y: f32
        let y = crate::util::read_f32_le(&mut r)?;

        // z: f32
        let z = crate::util::read_f32_le(&mut r)?;

        Ok(Self {
            x,
            y,
            z,
        })
    }

}

