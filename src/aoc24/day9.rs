use std::{
    collections::HashMap,
    fmt::{Display, Write},
};

use itertools::Itertools;

type FileId = u64;
type FileSize = u64;
type FileLoc = usize;

#[derive(Debug, Clone)]
struct FSFile {
    id: Option<FileId>,
    size: FileSize,
    start: FileLoc,
}

struct FS {
    blocks: Vec<FSFile>,
    free_blocks: Vec<FileLoc>,
    max_id: FileId,
}

impl FS {
    fn new(input: String) -> Self {
        let mut next_id = 0;
        let mut empty = false;
        let mut blocks = Vec::new();

        let mut free_blocks = Vec::new();
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
                    free_blocks.push(next_loc + i as FileLoc);
                }
            });
            if !empty {
                next_id += 1;
            }
            empty = !empty;
        });

        Self {
            blocks,
            free_blocks,
            max_id: next_id,
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
    loop {
        if let Some(file_loc) = fs.blocks.iter().rposition(|f| f.id.is_some()) {
            if let Some(free_idx) = fs.free_blocks.iter().position(|&loc| loc < file_loc) {
                let free_loc = fs.free_blocks.remove(free_idx);
                fs.blocks.swap(file_loc, free_loc);
            } else {
                break;
            }
        } else {
            break;
        }
    }
    fs.checksum()
}

// THIS GARBAGE DOESN'T WORK

pub fn part2(input: String) -> u64 {
    let mut fs = FS::new(input);

    let mut res = 0;

    for id in (0..fs.max_id).rev() {
        let file_loc = fs
            .blocks
            .iter()
            .enumerate()
            .position(|(l, f)| (f.start == l && f.id.is_some_and(|f_id| f_id == id)))
            .unwrap();
        let file = fs.blocks.remove(file_loc);
        if let Some(free_idx) = fs
            .blocks
            .iter()
            .position(|f| f.id.is_none() && f.start < file_loc && f.size >= file.size)
        {
            let free_file = fs.blocks.remove(free_idx);
            res += file.id.unwrap() * (free_file.start as u64);
            for i in 1..file.size {
                res += file.id.unwrap() * (free_file.start as u64 + i);
            }
        }
    }

    res
}
