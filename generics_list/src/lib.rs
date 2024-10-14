#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {List {head: None}}

    pub fn len(&self) -> usize {
        let mut length = 0;
        let mut current = self.head.as_ref();
        while let Some(node) = current {length += 1; current = node.next.as_deref()}
        length
    }

    pub fn pop(&mut self) {
        if self.head.is_none() {return}
        let mut current = self.head.take();
        if let Some(node) = current {current = if !node.next.is_none() {Some(*node.next.unwrap())} else {None}}
        self.head = current.take();
    }

    pub fn push(&mut self, value:T) {
        self.head = Some(Node{
            value: value,
            next: if self.head.is_some() {Some(Box::new(self.head.take().unwrap()))} else {None},
        })
    }
}
