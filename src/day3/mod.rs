use std::collections::HashSet;

pub fn solve() {
    let path = std::path::Path::new("inputs/day3.txt");
    let input = std::fs::read_to_string(path).expect("Something went wrong when reading the file");
    let directions: Vec<char> = input.chars().collect();
    let mut grid = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    for dir in &directions {
        grid.insert((x, y));
        match dir {
            '^' => y += 1,
            'v' => y -= 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => println!("input is not handled"),
        }
    }
    grid.insert((x, y));

    println!("part1 {}", grid.len());
    assert_eq!(2592, grid.len(), "part 1 solution should be 2592");

    // part 2
    let mut part2_grid = HashSet::new();
    let mut part2_x = [0, 0];
    let mut part2_y = [0, 0];
    let mut which = 0;
    for dir in directions {
        part2_grid.insert((part2_x[which], part2_y[which]));
        match dir {
            '^' => part2_y[which] += 1,
            'v' => part2_y[which] -= 1,
            '>' => part2_x[which] += 1,
            '<' => part2_x[which] -= 1,
            _ => println!("input is not handled"),
        }
        which = 1 - which;
    }
    part2_grid.insert((part2_x[which], part2_y[which]));

    println!("part1 {}", part2_grid.len());
    assert_eq!(2360, part2_grid.len(), "part 2 solution should be 2360");
}
