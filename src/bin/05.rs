advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<i64> {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let rules = rules.lines().map(|rule| rule.split_once('|').unwrap()).collect::<Vec<_>>();
    Some(updates.lines().filter_map(|update| {
        let update = update.split(',').collect::<Vec<_>>();
        rules.iter().all(|(first, then)| {
            if let (Some(first),Some(then)) = (update.iter().position(|e| e==first), update.iter().position(|e| e==then)) {
                first < then
            } else { true }
        }).then_some(update[update.len()/2].parse::<i64>().unwrap())
    }).sum())
}

pub fn part_two(input: &str) -> Option<i64> {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let rules = rules.lines().map(|rule| rule.split_once('|').unwrap()).collect::<Vec<_>>();
    Some(updates.lines().filter_map(|update| {
        let mut update = update.split(',').collect::<Vec<_>>();
        let mut changed = false;
        loop {
            let mut changed_this_time = false;
            for (first, then) in &rules {
                if let (Some(first),Some(then)) = (update.iter().position(|e| e==first), update.iter().position(|e| e==then)) {
                    if first > then {
                        update.swap(first, then);
                        changed = true;
                        changed_this_time = true;
                    }
                }
            }
            if !changed_this_time { break; }
        }
        //if changed { println!("{update:?}"); }
        changed.then_some(update[update.len()/2].parse::<i64>().unwrap())
    }).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
