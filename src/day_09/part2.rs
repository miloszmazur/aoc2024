use std::borrow::BorrowMut;

use anyhow::Result;

#[derive(Clone, Copy)]
enum BlockType {
    File { file_id: usize, file_size: usize },
    Free(usize),
}

struct Disk {
    content: Vec<BlockType>,
}

impl Disk {
    fn to_defrag_string(&self) -> String {
        self.content
            .iter()
            .flat_map(|elem| match elem {
                BlockType::File { file_id, file_size } => {
                    std::iter::repeat(file_id.to_string()).take(*file_size)
                }
                BlockType::Free(free_size) => std::iter::repeat(".".to_string()).take(*free_size),
            })
            .collect()
    }
    fn defrag2(&mut self) {
        let mut last_not_moved_id: usize = 99999999;
        // let mut last_checked_id;
        let mut rev_iter = self.content.clone().into_iter().rev().filter(|asd| {
            if let BlockType::File { .. } = asd {
                true
            } else {
                false
            }
        });
        while let Some(BlockType::File { file_id, file_size }) = rev_iter.next() {
            let find_free_space = self
                .content
                .iter()
                .enumerate()
                .take_while(|(_, block)| match block {
                    BlockType::File {
                        file_id: file_id2,
                        file_size: _,
                    } => *file_id2 != last_not_moved_id && *file_id2 != file_id,
                    BlockType::Free(_) => true,
                })
                .find(|(_, block)| {
                    if let BlockType::Free(size) = block {
                        *size >= file_size
                    } else {
                        false
                    }
                });

            if let Some((index, BlockType::Free(size))) = find_free_space {
                let new_size = size - file_size;
                self.content[index] = BlockType::Free(new_size);
                let current_position = self.content.iter().position(|value| match value {
                    BlockType::File {
                        file_id: id2,
                        file_size: _,
                    } => *id2 == file_id,
                    BlockType::Free(_) => false,
                });
                if let Some(current_index) = current_position {
                    let got = std::mem::replace(
                        self.content[current_index].borrow_mut(),
                        BlockType::Free(file_size),
                    );
                    self.content.insert(index, got);
                }
            } else {
                last_not_moved_id = file_id;
            }
        }
    }
}

fn parse(input: &str) -> Result<Disk> {
    let disk_content = input
        .chars()
        .enumerate()
        .map(|(index, charmander)| {
            let block_size = charmander
                .to_digit(10)
                .expect(&format!("Failed to parse char: {}", charmander));
            if index % 2 == 0 {
                BlockType::File {
                    file_id: index / 2,
                    file_size: block_size as usize,
                }
            } else {
                BlockType::Free(block_size as usize)
            }
        })
        .collect();
    Ok(Disk {
        content: disk_content,
    })
}

pub fn main(input: &str) -> Result<usize> {
    let mut fragged_disk = parse(input)?;
    let before_defrag_string = fragged_disk.to_defrag_string();
    dbg!(&before_defrag_string);
    fragged_disk.defrag2();

    let after_defrag_string = fragged_disk.to_defrag_string();
    dbg!(&after_defrag_string);

    Ok(fragged_disk
        .content
        .iter()
        .flat_map(|block| match block {
            BlockType::File { file_id, file_size } => std::iter::repeat(*file_id).take(*file_size),
            BlockType::Free(size) => std::iter::repeat(0_usize).take(*size),
        })
        .enumerate()
        .map(|(index, value)| index * value)
        .sum())
}

pub fn get_max_file_size(input: &Vec<i64>) -> i64 {
    input
        .iter()
        .enumerate()
        .filter_map(|(index, elem)| if index % 2 == 0 { Some(elem) } else { None })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p2() {
        let input = "2333133121414131402";
        let result = main(input);
        assert_eq!(result.unwrap(), 2858);
    }
    #[test]
    fn test_12345() {
        let input = "54321";
        let result = main(input);
        assert_eq!(result.unwrap(), 31);
    }
}
