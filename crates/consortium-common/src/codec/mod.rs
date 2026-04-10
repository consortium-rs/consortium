pub trait Codec: Sized {
    type Error: core::fmt::Debug;
}

pub trait CodecFor<T>: Codec {
    type Decoded<'buf>: 'buf
    where
        T: 'buf;

    fn encode<'buf>(msg: &T, buf: &'buf mut [u8]) -> Result<usize, Self::Error>;

    fn decode<'buf>(buf: &'buf [u8]) -> Result<Self::Decoded<'buf>, Self::Error>
    where
        T: 'buf;
}
