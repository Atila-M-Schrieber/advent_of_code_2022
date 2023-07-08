fn transpose<T: Clone + std::fmt::Debug>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let size = {
        let cols = matrix.len();
        let rows: Vec<usize> = matrix.iter().map(|r| r.len()).collect();
        if rows.is_empty()
            || !rows
                .iter()
                .fold((true, rows[0]), |prev, len| {
                    (prev.1 == *len && prev.0, *len)
                })
                .0
        {
            panic!("Not all row lengths are equal! Rows: {:?}", rows);
        }
        let rows: usize = rows[0];
        (cols, rows)
    };

    let mut out_matrix = Vec::new();
    for i in 0..size.1 {
        out_matrix.push(Vec::new()); // create new stack for every element of row
        for j in 0..size.0 {
            out_matrix[i].push(matrix[size.0 - j - 1][i].clone());
        }
        // println!("Row: {:?}", out_matrix[i]);
    }
    out_matrix
}

#[derive(Clone, Debug, PartialEq)]
struct Stacks(Vec<Vec<u8>>);

impl Stacks {
    fn execute_2(self, instructs: Vec<Instruction>) -> Self {
        instructs
            .into_iter()
            .fold(self, |stacks, instruction| instruction.execute_2(stacks))
    }

    fn execute(self, instructs: Vec<Instruction>) -> Self {
        instructs
            .into_iter()
            .fold(self, |stacks, instruction| instruction.execute(stacks))
    }

    fn message(&self) -> String {
        self.0
            .iter()
            .map(|v| v.last().copied().unwrap() as char)
            .collect()
    }
}

#[derive(Copy, Clone, Debug)]
struct Instruction {
    from: usize,
    to: usize,
    amount: usize,
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Instruction {
        let bits: Vec<&str> = s.split(" ").collect();
        let get_bit = |i: usize| {
            bits[i]
                .parse::<usize>()
                .expect(format!("Cannot parse {i}th word ({})", bits[i]).as_str())
        };
        let amount = get_bit(1);
        let from = get_bit(3) - 1; // subtract 1 for correct indexing
        let to = get_bit(5) - 1;
        Instruction { from, to, amount }
    }
}

impl Instruction {
    fn execute_2(self, stacks: Stacks) -> Stacks {
        // println!("Instruct: {:?}\nStart:\n{:?}\n", self, stacks);
        let (mut stacks_temp, mut temps) = (0..self.amount)
            .fold((stacks, Vec::new()), |(mut cs, mut temps), _| {
            if let Some(top) = cs.0[self.from].pop() {
                temps.push(top)
            }
            // println!("{:?}\n", cs);
            (cs, temps)
        });

        temps.reverse();
        stacks_temp.0[self.to].append(&mut temps);
        // println!("{:?}\n", stacks_temp);
        stacks_temp
    }

    fn execute(self, stacks: Stacks) -> Stacks {
        // println!("Instruct: {:?}\nStart:\n{:?}\n", self, stacks);
        (0..self.amount).fold(stacks, |mut cs, _| {
            if let Some(top) = cs.0[self.from].pop() {
                cs.0[self.to].push(top);
            }
            // println!("{:?}\n", cs);
            cs
        })
    }
}

fn main() {
    let binding = include_str!("../input.txt").lines().collect::<Vec<&str>>();
    let contents: Vec<&[&str]> = binding.split(|&v| v == "").collect();
    let stack_str: Vec<Vec<u8>> = contents[0]
        .into_iter()
        .map(|line| line.as_bytes().to_vec())
        .collect();
    let stacks: Stacks = Stacks(
        transpose(stack_str)
            .into_iter()
            .filter(|stack| stack[0] != 32)
            .map(|stack| {
                stack[1..]
                    .to_vec()
                    .into_iter()
                    .filter(|&i| i != 32)
                    .collect::<Vec<u8>>()
            })
            .collect(),
    );

    let instructions: Vec<Instruction> = contents[1].into_iter().map(|&line| line.into()).collect();

    println!("Message: {}", stacks.clone().execute(instructions.clone()).message());
    println!("Message 2: {}", stacks.execute_2(instructions).message());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_contents() -> Vec<Vec<String>> {
        "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2"
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .split(|v| v == "")
            .map(|chunk| chunk.to_vec())
            .collect()
    }

    fn example_stacks() -> Stacks {
        let stack_str: Vec<Vec<u8>> = example_contents()[0]
            .iter()
            .map(|line| line.as_bytes().to_vec())
            .collect();
        let stacks: Stacks = Stacks(
            transpose(stack_str)
                .into_iter()
                .filter(|stack| stack[0] != 32)
                .map(|stack| {
                    stack[1..]
                        .to_vec()
                        .into_iter()
                        .filter(|&i| i != 32)
                        .collect::<Vec<u8>>()
                })
                .collect(),
        );

        stacks
    }

    fn example_instructions() -> Vec<Instruction> {
        example_contents()[1]
            .iter()
            .map(|line| line.as_str().into())
            .collect()
    }

    fn correct_stacks() -> Stacks {
        let stack_str: Vec<Vec<u8>> =
            "        [Z]\n        [N]\n        [D]\n[C] [M] [P]\n 1   2   3 "
                .lines()
                .map(|s| s.to_string())
                // .collect::<Vec<String>>()
                .map(|line| line.as_bytes().to_vec())
                .collect();
        let stacks: Stacks = Stacks(
            transpose(stack_str)
                .into_iter()
                .filter(|stack| stack[0] != 32)
                .map(|stack| {
                    stack[1..]
                        .to_vec()
                        .into_iter()
                        .filter(|&i| i != 32)
                        .collect::<Vec<u8>>()
                })
                .collect(),
        );

        stacks
    }

    fn correct_stacks_2() -> Stacks {
        let stack_str: Vec<Vec<u8>> =
            "        [D]\n        [N]\n        [Z]\n[M] [C] [P]\n 1   2   3 "
                .lines()
                .map(|s| s.to_string())
                // .collect::<Vec<String>>()
                .map(|line| line.as_bytes().to_vec())
                .collect();
        let stacks: Stacks = Stacks(
            transpose(stack_str)
                .into_iter()
                .filter(|stack| stack[0] != 32)
                .map(|stack| {
                    stack[1..]
                        .to_vec()
                        .into_iter()
                        .filter(|&i| i != 32)
                        .collect::<Vec<u8>>()
                })
                .collect(),
        );

        stacks
    }

    #[test]
    fn correct_instruction_execution() {
        assert_eq!(
            correct_stacks(),
            example_stacks().execute(example_instructions())
        );
    }

    #[test]
    fn correct_example_message() {
        assert_eq!(
            "CMZ".to_string(),
            example_stacks().execute(example_instructions()).message(),
        );
    }

    #[test]
    fn correct_instruction_execution_2() {
        assert_eq!(
            correct_stacks_2(),
            example_stacks().execute_2(example_instructions())
        );
    }

    #[test]
    fn correct_example_message_2() {
        assert_eq!(
            "MCD".to_string(),
            example_stacks().execute_2(example_instructions()).message(),
        );
    }
}
