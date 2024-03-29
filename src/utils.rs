#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Endian {
    Big,
    #[default]
    Little,
}

// byte sequence to integer
pub trait ValidNums {
    fn from_big_bytes(seq: &[u8]) -> Self;
    fn from_little_bytes(seq: &[u8]) -> Self;
    fn from_bytes(en: Endian, seq: &[u8]) -> Self;
}

impl ValidNums for u16 {
    fn from_big_bytes(seq: &[u8]) -> Self {
        Self::from_be_bytes(seq[..2 as usize].try_into().unwrap())
    }
    fn from_little_bytes(seq: &[u8]) -> Self {
        Self::from_le_bytes(seq[..2 as usize].try_into().unwrap())
    }
    fn from_bytes(en: Endian, seq: &[u8]) -> Self {
        match en {
            Endian::Big => Self::from_big_bytes(seq[..2 as usize].try_into().unwrap()),
            Endian::Little => Self::from_le_bytes(seq[..2 as usize].try_into().unwrap()),
        }
    }
}
impl ValidNums for u32 {
    fn from_big_bytes(seq: &[u8]) -> Self {
        Self::from_be_bytes(seq[..4 as usize].try_into().unwrap())
    }
    fn from_little_bytes(seq: &[u8]) -> Self {
        Self::from_le_bytes(seq[..4 as usize].try_into().unwrap())
    }
    fn from_bytes(en: Endian, seq: &[u8]) -> Self {
        match en {
            Endian::Big => Self::from_big_bytes(seq[..4 as usize].try_into().unwrap()),
            Endian::Little => Self::from_le_bytes(seq[..4 as usize].try_into().unwrap()),
        }
    }
}
impl ValidNums for u64 {
    fn from_big_bytes(seq: &[u8]) -> Self {
        let len = if seq.len() > 8 { 8 as usize } else { seq.len() };
        if seq.len() == 4 {
            return u32::from_be_bytes(seq[..len].try_into().unwrap()) as u64;
        }
        Self::from_be_bytes(seq[..len].try_into().unwrap())
    }
    fn from_little_bytes(seq: &[u8]) -> Self {
        let len = if seq.len() > 8 { 8 as usize } else { seq.len() };
        if len == 4 {
            return u32::from_le_bytes(seq[..len].try_into().unwrap()) as u64;
        }
        Self::from_le_bytes(seq[..len].try_into().unwrap())
    }
    fn from_bytes(en: Endian, seq: &[u8]) -> Self {
        match en {
            Endian::Big => Self::from_big_bytes(seq[..seq.len() as usize].try_into().unwrap()),
            Endian::Little => Self::from_le_bytes(seq[..seq.len() as usize].try_into().unwrap()),
        }
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
