use consortium_common::codec::{Codec, CodecFor};

// consortium-codec-prost
pub struct ProstCodec;

#[derive(Debug)]
pub enum ProstError {
    Encode(prost::EncodeError),
    Decode(prost::DecodeError),
    BufferTooSmall,
}

impl Codec for ProstCodec {
    type Error = ProstError;
}

impl<T> CodecFor<T> for ProstCodec
where
    T: prost::Message + Default,
{
    type Decoded<'buf>
        = T
    where
        T: 'buf;

    fn encode<'buf>(msg: &T, buf: &'buf mut [u8]) -> Result<usize, ProstError> {
        let len = msg.encoded_len();
        if len > buf.len() {
            return Err(ProstError::BufferTooSmall);
        }
        let mut slice = &mut buf[..len];
        msg.encode(&mut slice).map_err(ProstError::Encode)?;
        Ok(len)
    }

    fn decode(buf: &[u8]) -> Result<T, ProstError> {
        T::decode(buf).map_err(ProstError::Decode)
    }
}
