use std::fmt::{Debug, Formatter};

#[derive(Debug)]
pub struct RDAFileSystem {
    root: RDANode,
}

#[derive(Debug)]
pub enum RDANode {
    File(RDAFile),
    Directory(RDADirectory),
    Empty,
}

pub struct RDAFile {
    filename: String,
    content: Vec<u8>,
}

const KIB: usize = 1024;
const MIB: usize = KIB * KIB;
const GIB: usize = KIB * MIB;

const EMPTY_NAME: &'static str = "EMPTY";

fn format_length(length: usize) -> String {
    if length > GIB {
        format!("{:.3} GIB", length as f32 / GIB as f32)
    } else if length > MIB {
        format!("{:.3} MIB", length as f32 / MIB as f32)
    } else if length > KIB {
        format!("{:.3} KIB", length as f32 / KIB as f32)
    } else {
        format!("{} B", length)
    }
}

impl Debug for RDAFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} [{}]",
            self.filename,
            format_length(self.content.len())
        )
    }
}

#[derive(Debug)]
pub struct RDADirectory {
    filename: String,
    nodes: Vec<RDANode>,
}

impl RDANode {
    pub fn name(&self) -> &str {
        match self {
            RDANode::Directory(dir) => &dir.filename,
            RDANode::File(file) => &file.filename,
            RDANode::Empty => EMPTY_NAME,
        }
    }
}

impl Default for RDAFileSystem {
    fn default() -> Self {
        RDAFileSystem {
            root: RDANode::Empty,
        }
    }
}

impl RDAFileSystem {
    pub fn push(&mut self, path: &[&str], node: RDANode) {
        match self.root {
            RDANode::Directory(ref mut dir) => dir.push(path, node),
            _ => panic!("Root is not a directory"),
        }
    }
}

impl RDADirectory {
    pub fn push(&mut self, path: &[&str], node: RDANode) {
        if path.len() == 0 {
            self.nodes.push(node);
        } else {
            match self.nodes.iter_mut().find(|child| child.name() == path[0]) {
                Some(childnode) => match childnode {
                    RDANode::Directory(dir) => dir.push(&path[1..], node),
                    _ => panic!("Node is not a directory"),
                },
                None => {
                    let mut new_dir = RDADirectory {
                        filename: path[0].to_string(),
                        nodes: vec![],
                    };
                    new_dir.push(&path[1..], node);
                    self.nodes.push(RDANode::Directory(new_dir));
                }
            }
        }
    }
    pub fn get_mut(&mut self, path: &[&str]) -> Option<&mut RDANode> {
        match self.nodes.iter_mut().find(|child| child.name() == path[0]) {
            Some(node) => {
                if path.len() == 1 {
                    Some(node)
                } else {
                    match node {
                        RDANode::Directory(dir) => dir.get_mut(&path[1..]),
                        _ => None,
                    }
                }
            }
            None => None,
        }
    }
}

impl std::fmt::Display for RDADirectory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [ ", self.filename)?;
        for node in self.nodes.iter() {
            write!(f, "{}, ", node)?;
        }
        write!(f, "]")
    }
}

impl std::fmt::Display for RDAFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::fmt::Display for RDANode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RDANode::Empty => write!(f, "Empty"),
            RDANode::File(file) => write!(f, "{}", file),
            RDANode::Directory(dir) => write!(f, "{}", dir)
        }
    }
}

impl std::fmt::Display for RDAFileSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RDAFileSystem")?;

        match &self.root {
            RDANode::Directory(dir) => {
                writeln!(f, "")?;
                for node in dir.nodes.iter() {
                    writeln!(f, "{}", node)?;
                }
            }
            RDANode::File(file) => write!(f, "{}", file)?,
            RDANode::Empty => write!(f, "Empty")?
        }
        Ok(())
    }

    }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_filesystem() {
        let fs = RDAFileSystem {
            root: RDANode::Directory(RDADirectory {
                filename: "/".to_string(),
                nodes: vec![RDANode::File(RDAFile {
                    filename: "testfile.txt".to_string(),
                    content: b"hello, world".to_vec(),
                })],
            }),
        };

        assert_eq!(format!("{:?}", fs),
            "RDAFileSystem { root: Directory(RDADirectory { filename: \"/\", nodes: [File(testfile.txt [12 B])] }) }"
        )
    }

    #[test]
    fn push_dir() {
        let mut fs = RDAFileSystem {
            root: RDANode::Directory(RDADirectory {
                filename: "/".to_string(),
                nodes: vec![RDANode::File(RDAFile {
                    filename: "testfile.txt".to_string(),
                    content: b"hello, world".to_vec(),
                })],
            }),
        };

        fs.push(&["test"], RDANode::Directory(RDADirectory {
            filename: "dir".to_string(), nodes: vec![]
        }));

        assert_eq!(format!("{}", fs),
            "RDAFileSystem\n\
            testfile.txt [12 B]\n\
            test [ dir [ ], ]\n"
        )
    }
}
