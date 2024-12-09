advent_of_code::solution!(9);

use itertools::Itertools;

// Each element in the file system is an ID of the file or a space
type FileSystem = Vec<Option<usize>>;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Block {
    File(usize, usize), // ID, Length
    Space(usize),
}

fn input_to_blocks(input: &str) -> Vec<Block> {
    // The input will be one line of pairs of single digits
    // Of each pair, the first digit will be file length
    // The second will be empty length
    // The ID is an autoincrementing number
    let mut blocks = Vec::new();
    for (i, chunk) in input
        .chars()
        // Filter map to convert to digit, ignoring anything that can't be converted
        .filter_map(|c| c.to_digit(10))
        .map(|x| x as usize)
        // Group in pairs
        .chunks(2)
        .into_iter()
        .map(|chunk| chunk.collect::<Vec<usize>>())
        .enumerate()
    {
        let file_length = chunk[0];
        blocks.push(Block::File(i, file_length));
        if chunk.len() > 1 {
            let space_length = chunk[1];
            blocks.push(Block::Space(space_length));
        }
    }
    blocks
}

fn try_move_block(blocks: &mut Vec<Block>, i: usize, j: usize) -> bool {
    // Try to move the block from index i to index j
    // Only if the block at j is a space and is big enough
    // Returns true if the move was successful
    let block = blocks[i];
    let space = blocks[j];
    match (block, space) {
        (Block::File(id, length), Block::Space(space_length)) => {
            if length == space_length {
                blocks[i] = Block::Space(length);
                blocks[j] = Block::File(id, length);
                true
            } else if length < space_length {
                // Replace the original block with a space of the same length
                // Do this first in case we inserted a space in the middle of the blocks
                blocks[i] = Block::Space(length);
                blocks[j] = Block::File(id, length);
                // Insert a new space block after the new file block
                blocks.insert(j + 1, Block::Space(space_length - length));
                true
            } else {
                false
            }
        }
        _ => false,
    }
}

fn try_all_move_block(blocks: &mut Vec<Block>, i: usize) -> bool {
    // Try to move the block at index i to all other spaces
    // Return true as soon as a move is successful
    for j in 0..i {
        if try_move_block(blocks, i, j) {
            return true;
        }
    }
    false
}

fn compress_blocks(blocks: &[Block]) -> Vec<Block> {
    // Compress by trying to move blocks to the left
    // Into any space big enough to fit them
    // Loop backwards through the file blocks
    // Trying to move them to the left
    let mut blocks = blocks.to_vec();
    for i in (0..blocks.len()).rev() {
        // Only move the block if it's a file
        if let Block::File(_, _) = blocks[i] {
            try_all_move_block(&mut blocks, i);
        }
    }

    blocks
}

fn input_to_file_system(input: &str) -> FileSystem {
    // The input will be one line of pairs of single digits
    // Of each pair, the first digit will be file length
    // The second will be empty length
    // The ID is an autoincrementing number
    let mut file_system = Vec::new();
    for (i, chunk) in input
        .chars()
        // Filter map to convert to digit, ignoring anything that can't be converted
        .filter_map(|c| c.to_digit(10))
        .map(|x| x as usize)
        // Group in pairs
        .chunks(2)
        .into_iter()
        .map(|chunk| chunk.collect::<Vec<usize>>())
        .enumerate()
    {
        let file_length = chunk[0];
        for _ in 0..file_length {
            file_system.push(Some(i));
        }
        if chunk.len() > 1 {
            let space_length = chunk[1];
            for _ in 0..space_length {
                file_system.push(None);
            }
        }
    }
    file_system
}

fn checksum_for_blocks(blocks: &[Block]) -> usize {
    let mut checksum = 0;
    let mut i = 0;
    for block in blocks {
        match block {
            Block::File(id, length) => {
                for _ in 0..*length {
                    checksum += i * id;
                    i += 1;
                }
            }
            Block::Space(length) => {
                i += length;
            }
        }
    }
    checksum
}

fn compress_filesystem(file_system: &FileSystem) -> Vec<usize> {
    // Make a copy of the file system reversed with all the Nones removed
    let mut empty_removed = file_system
        .iter()
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect::<Vec<usize>>();
    let num_chunks = empty_removed.len();

    let compressed: Vec<usize> = file_system
        .iter()
        .map(|x| match x {
            Some(id) => *id,
            None => empty_removed.pop().unwrap(),
        })
        .take(num_chunks)
        .collect();
    compressed
}

pub fn part_one(input: &str) -> Option<usize> {
    let file_system = input_to_file_system(input);
    let compressed = compress_filesystem(&file_system);
    let checksum = compressed.iter().enumerate().map(|(i, id)| i * id).sum();
    Some(checksum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let blocks = input_to_blocks(input);
    let compressed = compress_blocks(&blocks);
    let checksum = checksum_for_blocks(&compressed);
    Some(checksum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
    #[test]
    fn test_input_to_file_system() {
        let input = "2111";
        let expected = vec![Some(0), Some(0), None, Some(1), None];
        assert_eq!(input_to_file_system(input), expected);
    }
    #[test]
    fn test_compressed_filesystem() {
        let input = vec![None, None, Some(0), None, Some(1)];
        let expected = vec![1, 0];
        assert_eq!(compress_filesystem(&input), expected);
    }
    #[test]
    fn test_medium() {
        let input = "21212";
        // 00.11.22
        let fs = input_to_file_system(input);
        let compressed = compress_filesystem(&fs);
        let expected = vec![0, 0, 2, 1, 1, 2];
        assert_eq!(compressed, expected);
    }
    #[test]
    fn test_compressed_filesystem_more_complex() {
        let input = "2333133121414131402";
        let fs = input_to_file_system(input);
        let expected_fs = Vec::from([
            Some(0),
            Some(0),
            None,
            None,
            None,
            Some(1),
            Some(1),
            Some(1),
            None,
            None,
            None,
            Some(2),
            None,
            None,
            None,
            Some(3),
            Some(3),
            Some(3),
            None,
            Some(4),
            Some(4),
            None,
            Some(5),
            Some(5),
            Some(5),
            Some(5),
            None,
            Some(6),
            Some(6),
            Some(6),
            Some(6),
            None,
            Some(7),
            Some(7),
            Some(7),
            None,
            Some(8),
            Some(8),
            Some(8),
            Some(8),
            Some(9),
            Some(9),
        ]);
        assert_eq!(fs, expected_fs);
        let compressed = compress_filesystem(&fs);
        let expected = vec![
            0, 0, 9, 9, 8, 1, 1, 1, 8, 8, 8, 2, 7, 7, 7, 3, 3, 3, 6, 4, 4, 6, 5, 5, 5, 5, 6, 6,
        ];
        assert_eq!(compressed, expected);
    }

    #[test]
    fn test_checksum_for_blocks() {
        // 00992111777.44.333....5555.6666.....8888..
        let blocks = vec![
            Block::File(0, 2),
            Block::File(9, 2),
            Block::File(2, 1),
            Block::File(1, 3),
            Block::File(7, 3),
            Block::Space(1),
            Block::File(4, 2),
            Block::Space(1),
            Block::File(3, 3),
            Block::Space(4),
            Block::File(5, 4),
            Block::Space(1),
            Block::File(6, 4),
            Block::Space(5),
            Block::File(8, 4),
            Block::Space(2),
        ];
        assert_eq!(checksum_for_blocks(&blocks), 2858);
    }

    #[test]
    fn test_compress_blocks() {
        // 00.1..2.33.4
        // Should compress to
        // 0041332
        let mut blocks = vec![
            Block::File(0, 2),
            Block::Space(1),
            Block::File(1, 1),
            Block::Space(2),
            Block::File(2, 1),
            Block::Space(1),
            Block::File(3, 2),
            Block::Space(1),
            Block::File(4, 1),
        ];
        // let success = try_all_move_block(&mut blocks, 6);
        // assert_eq!(success, true);
        let compressed = compress_blocks(&blocks);
        assert_eq!(
            compressed,
            vec![
                Block::File(0, 2),
                Block::File(4, 1),
                Block::File(1, 1),
                Block::File(3, 2),
                Block::File(2, 1),
                Block::Space(1),
                Block::Space(2),
                Block::Space(1),
                Block::Space(1),
            ]
        );
    }
}
