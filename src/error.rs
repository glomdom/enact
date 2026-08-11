use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnactError {
    #[error("offset arithmetic overflowed")]
    Overflow,

    #[error("expected magic 'XNB'")]
    BadMagic,

    #[error("unknown target platform {byte:X}")]
    BadTargetPlatform { byte: u8 },

    #[error("unknown XNB format version {version}")]
    BadFormatVersion { version: u8 },

    #[error("unknown XNB flag bits")]
    BadFlagBits,

    #[error("unaligned bit reader after moving")]
    Unaligned { at: usize },

    #[error("invalid LZX window size {size}")]
    InvalidWindowSize { size: u8 },

    #[error("invalid LZX block type {t}")]
    BadLZXBlockType { t: u32 },

    #[error("LZX stream is corrupted")]
    CorruptedStream,

    #[error("no tree")]
    NoTree,

    #[error("decoder not implemented")]
    NotImplemented,

    #[error("unexpected eof at {at}, wanted {want} bytes")]
    Eof { at: usize, want: usize },
}
