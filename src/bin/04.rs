advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let input = input.lines().map(|row| row.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let [w, h] = [input[0].len(), input.len()];
    let mut count = 0;
    for y in 0..h {
        for x in 0..w {
            'next: for [dx,dy] in [[-1,-1],[0,-1],[1,-1], [-1,0],[1,0], [-1,1],[0,1],[1,1]] {
                for (i, needle) in ['X','M','A','S'].iter().enumerate() {
                    let [x,y] = [x as isize + i as isize * dx, y as isize + i as isize * dy];
                    if let Some(hay) = input.get(y as usize).map(|row| row.get(x as usize)).flatten() {
                        if hay == needle {} else { continue 'next; }
                    } else { continue 'next; }
                }
                count += 1;
            }
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = input.lines().map(|row| row.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let [w, h] = [input[0].len(), input.len()];
    let mut count = 0;
    for y in 1..h-1 {
        for x in 1..w-1 {
            if input[y][x] != 'A' { continue; }
            if {
                let mut count = 0;
                for [dx,dy] in [[-1,-1],[1,-1],[-1,1],[1,1]] {
                    let get = |dx,dy| input[(y as isize + dy) as usize][(x as isize + dx) as usize];
                    if get(dx,dy) == 'M' && get(-dx,-dy) == 'S' { count += 1; }
                }
                if count < 2 { false }
                else if count == 2 { true }
                else { unreachable!() }
            } {
                count += 1;
            }
        }
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
