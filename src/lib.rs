
//! A repeating-key XOR functions.
//!
//! This functions might be useful to play with
//! [the matasano crypto challenges](http://cryptopals.com).

#[cfg(test)]
#[macro_use(expect)]
extern crate expectest;

/// Returns result of a XOR operation applied to a `source` byte sequence.
///
/// `key` will be an infinitely repeating byte sequence.
pub fn xor(source: &[u8], key: &[u8]) -> Vec<u8> {
    match key.len() {
        0 => source.into(),
        1 => xor_with_byte(source, key[0]),
        _ => {
            let key_iter = InfiniteByteIterator::new(key);
            source.iter().zip(key_iter).map(|(&a, b)| a ^ b).collect()
        },
    }
}

/// Returns result of a XOR operation applied to a `source` byte sequence.
///
/// `byte` will be an infinitely repeating byte sequence.
pub fn xor_with_byte(source: &[u8], byte: u8) -> Vec<u8> {
    source.iter().map(|&a| a ^ byte).collect()
}

struct InfiniteByteIterator<'a> {
    bytes: &'a [u8],
    index: usize
}

impl<'a> InfiniteByteIterator<'a> {
    pub fn new(bytes: &'a [u8]) -> InfiniteByteIterator<'a> {
        InfiniteByteIterator {
            bytes: bytes,
            index: 0
        }
    }
}

impl<'a> Iterator for InfiniteByteIterator<'a> {
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        let byte = self.bytes[self.index];
        self.index = next_index(self.index, self.bytes.len());
        Some(byte)
    }
}

fn next_index(index: usize, count: usize) -> usize {
    if index + 1 < count { index + 1 } else { 0 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use expectest::prelude::*;

    #[test]
    fn xor_valid_result() {
        let source = &[0, 1, 2, 3];
        let result = xor(source, &[34, 52]);
        expect!(result).to(be_equal_to([34, 53, 32, 55]));
    }

    #[test]
    fn xor_valid_result_with_one_byte() {
        let source = &[0, 1, 2, 3];
        let result = xor(source, &[47]);
        expect!(result).to(be_equal_to([47, 46, 45, 44]));
    }

    #[test]
    fn xor_empty_key() {
        let source = &[0, 1, 2, 3];
        let result = xor(source, &[]);
        expect!(result).to(be_equal_to(source));
    }

    #[test]
    fn xor_empty_source() {
        let source = &[];
        let result = xor(source, &[45, 32, 56]);
        expect!(result).to(be_empty());
    }

    #[test]
    fn xor_with_byte_valid_result() {
        let source = &[0, 1, 2, 3];
        let result = xor_with_byte(source, 23);
        expect!(result).to(be_equal_to([23, 22, 21, 20]));
    }
}
