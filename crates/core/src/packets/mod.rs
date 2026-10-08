use super::Error;
use rkyv::{Archive, Deserialize, Serialize};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
#[rkyv(
    // This will generate a PartialEq impl between our unarchived
    // and archived types
    compare(PartialEq),
    // Derives can be passed through to the generated type:
    derive(Debug),
)]
pub enum Packet {
    Get(String),
    Set(String, String),
    Del(String),
    Exists(String),
}

impl Packet {
    pub fn encode(&self) -> Result<Vec<u8>, rkyv::rancor::Error> {
        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(self)?;
        Ok(bytes.to_vec())
    }

    pub fn decode(bytes: Vec<u8>) -> Result<Self, rkyv::rancor::Error> {
        rkyv::from_bytes::<Packet, rkyv::rancor::Error>(&bytes)
    }
}

pub async fn send<S: AsyncWrite + Unpin, T>(stream: &mut S, value: &T) -> Result<(), Error>
where
    T: for<'a> Serialize<
        rkyv::api::high::HighSerializer<
            rkyv::util::AlignedVec,
            rkyv::ser::allocator::ArenaHandle<'a>,
            rkyv::rancor::Error,
        >,
    >,
{
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(value)?;
    let len = bytes.len() as u32;

    stream.write_all(&len.to_be_bytes()).await?;
    stream.write_all(&bytes).await?;

    Ok(())
}

async fn read_length<S: AsyncRead + Unpin>(stream: &mut S) -> Result<u32, Error> {
    let mut buf = [0u8; 4];

    stream.read_exact(&mut buf).await?;

    Ok(u32::from_be_bytes(buf))
}

pub async fn read<S: AsyncRead + Unpin>(stream: &mut S) -> Result<Packet, Error> {
    let len = read_length(stream).await?;

    if len > 1024 {
        return Err(format!("excessive packet size: {}", len).into());
    }

    let mut bytes = vec![0u8; len as usize];
    stream.read_exact(&mut bytes).await?;

    Ok(rkyv::from_bytes::<Packet, rkyv::rancor::Error>(&bytes)?)
}
