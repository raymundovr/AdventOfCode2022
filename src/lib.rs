use std::fmt::Display;
use std::cmp::Ordering;

#[derive(Debug, Eq)]
pub struct Directory {
    pub index: usize,
    pub name: String,
    pub size: usize,
    pub parent: usize,
}

impl Directory {
    pub fn update_size(&mut self, size_add: usize) {
        self.size += size_add;
    }   
}

impl Display for Directory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "- {} (size={})", self.name, self.size)
    }
}

impl Ord for Directory {
    fn cmp(&self, other: &Self) -> Ordering {
        self.size.cmp(&other.size)
    }
}

impl PartialOrd for Directory {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Directory {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size
    }
}