use consortium_common::codec::{Codec, CodecFor};

pub struct PostcardCodec;

impl Codec for PostcardCodec {
    type Error = postcard::Error;
}

impl<T> CodecFor<T> for PostcardCodec
where
    T: serde::Serialize + serde::de::DeserializeOwned,
{
    type Decoded<'buf>
        = T
    where
        T: 'buf;

    fn encode<'buf>(msg: &T, buf: &'buf mut [u8]) -> Result<usize, postcard::Error> {
        let written = postcard::to_slice(msg, buf)?;
        Ok(written.len())
    }

    fn decode(buf: &[u8]) -> Result<T, postcard::Error> {
        postcard::from_bytes(buf)
    }
}
