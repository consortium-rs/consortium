#[derive(
    Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash, Default,
)]
#[serde(rename_all = "lowercase")]
pub enum Codec {
    /// Use `consortium-codec-postcard` for encoding/decoding messages.
    #[default]
    Postcard,
    /// Use `consortium-codec-prost` for encoding/decoding messages.
    Prost,
    /// Use `consortium-codec-rkyv` for zero-copy encoding/decoding of messages.
    Rkyv,
}
