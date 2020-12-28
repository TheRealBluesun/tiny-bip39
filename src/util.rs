use heapless::{ArrayLength, Vec};
// use unicode_normalization::Decompositions;

// #[cfg(feature = "std")]
pub(crate) trait IterExt: Iterator {
    // fn join<R>(&mut self, glue: &str) -> R
    // where
    //     R: From<&'static str> + Default,
    //     Self::Item: Joinable,
    // {
    //     let first = match self.next() {
    //         Some(first) => first,
    //         None => return R::default().into(),
    //     };

    //     let (lower, _) = self.size_hint();

    //     let mut buffer = String::with_capacity(lower * (10 + glue.len()));

    //     first.write_into(&mut buffer);

    //     for item in self {
    //         buffer.push_str(glue);
    //         item.write_into(&mut buffer);
    //     }

    //     buffer.into()
    // }

    fn bits<Out>(self) -> BitIter<Self::Item, Out, Self>
    where
        Out: Bits,
        Self::Item: Bits,
        Self: Sized,
    {
        BitIter::new(self)
    }
}

impl<I: Iterator> IterExt for I {}

pub(crate) trait Bits {
    const SIZE: usize;

    fn bits(self) -> u32;
}

impl Bits for u8 {
    const SIZE: usize = 8;

    fn bits(self) -> u32 {
        self as u32
    }
}

impl<'a> Bits for &'a u8 {
    const SIZE: usize = 8;

    fn bits(self) -> u32 {
        *self as u32
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Bits11(u16);

impl Bits for Bits11 {
    const SIZE: usize = 11;

    fn bits(self) -> u32 {
        self.0 as u32
    }
}

impl From<u16> for Bits11 {
    fn from(val: u16) -> Self {
        Bits11(val)
    }
}

impl From<Bits11> for u16 {
    fn from(val: Bits11) -> Self {
        val.0
    }
}

pub(crate) struct BitWriter<B: ArrayLength<u8>> {
    offset: usize,
    remainder: u32,
    inner: Vec<u8, B>,
}

impl<B> BitWriter<B>
where
    B: ArrayLength<u8>,
{
    pub fn new() -> Self {
        // let mut bytes = capacity / 8;

        // if capacity % 8 != 0 {
        //     bytes += 1;
        // }

        Self {
            offset: 0,
            remainder: 0,
            inner: Vec::<u8, B>::new(),
        }
    }

    pub fn push<B2: Bits>(&mut self, source: B2) {
        let shift = 32 - B2::SIZE;

        self.remainder |= (source.bits() << shift) >> self.offset;
        self.offset += B2::SIZE;

        while self.offset >= 8 {
            // TODO: something on Err?
            let _ = self.inner.push((self.remainder >> 24) as u8);
            self.remainder <<= 8;
            self.offset -= 8;
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len() * 8 + self.offset
    }

    pub fn into_bytes(mut self) -> Vec<u8, B> {
        if self.offset != 0 {
            // TODO: something on Err?
            let _ = self.inner.push((self.remainder >> 24) as u8);
        }

        self.inner
    }
}

pub(crate) struct BitIter<In: Bits, Out: Bits, I: Iterator<Item = In> + Sized> {
    _phantom: core::marker::PhantomData<Out>,
    source: I,
    read: usize,
    buffer: u64,
}

impl<In, Out, I> BitIter<In, Out, I>
where
    In: Bits,
    Out: Bits,
    I: Iterator<Item = In>,
{
    fn new(source: I) -> Self {
        let source = source.into_iter();

        BitIter {
            _phantom: core::marker::PhantomData,
            source,
            read: 0,
            buffer: 0,
        }
    }
}

impl<In, Out, I> Iterator for BitIter<In, Out, I>
where
    In: Bits,
    Out: Bits + From<u16>,
    I: Iterator<Item = In>,
{
    type Item = Out;

    fn next(&mut self) -> Option<Out> {
        while self.read < Out::SIZE {
            let bits = self.source.next()?.bits() as u64;

            self.read += In::SIZE;
            self.buffer |= bits << (64 - self.read);
        }

        let result = (self.buffer >> (64 - Out::SIZE)) as u16;

        self.buffer <<= Out::SIZE;
        self.read -= Out::SIZE;

        Some(result.into())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.source.size_hint();

        (
            (lower * In::SIZE) / Out::SIZE,
            upper.map(|n| (n * In::SIZE) / Out::SIZE),
        )
    }
}

/// Extract the first `bits` from the `source` byte
pub(crate) fn checksum(source: u8, bits: u8) -> u8 {
    debug_assert!(bits <= 8, "Can operate on 8-bit integers only");

    source >> (8 - bits)
}
