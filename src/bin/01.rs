advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i64> {
    let (mut a, mut b) : (Vec<_>, Vec<_>) = input.lines().map(|line| {
        let (a,b) = line.split_once(' ').unwrap();
        let [a,b] = [a,b].map(|field| field.trim().parse::<i64>().unwrap());
        (a,b)
    }).unzip();
    a.sort();
    b.sort();
    Some(a.iter().zip(b).map(|(a,b)| (b-a).abs()).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let (a, b) : (Vec<_>, Vec<_>) = input.lines().map(|line| {
        let (a,b) = line.split_once(' ').unwrap();
        let [a,b] = [a,b].map(|field| field.trim().parse::<usize>().unwrap());
        (a,b)
    }).unzip();
    Some(a.iter().map(|&a| a * b.iter().filter(|&&b| a==b).count()).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
