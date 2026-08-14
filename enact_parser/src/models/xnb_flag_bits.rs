use bitflags::bitflags;

bitflags! {
    #[derive(Debug)]
    pub struct XNBFlagBits: u8 {
        const HIDEF = 1 << 0;
        const COMPRESSED = 1 << 7;
    }
}
