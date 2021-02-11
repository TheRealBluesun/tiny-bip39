pub mod english;
pub use english::ENG_WORDSLIST;
#[cfg(feature = "chinese-simplified")]
pub mod chinese_simplified;
#[cfg(feature = "chinese-simplified")]
pub use chinese_simplified::ZH_WORDSLIST;
