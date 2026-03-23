/// Linked list node for storing numbers and operators in expressions
#[derive(Debug, Clone)]
pub struct Node {
    pub value: String,
    pub next: Option<Box<Node>>,
}

impl Node {
    pub fn new(value: String) -> Self {
        Node { value, next: None }
    }
}

/// Linked list structure for storing expressions
pub struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn push_back(&mut self, value: String) {
        let new_node = Box::new(Node::new(value));
        match self.head {
            None => {
                self.head = Some(new_node);
            }
            Some(ref mut current) => {
                let mut ptr = current.as_mut();
                while let Some(ref mut next) = ptr.next {
                    ptr = next.as_mut();
                }
                ptr.next = Some(new_node);
            }
        }
    }

    pub fn to_vec(&self) -> Vec<String> {
        let mut result = Vec::new();
        let mut current = &self.head;
        while let Some(node) = current {
            result.push(node.value.clone());
            current = &node.next;
        }
        result
    }
}
