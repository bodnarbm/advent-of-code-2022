use std::{collections::HashMap, fmt::Debug, io, str::FromStr};

use nom::{
    branch::alt,
    bytes::{complete::take_while, streaming::tag},
    character::{
        complete::{alpha1, digit1, newline},
        streaming::space1,
    },
    combinator::{map, map_res, opt},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
};

fn main() -> io::Result<()> {
    let input = io::read_to_string(io::stdin())?;
    match part1(&input) {
        Ok(result) => println!("Part 1: {}", result),
        Err(err) => eprintln!("Part 1: Error= {}", err),
    }
    match part2(&input) {
        Ok(result) => println!("Part 1: {}", result),
        Err(err) => eprintln!("Part 1: Error= {}", err),
    }
    Ok(())
}

type Error = String;
type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
enum Node {
    File(FileNode),
    Directory(usize),
}

#[derive(Debug)]
struct FileNode {
    name: String,
    size: usize,
}

#[derive(Debug)]
struct DirectoryNode {
    name: String,
    contents: Vec<Node>,
}

impl DirectoryNode {
    fn root() -> Self {
        Self::new("/")
    }

    fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            contents: vec![],
        }
    }
}

#[derive(Debug)]
struct FileSystem {
    cwd_idx: usize,
    directories: Vec<DirectoryNode>,
    directory_idx_to_parent_idx: HashMap<usize, usize>,
}

impl FileSystem {
    fn new() -> Self {
        let root = DirectoryNode::root();

        Self {
            cwd_idx: 0,
            directories: vec![root],
            directory_idx_to_parent_idx: HashMap::from([(0, 0)]),
        }
    }

    fn cwd(&self) -> &DirectoryNode {
        &self.directories[self.cwd_idx]
    }

    fn cwd_mut(&mut self) -> &mut DirectoryNode {
        &mut self.directories[self.cwd_idx]
    }

    fn change_directory(&mut self, path: &str) -> Result<()> {
        match path {
            "/" => self.cwd_idx = 0,
            ".." => {
                self.cwd_idx = self.directory_idx_to_parent_idx[&self.cwd_idx];
            }
            _ => {
                let Some(idx) = self.child_directory_idx(path) else {
                    return Err(format!("tried to cd to unknown directory {}", path));
                };
                self.cwd_idx = idx;
            }
        }
        Ok(())
    }

    fn child_directory_idx(&self, path: &str) -> Option<usize> {
        self.cwd().contents.iter().find_map(|child| {
            let Node::Directory(directory_idx) = child else {
                return None;
            };
            let directory_node = &self.directories[*directory_idx];
            if directory_node.name == path {
                Some(*directory_idx)
            } else {
                None
            }
        })
    }

    fn mkdir(&mut self, path: &str) -> bool {
        let existing = self
            .child_directory_idx(path)
            .map(|idx| &self.directories[idx]);
        match existing {
            Some(_) => false,
            None => {
                let next_idx = self.directories.len();
                self.directory_idx_to_parent_idx
                    .insert(next_idx, self.cwd_idx);
                let new_directory = DirectoryNode::new(path);
                self.directories.push(new_directory);
                self.cwd_mut().contents.push(Node::Directory(next_idx));
                true
            }
        }
    }

    fn create_file(&mut self, name: &str, size: usize) -> bool {
        let existing_file = self.cwd().contents.iter().find_map(|child| {
            let Node::File(file_node) = child else {
                return None;
            };
            if file_node.name == name {
                Some(file_node)
            } else {
                None
            }
        });
        match existing_file {
            Some(_) => false,
            None => {
                let new_file_node = FileNode {
                    name: name.to_string(),
                    size,
                };
                self.cwd_mut().contents.push(Node::File(new_file_node));
                true
            }
        }
    }

    fn du(&self, directory: &DirectoryNode) -> usize {
        directory
            .contents
            .iter()
            .map(|node| match node {
                Node::Directory(d) => self.du(&self.directories[*d]),
                Node::File(FileNode { size, .. }) => *size,
            })
            .sum()
    }

    fn print_directory(
        &self,
        directory_idx: usize,
        indent_lvl: usize,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let directory_node = &self.directories[directory_idx];
        let name = &directory_node.name;
        writeln!(f, "{:indent$}- {name} (dir)", "", indent = indent_lvl * 2)?;
        for node in &directory_node.contents {
            match node {
                Node::Directory(child_idx) => {
                    self.print_directory(*child_idx, indent_lvl + 1, f)?
                }
                Node::File(file_node) => {
                    let name = &file_node.name;
                    let size = file_node.size;
                    writeln!(
                        f,
                        "{:indent$}- {name} (file, size={size})",
                        "",
                        indent = indent_lvl * 2 + 2
                    )?
                }
            }
        }
        Ok(())
    }
}

impl std::fmt::Display for FileSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.print_directory(0, 0, f)
    }
}

struct Commands(Vec<Command>);

impl FromStr for Commands {
    type Err = String;
    fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
        let Ok((_, commands)) = separated_list1(newline, parse_command)(input) else {
            return Err("Could not parse input".into())
        };
        Ok(Commands(commands))
    }
}

impl TryFrom<Commands> for FileSystem {
    type Error = String;
    fn try_from(commands: Commands) -> std::result::Result<Self, Self::Error> {
        let mut file_system = FileSystem::new();

        for command in commands.0 {
            match command {
                Command::ChangeDirectory(directory) => file_system.change_directory(&directory)?,
                Command::List(entries) => {
                    for entry in entries {
                        match entry {
                            ListOutputEntry::Directory(directory) => {
                                file_system.mkdir(&directory);
                            }
                            ListOutputEntry::File { name, size } => {
                                file_system.create_file(&name, size);
                            }
                        }
                    }
                }
            }
        }
        Ok(file_system)
    }
}

fn part1(input: &str) -> Result<usize> {
    let commands: Commands = input.parse()?;
    let file_system = FileSystem::try_from(commands)?;

    let total_size = file_system
        .directories
        .iter()
        .map(|directory| file_system.du(directory))
        .filter(|s| *s <= 100000)
        .sum();
    Ok(total_size)
}

fn part2(input: &str) -> Result<usize> {
    let commands: Commands = input.parse()?;
    let file_system = FileSystem::try_from(commands)?;

    let total_used = file_system.du(&file_system.directories[0]);
    let total_disk_space: usize = 70000000;
    let free_space = total_disk_space.saturating_sub(total_used);
    let needed_for_patch: usize = 30000000;
    let must_free_up = needed_for_patch.saturating_sub(free_space);

    let space_to_free_up = file_system
        .directories
        .iter()
        .map(|directory| file_system.du(directory))
        .filter(|s| *s >= must_free_up)
        .min()
        .unwrap_or_default();
    Ok(space_to_free_up)
}

#[derive(Debug, PartialEq)]
enum Command {
    List(Vec<ListOutputEntry>),
    ChangeDirectory(String),
}

#[derive(Debug, PartialEq)]
enum ListOutputEntry {
    Directory(String),
    File { name: String, size: usize },
}

fn parse_change_directory_command(input: &str) -> nom::IResult<&str, Command> {
    map(
        preceded(tag("cd "), alt((alpha1, tag("/"), tag("..")))),
        |d: &str| Command::ChangeDirectory(d.into()),
    )(input)
}

fn parse_ls_command(input: &str) -> nom::IResult<&str, Command> {
    let (input, _) = tag("ls")(input)?;
    let (input, entries) = opt(preceded(newline, parse_ls_output))(input)?;
    Ok((input, Command::List(entries.unwrap_or_default())))
}

fn parse_command(input: &str) -> nom::IResult<&str, Command> {
    preceded(
        tag("$ "),
        alt((parse_ls_command, parse_change_directory_command)),
    )(input)
}

fn parse_ls_directory_entry(input: &str) -> nom::IResult<&str, ListOutputEntry> {
    map(preceded(tag("dir "), alpha1), |name: &str| {
        ListOutputEntry::Directory(name.to_string())
    })(input)
}

fn parse_ls_file_entry(input: &str) -> nom::IResult<&str, ListOutputEntry> {
    map(
        separated_pair(
            map_res(digit1, |size: &str| size.parse()),
            space1,
            take_while(|c: char| c.is_alphanumeric() || c == '.'),
        ),
        |(size, name)| ListOutputEntry::File {
            name: name.to_string(),
            size,
        },
    )(input)
}

fn parse_ls_output(input: &str) -> nom::IResult<&str, Vec<ListOutputEntry>> {
    separated_list1(
        newline,
        alt((parse_ls_directory_entry, parse_ls_file_entry)),
    )(input)
}

#[cfg(test)]
mod tests {

    use super::*;
    const EXAMPLE_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), Ok(95437));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), Ok(24933642));
    }

    #[test]
    fn test_parse_ls_command() {
        assert_eq!(parse_command("$ ls"), Ok(("", Command::List(vec![]))));
        let ls_with_entries = "$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d";
        assert_eq!(
            parse_command(ls_with_entries),
            Ok((
                "",
                Command::List(vec![
                    ListOutputEntry::Directory("a".into()),
                    ListOutputEntry::File {
                        name: "b.txt".into(),
                        size: 14848514
                    },
                    ListOutputEntry::File {
                        name: "c.dat".into(),
                        size: 8504156
                    },
                    ListOutputEntry::Directory("d".into()),
                ])
            ))
        );
    }

    #[test]
    fn test_parse_change_directory_command() {
        assert_eq!(
            parse_command("$ cd foo"),
            Ok(("", Command::ChangeDirectory("foo".into())))
        );
        assert_eq!(
            parse_command("$ cd /"),
            Ok(("", Command::ChangeDirectory("/".into())))
        );
        assert_eq!(
            parse_command("$ cd .."),
            Ok(("", Command::ChangeDirectory("..".into())))
        );
    }

    #[test]
    fn test_parse_ls_output() {
        let input = "dir a
14848514 b.txt
8504156 c.dat
dir d";
        assert_eq!(
            parse_ls_output(input),
            Ok((
                "",
                vec![
                    ListOutputEntry::Directory("a".into()),
                    ListOutputEntry::File {
                        name: "b.txt".into(),
                        size: 14848514
                    },
                    ListOutputEntry::File {
                        name: "c.dat".into(),
                        size: 8504156
                    },
                    ListOutputEntry::Directory("d".into()),
                ]
            ))
        )
    }
}
