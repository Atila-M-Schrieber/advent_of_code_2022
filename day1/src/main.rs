
struct Elf {
    food: Vec<usize>,
}

impl Elf{
    fn calories(&self) -> usize {
        self.food.iter().sum()
    }
}

impl From<&Vec<&String>> for Elf {
    fn from(elf_vec: &Vec<&String>) -> Self {
        Elf {
            food: elf_vec.iter()
                .map(|s| s.parse::<usize>().unwrap())
                .collect(),
        }
    }
}


fn subvector<'a, 'b, T: PartialEq>(vec: &'a Vec<T>, delimiter: &'b T) -> Vec<Vec<&'a T>> {
    let mut temp = Vec::new();
    let mut output = Vec::new();

    for item in vec {
        if item == delimiter && !temp.is_empty() {
            output.push(temp.clone());
            temp.clear();
        } else {
            temp.push(item.clone());
        }
    }
    if !temp.is_empty() {
        output.push(temp.clone());
    }

    output
}


fn main() {
    let contents: Vec<String> = include_str!("../input.txt").lines()
        .map(|s| s.to_string()).collect();
    let elves: Vec<Elf> = subvector(&contents, &String::from("")).iter()
        .map(|elf_vec| elf_vec.into() ).collect();
        // .map(|elf| elf.iter()
                    // .map(|s| s.parse::<usize>().unwrap()).collect()
                        // ).collect();

    let mut top_3_elves = elves;
    top_3_elves.sort_by_key(|elf| elf.calories() );
    top_3_elves.reverse();
    let top_3_elves: Vec<Elf> = top_3_elves.into_iter().take(3).collect();

    println!("Most calories on top 3 elves: {}", 
             top_3_elves.iter().map(|elf| elf.calories()).sum::<usize>());
}
