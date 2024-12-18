#![feature(array_windows)]
advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<usize> {
    let reports = input.lines().map(|line| Vec::from_iter(line.split(' ').map(|level| level.parse::<i64>().unwrap())));
    Some(reports.filter(|report| {
        let [a, b, ..] = report[..] else {unreachable!()};
        if a < b {
            report.array_windows().all(|[a,b]| a < b && (a-b).abs() <= 3)
        } else {
            report.array_windows().all(|[a,b]| a > b && (a-b).abs() <= 3)
        }
    }).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    let reports = input.lines().map(|line| Vec::from_iter(line.split(' ').map(|level| level.parse::<i64>().unwrap())));
    Some(reports.filter(|report| {
        let [a, b, ..] = report[..] else {unreachable!()};
        if a < b && report.array_windows().all(|[a,b]| a < b && (a-b).abs() <= 3) { true }
        else if a > b && report.array_windows().all(|[a,b]| a > b && (a-b).abs() <= 3) { true }
        else { (0..report.len()).any(|i| {
                let mut report = report.clone();
                report.remove(i);
                let [a, b, ..] = report[..] else {unreachable!()};
                if a < b {
                    report.array_windows().all(|[a,b]| a < b && (a-b).abs() <= 3)
                } else {
                    report.array_windows().all(|[a,b]| a > b && (a-b).abs() <= 3)
                }
            })
        }
    }).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
