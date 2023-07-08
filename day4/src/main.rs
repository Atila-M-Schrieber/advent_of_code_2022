fn main() {
    let contents: Vec<String> = include_str!("../input.txt").lines()
        .map(|s| s.to_string()).collect();
    // let contents: Vec<String> = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8".lines().map(|s| s.to_string()).collect();
    let pairs: Vec<((usize,usize), (usize,usize))> = contents.iter()
        .map(|s| {
            let sections: Vec<usize> = s.split(",")
                .flat_map(|p| p.split("-")
                          .map(|sec| sec.parse::<usize>().unwrap() ))
                .collect();
            ((sections[0], sections[1]), (sections[2], sections[3]))
        })
        .collect();
    let full_overlaps = pairs.iter()
        .filter(|((a, b), (c, d))| (a >= c && b <= d) || (c >= a && d <= b))
        .collect::<Vec<&((usize,usize), (usize,usize))>>();
    let partial_overlaps = pairs.iter()
        .filter(|&&((a, b), (c, d))| (a..b+1).contains(&c) || (c..d+1).contains(&a))
        .collect::<Vec<&((usize,usize), (usize,usize))>>();

    println!("Number of full overlaps: {}", full_overlaps.len());
    println!("Number of partial overlaps: {}", partial_overlaps.len());
}
