const INPUT: &str = include_str!("../../inputs/day9.txt");

struct File {
    offset: usize,
    size: usize,
    id: usize,
}

struct Gap {
    offset: usize,
    size: usize,
}

fn main() {
    let mut gaps = Vec::new();
    let mut files = Vec::new();
    let mut offset = 0;
    for (i, c) in INPUT.trim().chars().enumerate() {
        let size = c.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            files.push(File {
                offset,
                size,
                id: i / 2,
            });
        } else {
            gaps.push(Gap { offset, size })
        };
        offset += size;
    }

    'next_file: for file in files.iter_mut().rev() {
        for gap in gaps.iter_mut() {
            if gap.offset > file.offset {
                continue 'next_file;
            }
            if gap.size >= file.size {
                file.offset = gap.offset;
                gap.offset += file.size;
                gap.size -= file.size;
                continue 'next_file;
            }
        }
    }

    let checksum: usize = files
        .iter()
        .map(|file| {
            (0..file.size)
                .map(|x| (file.offset + x) * file.id)
                .sum::<usize>()
        })
        .sum();

    println!("{}", checksum);
}
