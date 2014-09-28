
pub fn xor(source: &[u8], key: &[u8]) -> Vec<u8> {
    let key_iter = InfiniteByteIterator::new(key);
    source.iter().zip(key_iter).map(|(&a, b)| a ^ b).collect()
}

struct InfiniteByteIterator {
    bytes: Vec<u8>,
    index: uint
}

impl InfiniteByteIterator {
    pub fn new(bytes: &[u8]) -> InfiniteByteIterator {
        InfiniteByteIterator {
            bytes: bytes.to_vec(),
            index: 0
        }
    }
}

impl Iterator<u8> for InfiniteByteIterator {
    fn next(&mut self) -> Option<u8> {
        let byte = self.bytes[self.index];
        self.index = next_index(self.index, self.bytes.len());
        Some(byte)
    }
}

fn next_index(current: uint, count: uint) -> uint {
    let index = current + 1;
    if index < count {
        index
    } else {
        0
    }
}

#[cfg(test)]
mod test {

    use super::xor;

    #[test]
    fn test_xor_len() {
        let source = [0u8, 1, 2, 3];
        let key = [34u8, 52];
        assert!(xor(source, key).len() == source.len());
    }

    #[test]
    fn test_xor_result() {
        let source = [0u8, 1, 2, 3];
        let key = [34u8, 52];
        assert!(xor(source, key) == vec![34, 53, 32, 55]);
    }
}
