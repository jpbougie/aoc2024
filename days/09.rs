use std::fmt::Display;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct FreeBlock {
    offset: usize,
    len: u32,
}

impl Ord for FreeBlock {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.offset.cmp(&other.offset)
    }
}

impl PartialOrd for FreeBlock {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Block {
    fileid: usize,
    offset: usize,
    len: u32,
}

impl Block {
    fn checksum(&self) -> usize {
        let mut s = 0;
        for i in 0..self.len {
            s += self.fileid * (self.offset + i as usize);
        }
        s
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.fileid > 9 {
            panic!("Can only display up to file id 10");
        }

        for _i in 0..self.len {
            write!(f, "{}", self.fileid)?;
        }

        Ok(())
    }
}

impl Ord for Block {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.offset.cmp(&other.offset)
    }
}

impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Default, Clone)]
struct AllocMap {
    block_list: Vec<Block>,
    free_list: Vec<FreeBlock>,
}

fn parse_input() -> AllocMap {
    let input = include_str!("../inputs/09.txt");
    let mut map = AllocMap::default();
    let mut offset = 0;
    for (i, ch) in input.chars().enumerate() {
        if ch == '0' {
            continue;
        }

        if i % 2 == 0 {
            map.block_list.push(Block {
                offset,
                fileid: i / 2,
                len: ch.to_digit(10).unwrap(),
            });
        } else {
            map.free_list.push(FreeBlock {
                offset,
                len: ch.to_digit(10).unwrap(),
            });
        }

        offset += ch.to_digit(10).unwrap() as usize;
    }

    map
}

fn part1(mut allocs: AllocMap) -> usize {
    let mut new_blocks = Vec::new();

    // Keep it reverse-sorted as we can then just pop them off
    allocs.free_list.reverse();

    while let Some(free) = allocs.free_list.pop() {
        let Some(block_to_realloc) = allocs.block_list.pop() else {
            break;
        };

        if block_to_realloc.offset <= free.offset {
            allocs.block_list.push(block_to_realloc);
            break;
        }

        match free.len.cmp(&block_to_realloc.len) {
            std::cmp::Ordering::Equal => {
                new_blocks.push(Block {
                    fileid: block_to_realloc.fileid,
                    len: free.len,
                    offset: free.offset,
                });
            }
            std::cmp::Ordering::Less => {
                new_blocks.push(Block {
                    fileid: block_to_realloc.fileid,
                    offset: free.offset,
                    len: free.len,
                });
                allocs.block_list.push(Block {
                    fileid: block_to_realloc.fileid,
                    offset: block_to_realloc.offset,
                    len: block_to_realloc.len - free.len,
                });
            }
            std::cmp::Ordering::Greater => {
                new_blocks.push(Block {
                    fileid: block_to_realloc.fileid,
                    offset: free.offset,
                    len: block_to_realloc.len,
                });
                allocs.free_list.push(FreeBlock {
                    offset: free.offset + block_to_realloc.len as usize,
                    len: free.len - block_to_realloc.len,
                })
            }
        }
    }

    allocs.block_list.append(&mut new_blocks);
    allocs.block_list.sort(); // For debugging purposes

    allocs.block_list.iter().fold(0, |s, a| s + a.checksum())
}

fn part2(mut input: AllocMap) -> usize {
    for block in input.block_list.iter_mut().rev() {
        if input.free_list.is_empty() {
            break;
        }

        let Some(free_index) = input
            .free_list
            .iter()
            .position(|free| free.len >= block.len && free.offset < block.offset)
        else {
            continue;
        };

        let mut free = input.free_list.remove(free_index);

        block.offset = free.offset;

        if free.len > block.len {
            free.offset += block.len as usize;
            free.len -= block.len;
            input.free_list.push(free);
            input.free_list.sort();
        }

        // TODO: CHECK IF WE HAVE contiguous spots
    }

    input.block_list.iter().fold(0, |s, a| s + a.checksum())
}

pub fn main() {
    let input = parse_input();
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input.clone()));
}
