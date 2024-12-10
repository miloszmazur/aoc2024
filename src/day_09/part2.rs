use std::{borrow::BorrowMut, collections::HashSet, iter::repeat};

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
    fn defrag(&mut self) {
        let cursor_b = self.content.len();
        let mut i = 0;
        while i < self.content.len() {
            let mut j = cursor_b - 1;
            while &j > &0 {
                while let BlockType::Free(_) = self.content[j] {
                    j -= 1;
                }
                let next_file_to_place = self.content[j];
                while let Some(BlockType::File { .. }) = self.content.get(i) {
                    i += 1;
                }
                if i >= self.content.len() {
                    break;
                }
                match self.content[i] {
                    BlockType::File { .. } => panic!("co"),
                    BlockType::Free(free_size) => {
                        if let BlockType::File { file_id, file_size } = next_file_to_place {
                            if file_size <= free_size {
                                let diff = free_size - file_size;
                                if diff == 0 {
                                    self.content.swap(i, j);
                                } else {
                                    // swap and add a difference
                                    self.content.swap(i, j);
                                    self.content[j] = BlockType::Free(file_size);
                                    self.content.insert(i + 1, BlockType::Free(diff));
                                }
                            }
                            else {
                                i+=1
                            }
                        }
                    } 
                }
          }
            i += 1;
        }
    }

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
}

impl Disk {
    fn defrag2(&mut self) {
        let mut not_moved = HashSet::<usize>::new();
        let mut last_not_moved_id: usize = 99999999;
        // let mut last_checked_id;
        let mut rev_iter = self.content.clone().into_iter().rev().filter(|asd| {
            if let BlockType::File { file_id, file_size } = asd {
                true
            } else {
                false
            }
        });
        // match self.content.last().unwrap() {
        //     BlockType::File { file_id, file_size } => {
        //         last_checked_id = *file_id;
        //     }
        //     BlockType::Free(_) => panic!(),
        // }
        while let Some(BlockType::File { file_id, file_size }) = rev_iter.next() {
            let find_free_space = self
                .content
                .iter()
                .enumerate()
                .take_while(|(_, block)| match block {
                    BlockType::File { file_id: file_id2, file_size } => *file_id2 != last_not_moved_id && *file_id2 != file_id,
                    BlockType::Free(_) => true,
                })
                .find(|(index, block)| {
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
                        file_size,
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
                // not_moved.insert(file_id);
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

    Ok(fragged_disk.content.iter().flat_map(|block| match block  {
        BlockType::File { file_id, file_size } => std::iter::repeat(*file_id).take(*file_size),
        BlockType::Free(size) => std::iter::repeat(0_usize).take(*size),
    }).enumerate().map(|(index, value)| index * value).sum())
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

    // #[test]
    // fn test_p2() {
    //     let input = "2333133121414131402";
    //     let result = main(input);
    //     assert_eq!(result.unwrap(), 2858);
    // }
    // #[test]
    // fn test_12345() {
    //     let input = "54321";
    //     let result = main(input);
    //     assert_eq!(result.unwrap(), 31);
    // }
    // #[test]
    // fn test_defragging() {
    //     let input = "2333133121414131402";
    //     let result = defrag_full_files(parse(input).unwrap())
    //         .unwrap()
    //         .iter()
    //         .map(|e| e.to_string())
    //         .collect::<Vec<String>>()
    //         .join("");
    //     assert_eq!(result, "0099811188827773336446555566");
    // }
}
