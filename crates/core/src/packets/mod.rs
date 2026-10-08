use super::Error;
use rkyv::{Archive, Deserialize, Serialize};
use std::io::{Read, Write};

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

pub fn send<S: Write, T>(stream: &mut S, value: &T) -> Result<(), Error>
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

    let len = bytes.len() as u8;

    stream.write_all(&len.to_be_bytes())?;
    stream.write_all(&bytes)?;

    Ok(())
}

fn read_length<S: Read>(stream: &mut S) -> Result<u32, Error> {
    let mut buf = [0u8; 4];

    stream.read_exact(&mut buf)?;

    Ok(u32::from_be_bytes(buf))
}

pub fn read<S: Read>(stream: &mut S) -> Result<Packet, Error> {
    let len = match read_length(stream) {
        Ok(len) => len,
        Err(e) => return Err(e.into()),
    };

    if len > 1024 {
        return Err(format!("excessive packet size: {}", len).into());
    }

    let mut bytes = vec![0u8; len as usize];
    if let Err(e) = stream.read_exact(&mut bytes) {
        return Err(e.into());
    };

    Ok(rkyv::from_bytes::<Packet, rkyv::rancor::Error>(&bytes)?)
}
