use file_handler::LineAwareFile;
use std::fmt::Debug;
use std::collections::HashMap;

pub struct Differenciator {
    left: LineAwareFile,
    right: LineAwareFile,
}

impl Differenciator {
    pub fn new(left: LineAwareFile, right: LineAwareFile) -> Differenciator {
        Differenciator { left, right }
    }

    pub fn diff(self) -> Changes {
        let left = self.left;
        let right = self.right;

        let additions = LineAwareFile::new();
        let deletions = LineAwareFile::new();

        let changed_rows: HashMap<usize, String> = right.contents.into_iter()
            .filter(|&(ref right_index, ref row) | {
                let left_value = left.contents.get(&right_index);
                match left_value {
                    Some(val) if *val == *row => false,
                    _ => true
                }
            })
        .collect();

        Changes::new(additions, deletions)
    }
}

#[derive(Clone, Debug)]
struct Changes {
    additions: LineAwareFile,
    deletions: LineAwareFile,
}

impl Changes {
    pub fn new(additions: LineAwareFile, deletions: LineAwareFile) -> Changes {
        Changes { additions, deletions }
    }
}

#[test]
pub fn test_diff() {
    use std::collections::HashMap;
    let left_contents: HashMap<usize, String> = [(1, "foo".to_string())].iter().cloned().collect();
    let right_contents: HashMap<usize, String> = [(1, "foo".to_string()), (2,"bar".to_string())].iter().cloned().collect();
    let differenciator = Differenciator { left: LineAwareFile::from(left_contents), right: LineAwareFile::from(right_contents)};
    let diff = differenciator.diff();

}