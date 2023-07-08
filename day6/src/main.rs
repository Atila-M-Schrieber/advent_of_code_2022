fn markers(chars: &[u8], u: usize) -> Vec<(usize, u8)> {
    chars
        .iter()
        .enumerate()
        .filter_map(|(i, c)| match (0..u).contains(&i) {
            true => None,
            false => {
                let non_u: Vec<u8> = chars[i - u..i]
                    .iter()
                    .enumerate()
                    .filter(|(j, h)| chars[i-u..i-u+j].contains(h))
                    .map(|t| t.1)
                    .copied()
                    .collect();
/*                if chars[i - u..=i]
                    .iter()
                    .enumerate()
                    .all(|(j, h)| !&chars[i-u..i-u+j].contains(h))
                    */
                if non_u.len() == 0
                {
                    println!("Yep:{} {}", i, chars[i-u..=i].iter().map(|&c| c as char).collect::<String>());
                    Some((i, c.to_owned()))
                } else {
                    println!("Bruh: {} - {}", non_u.iter().map(|&c| c as char).collect::<String>(),
                    chars[i-u..=i].iter().map(|&c| c as char).collect::<String>());
                    None
                }
            }
        })
        .collect()
}

fn main() {
    let contents: &[u8] = include_str!("../input.txt").as_bytes();
    println!("First start-of-packet marker: {}", markers(contents, 4)[0].0);
    println!("First start-of-message marker: {}", markers(contents, 14)[0].0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_for_examples_message() {
        let examples: &[&[u8]] = &[
            "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
            "bvwbjplbgvbhsrlpgdmjqwftvncz",
            "nppdvjthqldpwncqszvftbrmjlhg",
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
        ]
        .map(|s| s.as_bytes()); //.collect();
        let mks = examples.iter().map(|s| markers(s, 14)).collect::<Vec<_>>();
        println!("{:?}", mks);
        assert_eq!(
            mks.iter().map(|ms| ms[0].0).collect::<Vec<_>>(),
            [19, 23, 23, 29, 26]
        )
    }

    #[test]
    fn check_for_examples_packet() {
        let examples: &[&[u8]] = &[
            "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
            "bvwbjplbgvbhsrlpgdmjqwftvncz",
            "nppdvjthqldpwncqszvftbrmjlhg",
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
        ]
        .map(|s| s.as_bytes()); //.collect();
        let mks = examples.iter().map(|s| markers(s, 4)).collect::<Vec<_>>();
        println!("{:?}", mks);
        assert_eq!(
            mks.iter().map(|ms| ms[0].0).collect::<Vec<_>>(),
            [7, 5, 6, 10, 11]
        )
    }
}
