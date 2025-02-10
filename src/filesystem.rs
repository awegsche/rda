use std::fmt::{Debug, Formatter};

#[derive(Debug)]
pub struct RDAFileSystem {
    root: RDANode
}

#[derive(Debug)]
pub enum RDANode {
    File(RDAFile),
    Directory(RDADirectory),
}

pub struct RDAFile {
    filename: String,
    content: Vec<u8>
}

const KIB: usize = 1024;
const MIB: usize = KIB*KIB;
const GIB: usize = KIB*MIB;
fn format_length(length: usize) -> String {
    if length > GIB {
        format!("{:.3} GIB", length as f32 / GIB as f32)
    }
    else if length > MIB {
        format!("{:.3} MIB", length as f32 / MIB as f32)
    }
    else if length > KIB {
        format!("{:.3} KIB", length as f32 / KIB as f32)
    }
    else {
        format!("{} B", length)
    }
}

impl Debug for RDAFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{}]", self.filename, format_length(self.content.len()))
    }
}

#[derive(Debug)]
pub struct RDADirectory {
    filename: String,
    nodes: Vec<RDANode>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_filesystem() {
        let fs = RDAFileSystem {
            root: RDANode::Directory(
                RDADirectory {
                    filename: "/".to_string(),
                    nodes: vec![
                        RDANode::File(RDAFile{
                            filename: "testfile.txt".to_string(),
                            content: b"hello, world".to_vec()
                        })
                    ]
                }
            )
        };

        assert_eq!(format!("{:?}", fs),
            "RDAFileSystem { root: Directory(RDADirectory { filename: \"/\", nodes: [File(testfile.txt [12 B])] }) }"
        )
    }
}