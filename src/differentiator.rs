use file_handler::LineAwareFile;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Differ<'a> {
    left: &'a LineAwareFile,
    right: &'a LineAwareFile,
}

impl<'a> Differ<'a> {
    pub fn new(left: &'a LineAwareFile, right: &'a LineAwareFile) -> Differ<'a> {
        Differ { left, right }
    }

    pub fn diff(&self) -> Changes {

        let additions= self.filter_changed_values(Direction::LeftToRight);
        let deletions= self.filter_changed_values(Direction::RightToLeft);

        Changes::new(additions, deletions)
    }

    fn filter_changed_values(&self, direction: Direction) -> LineAwareFile {
        let (subject, object) = match direction {
            Direction::LeftToRight => (self.left,self.right),
            Direction::RightToLeft => (self.right, self.left)
        };

        let values: HashMap<usize, String> = object.clone().contents.into_iter()
            .filter(|&(ref index, ref row) | {
                let value = subject.contents.get(&index);
                match value {
                    Some(val) if *val == *row => false,
                    _ => true
                }
            })
            .collect();

        LineAwareFile::from(values.clone())
    }
}

#[derive(Clone, Debug)]
pub struct Changes {
    additions: LineAwareFile,
    deletions: LineAwareFile,
}

impl Changes {
    pub fn new(additions: LineAwareFile, deletions: LineAwareFile) -> Changes {
        Changes { additions, deletions }
    }
}

enum Direction {
    LeftToRight,
    RightToLeft
}

#[test]
pub fn test_diff() {
    use std::collections::HashMap;
    let left_contents: HashMap<usize, String> = [(1, "foo".to_string()), (2, "baz".to_string())].iter().cloned().collect();
    let right_contents: HashMap<usize, String> = [(1, "foo".to_string()), (2,"bar".to_string())].iter().cloned().collect();
    let differenciator = Differ { left: &LineAwareFile::from(left_contents), right: &LineAwareFile::from(right_contents)};
    let diff = differenciator.diff();
    let expected_additions: HashMap<usize, String> = [(2, "bar".to_string())].iter().cloned().collect();
    let expected_deletions: HashMap<usize, String> = [(2, "baz".to_string())].iter().cloned().collect();
    assert_eq!(diff.additions.contents, expected_additions);
    assert_eq!(diff.deletions.contents, expected_deletions);
}