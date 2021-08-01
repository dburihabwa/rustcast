/*
Functions and utilities to operate a recast instance
*/

fn split(data: &[u8], chunk_size: usize) -> Vec<&[u8]>  {
    let mut chunks = Vec::new();
    for chunk in data.chunks(chunk_size) {
        chunks.push(chunk);
    }
    return chunks;
}

fn merge(chunks: Vec<&[u8]>) -> Vec<u8> {
    return chunks.as_slice().concat();
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
}