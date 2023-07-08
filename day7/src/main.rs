use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use Command::*;
// use Filesystem::*;
use std::collections::HashMap;

#[derive(Debug)]
struct Dir {
    name: String,
    entries: HashMap<String, Entry>,
}

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug)]
enum Entry {
    Dir(Dir),
    File(File),
}

#[derive(Debug)]
enum Command {
    CD(String),
    LS(HashMap<String, Entry>),
}

struct Shell {
    wd: Vec<String>,
    fs: Dir,
}

fn print_fs(Dir { name, entries }: &Dir, depth: usize) {
    println!("{:indent$}Dir: {}", "", name, indent = depth);
    for entry in entries.values() {
        match entry {
            Entry::Dir(dir) => print_fs(dir, depth + 2),
            Entry::File(File { name, size }) => {
                println!("{:indent$}{} {}", "", name, size, indent = depth + 2)
            }
        }
    }
}

impl Dir {
    fn new(name: String) -> Self {
        Dir {
            name,
            entries: HashMap::new(),
        }
    }

    fn print(&self) {
        print_fs(self, 0)
    }

    fn size(&self) -> usize {
        self.entries
            .iter()
            .map(|(_, entry)| match entry {
                Entry::File(file) => file.size,
                Entry::Dir(dir) => dir.size(),
            })
            .sum()
    }
    
}

impl Shell {
    fn new() -> Self {
        Shell {
            wd: Vec::new(),
            fs: Dir::new("".into()),
        }
    }
    fn pwd(&self) {
        println!("{}", self.wd.join("/"))
    }

    fn extend(&mut self, fs: HashMap<String, Entry>) {
        let mut current_dir = self.wd.iter_mut().fold(&mut self.fs, |dir, name| {
            if let Entry::Dir(d) = dir.entries.get_mut(name).expect("Directory not found") {
                d
            } else {
                panic!("That's a file, not a directory")
            }
        });

        current_dir.entries.extend(fs);
    }

    fn command(mut self, cmd: Command) -> Self {
        match cmd {
            CD(dir) => match dir.as_str() {
                ".." => {
                    self.wd.pop();
                }
                "/" | "" => {
                    self.wd.clear();
                }
                _ => self.wd.push(dir),
            },
            LS(mut fs) => self.extend(fs),
        }
        self
    }

    fn sizes(&self) -> Vec<usize> {
        let mut sizes = Vec::new();

        fn get_sizes(fs: &Dir, sizes: &mut Vec<usize>) {
            let _ = fs
                .entries
                .iter()
                .map(|(_, entry)| match entry {
                    Entry::File(file) => (),
                    Entry::Dir(dir) => sizes.append(&mut {
                        let mut subsizes = Vec::new();
                        subsizes.push(dir.size());
                        get_sizes(dir, &mut subsizes);
                        // subsizes.append(&mut subsubsizes);
                        subsizes
                    }),
                })
                .collect::<Vec<()>>();
        }

        sizes.push((&self).fs.size());
        get_sizes(&self.fs, &mut sizes);
        sizes.sort();
        sizes
    }
}

impl From<&str> for Command {
    fn from(command: &str) -> Command {
        let cmd: String = command.chars().take(2).collect();
        match cmd.as_str() {
            "cd" => CD(command.chars().skip(3).collect::<String>().trim().into()),
            "ls" => {
                let mut entries = HashMap::new();
                let _: Vec<_> = command
                    .lines()
                    .skip(1)
                    .map(|s| {
                        let bits: Vec<&str> = s.split(" ").collect();
                        let entry = match bits[0] {
                            "dir" => Entry::Dir(Dir::new(bits[1].into())),
                            _ => Entry::File(File {
                                name: bits[1].into(),
                                size: bits[0].parse().expect("Can't parse to usize"),
                            }),
                        };
                        entries.insert(bits[1].into(), entry)
                    })
                    .collect();
                LS(entries)
            }
            _ => panic!("Invalid command: {command}"),
        }
    }
}

fn get_commands(contents: &str) -> Vec<Command> {
    contents.split("$ ").skip(1).map(|c| c.into()).collect()
}

fn build_shell(contents: &str) -> Shell {
        let commands = get_commands(&contents);
        // println!("Commands:\n{:?}", commands);
        let mut shell = Shell::new();
        for command in commands.into_iter() {
            shell = shell.command(command);
        }

        shell
}

fn main() {
    let contents = include_str!("../input.txt");
    let shell = build_shell(&contents);
    let to_free = shell.fs.size() - 40000000;

    println!("Sum of sizes under 100k: {}", shell.sizes().iter().filter(|&&size| size <= 100000).sum::<usize>());
    println!("Size of smallest dir to delete: {}", shell.sizes().iter().filter(|&&size| size >= to_free ).min().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shell() -> Shell {
        let contents = "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k";
        build_shell(&contents)
    }

    #[test]
    fn check_sizes() {
        let mut sizes = Vec::new();

        fn get_sizes(fs: &Dir, sizes: &mut Vec<usize>) {
            let _ = fs
                .entries
                .iter()
                .map(|(_, entry)| match entry {
                    Entry::File(_) => (),
                    Entry::Dir(dir) => sizes.append(&mut {
                        let mut subsizes = Vec::new();
                        subsizes.push(dir.size());
                        get_sizes(dir, &mut subsizes);
                        // subsizes.append(&mut subsubsizes);
                        subsizes
                    }),
                })
                .collect::<Vec<()>>();
        }

        sizes.push((&shell()).fs.size());
        get_sizes(&shell().fs, &mut sizes);
        sizes.sort();

        assert_eq!(vec![584, 94853, 24933642, 48381165], shell().sizes())
    }
}
