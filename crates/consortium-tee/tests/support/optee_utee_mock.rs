// Host stand-in for the slice of `optee_utee` 0.10's typed parameter API that
// `#[tee_command]`/`tee_service!` expand to. Include it at a test crate's root next to
// `extern crate self as optee_utee;` so `::optee_utee::...` paths resolve here.
//
// Method names, receivers, and return types follow the real crate (`optee_utee::parameter`)
// so that an expansion that type-checks against this mock also type-checks against
// OP-TEE. What differs is the storage: the real wrappers point into `TEE_Param` unions
// owned by the TEE, these borrow a test-owned `Raw` slot instead. Memref semantics that
// the generated code relies on are kept: reads copy, writes are bounds-checked and set
// the size the client reads back, and every accessor checks the slot's type.

#[allow(dead_code)]
mod optee_utee_mock {
    use std::vec::Vec;

    pub type Result<T> = core::result::Result<T, Error>;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ErrorKind {
        BadParameters,
        ShortBuffer,
        Generic,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct Error(ErrorKind);

    impl Error {
        pub fn new(kind: ErrorKind) -> Self {
            Self(kind)
        }

        pub fn kind(&self) -> ErrorKind {
            self.0
        }
    }

    impl From<ErrorKind> for Error {
        fn from(kind: ErrorKind) -> Self {
            Self(kind)
        }
    }

    fn bad<T>() -> Result<T> {
        Err(ErrorKind::BadParameters.into())
    }

    /// One test-owned parameter slot, tagged with its GlobalPlatform type. Memref sizes
    /// start at the buffer length and are updated by TA writes, as in `TEE_Param`.
    pub enum Raw {
        None,
        ValueInput(u32, u32),
        ValueOutput(u32, u32),
        ValueInout(u32, u32),
        MemrefInput(Vec<u8>),
        MemrefOutput(Vec<u8>, usize),
        MemrefInout(Vec<u8>, usize),
    }

    impl Raw {
        pub fn output(len: usize) -> Self {
            Self::MemrefOutput(std::vec![0; len], len)
        }

        pub fn inout(buf: Vec<u8>) -> Self {
            let len = buf.len();
            Self::MemrefInout(buf, len)
        }

        pub fn value(&self) -> (u32, u32) {
            match self {
                Self::ValueInput(a, b) | Self::ValueOutput(a, b) | Self::ValueInout(a, b) => (*a, *b),
                _ => panic!("not a value slot"),
            }
        }

        /// The bytes a client would read back: the buffer up to the reported size.
        pub fn written(&self) -> &[u8] {
            match self {
                Self::MemrefOutput(buf, size) | Self::MemrefInout(buf, size) => &buf[..*size],
                _ => panic!("not a writable memref slot"),
            }
        }
    }

    pub trait ParameterValueRead {
        fn get_a(&self) -> u32;
        fn get_b(&self) -> u32;
    }

    pub trait ParameterValueWrite {
        fn set_a(&mut self, a: u32);
        fn set_b(&mut self, b: u32);
    }

    pub trait ParameterMemref {
        fn buffer_len(&self) -> usize;
    }

    pub trait ParameterMemrefRead: ParameterMemref {
        fn read_to_vec(&self) -> Vec<u8>;
    }

    pub trait ParameterMemrefWrite: ParameterMemref {
        fn set_output<T: AsRef<[u8]>>(&mut self, data: T) -> Result<()> {
            self.write_at(0, data)
        }

        fn write_at<T: AsRef<[u8]>>(&mut self, offset: usize, data: T) -> Result<()>;
    }

    pub struct ParameterValueInput {
        a: u32,
        b: u32,
    }

    pub struct ParameterValueOutput<'a>(&'a mut u32, &'a mut u32);

    pub struct ParameterValueInout<'a>(&'a mut u32, &'a mut u32);

    pub struct ParameterMemrefInput<'a>(&'a [u8]);

    pub struct ParameterMemrefOutput<'a>(&'a mut [u8], &'a mut usize);

    pub struct ParameterMemrefInout<'a>(&'a mut [u8], &'a mut usize);

    impl ParameterValueRead for ParameterValueInput {
        fn get_a(&self) -> u32 {
            self.a
        }
        fn get_b(&self) -> u32 {
            self.b
        }
    }

    impl ParameterValueRead for ParameterValueInout<'_> {
        fn get_a(&self) -> u32 {
            *self.0
        }
        fn get_b(&self) -> u32 {
            *self.1
        }
    }

    impl ParameterValueWrite for ParameterValueOutput<'_> {
        fn set_a(&mut self, a: u32) {
            *self.0 = a;
        }
        fn set_b(&mut self, b: u32) {
            *self.1 = b;
        }
    }

    impl ParameterValueWrite for ParameterValueInout<'_> {
        fn set_a(&mut self, a: u32) {
            *self.0 = a;
        }
        fn set_b(&mut self, b: u32) {
            *self.1 = b;
        }
    }

    impl ParameterMemref for ParameterMemrefInput<'_> {
        fn buffer_len(&self) -> usize {
            self.0.len()
        }
    }

    impl ParameterMemref for ParameterMemrefOutput<'_> {
        fn buffer_len(&self) -> usize {
            self.0.len()
        }
    }

    impl ParameterMemref for ParameterMemrefInout<'_> {
        fn buffer_len(&self) -> usize {
            self.0.len()
        }
    }

    impl ParameterMemrefRead for ParameterMemrefInput<'_> {
        fn read_to_vec(&self) -> Vec<u8> {
            self.0.to_vec()
        }
    }

    impl ParameterMemrefRead for ParameterMemrefInout<'_> {
        fn read_to_vec(&self) -> Vec<u8> {
            self.0.to_vec()
        }
    }

    fn write_at(buf: &mut [u8], size: &mut usize, offset: usize, data: &[u8]) -> Result<()> {
        let end = offset.checked_add(data.len()).ok_or(ErrorKind::ShortBuffer)?;
        if end > buf.len() {
            return Err(ErrorKind::ShortBuffer.into());
        }
        buf[offset..end].copy_from_slice(data);
        *size = end;
        Ok(())
    }

    impl ParameterMemrefWrite for ParameterMemrefOutput<'_> {
        fn write_at<T: AsRef<[u8]>>(&mut self, offset: usize, data: T) -> Result<()> {
            write_at(self.0, self.1, offset, data.as_ref())
        }
    }

    impl ParameterMemrefWrite for ParameterMemrefInout<'_> {
        fn write_at<T: AsRef<[u8]>>(&mut self, offset: usize, data: T) -> Result<()> {
            write_at(self.0, self.1, offset, data.as_ref())
        }
    }

    pub enum ParameterAny<'a> {
        None,
        ValueInput(ParameterValueInput),
        ValueInout(ParameterValueInout<'a>),
        ValueOutput(ParameterValueOutput<'a>),
        MemrefInput(ParameterMemrefInput<'a>),
        MemrefInout(ParameterMemrefInout<'a>),
        MemrefOutput(ParameterMemrefOutput<'a>),
    }

    impl<'a> ParameterAny<'a> {
        pub fn from_raw(raw: &'a mut Raw) -> Self {
            match raw {
                Raw::None => Self::None,
                Raw::ValueInput(a, b) => Self::ValueInput(ParameterValueInput { a: *a, b: *b }),
                Raw::ValueOutput(a, b) => Self::ValueOutput(ParameterValueOutput(a, b)),
                Raw::ValueInout(a, b) => Self::ValueInout(ParameterValueInout(a, b)),
                Raw::MemrefInput(buf) => Self::MemrefInput(ParameterMemrefInput(buf)),
                Raw::MemrefOutput(buf, size) => Self::MemrefOutput(ParameterMemrefOutput(buf, size)),
                Raw::MemrefInout(buf, size) => Self::MemrefInout(ParameterMemrefInout(buf, size)),
            }
        }

        pub fn as_value_input(&self) -> Result<&ParameterValueInput> {
            match self {
                Self::ValueInput(p) => Ok(p),
                _ => bad(),
            }
        }
        pub fn as_value_output(&mut self) -> Result<&mut ParameterValueOutput<'a>> {
            match self {
                Self::ValueOutput(p) => Ok(p),
                _ => bad(),
            }
        }
        pub fn as_value_inout(&mut self) -> Result<&mut ParameterValueInout<'a>> {
            match self {
                Self::ValueInout(p) => Ok(p),
                _ => bad(),
            }
        }
        pub fn as_memref_input(&self) -> Result<&ParameterMemrefInput<'a>> {
            match self {
                Self::MemrefInput(p) => Ok(p),
                _ => bad(),
            }
        }
        pub fn as_memref_inout(&mut self) -> Result<&mut ParameterMemrefInout<'a>> {
            match self {
                Self::MemrefInout(p) => Ok(p),
                _ => bad(),
            }
        }
        pub fn as_memref_output(&mut self) -> Result<&mut ParameterMemrefOutput<'a>> {
            match self {
                Self::MemrefOutput(p) => Ok(p),
                _ => bad(),
            }
        }
        pub fn as_none(&self) -> Result<()> {
            match self {
                Self::None => Ok(()),
                _ => bad(),
            }
        }
    }

    pub type ParametersAny<'a> = (ParameterAny<'a>, ParameterAny<'a>, ParameterAny<'a>, ParameterAny<'a>);

    /// Builds the entry point's view of four test-owned slots.
    pub fn params(raw: &mut [Raw; 4]) -> ParametersAny<'_> {
        let [p0, p1, p2, p3] = raw;
        (
            ParameterAny::from_raw(p0),
            ParameterAny::from_raw(p1),
            ParameterAny::from_raw(p2),
            ParameterAny::from_raw(p3),
        )
    }
}

pub use optee_utee_mock::*;
