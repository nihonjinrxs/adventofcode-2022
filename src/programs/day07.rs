use crate::parts::Parts;

use camino::Utf8PathBuf;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::{all_consuming, map},
    sequence::{preceded, separated_pair},
    Finish, IResult,
};

pub fn run(part_number: Parts, input: &str) -> String {
    let result = compute_result(part_number, &input);
    format!("{}", result)
}

fn compute_result(part_number: Parts, input: &str) -> i32 {
    match part_number {
        Parts::One => directories_total_size(input),
        Parts::Two => directories_total_size(input),
    }
}

fn directories_total_size(input: &str) -> i32 {
    0
}

#[derive(Clone, Debug)]
struct FileSystemNode {
    path: Utf8PathBuf,
    size: u64,
    children: Vec<FileSystemNode>,
}

impl FileSystemNode {
    fn total_size(&self) -> u64 {
        self.size + self.children.iter().map(
            |child| child.total_size()
        ).sum::<u64>()
    }

    fn all_directories(&self) -> Box<dyn Iterator<Item = &FileSystemNode> + '_> {
        // Couldn't have figured this one out on my own. Found a nice solution by
        // fasterthanlime in this blog post:
        // https://fasterthanli.me/series/advent-of-code-2022/part-7
        Box::new(
            std::iter::once(self).chain(
                self.children
                    .iter()
                    .filter(|child| !child.children.is_empty())
                    .flat_map(|child| child.all_directories()),
            ),
        )
    }
}

#[derive(Debug)]
struct CD(Utf8PathBuf);

#[derive(Debug)]
struct LS;

#[derive(Debug)]
enum Command {
    LS(LS),
    CD(CD),
}

#[derive(Debug)]
enum FileSystemEntry {
    Directory(Utf8PathBuf),
    File(u64, Utf8PathBuf),
}

#[derive(Debug)]
enum ShellLogLine {
    Command(Command),
    FileSystemEntry(FileSystemEntry),
}

fn parse_entry(input: &str) -> IResult<&str, FileSystemEntry> {
    let parse_file = map(
        separated_pair(nom::character::complete::u64, tag(" "), parse_path_string),
        |(size, path)| FileSystemEntry::File(size, path),
    );
    let parse_directory = map(
        preceded(tag("dir "), parse_path_string),
        FileSystemEntry::Directory,
    );

    alt((parse_file, parse_directory))(input)
}

fn parse_path_string(input: &str) -> IResult<&str, Utf8PathBuf> {
    map(
        take_while1(|ch: char| "/.abcdefghijklmnopqrstuvwxyz".contains(ch)),
        Into::into,
    )(input)
}

fn parse_cd(input: &str) -> IResult<&str, CD> {
    map(preceded(tag("cd"), parse_path_string), CD)(input)
}

fn parse_ls(input: &str) -> IResult<&str, LS> {
    map(tag("ls"), |_| LS)(input)
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("$ ")(input)?;
    alt((map(parse_ls, Command::LS), map(parse_cd, Command::CD)))(input)
}

fn parse_line(input: &str) -> IResult<&str, ShellLogLine> {
    alt((
        map(parse_command, ShellLogLine::Command),
        map(parse_entry, ShellLogLine::FileSystemEntry),
    ))(input)
}

fn parse_input_to_file_system(input: &str) -> impl Iterator<Item = ShellLogLine> {
    let lines = input
        .lines()
        .map(|line| all_consuming(parse_line)(line).finish().unwrap().1);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::TestCase;

    use std::fs;

    fn fixture_directory_tree() -> FileSystemNode {
        let mut test_root = FileSystemNode::create_dir("/", None);
        let mut a = FileSystemNode::create_dir("a", test_root);
        if let FileSystemNodeType::DirectoryNode(mut children) = test_root.kind {
            children.push(a);
        }
        let mut e = FileSystemNode::create_dir("e", a);
        let mut i = FileSystemNode::create_file("i", 584, e);
        let f = FileSystemNode::create_file("f", 29116, a);
        let g = FileSystemNode::create_file("g", 2557, a);
        let h_lst = FileSystemNode::create_file("h.lst", 62596, a);
        let b_txt = FileSystemNode::create_file("b.txt", 14848514, test_root);
        let c_dat = FileSystemRoot::create_file("c.dat", 8504156, test_root);
        let d = FileSystemNode::create_dir("d", test_root);
        let j = FileSystemNode::create_file("j", 4060174, d);
        let d_log = FileSystemNode::create_file("d.log", 8033020, d);
        let d_ext = FileSystemNode::create_file("d.ext", 5626152, d);
        let k = FileSystemNode::create_file("k", 7214296, d);

        test_root
    }

    #[test]
    fn test_file_system_node_size_with_directory() {
        let test_input = fixture_directory_tree();
        assert_eq!(test_input.size(), 48381165);
        let root_children = test_input.children().unwrap();
        let a = root_children
            .iter()
            .find(|child| child.name == String::from("a"))
            .unwrap();
        assert_eq!(a.size(), 94853);
        let d = root_children
            .iter()
            .find(|child| child.name == String::from("d"))
            .unwrap();
        assert_eq!(d.size(), 24933642);
    }

    #[test]
    fn test_file_system_node_size_with_file() {
        let test_input = fixture_directory_tree();
        let root_children = test_input.children().unwrap();
        let b_txt = root_children
            .iter()
            .find(|child| child.name == String::from("b.txt"))
            .unwrap();
        assert_eq!(b_txt.size(), 14848514);
        let c_dat = root_children
            .iter()
            .find(|child| child.name == String::from("c.dat"))
            .unwrap();
        assert_eq!(c_dat.size(), 8504156);
    }
}
