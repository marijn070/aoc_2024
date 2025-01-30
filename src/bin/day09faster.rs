use advent_of_code_2024::file_reader;

fn main() {
    let input = file_reader::get_input("src/inputs/input_day09.txt");

    // parse the input into a vector of options
    let diskmap: Vec<u32> = input.chars().filter_map(|c| c.to_digit(10)).collect();

    let mut disk: Vec<Option<u32>> = vec![];

    let mut id: u32 = 0;

    for (i, block) in diskmap.iter().enumerate() {
        if i % 2 == 0 {
            for _ in 0..*block {
                disk.push(Some(id));
            }
            id += 1;
        } else {
            for _ in 0..*block {
                disk.push(None);
            }
        }
    }

    let mut right_idx: usize = disk.len() - 1;
    let mut left_idx: usize = 0;

    let mut answer_a: u64 = 0;

    while left_idx <= right_idx {
        match disk[left_idx] {
            Some(id) => answer_a += id as u64 * left_idx as u64,
            None => {
                while disk[right_idx].is_none() {
                    right_idx -= 1;
                }
                if let Some(id) = disk[right_idx] {
                    answer_a += id as u64 * left_idx as u64;
                    right_idx -= 1;
                }
            }
        }
        left_idx += 1;
    }

    println!("For answer a i have {answer_a}");

    // for part b, stuff on the right can be moved.
    // We can use the diskmap to find large enough free spaces.
    // each time we get an element on the right, we can slice into the array
    // only considering the stuff that comes before it.
    // So
    // 1. start at the rightmost element that is data (idx % 2 == 0)
    // 2. get the id, which is the index / 2
    // 3. slice throught the array[..idx], and look for a number that is larger or equal and is not data
    // 4. if found, update the checksum, and decreas that number in the diskmap, if not found, update checksum
    // 5. go one element to the left and repeat untill done.

    let mut block_nr = diskmap.len();
    let mut default_block_start_idx = diskmap[..block_nr - 1].iter().sum::<u32>() as usize;
    let mut answer_b: u64 = 0;
    // we use this diskmap to record the shrinking free spaces
    let mut free_space_diskmap: Vec<u32> = diskmap.clone();

    while block_nr > 0 {
        if block_nr % 2 != 0 {
            continue;
        }

        let block_size: u32 = diskmap[block_nr];
        let mut block_start_idx = default_block_start_idx;
        let block_id: u64 = block_nr as u64 / 2;

        if let Some(fs_idx) = free_space_diskmap[..block_nr]
            .iter()
            .enumerate()
            .filter(|(i, &x)| i % 2 != 0 && x >= block_size)
            .map(|(i, _)| i)
            .next()
        {
            block_start_idx = diskmap[..fs_idx].iter().map(|&x| x as usize).sum::<usize>();
            block_start_idx += (diskmap[fs_idx] - free_space_diskmap[fs_idx]) as usize;
            free_space_diskmap[fs_idx] -= block_size;
        }

        // add to the result
        for i in 0..block_size {
            answer_b += (block_start_idx as u64 + i as u64) * block_id;
        }

        block_nr -= 1;
        default_block_start_idx -= block_size as usize;
    }

    println!("The result for part b was {answer_b}");
}
