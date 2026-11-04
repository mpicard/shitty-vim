struct RopeNode {
    left: Option<Box<RopeNode>>,
    right: Option<Box<RopeNode>>,
    string: String,
    weight: usize,
}

impl RopeNode {
    fn new(left: Option<Box<RopeNode>>, right: Option<Box<RopeNode>>, string: &str) -> Self {
        let weight = left.as_deref().map_or(0, |left| left.length());

        Self {
            left,
            right,
            string: string.to_string(),
            weight,
        }
    }

    fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    fn length(&self) -> usize {
        if self.is_leaf() {
            self.string.chars().count()
        } else {
            self.weight + self.right.as_ref().map_or(0, |right| right.length())
        }
    }
}

struct Rope {
    root: Option<Box<RopeNode>>,
}

impl Rope {
    pub fn new(root: Option<Box<RopeNode>>) -> Self {
        Self { root }
    }

    pub fn from_string(string: &str) -> Self {
        Rope::new(Some(Box::new(RopeNode::new(None, None, string))))
    }

    pub fn length(&self) -> usize {
        match &self.root {
            Some(root) => root.length(),
            None => 0,
        }
    }

    pub fn char_at(&self, index: usize) -> Result<char, RopeError> {
        if index >= self.length() {
            return Err(RopeError::InvalidIndex);
        }
        Self::char_at_recursive(&self.root, index)
    }

    fn char_at_recursive(node: &Option<Box<RopeNode>>, index: usize) -> Result<char, RopeError> {
        match node {
            Some(node) if node.is_leaf() => node
                .string
                .chars()
                .nth(index)
                .ok_or(RopeError::InvalidIndex),
            Some(node) if index < node.weight => Self::char_at_recursive(&node.left, index),
            Some(node) => Self::char_at_recursive(&node.right, index - node.weight),
            None => Err(RopeError::InvalidIndex),
        }
    }

    pub fn concat(self, other: Rope) -> Rope {
        let left = self.root;
        let right = other.root;
        let weight = left.as_deref().map_or(0, |node| node.length());

        Rope::new(Some(Box::new(RopeNode {
            left,
            right,
            string: String::new(),
            weight,
        })))
    }

    pub fn to_string(&self) -> String {}
}

#[derive(Debug)]
enum RopeError {
    InvalidIndex,
}
