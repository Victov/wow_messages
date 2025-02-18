use crate::util::CSTRING_LARGEST_ALLOWED;
use async_std::io::{ReadExt, WriteExt};

pub async fn astd_read_fixed_string_to_vec<R: ReadExt + Unpin + Unpin>(
    mut r: R,
    size: usize,
) -> Result<Vec<u8>, std::io::Error> {
    let mut v = Vec::with_capacity(size);

    for _ in 0..size {
        v.push(astd_read_u8_le(&mut r).await?);
    }

    Ok(v)
}

pub async fn astd_read_c_string_to_vec<R: ReadExt + Unpin>(
    mut r: R,
) -> Result<Vec<u8>, std::io::Error> {
    let mut v = Vec::with_capacity(CSTRING_LARGEST_ALLOWED);

    let mut byte = astd_read_u8_le(&mut r).await?;
    let mut count = 0;
    while byte != 0 && count != CSTRING_LARGEST_ALLOWED {
        v.push(byte);
        byte = astd_read_u8_le(&mut r).await?;
        count += 1;
    }

    Ok(v)
}

// u8
pub async fn astd_read_u8_le<R: ReadExt + Unpin + Unpin>(mut r: R) -> Result<u8, std::io::Error> {
    let mut v = [0_u8; 1];
    r.read_exact(&mut v).await?;
    Ok(u8::from_le_bytes(v))
}

pub async fn astd_write_u8_le<W: WriteExt + Unpin>(w: &mut W, v: u8) -> Result<(), std::io::Error> {
    w.write_all(&v.to_le_bytes()).await?;
    Ok(())
}

// u16
pub async fn astd_read_u16_le<R: ReadExt + Unpin>(mut r: R) -> Result<u16, std::io::Error> {
    let mut v = [0_u8; 2];
    r.read_exact(&mut v).await?;
    Ok(u16::from_le_bytes(v))
}

pub async fn astd_read_u16_be<R: ReadExt + Unpin>(mut r: R) -> Result<u16, std::io::Error> {
    let mut v = [0_u8; 2];
    r.read_exact(&mut v).await?;
    Ok(u16::from_be_bytes(v))
}

pub async fn astd_write_u16_le<W: WriteExt + Unpin>(
    w: &mut W,
    v: u16,
) -> Result<(), std::io::Error> {
    w.write_all(&v.to_le_bytes()).await?;
    Ok(())
}

pub async fn astd_write_u16_be<W: WriteExt + Unpin>(
    w: &mut W,
    v: u16,
) -> Result<(), std::io::Error> {
    w.write_all(&v.to_be_bytes()).await?;
    Ok(())
}

// u32
pub async fn astd_read_u32_le<R: ReadExt + Unpin>(mut r: R) -> Result<u32, std::io::Error> {
    let mut v = [0_u8; 4];
    r.read_exact(&mut v).await?;
    Ok(u32::from_le_bytes(v))
}

pub async fn astd_read_u32_be<R: ReadExt + Unpin>(mut r: R) -> Result<u32, std::io::Error> {
    let mut v = [0_u8; 4];
    r.read_exact(&mut v).await?;
    Ok(u32::from_be_bytes(v))
}

pub async fn astd_write_u32_le<W: WriteExt + Unpin>(
    w: &mut W,
    v: u32,
) -> Result<(), std::io::Error> {
    w.write_all(&v.to_le_bytes()).await?;
    Ok(())
}

pub async fn astd_write_u32_be<W: WriteExt + Unpin>(
    w: &mut W,
    v: u32,
) -> Result<(), std::io::Error> {
    w.write_all(&v.to_be_bytes()).await?;
    Ok(())
}

// u64
pub async fn astd_read_u64_le<R: ReadExt + Unpin>(mut r: R) -> Result<u64, std::io::Error> {
    let mut v = [0_u8; 8];
    r.read_exact(&mut v).await?;
    Ok(u64::from_le_bytes(v))
}

pub async fn astd_read_u64_be<R: ReadExt + Unpin>(mut r: R) -> Result<u64, std::io::Error> {
    let mut v = [0_u8; 8];
    r.read_exact(&mut v).await?;
    Ok(u64::from_be_bytes(v))
}

pub async fn astd_write_u64_le<W: WriteExt + Unpin>(
    w: &mut W,
    v: u64,
) -> Result<(), std::io::Error> {
    w.write_all(&v.to_le_bytes()).await?;
    Ok(())
}

pub async fn astd_write_u64_be<W: WriteExt + Unpin>(
    w: &mut W,
    v: u64,
) -> Result<(), std::io::Error> {
    w.write_all(&v.to_be_bytes()).await?;
    Ok(())
}
