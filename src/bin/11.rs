use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<usize> {
    //let mut stones = input.split(' ').map(|e| e.parse::<i64>().unwrap()).collect::<Vec<>>();
    let mut stones = input.split(' ').map(|e| e.to_owned()).collect::<Vec<String>>();
    for i in 0..25 {
        println!("{i}");
        //for stone in &mut stones { if *stone == 0 { *stone = 1; } }
        /*for stone in &mut stones { if *stone == "0" { *stone = "1".to_owned(); } }
        stones = stones.flat_map(|s| {
            //let s = s.to_string();
            let digits = s.len();
            if digits.is_even() { vec![s[0..digits/2].to_owned(), s[digits/2..].parse().unwrap().to_string()] }
        }*/
        stones = stones.into_iter().flat_map(|stone| {
            //let s = s.to_string();
            if stone == "0" { vec!["1".to_owned()] }
            else {
                let digits = stone.len();
                if digits%2==0 { vec![stone[0..digits/2].to_owned(), stone[digits/2..].parse::<i64>().unwrap().to_string()] }
                else { vec![(stone.parse::<i64>().unwrap()*2024).to_string()] }
            }
        }).collect();
    }
    Some(stones.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    fn eval(cache: &mut HashMap<(usize, u64), usize>, blink: usize, stone: u64) -> usize {
        if blink == 0 { return 1; }
        if let Some(&count) = cache.get(&(blink, stone)) { return count; }
        let count = {
            if stone == 0 { eval(cache, blink-1, 1) }
            else {
                let digits = stone.ilog10()+1;
                if digits%2 == 0 {
                    eval(cache, blink-1, stone/10_u64.pow(digits/2))+eval(cache, blink-1, stone%10_u64.pow(digits/2))
                }
                else { eval(cache, blink-1, stone*2024) }
            }
        };
        cache.insert((blink,stone), count);
        count
    }
    Some(input.split(' ').map(|e| eval(&mut HashMap::new(), 75, e.parse::<u64>().unwrap())).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
