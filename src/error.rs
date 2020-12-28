use crate::mnemonic_type::MnemonicType;
use heapless::{consts::*, String};

#[derive(Debug)]
pub enum ErrorKind {
    InvalidChecksum,
    InvalidWord(String<U16>),
    InvalidKeysize(usize),
    InvalidWordLength(usize),
    InvalidEntropyLength(usize, MnemonicType),
}
