use std::{
    collections::BTreeMap,
    io::{stdin, BufRead},
    ops::Bound,
};

#[derive(Clone, Copy, Debug)]
struct FileId(u64);

#[derive(Clone, Copy, Debug)]
enum Chunk {
    Free(u64),
    Used(u64, FileId),
}
#[derive(Debug)]
struct Memory(BTreeMap<u64, Chunk>);

impl From<Vec<u64>> for Memory {
    fn from(input: Vec<u64>) -> Memory {
        Memory(
            input
                .into_iter()
                .enumerate()
                .map(|(idx, size)| match idx % 2 {
                    0 => Chunk::Used(size, FileId(idx as u64 / 2)),
                    1 => Chunk::Free(size),
                    _ => panic!(),
                })
                .fold(BTreeMap::new(), |mut addressed_chunks, chunk| {
                    let addr = addressed_chunks
                        .last_key_value()
                        .map(|(last_addr, last_chunk)| {
                            last_addr
                                + match last_chunk {
                                    Chunk::Free(size) => size,
                                    Chunk::Used(size, _) => size,
                                }
                        })
                        .unwrap_or(0);
                    addressed_chunks.insert(addr, chunk);
                    addressed_chunks
                }),
        )
    }
}

impl Memory {
    fn alloc(&mut self, addr: u64, size: u64, file: FileId) -> u64 {
        if let Some(&Chunk::Free(chunk_size)) = self.0.get(&addr) {
            let size = size.min(chunk_size);
            self.0.insert(addr, Chunk::Used(size, file));
            let remaining = chunk_size - size;
            if remaining != 0 {
                self.0.insert(addr + size, Chunk::Free(remaining));
            }
            return size;
        }
        0
    }

    fn free(&mut self, addr: u64, size: u64) {
        if let Some(&Chunk::Used(used_size, file)) = self.0.get(&addr) {
            if size < used_size {
                self.0.insert(addr, Chunk::Used(used_size - size, file));
                match self.0.get(&(addr + used_size)) {
                    Some(Chunk::Free(next_size)) => {
                        self.0
                            .insert(addr + used_size - size, Chunk::Free(size + next_size));
                        self.0.remove(&(addr + used_size));
                    }
                    _ => {
                        self.0.insert(addr + used_size - size, Chunk::Free(size));
                    }
                };
            } else {
                let prev_chunk = self
                    .0
                    .range((Bound::Unbounded, Bound::Excluded(addr)))
                    .next_back();
                let next_chunk = self
                    .0
                    .range((Bound::Excluded(addr), Bound::Unbounded))
                    .next();
                match (prev_chunk, next_chunk) {
                    (
                        Some((&prev_addr, &Chunk::Free(prev_size))),
                        Some((&next_addr, &Chunk::Free(next_size))),
                    ) => {
                        self.0.remove(&prev_addr);
                        self.0.remove(&addr);
                        self.0.remove(&next_addr);
                        self.0
                            .insert(prev_addr, Chunk::Free(prev_size + size + next_size));
                    }
                    (_, Some((&next_addr, &Chunk::Free(next_size)))) => {
                        self.0.remove(&addr);
                        self.0.remove(&next_addr);
                        self.0.insert(addr, Chunk::Free(size + next_size));
                    }
                    (Some((&prev_addr, &Chunk::Free(prev_size))), _) => {
                        self.0.remove(&prev_addr);
                        self.0.remove(&addr);
                        self.0.insert(prev_addr, Chunk::Free(prev_size + size));
                    }
                    _ => {
                        self.0.remove(&addr);
                        self.0.insert(addr, Chunk::Free(size));
                    }
                }
            }
        }
    }

    fn checksum(&self) -> u64 {
        self.0
            .iter()
            .filter_map(|(&addr, chunk)| {
                if let &Chunk::Used(size, FileId(fid)) = chunk {
                    Some(
                        (addr..(addr + size))
                            .map(|offset| offset * fid)
                            .sum::<u64>(),
                    )
                } else {
                    None
                }
            })
            .sum()
    }
}

fn part1(mut input: Vec<u64>) -> usize {
    let mut l_file = 0;
    let mut r_file = (input.len() + 1) / 2 - 1;
    let mut l_idx = 0;
    let mut r_idx = r_file * 2;
    let mut checksum_idx = 0;
    let mut checksum = 0;
    'main: while l_idx <= r_idx {
        for _ in 0..input[l_idx] {
            checksum += l_file * checksum_idx;
            checksum_idx += 1;
        }
        input[l_idx] = 0;
        l_file += 1;
        l_idx += 1;

        for _ in 0..input[l_idx] {
            while input[r_idx] == 0 {
                r_idx -= 2;
                r_file -= 1;
                if l_idx > r_idx {
                    break 'main;
                }
            }
            if input[r_idx] > 0 {
                checksum += r_file * checksum_idx;
                checksum_idx += 1;
                input[r_idx] -= 1;
            }
        }
        l_idx += 1;
    }
    checksum
}

fn part2(mut memory: Memory) -> u64 {
    let last_file_id = match memory.0.last_key_value() {
        Some((_, &Chunk::Used(_, FileId(fid)))) => fid,
        _ => 0,
    };
    for file_id in (0..=last_file_id).rev() {
        if let Some((&file_addr, &Chunk::Used(file_size, file_id))) = memory
            .0
            .iter()
            .rfind(|(_, &chunk)| matches!(chunk, Chunk::Used(_, FileId(fid)) if fid == file_id))
        {
            if let Some((&free_addr, &Chunk::Free(_))) = memory
                .0
                .iter()
                .filter(
                    |(_, &chunk)| matches!(chunk, Chunk::Free(free_size) if free_size >= file_size),
                )
                .take_while(|(&free_addr, _)| free_addr < file_addr)
                .next()
            {
                memory.alloc(free_addr, file_size, file_id);
                memory.free(file_addr, file_size);
            }
        }
    }
    memory.checksum()
}

fn main() {
    let mut input = String::new();
    let _ = stdin().lock().read_line(&mut input);
    let input = input
        .chars()
        .filter_map(|ch| ch.to_digit(10))
        .map(|n| n as u64)
        .collect::<Vec<_>>();
    println!("Part 1: {}", part1(input.clone()));

    let memory = Memory::from(input);
    println!("Part 2: {}", part2(memory));
}
