use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Mapping {
    // (line, column)
    pub generated: (usize, usize),
    pub source: Option<i32>,
    pub name: Option<i32>,
    pub original: Option<(usize, usize)>,
}

impl Mapping {
    pub fn from_tuple(input: (i32, usize, usize, usize, usize, Option<i32>)) -> Mapping {
        Mapping {
            generated: (input.1, input.2),
            source: Some(input.0),
            name: input.5,
            original: Some((input.3, input.4)),
        }
    }
}

impl Ord for Mapping {
    fn cmp(&self, other: &Mapping) -> Ordering {
        let cmp = self.generated.cmp(&other.generated);
        if cmp != Ordering::Equal {
            return cmp;
        }

        let cmp = self.source.cmp(&other.source);
        if cmp != Ordering::Equal {
            return cmp;
        }

        let cmp = self.original.cmp(&other.original);
        if cmp != Ordering::Equal {
            return cmp;
        }

        return self.name.cmp(&other.name);
    }
}

impl PartialOrd for Mapping {
    fn partial_cmp(&self, other: &Mapping) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
