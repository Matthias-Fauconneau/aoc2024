advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<usize> {
    let input = input.lines().map(|row| row.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let [w, h] = [input[0].len() as isize, input.len() as isize];
    let map = input.concat();
    let mut frequencies = vec![];
    for &c in &map {
        if c == '.' { continue; }
        if !frequencies.contains(&c) { frequencies.push(c); }
    }
    let nodes = frequencies.iter().map(|&frequency| map.iter().enumerate().filter(move |&(_,f)| *f==frequency).map(|(i,_)| i as isize).collect::<Vec<_>>() ).collect::<Vec<_>>();
    let mut marks = vec![' '; map.len()];
    for (&frequency, nodes) in frequencies.iter().zip(nodes) {
        for a in &nodes {
            let a = [a%w, a/w];
            for b in &nodes {
                let b = [b%w, b/w];
                if a == b { continue; }
                let [x,y] = [0,1].map(|i| a[i]+a[i]-b[i]);
                if x >= 0 && x < w && y >= 0 && y < h { marks[(y*w+x) as usize] = frequency; }
            }
        }
    }
    for y in 0..h { 
        println!("{}", (0..w).map(|x| if marks[(y*w+x) as usize] != ' ' { '#' } else { map[(y*w+x) as usize] }).collect::<String>())
    }
    Some(marks.iter().filter(|&&mark| mark!=' ').count())
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = input.lines().map(|row| row.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let [w, h] = [input[0].len() as isize, input.len() as isize];
    let map = input.concat();
    let mut frequencies = vec![];
    for &c in &map {
        if c == '.' { continue; }
        if !frequencies.contains(&c) { frequencies.push(c); }
    }
    let nodes = frequencies.iter().map(|&frequency| map.iter().enumerate().filter(move |&(_,f)| *f==frequency).map(|(i,_)| i as isize).collect::<Vec<_>>() ).collect::<Vec<_>>();
    let mut marks = vec![' '; map.len()];
    for (&frequency, nodes) in frequencies.iter().zip(nodes) {
        for a in &nodes {
            let a = [a%w, a/w];
            for b in &nodes {
                let b = [b%w, b/w];
                if a == b { continue; }
                for n in -isize::max(w,h)..isize::max(w,h) {
                    let [x,y] = [0,1].map(|i| a[i]+n*(a[i]-b[i]));
                    if x >= 0 && x < w && y >= 0 && y < h { marks[(y*w+x) as usize] = frequency; }
                }
            }
        }
    }
    for y in 0..h { 
        println!("{}", (0..w).map(|x| if marks[(y*w+x) as usize] != ' ' { '#' } else { map[(y*w+x) as usize] }).collect::<String>())
    }
    Some(marks.iter().filter(|&&mark| mark!=' ').count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
