advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<usize> {
    let map = input.lines().map(|row| row.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let [w, h] = [map[0].len() as isize, map.len() as isize];
    let mut map = map.concat();
    let guard = map.iter().position(|&e| e=='^').unwrap() as isize;
    let [mut x, mut y] = [guard%w, guard/w];
    let [mut dx, mut dy] = [0,-1];
    loop {
        map[(y*w+x) as usize] = 'X';
        let [nx, ny] = [x as isize + dx, y as isize + dy];
        if nx < 0 || nx >= w || ny < 0 || ny >= h { break; }
        if map[(ny*w+nx) as usize] == '#' {
            [dx, dy] = [-dy, dx]
        } else {
            [x, y] = [nx, ny]
        }
    }
    Some(map.iter().filter(|&&e| e=='X').count())
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = input.lines().map(|row| row.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let [w, h] = [map[0].len() as isize, map.len() as isize];
    let map = map.concat();
    let guard = map.iter().position(|&e| e=='^').unwrap() as isize;
    let guard = [guard%w, guard/w];
    let mut count = 0;
    let mut max = 0;
    for y in 0..h { for x in 0..w {
        let mut i = 0;
        {
            if map[(y*w+x) as usize] != '.' { continue; }
            let map = {let mut map = map.clone(); map[(y*w+x) as usize] = '#'; map};
            let [mut x, mut y] = guard;
            let [mut dx, mut dy] = [0,-1];
            let mut visited = vec![vec![]; map.len()];
            loop {
                i += 1;
                /*if i > 5394 {
                    for y in 0..h {
                        println!("{}", ((y*w) as usize..(y*w+w) as usize).map(|i| match (map[i], visited[i]) {
                            ('#', _) => '#',
                            ('^', _) => 'X',
                            (_, [0,0]) => '.',
                            (_, [0,-1]) => '^',
                            (_, [0,1]) => 'V',
                            (_, [1,0]) => '>',
                            (_, [-1,0]) => '<',
                            _ => unreachable!()
                        }).collect::<String>())
                    }
                    panic!();
                    count +=1;
                    break;
                }*/
                if visited[(y*w+x) as usize].contains(&[dx,dy]) { count +=1; break; }
                visited[(y*w+x) as usize].push([dx,dy]);
                let [nx, ny] = [x as isize + dx, y as isize + dy];
                if nx < 0 || nx >= w || ny < 0 || ny >= h { break; }
                if map[(ny*w+nx) as usize] == '#' {
                    [dx, dy] = [-dy, dx]
                } else {
                    [x, y] = [nx, ny]
                }
            }
        }
        max = max.max(i);
        //println!("# {x} {y} {i} {max}");
    }
    println!("{y}/{h} {max} {count}");}
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
