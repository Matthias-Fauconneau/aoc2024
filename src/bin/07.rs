advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<i64> {
    Some(input.lines().filter_map(|line| {
        let (result, operands) = line.split_once(':').unwrap();
        let result = result.parse::<i64>().unwrap();
        let operands = operands.trim().split(' ').map(|o| o.parse::<i64>().unwrap()).collect::<Vec<_>>();
        for mask in 0..(1<<(operands.len()-1)) {
            let mut value = operands[0];
            for (i, operand) in operands[1..].iter().enumerate() {
                if mask&(1<<i) == 0 { value += operand }
                else { value *= operand }
            }
            if value == result { return Some(result) }
        }
        None
    }).sum())
}

pub fn part_two(input: &str) -> Option<i64> {
    Some(input.lines().filter_map(|line| {
        let (result, operands) = line.split_once(':').unwrap();
        let result = result.parse::<i64>().unwrap();
        let operands = operands.trim().split(' ').map(|o| o.parse::<i64>().unwrap()).collect::<Vec<_>>();
        let mut counter = vec![0; operands.len()-1];
        loop {
            let mut value = operands[0];
            for (i, operand) in operands[1..].iter().enumerate() {
                match counter[i] {
                    0 => value += operand,
                    1 => value *= operand,
                    2 => value = (value.to_string()+&operand.to_string()).parse().unwrap(),
                    _ => unreachable!("{counter:?}")
                }
            }
            if value == result { return Some(result) }
            for digit in &mut counter {
                *digit += 1; 
                if *digit == 3 { *digit = 0; continue; /*carry*/} else { break; }
            }
            //println!("{counter:?}");
            if counter == vec![0; operands.len()-1] { break; }
        }
        None
    }).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
