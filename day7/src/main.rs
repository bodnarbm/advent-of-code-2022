use std::{
    cell::RefCell,
    fmt::Debug,
    io,
    rc::{Rc, Weak},
    str::FromStr,
};

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

struct FileTree {
    root: Rc<RefCell<Directory>>,
    cwd: Weak<RefCell<Directory>>,
}

impl FileTree {
    fn new() -> Self {
        let root = Rc::new(RefCell::new(Directory::root("/")));
        let cwd = Rc::downgrade(&root);
        Self { root, cwd }
    }
    fn change_directory(&mut self, path: &str) {
        match path {
            "/" => self.cwd = Rc::downgrade(&self.root),
            ".." => {
                let Some(strong) = self.cwd.upgrade() else {
                    return;
                };
                let borrowed = strong.borrow_mut();
                let Some(parent) = borrowed.parent.clone() else {
                    return;
                };
                self.cwd = parent
            }
            _ => {
                let Some(strong) = self.cwd.upgrade() else {
                    return;
                };
                for child in &strong.as_ref().borrow().contents {
                    if let FileType::Directory(child) = child {
                        if child.as_ref().borrow().name == path {
                            self.cwd = Rc::downgrade(child)
                        }
                    }
                }
            }
        }
    }

    fn mkdir(&mut self, name: impl Into<String>) {
        let Some(cwd) = self.cwd.upgrade() else {
            return;
        };
        let new_directory = Directory::new(name, &cwd);
        cwd.borrow_mut()
            .contents
            .push(FileType::Directory(Rc::new(RefCell::new(new_directory))));
    }

    fn touch(&mut self, name: impl Into<String>, size: usize) {
        let Some(cwd) = self.cwd.upgrade() else {
            return;
        };
        let new_file = File::new(name, size);
        cwd.borrow_mut().contents.push(FileType::File(new_file));
    }

    fn directories(&self) -> DirectoryIterator {
        DirectoryIterator {
            stack: vec![self.root.clone()],
        }
    }

    fn total_size(&self) -> usize {
        FileType::Directory(self.root.clone()).disk_size()
    }
}

struct DirectoryIterator {
    stack: Vec<Rc<RefCell<Directory>>>,
}

impl Iterator for DirectoryIterator {
    type Item = FileType;
    fn next(&mut self) -> Option<Self::Item> {
        let Some(next) = self.stack.pop() else {
            return None;
        };

        for child in &next.as_ref().borrow().contents {
            if let FileType::Directory(child) = child {
                self.stack.push(child.clone())
            }
        }
        Some(FileType::Directory(next))
    }
}

struct Directory {
    name: String,
    parent: Option<Weak<RefCell<Directory>>>,
    contents: Vec<FileType>,
}

impl Directory {
    fn root(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            parent: None,
            contents: vec![],
        }
    }

    fn new(name: impl Into<String>, parent: &Rc<RefCell<Directory>>) -> Self {
        let parent = Rc::downgrade(parent);
        Self {
            name: name.into(),
            parent: Some(parent),
            contents: vec![],
        }
    }
}

struct File {
    _name: String,
    size: usize,
}

impl File {
    fn new(name: impl Into<String>, size: usize) -> Self {
        Self {
            _name: name.into(),
            size,
        }
    }
}

enum FileType {
    File(File),
    Directory(Rc<RefCell<Directory>>),
}

impl FileType {
    fn disk_size(&self) -> usize {
        match self {
            Self::Directory(directory) => directory
                .as_ref()
                .borrow()
                .contents
                .iter()
                .map(|c| c.disk_size())
                .sum(),
            Self::File(file) => file.size,
        }
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

impl From<Commands> for FileTree {
    fn from(commands: Commands) -> Self {
        let mut file_system = FileTree::new();

        for command in commands.0 {
            match command {
                Command::ChangeDirectory(directory) => file_system.change_directory(&directory),
                Command::List(entries) => {
                    for entry in entries {
                        match entry {
                            ListOutputEntry::Directory(directory) => {
                                file_system.mkdir(&directory);
                            }
                            ListOutputEntry::File { name, size } => file_system.touch(name, size),
                        }
                    }
                }
            }
        }
        file_system
    }
}

fn part1(input: &str) -> Result<usize> {
    let commands: Commands = input.parse()?;
    let file_system = FileTree::from(commands);

    let total_size = file_system
        .directories()
        .map(|directory| directory.disk_size())
        .filter(|s| *s <= 100000)
        .sum();
    Ok(total_size)
}

fn part2(input: &str) -> Result<usize> {
    let commands: Commands = input.parse()?;
    let file_system = FileTree::from(commands);

    let total_used = file_system.total_size();
    let total_disk_space: usize = 70000000;
    let free_space = total_disk_space.saturating_sub(total_used);
    let needed_for_patch: usize = 30000000;
    let must_free_up = needed_for_patch.saturating_sub(free_space);

    let space_to_free_up = file_system
        .directories()
        .map(|directory| directory.disk_size())
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
