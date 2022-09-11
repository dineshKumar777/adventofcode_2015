pub fn solve() {
    let input = "ckczppom";

    let output = find_matching_md5(input, "00000");
    println!("part1 {output}");
    assert_eq!(117946, output, "part 1 solution should be 117946");

    //part 2
    let output = find_matching_md5(input, "000000");
    println!("part2 {output}");
    assert_eq!(3938038, output, "part 2 solution should be 3938038");
}

fn find_matching_md5(input: &str, zero_trial: &str) -> usize {
    let mut index: usize = 0;
    loop {
        let data = format!("{input}{index}");
        let digest = md5::compute(data);
        let hash = format!("{:x}", digest);

        if hash.starts_with(zero_trial) {
            return index;
        }

        index += 1;
    }
}
