pub fn solve() {
    let path = std::path::Path::new("inputs/day2.txt");
    let input = std::fs::read_to_string(path).expect("Something went wrong when reading the file");
    let mut output = 0;

    for line in input.lines() {
        let rect_dimensions: Vec<i32> = line.split('x').map(|s| s.parse().unwrap()).collect();
        let length = rect_dimensions[0];
        let width = rect_dimensions[1];
        let height = rect_dimensions[2];

        let lw_side = length * width;
        let wh_side = width * height;
        let hl_side = height * length;

        let sides = vec![lw_side, wh_side, hl_side];
        let smallest_side = sides.iter().min().unwrap();
        let surface_area = 2 * lw_side + 2 * wh_side + 2 * hl_side;

        output += surface_area + smallest_side;
    }

    println!("part1 {output}");
    assert_eq!(1606483, output, "part 1 solution should be 1606483");

    // part 2
    let mut part2_output = 0;
    for line in input.lines() {
        let mut rect_dimensions: Vec<i32> =
            line.split('x').map(|s| s.parse::<i32>().unwrap()).collect();
        rect_dimensions.sort();
        // println!("{rect_dimensions:?}");
        let present_ribbon =
            rect_dimensions[0] + rect_dimensions[0] + rect_dimensions[1] + rect_dimensions[1];
        let bow_ribbon = rect_dimensions[0] * rect_dimensions[1] * rect_dimensions[2];
        part2_output += present_ribbon + bow_ribbon;
    }
    println!("part2 {part2_output}");
    assert_eq!(3842356, part2_output, "part2 solution should be 3842356");
}
