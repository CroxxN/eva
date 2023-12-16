// byte sequence to integer
pub trait ValidNums {
    fn from_big_bytes(seq: &[u8]) -> Self;
    fn from_little_bytes(seq: &[u8]) -> Self;
}

impl ValidNums for u16 {
    fn from_big_bytes(seq: &[u8]) -> Self {
        Self::from_be_bytes(seq[..2 as usize].try_into().unwrap())
    }
    fn from_little_bytes(seq: &[u8]) -> Self {
        Self::from_le_bytes(seq[..2 as usize].try_into().unwrap())
    }
}
impl ValidNums for u32 {
    fn from_big_bytes(seq: &[u8]) -> Self {
        Self::from_be_bytes(seq[..4 as usize].try_into().unwrap())
    }
    fn from_little_bytes(seq: &[u8]) -> Self {
        Self::from_le_bytes(seq[..4 as usize].try_into().unwrap())
    }
}
impl ValidNums for u64 {
    fn from_big_bytes(seq: &[u8]) -> Self {
        Self::from_be_bytes(seq[..8 as usize].try_into().unwrap())
    }
    fn from_little_bytes(seq: &[u8]) -> Self {
        Self::from_le_bytes(seq[..8 as usize].try_into().unwrap())
    }
}

// "extensive" testing

#[cfg(test)]
mod test {
    use crate::utils::ValidNums;
    #[test]
    fn bit16big() {
        let seq = [0x0, 0xFE];
        let big = u16::from_big_bytes(&seq);
        assert!(big == u16::from_be_bytes(seq));
    }
    #[test]
    fn bit16little() {
        let seq = [0x0, 0xFE];
        let little = u16::from_little_bytes(&seq);
        assert!(little == u16::from_le_bytes(seq));
    }
    #[test]
    fn bit16overflowbig() {
        let seq = [0x0, 0xFE, 0x7];
        let big = u16::from_big_bytes(&seq);
        assert!(big == u16::from_be_bytes(seq[..2].try_into().unwrap()));
    }

    #[test]
    fn bit16overflowlittle() {
        let seq = [0x0, 0xFE, 0x7];
        let little = u16::from_little_bytes(&seq);
        assert!(little == u16::from_le_bytes(seq[..2].try_into().unwrap()));
    }
    #[test]
    fn bit32big() {
        let seq = [0x0, 0xFE, 0x7, 0x4];
        let big = u32::from_big_bytes(&seq);
        assert!(big == u32::from_be_bytes(seq[..4].try_into().unwrap()));
    }
    #[test]
    fn bit32little() {
        let seq = [0x0, 0xFE, 0x7, 0x4];
        let big = u32::from_little_bytes(&seq);
        assert!(big == u32::from_le_bytes(seq[..4].try_into().unwrap()));
    }
    #[test]
    fn bit32overflowbig() {
        let seq = [0x0, 0xFE, 0x7, 0x9, 0x9, 0x10];
        let big = u32::from_big_bytes(&seq);
        assert!(big == u32::from_be_bytes(seq[..4].try_into().unwrap()));
    }
    #[test]
    fn bit32overflowlittle() {
        let seq = [0x0, 0xFE, 0x7, 0x9, 0x9, 0x10];
        let big = u32::from_little_bytes(&seq);
        assert!(big == u32::from_le_bytes(seq[..4].try_into().unwrap()));
    }
    #[test]
    fn bit64big() {
        let seq = [0x0, 0xFE, 0x7, 0x4, 0x7, 0x9, 0x9, 0x1];
        let big = u64::from_big_bytes(&seq);
        assert!(big == u64::from_be_bytes(seq[..8].try_into().unwrap()));
    }
    #[test]
    fn bit64little() {
        let seq = [0x0, 0xFE, 0x7, 0x4, 0x7, 0x9, 0x9, 0x1];
        let big = u64::from_little_bytes(&seq);
        assert!(big == u64::from_le_bytes(seq[..8].try_into().unwrap()));
    }
    #[test]
    fn bit64overflowbig() {
        let seq = [0x0, 0xFE, 0x7, 0x4, 0x7, 0x9, 0x9, 0x1, 0x5, 0xA];
        let big = u64::from_big_bytes(&seq);
        assert!(big == u64::from_be_bytes(seq[..8].try_into().unwrap()));
    }
    #[test]
    fn bit64overflowlittle() {
        let seq = [0x0, 0xFE, 0x7, 0x4, 0x7, 0x9, 0x9, 0x1, 0xA, 0xB];
        let big = u64::from_little_bytes(&seq);
        assert!(big == u64::from_le_bytes(seq[..8].try_into().unwrap()));
    }
}
