advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<usize> {
    let map = input.lines().map(|row| row.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let [w, h] = [map[0].len() as isize, map.len() as isize];
    let map = map.concat();
    let mut plants = vec![];
    for &plant in &map { if !plants.contains(&plant) { plants.push(plant)}; }
    let mut visited = vec![false; map.len()];
    let mut sum = 0;
    for y in 0..h { for x in 0..w {
        if visited[(y*w+x) as usize] { continue; }
        let plant = map[(y*w+x) as usize];
        fn visit(visited: &mut [bool], map: &[char], [w,h]: [isize; 2], plant: char, [x,y]: [isize; 2]) -> (usize, usize) {
            visited[(y*w+x) as usize] = true;
            let (mut area, mut perimeter) = (1, 0);
            for [dx,dy] in [[0,-1],[-1,0],[1,0],[0,1]] {
                let [x,y] = [x+dx, y+dy];
                if x >= 0 && x < w && y >= 0 && y < h {
                    if map[(y*w+x) as usize] != plant { perimeter += 1; }
                    else if !visited[(y*w+x) as usize] { 
                        let (a, p) = visit(visited, map, [w,h], plant, [x, y]);
                        area += a;
                        perimeter += p;
                    }
                } else { 
                    perimeter += 1; 
                }
            }
            (area, perimeter)
        }
        let (area, perimeter) = visit(&mut visited, &map, [w,h], plant, [x,y]);
        println!("{plant} {perimeter} * {area} = {}", perimeter*area);
        sum += area * perimeter;
    }}
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
