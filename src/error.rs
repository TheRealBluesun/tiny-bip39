use crate::mnemonic_type::MnemonicType;
#[cfg(feature = "std")]
use thiserror::Error;

#[derive(Debug)]
#[cfg_attr(feature = "std", derive(Error))]
pub enum ErrorKind {
    #[cfg_attr(feature = "std", error("invalid checksum"))]
    InvalidChecksum,
    #[cfg_attr(feature = "std", error("invalid word in phrase"))]
    InvalidWord,
    #[cfg_attr(feature = "std", error("invalid keysize: {0}"))]
    InvalidKeysize(usize),
    #[cfg_attr(feature = "std", error("invalid number of words in phrase: {0}"))]
    InvalidWordLength(usize),
    #[cfg_attr(
        feature = "std",
        error("invalid entropy length {0}bits for mnemonic type {1:?}")
    )]
    InvalidEntropyLength(usize, MnemonicType),
}

#[cfg(feature = "std")]
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn prints_correctly() {
        assert_eq!(
            format!("{}", ErrorKind::InvalidChecksum),
            "invalid checksum",
        );
        assert_eq!(
            format!("{}", ErrorKind::InvalidKeysize(42)),
            "invalid keysize: 42",
        );
        assert_eq!(
            format!(
                "{}",
                ErrorKind::InvalidEntropyLength(42, MnemonicType::Words12)
            ),
            "invalid entropy length 42bits for mnemonic type Words12",
        );
    }
}
