// use byte_string::ByteStr;
use std::fmt;
use std::str;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Item(u8);

impl Item {
    fn priority(&self) -> u8 {
        if (65..91).contains(&self.0) {
            self.0 - 38
        } else if (97..123).contains(&self.0) {
            self.0 - 96
        } else {
            panic!("Invalid character: {}", self.0)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Rucksack(Vec<Item>, Vec<Item>);

impl From<&Vec<Item>> for Rucksack {
    fn from(items: &Vec<Item>) -> Rucksack {
        let length = items.len();
        match length % 2 {
            0 => {
                Rucksack(items[..length / 2].to_vec(), items[length / 2..].to_vec())
            },
            _ => panic!("Cannot create rucksack from odd number of items"),
        }
    }
}

impl fmt::Display for Rucksack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn to_str(items: Vec<&Item>) -> String {
            str::from_utf8(items.iter().map(|i| i.0).collect::<Vec<_>>().as_slice()).unwrap().into()
        }
        let s0 = to_str(self.0.iter().collect());
        let s1 = to_str(self.1.iter().collect());

        write!(f, "L: {}; R: {} \nOverlap: {:?} (Priority {})",
            s0, s1, self.overlap(), self.priority())
    }
}

impl Rucksack {
    fn consolidate(&self) -> Vec<Item> {
        self.0.clone().into_iter().chain(self.1.clone().into_iter()).collect()
    }

    fn overlap(&self) -> Option<&Item> {
        self.0.iter().filter(|i| self.1.contains(i)).collect::<Vec<&Item>>().first().copied()
    }

    fn priority(&self) -> u8 {
        self.overlap().unwrap_or(&Item(97)).priority()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Group(Vec<Rucksack>);

impl From<Vec<Rucksack>> for Group {
    fn from(sacks: Vec<Rucksack>) -> Group {
        match sacks.len() {
            3 => Group(sacks),
            _ => panic!("Invalid number of rucksacks ({}) per group, only 3 allowed!", sacks.len()),
        }
    }
}

impl Group {
    fn consolidate(&self) -> Vec<Vec<Item>> {
        self.0.clone().into_iter()
            .map(move |sack| sack.0.into_iter().chain(sack.1.into_iter()).collect())
            .collect::<Vec<Vec<Item>>>()
    }

    fn badge(&self) -> Option<Item> {
        // let cons = self.consolidate();
        // cons[0].iter().filter(|i| cons[1].contains(i) && cons[2].contains(i))
            // .collect::<Vec<&Item>>().first().cloned()
        self.0[0].consolidate().into_iter()
            .filter(|i| self.0[1].consolidate().contains(i)
                        && self.0[2].consolidate().contains(i))
            .collect::<Vec<Item>>().first().cloned()
    }

    fn priority(&self) -> u8 {
        self.badge().unwrap().priority()
    }
}

fn get_groups(sacks: Vec<Rucksack>) -> Vec<Group> {
    let l = sacks.len();
    let is = match l % 3 {
        0 => (0..l).step_by(3),
        _ => panic!("Number of groups must be divisible by 3"),
    };

    let mut groups = Vec::new();
    for i in is {
        let group: Group = sacks[i..i+3].to_owned().into();
        groups.push(group);
    }
    groups
}

fn main() {
    let contents: Vec<&[u8]> = include_str!("../input.txt").lines()
        .map(|s| s.as_bytes()).collect();
    let items: Vec<Vec<Item>> = contents.iter()
        .map(|s| s.iter().map(|&i| Item(i)).collect() ).collect();
    let rucksacks: Vec<Rucksack> = items.iter()
        .map(|vec| vec.into()).collect();
    let sum_of_priorities = rucksacks.iter()
        .map(|rucksack| rucksack.priority() as u32).sum::<u32>();

    println!("Sum of priorities: {}", sum_of_priorities);

    let groups = get_groups(rucksacks);
    let sum_of_badge_priorities = groups.iter()
        .map(|g| g.priority() as u32).sum::<u32>();

    println!("Sum of badge priorities: {}", sum_of_badge_priorities);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_contents() -> Vec<&'static [u8]> {
        b"vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw"
            .split(|&b| b == 10).collect()
    }
    fn example_items() -> Vec<Vec<Item>> {
        example_contents().iter()
            .map(|s| s.iter().map(|&i| Item(i)).collect() ).collect()
    }
    fn example_rucksacks() -> Vec<Rucksack> {
        example_items().iter()
            .map(|vec| vec.into()).collect()
    }
    fn shared_items() -> Vec<Item> {
        (vec![b'p', b'L', b'P', b'v', b't', b's']).iter()
            .map(|&c| Item(c)).collect()
    }

    #[test]
    fn check_correct_priorities_1() {
        // let items: Vec<Item> = (vec![b'p', b'L', b'P', b'v', b't', b's']).iter()
            // .map(|&c| Item(c)).collect();
        let priorities = vec![16, 38, 42, 22, 20, 19];
        assert_eq!(priorities, shared_items().iter().map(|i| i.priority()).collect::<Vec<_>>() );
    }

    #[test]
    fn check_example_rucksacks() {
        let example_rucksacks = example_rucksacks();
        let overlaps: Vec<&Item> = example_rucksacks.iter()
                   .map(|rucksack| rucksack.overlap())
                   .flatten()
                   .collect();

        let shared_items = shared_items();
        let shared_items_ref = shared_items.iter().collect::<Vec<&Item>>();

        assert_eq!(shared_items_ref, overlaps);

        assert_eq!(157, example_rucksacks.iter()
                   .map(|rucksack| rucksack.overlap().iter()
                        .map(|item| item.priority()).sum::<u8>() as u16).sum::<u16>())
    }

    #[test]
    fn check_example_group_badges() {
        let example_rucksacks = example_rucksacks();
        let groups = get_groups(example_rucksacks);
        let badges: Vec<Item> = vec![b'r', b'Z'].iter().map(|&c| Item(c)).collect();
        // let badges: Vec<Item> = badges_unref.iter().collect::<Vec<&Item>>();
        assert_eq!(badges, groups.iter().map(|g| g.badge().unwrap()).collect::<Vec<Item>>())
    }

    #[test]
    fn check_example_group_priorities() {
        let example_rucksacks = example_rucksacks();
        let groups = get_groups(example_rucksacks);
        let prios = vec![18, 52];
        assert_eq!(prios, groups.iter().map(|g| g.priority()).collect::<Vec<u8>>())
    }
}
