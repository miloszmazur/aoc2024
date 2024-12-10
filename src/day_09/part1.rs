use anyhow::Result;

enum BlockType {
    File(i64),
    Free,
}

struct BlockSpace {
    disk: Vec<i64>,
}

impl BlockSpace {
    fn iter(&self) -> BlockSpaceIter {
        BlockSpaceIter {
            cursor: 0,
            number_under_cursor: self.disk[0] as i64,
            max_cursor: get_max_file_size(&self.disk),
            disk: self.disk.clone(),
        }
    }

    fn iter_reverse(&self) -> BlockSpaceIter {
        let reverse_disk: Vec<i64> = self.disk.clone().into_iter().rev().collect();
        // let cursor = if reverse_disk.len() % 2 == 0 { 0 } else { 1 };
        BlockSpaceIter {
            cursor: 0,
            number_under_cursor: reverse_disk[0] as i64,
            max_cursor: get_max_file_size(&self.disk),
            disk: reverse_disk,
        }
    }
}

struct BlockSpaceIter {
    cursor: i64,
    max_cursor: i64,
    number_under_cursor: i64,
    disk: Vec<i64>,
}

impl Iterator for BlockSpaceIter {
    type Item = BlockType;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor > self.max_cursor {
            return None;
        }
        if self.number_under_cursor <= 0 {
            self.cursor += 1;
            while self.disk[self.cursor as usize] <= 0 {
                self.cursor += 1;
            }
            self.number_under_cursor = self.disk[self.cursor as usize] as i64;
        }
        self.number_under_cursor -= 1;
        if self.cursor % 2 == 0 {
            let file_id_at_cursor = self.cursor / 2;
            Some(BlockType::File(file_id_at_cursor))
        } else {
            Some(BlockType::Free)
        }
    }
}

fn parse(input: &str) -> Result<Vec<i64>> {
    Ok(input
        .chars()
        .map(|charmander| {
            charmander
                .to_digit(10)
                .map(|f| f as i64)
                .expect(&format!("Failed to parse char: {}", charmander))
        })
        .collect())
}

pub fn main(input: &str) -> Result<i64> {
    let fragged_disk = parse(input);
    let defragged_disk = defrag(fragged_disk?)?;
    Ok(defragged_disk
        .into_iter()
        .enumerate()
        .map(|(idx, elem)| idx as i64 * elem)
        .sum())
}

fn defrag(fragged_disk: Vec<i64>) -> Result<Vec<i64>> {
    let disk = BlockSpace {
        disk: fragged_disk.to_owned(),
    };

    let mut iterator = disk.iter();
    let mut iterator_reverse = disk.iter_reverse();
    let max_space = get_max_file_size(&fragged_disk);
    let mut cursor: i64 = 0;
    let mut defragged_disk = Vec::new();
    let max_id = (fragged_disk.len() / 2) as i64;

    while cursor < max_space {
        if let Some(next_file_block) = iterator.next() {
            match next_file_block {
                BlockType::File(id) => {
                    defragged_disk.push(id);
                }
                BlockType::Free => {
                    while let Some(asd) = iterator_reverse.next() {
                        if let BlockType::File(id) = asd {
                            defragged_disk.push(max_id - id);
                            break;
                        }
                    }
                }
            }

            cursor += 1
        }
    }

    Ok(defragged_disk)
}

pub fn get_max_file_size(input: &Vec<i64>) -> i64 {
    input
        .iter()
        .enumerate()
        .filter_map(|(index, elem)| if index % 2 == 0 { Some(elem) } else { None })
        .sum()
}

pub fn get_max_free_space(input: &Vec<i64>) -> i64 {
    input
        .iter()
        .enumerate()
        .filter_map(|(index, elem)| if index % 2 == 1 { Some(elem) } else { None })
        .sum()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "2333133121414131402";
        let result = main(input);
        assert_eq!(result.unwrap(), 1928);
    }
    #[test]
    fn test_defragging() {
        let input = "2333133121414131402";
        let result = defrag(parse(input).unwrap())
            .unwrap()
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join("");
        assert_eq!(result, "0099811188827773336446555566");
    }
    #[test]
    fn test_very_simple_defragging() {
        let input = "12345";
        let result = defrag(parse(input).unwrap())
            .unwrap()
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join("");
        assert_eq!(result, "022111222");
    }
}
