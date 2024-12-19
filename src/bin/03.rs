advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<i64> {
    use regex::Regex;
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    Some(re.captures_iter(input).map(|c| {
        let (_, [a, b]) = c.extract();
        let [a,b] = [a,b].map(|n| n.parse::<i64>().unwrap());
        a*b
    }).sum())
}

pub fn part_two(mut input: &str) -> Option<i64> {
    let mut sum = 0;
    let mut enabled = true;
    while !input.is_empty() {
        if input.starts_with("do()") {
            input = &input["do()".len()..];
            enabled = true;
        } else
        if input.starts_with("don't()") {
            input = &input["don't()".len()..];
            enabled = false;
        } else
        if enabled && input.starts_with("mul(") {
            input = &input["mul(".len()..];
            let a = {let n=input.chars().take_while(|c| c.is_digit(10)).count(); let digits; (digits, input) = input.split_at(n); digits};
            if a.is_empty() { continue; }
            if !input.starts_with(",") { continue; }
            input = &input[",".len()..];
            let b = {let n=input.chars().take_while(|c| c.is_digit(10)).count(); let digits; (digits, input) = input.split_at(n); digits};
            if !input.starts_with(")") { continue; }
            input = &input[")".len()..];
            sum += a.parse::<i64>().unwrap() * b.parse::<i64>().unwrap();
        } else {
            input = &input[1..];
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(48));
    }
}
