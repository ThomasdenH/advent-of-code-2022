use std::{collections::HashMap, fmt::Debug, str::from_utf8};

struct FileTree<'a>(HashMap<Path<'a>, Directory<'a>>);

#[derive(Clone, Default, Hash, Eq, PartialEq)]
struct Path<'a>(Vec<&'a [u8]>);

impl<'a> Path<'a> {
    fn go_up(&mut self) {
        self.0.pop();
    }

    fn go_to_root(&mut self) {
        self.0.clear();
    }

    fn go_to_subdir(&mut self, subdir: &'a [u8]) {
        self.0.push(subdir);
    }
}

impl<'a> Debug for Path<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "/")?;
        for subdir in &self.0 {
            write!(f, "{}/", from_utf8(subdir).unwrap())?;
        }
        writeln!(f)
    }
}

#[derive(Default, Debug)]
struct Directory<'a> {
    size: usize,
    subdirs: Vec<&'a [u8]>,
}

#[derive(Debug)]
struct SizeInfo {
    sum_of_small_subdirectory_sizes: usize,
    total_size: usize,
}

impl<'a> FileTree<'a> {
    fn build(s: &'a str) -> Self {
        let mut map: HashMap<Path<'_>, Directory<'_>> = HashMap::new();
        let lines = s.split_terminator('\n').map(str::as_bytes);
        let mut current_path = Path::default();
        let mut directory_info = None;
        for line in lines {
            if line[0] == b'$' {
                // Command
                if line[2] == b'c' {
                    // $ cd
                    // Store the info for the current directory
                    if let Some(current_dirinfo) = directory_info.take() {
                        map.insert(current_path.clone(), current_dirinfo);
                    }

                    // Go to directory
                    if line[5] == b'.' {
                        // ..
                        current_path.go_up();
                    } else if line[5] == b'/' {
                        current_path.go_to_root();
                    } else {
                        current_path.go_to_subdir(&line[5..]);
                    }
                } else if line[2] == b'l' {
                    // $ ls
                    directory_info = Some(Directory::default());
                }
            } else if line[0] == b'd' {
                // dir
                let dirname = &line[4..];
                directory_info.as_mut().unwrap().subdirs.push(dirname);
            } else {
                directory_info.as_mut().unwrap().size += line
                    .iter()
                    .take_while(|b| **b != b' ')
                    .fold(0, |acc, d| acc * 10 + usize::from(d & 0b1111));
            }
        }
        if let Some(current_dirinfo) = directory_info.take() {
            map.insert(current_path.clone(), current_dirinfo);
        }
        FileTree(map)
    }

    fn total_size_of_small_directories(&self) -> usize {
        self.subdirectory_size_info(&mut Path::default())
            .sum_of_small_subdirectory_sizes
    }

    fn subdirectory_size_info(&self, path: &mut Path<'a>) -> SizeInfo {
        let dirinfo = self.0.get(path).unwrap();
        let mut size_info = SizeInfo {
            sum_of_small_subdirectory_sizes: 0,
            total_size: dirinfo.size,
        };
        for subdirectory in &dirinfo.subdirs {
            path.go_to_subdir(subdirectory);
            let subdir_size_info = self.subdirectory_size_info(path);
            size_info.total_size += subdir_size_info.total_size;
            size_info.sum_of_small_subdirectory_sizes +=
                subdir_size_info.sum_of_small_subdirectory_sizes;
            path.go_up();
        }
        if size_info.total_size <= 100_000 {
            size_info.sum_of_small_subdirectory_sizes += size_info.total_size;
        }
        size_info
    }

    fn total_size_used(&self) -> usize {
        self.subdirectory_size_info(&mut Path::default()).total_size
    }

    /// Returns the smallest subdirectory above threshold, or potentially itself
    fn smallest_subdirectory_above_threshold(
        &self,
        path: &mut Path<'a>,
        threshold: usize,
    ) -> Option<usize> {
        let dirinfo = self.0.get(path).unwrap();
        let mut size_current_directory = dirinfo.size;
        let mut smallest_subdir_size_above_threshold: Option<usize> = None;
        for subdir in &dirinfo.subdirs {
            path.go_to_subdir(subdir);
            size_current_directory += self.subdirectory_size_info(path).total_size;
            smallest_subdir_size_above_threshold = match (
                smallest_subdir_size_above_threshold,
                self.smallest_subdirectory_above_threshold(path, threshold),
            ) {
                (Some(a), Some(b)) => Some(a.min(b)),
                (a, None) => a,
                (None, b) => b,
            };
            path.go_up();
        }
        smallest_subdir_size_above_threshold.or({
            if size_current_directory > threshold {
                Some(size_current_directory)
            } else {
                None
            }
        })
    }
}

pub fn part_1(s: &str) -> usize {
    FileTree::build(s).total_size_of_small_directories()
}

pub fn part_2(s: &str) -> usize {
    let file_tree = FileTree::build(s);
    let size_needed = file_tree.total_size_used() + 30_000_000 - 70_000_000;
    file_tree
        .smallest_subdirectory_above_threshold(&mut Path::default(), size_needed)
        .unwrap_or_default()
}

#[test]
fn test_part_1_example() {
    let input = "$ cd /
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
7214296 k
";
    assert_eq!(part_1(input), 95437);
}

#[test]
fn test_part_2_example() {
    let input = "$ cd /
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
7214296 k
";
    assert_eq!(part_2(input), 24933642);
}

#[test]
fn test_part_1() {
    let input = include_str!("../input/2022/day7.txt");
    assert_eq!(part_1(input), 1334506);
}

#[test]
fn test_part_2() {
    let input = include_str!("../input/2022/day7.txt");
    assert_eq!(part_2(input), 7421137);
}
