#![feature(iter_array_chunks)]
advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<usize> {
    let input = {let mut input = input.to_owned(); if input.len()%2==1 { input.push('0'); } input};
    const FREE_BLOCK : usize = usize::MAX;
    let mut disk = input.chars().array_chunks().enumerate().map(|(id, [file, free])| std::iter::repeat_n(id, file.to_digit(10).expect("len") as usize).chain(std::iter::repeat_n(FREE_BLOCK,free.to_digit(10).expect("free") as usize))).flatten().collect::<Vec<_>>();
    //println!("{}", disk.iter().map(|&id| if id == FREE_BLOCK { '.' } else { char::from_digit(id as u32,36).unwrap() }).collect::<String>());
    let mut first_free = 0;
    loop {
        loop {
            if disk[first_free] == FREE_BLOCK { break; }
            first_free += 1;
            if first_free == disk.len() { 
                return Some(disk.iter().enumerate().map(|(i,id)| i*id).sum()); 
            }
        }
        disk[first_free] = disk.pop().unwrap();
        while *disk.last().unwrap() == FREE_BLOCK { disk.pop(); }
        //println!("{}", disk.iter().map(|&id| if id == FREE_BLOCK { '.' } else { char::from_digit(id as u32,36).unwrap() }).collect::<String>());
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = {let mut input = input.to_owned(); if input.len()%2==1 { input.push('0'); } input};
    const FREE_BLOCK : usize = usize::MAX;
    let map = input.chars().array_chunks().map(|[file, free]| [file, free].map(|digit| digit.to_digit(10).unwrap() as usize)).collect::<Vec<_>>();
    let file_offsets = map.iter().scan(0, |offset, [file, free]| { let file_offset = *offset; *offset += file+free; Some(file_offset) }).collect::<Vec<_>>();
    let mut disk = map.iter().enumerate().map(|(id, &[file, free])| std::iter::repeat_n(id, file).chain(std::iter::repeat_n(FREE_BLOCK, free))).flatten().collect::<Vec<_>>();
    //println!("{}", disk.iter().map(|&id| if id == FREE_BLOCK { '.' } else { char::from_digit(id as u32,36).unwrap() }).collect::<String>());
    for (id, &file_offset) in file_offsets.iter().enumerate().rev() {
        println!("{id}");
        let [file_len, _] = map[id];
        for start in 0..=(disk.len()-file_len).min(file_offset) {
            if disk[start..start+file_len].iter().all(|&b| b==FREE_BLOCK) {
                disk[start..start+file_len].fill(id);
                disk[file_offset..file_offset+file_len].fill(FREE_BLOCK);
                //println!("{}", disk.iter().map(|&id| if id == FREE_BLOCK { '.' } else { char::from_digit(id as u32,36).unwrap() }).collect::<String>());
                break;
            }
        }
    }
    Some(disk.iter().enumerate().filter_map(|(i,&id)| (id != FREE_BLOCK).then(|| i*id)).sum())
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
}
