use std::{collections::BTreeMap, fmt::Display};

use itertools::Itertools;

type FileId = u64;
type FileSize = u64;
type FileLoc = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct FSFile {
    id: Option<FileId>,
    size: FileSize,
    start: FileLoc,
}

struct FS {
    blocks: Vec<FSFile>,
    free_block_indexes: Vec<FileLoc>,
    free_blocks: BTreeMap<FileLoc, FileSize>,
}

impl FS {
    fn new(input: String) -> Self {
        let mut next_id = 0;
        let mut empty = false;
        let mut blocks = Vec::new();

        let mut free_blocks = BTreeMap::new();
        let mut free_block_indexes = Vec::new();
        input.bytes().for_each(|byte| {
            let id = (!empty).then_some(next_id);
            let next_loc = blocks.len();
            let size = (byte - b'0') as FileSize;
            (0..size).for_each(|i| {
                blocks.push(FSFile {
                    id,
                    size,
                    start: next_loc,
                });
                if id.is_none() {
                    free_block_indexes.push(next_loc + i as FileLoc);
                }
            });
            if !empty {
                next_id += 1;
            } else {
                free_blocks.insert(next_loc, size);
            }
            empty = !empty;
        });

        Self {
            blocks,
            free_blocks,
            free_block_indexes,
        }
    }

    fn checksum(&self) -> u64 {
        self.blocks
            .iter()
            .enumerate()
            .map(|(i, f)| f.id.unwrap_or_default() * (i as u64))
            .sum()
    }
}

impl Display for FS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for file in self.blocks.iter() {
            f.write_str(&file.id.map(|id| id.to_string()).unwrap_or(".".to_string()))?;
        }
        Ok(())
    }
}

pub fn part1(input: String) -> u64 {
    let mut fs = FS::new(input);
    let mut swaps = Vec::new();
    fs.blocks
        .iter()
        .enumerate()
        .filter_map(|(loc, f)| f.id.and(Some(loc)))
        .rev()
        .for_each(|file_loc| {
            if let Some((free_idx, free_loc)) = fs
                .free_block_indexes
                .iter()
                .enumerate()
                .find_map(|(idx, &loc)| (loc < file_loc).then_some((idx, loc)))
            {
                fs.free_block_indexes.remove(free_idx);
                swaps.push((file_loc, free_loc));
            }
        });
    for (a, b) in swaps {
        fs.blocks.swap(a, b);
    }
    fs.checksum()
}

pub fn part2(input: String) -> u64 {
    let mut fs = FS::new(input);
    let mut swaps = Vec::new();
    fs.blocks
        .iter()
        .unique()
        .filter(|b| b.id.is_some())
        .rev()
        .for_each(|file| {
            if let Some((&free_loc, _)) = fs
                .free_blocks
                .iter()
                .find(|&(&free_loc, &size)| free_loc < file.start && size >= file.size)
            {
                (0..file.size)
                    .for_each(|i| swaps.push((file.start + i as usize, free_loc + i as usize)));
                let new_free_size = fs.free_blocks.remove(&free_loc).unwrap() - file.size;
                if new_free_size > 0 {
                    fs.free_blocks
                        .insert(free_loc + file.size as usize, new_free_size);
                }
            }
        });
    for (a, b) in swaps {
        fs.blocks.swap(a, b);
    }
    fs.checksum()
}
