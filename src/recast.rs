/*
Functions and utilities to operate a recast instance
*/

pub struct Block {
    data: Vec<u8>,
    index: usize
}

pub struct Encoded {
    blocks: Vec<Block>
}

pub fn split(data: &[u8], chunk_size: usize) -> Vec<&[u8]>  {
    let mut chunks = Vec::new();
    for chunk in data.chunks(chunk_size) {
        chunks.push(chunk);
    }
    return chunks;
}

pub fn merge(chunks: Vec<&[u8]>) -> Vec<u8> {
    return chunks.as_slice().concat();
}

//pub fn encode(data: &[u8], k:usize, m: usize) -> &Vec<&[u8]> {
pub fn encode(data: &[u8], k:usize, m: usize) -> Encoded {
    let n = k + m;
    let mut encoded = Vec::with_capacity(n);
    let chunk_length = data.len() / k;
    for i in 0..k {
        let block_start = i * chunk_length;
        let block_end = block_start + chunk_length;
        let block_data = data[block_start..block_end].to_vec();
        let block = Block { data: block_data , index: i };
        encoded.push(block);
    }
    for i in k..n {
        let block = Block { data: data[0..chunk_length].to_vec(), index: i };
        encoded.push(block);
    }
    Encoded {blocks: encoded}
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_split_single() {
        let data: [u8; 1] = [0; 1];
        let blocks = split(&data, 1);
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].len(), 1);
    }

    #[test]
    fn test_split_overflow() {
        let data: [u8; 2] = [0; 2];
        let blocks = split(&data, 1);
        assert_eq!(blocks.len(), 2);
        assert_eq!(blocks[0].len(), 1);
        assert_eq!(blocks[1].len(), 1);
    }

    #[test]
    fn test_split_uneven() {
        let data: [u8; 3] = [0; 3];
        let blocks = split(&data, 2);
        assert_eq!(blocks.len(), 2);
        assert_eq!(blocks[0].len(), 2);
        assert_eq!(blocks[1].len(), 1);
    }

    #[test]
    fn test_merge_empty() {
        let chunks: Vec<&[u8]> = Vec::new();
        let merged = merge(chunks);
        assert_eq!(merged.len(), 0);
    }

    #[test]
    fn test_merge_one() {
        let mut chunks: Vec<&[u8]> = Vec::new();
        chunks.push(&[0]);
        let merged = merge(chunks);
        assert_eq!(merged.len(), 1);
    }

    #[test]
    fn test_merge_more() {
        let mut chunks: Vec<&[u8]> = Vec::new();
        chunks.push(&[0]);
        chunks.push(&[0]);
        let merged = merge(chunks);
        assert_eq!(merged.len(), 2);
    }

    #[test]
    fn test_encode() {
        let data: [u8; 10] = [0; 10];
        let k = 10;
        let m = 5;
        let encoded = encode(&data, k, m);
        assert_eq!(encoded.blocks.len(), 15);
        for (index, block) in encoded.blocks.iter().enumerate() {
            println!("{}", index);
            assert_eq!(block.index, index);
            assert_eq!(block.data.len(), 1);
        }
    }
}