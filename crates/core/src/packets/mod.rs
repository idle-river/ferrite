use rkyv::{Archive, Deserialize, Serialize};

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
