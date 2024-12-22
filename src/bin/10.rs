advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<usize> {
    let map = input.lines().map(|row| row.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let [w, h] = [map[0].len() as isize, map.len() as isize];
    let map = map.concat();
    let mut count = 0;
    for y in 0..h { for x in 0..w {
        if map[(y*w+x) as usize] != '0' { continue; }
        let mut peaks = vec![];
        let mut visited = map.clone();
        fn walk(peaks: &mut Vec<[isize; 2]>, map: &[char], [w,h]: [isize; 2], [x,y]: [isize; 2], height: char, visited: &mut [char]) {
            visited[(y*w+x) as usize] = char::from_u32('𝟎' as u32+char::to_digit(height, 10).unwrap()).unwrap();
            if height == '9' {
                if !peaks.contains(&[x,y]) { peaks.push([x,y]) }
                return;
            }
            for [dx,dy] in [[0,-1],[-1,0],[1,0],[0,1]] {
                let [x,y] = [x+dx, y+dy];
                let next = char::from_digit(char::to_digit(height, 10).unwrap()+1, 10).unwrap();
                if x >= 0 && x < w && y >= 0 && y < h && map[(y*w+x) as usize] == next {
                    walk(peaks, map, [w,h], [x,y], next, visited);
                }
            }
        }
        walk(&mut peaks, &map, [w, h], [x,y], '0', &mut visited);
        //for y in 
        //println!("{}", peaks.len());
        count += peaks.len();
    }}
    Some(count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = input.lines().map(|row| row.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let [w, h] = [map[0].len() as isize, map.len() as isize];
    let map = map.concat();
    let mut count = 0;
    for y in 0..h { for x in 0..w {
        if map[(y*w+x) as usize] != '0' { continue; }
        let mut peaks = vec![];
        let mut visited = map.clone();
        fn walk(peaks: &mut Vec<[isize; 2]>, map: &[char], [w,h]: [isize; 2], [x,y]: [isize; 2], height: char, visited: &mut [char]) {
            visited[(y*w+x) as usize] = char::from_u32('𝟎' as u32+char::to_digit(height, 10).unwrap()).unwrap();
            if height == '9' {
                peaks.push([x,y]);
                return;
            }
            for [dx,dy] in [[0,-1],[-1,0],[1,0],[0,1]] {
                let [x,y] = [x+dx, y+dy];
                let next = char::from_digit(char::to_digit(height, 10).unwrap()+1, 10).unwrap();
                if x >= 0 && x < w && y >= 0 && y < h && map[(y*w+x) as usize] == next {
                    walk(peaks, map, [w,h], [x,y], next, visited);
                }
            }
        }
        walk(&mut peaks, &map, [w, h], [x,y], '0', &mut visited);
        //for y in 
        //println!("{}", peaks.len());
        count += peaks.len();
    }}
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
