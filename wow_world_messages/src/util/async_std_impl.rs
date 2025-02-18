use crate::util::CSTRING_LARGEST_ALLOWED;
use async_std::io::ReadExt;

pub async fn astd_read_fixed_string_to_vec<R: ReadExt + Unpin + Unpin>(
    r: &mut R,
    size: usize,
) -> Result<Vec<u8>, std::io::Error> {
    let mut v = Vec::with_capacity(size);

    for _ in 0..size {
        v.push(astd_read_u8_le(r).await?);
    }

    Ok(v)
}

pub async fn astd_read_c_string_to_vec<R: ReadExt + Unpin>(
    r: &mut R,
) -> Result<Vec<u8>, std::io::Error> {
    let mut v = Vec::with_capacity(CSTRING_LARGEST_ALLOWED);

    let mut byte = astd_read_u8_le(r).await?;
    let mut count = 0;
    while byte != 0 && count != CSTRING_LARGEST_ALLOWED {
        v.push(byte);
        byte = astd_read_u8_le(r).await?;
        count += 1;
    }

    Ok(v)
}
// u8
pub async fn astd_read_u8_le<R: ReadExt + Unpin + Unpin>(r: &mut R) -> Result<u8, std::io::Error> {
    let mut v = [0_u8; 1];
    r.read_exact(&mut v).await?;
    Ok(u8::from_le_bytes(v))
}

// u16
pub async fn astd_read_u16_le<R: ReadExt + Unpin>(r: &mut R) -> Result<u16, std::io::Error> {
    let mut v = [0_u8; 2];
    r.read_exact(&mut v).await?;
    Ok(u16::from_le_bytes(v))
}

pub async fn astd_read_u16_be<R: ReadExt + Unpin>(r: &mut R) -> Result<u16, std::io::Error> {
    let mut v = [0_u8; 2];
    r.read_exact(&mut v).await?;
    Ok(u16::from_be_bytes(v))
}

// u32
pub async fn astd_read_u32_le<R: ReadExt + Unpin>(r: &mut R) -> Result<u32, std::io::Error> {
    let mut v = [0_u8; 4];
    r.read_exact(&mut v).await?;
    Ok(u32::from_le_bytes(v))
}

pub async fn astd_read_u32_be<R: ReadExt + Unpin>(r: &mut R) -> Result<u32, std::io::Error> {
    let mut v = [0_u8; 4];
    r.read_exact(&mut v).await?;
    Ok(u32::from_be_bytes(v))
}

// u64
pub async fn astd_read_u64_le<R: ReadExt + Unpin>(r: &mut R) -> Result<u64, std::io::Error> {
    let mut v = [0_u8; 8];
    r.read_exact(&mut v).await?;
    Ok(u64::from_le_bytes(v))
}

pub async fn astd_read_u64_be<R: ReadExt + Unpin>(r: &mut R) -> Result<u64, std::io::Error> {
    let mut v = [0_u8; 8];
    r.read_exact(&mut v).await?;
    Ok(u64::from_be_bytes(v))
}

// f32
pub async fn astd_read_f32_le<R: ReadExt + Unpin + Send>(r: &mut R) -> Result<f32, std::io::Error> {
    let mut v = [0_u8; 4];
    r.read_exact(&mut v).await?;
    Ok(f32::from_le_bytes(v))
}

pub async fn astd_read_f32_be<R: ReadExt + Unpin + Send>(r: &mut R) -> Result<f32, std::io::Error> {
    let mut v = [0_u8; 4];
    r.read_exact(&mut v).await?;
    Ok(f32::from_be_bytes(v))
}

// f64
pub async fn astd_read_f64_le<R: ReadExt + Unpin + Send>(r: &mut R) -> Result<f64, std::io::Error> {
    let mut v = [0_u8; 8];
    r.read_exact(&mut v).await?;
    Ok(f64::from_le_bytes(v))
}

pub async fn astd_read_f64_be<R: ReadExt + Unpin + Send>(r: &mut R) -> Result<f64, std::io::Error> {
    let mut v = [0_u8; 8];
    r.read_exact(&mut v).await?;
    Ok(f64::from_be_bytes(v))
}
