use std::cmp::PartialEq;

#[derive(Debug)]
pub struct BST {
    root: Link,
    num_nodes: i32,
}

#[derive(Debug)]
struct Node {
    elem: i32,
    left: Link,
    right: Link,
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

impl Link {
    fn insert(&mut self, elem: i32) -> bool {
        use Link::*;
        match self {
            Empty => {
                let new_node = Box::new(Node {
                    elem,
                    left: Link::Empty,
                    right: Link::Empty,
                });
                *self = More(new_node);
                true
            }
            More(node) => {
                if node.elem > elem {
                    node.left.insert(elem)
                } else if node.elem < elem {
                    node.right.insert(elem)
                } else {
                    false
                }
            }
        }
    }

    fn search(&self, elem: i32) -> bool {
        use Link::*;

        match self {
            Empty => false,
            More(node) => {
                if node.elem > elem {
                    node.left.search(elem)
                } else if node.elem < elem {
                    node.right.search(elem)
                } else {
                    true
                }
            }
        }
    }
}

impl PartialEq for Link {
    fn eq(&self, other: &Link) -> bool {
        use Link::*;
        match (self, other) {
            (Empty, Empty) => true,
            (More(a), More(b)) => a.elem == b.elem,
            _ => false,
        }
    }
}

impl BST {
    pub fn new() -> Self {
        BST {
            root: Link::Empty,
            num_nodes: 0,
        }
    }

    pub fn insert(&mut self, elem: i32) -> bool {
        match self.root.insert(elem) {
            true => {
                self.num_nodes += 1;
                true
            }
            false => false,
        }
    }

    pub fn search(&self, elem: i32) -> bool {
        self.root.search(elem)
    }
}

fn main() {
    let mut bst = BST::new();
    bst.insert(12);
    bst.insert(17);
    bst.insert(29);
    bst.insert(2);
    bst.insert(8);
    bst.insert(90);
    println!("{:?}", bst);
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty() {
        let bst = BST::new();
        assert_eq!(bst.root, Link::Empty);
    }

    #[test]
    fn insert() {
        let mut bst = BST::new();
        assert_eq!(bst.insert(55), true);
        let new_node = Box::new(Node {
            elem: 55,
            left: Link::Empty,
            right: Link::Empty,
        });
        assert_eq!(bst.root, Link::More(new_node));
        assert_eq!(bst.insert(55), false);
        assert_eq!(bst.insert(22), true);
        assert_eq!(bst.insert(85), true);
        assert_eq!(bst.insert(80), true);
        match bst.root {
            Link::Empty => panic!("Should not be empty!"),
            Link::More(node) => {
                assert_eq!(node.elem, 55);
                match node.left {
                    Link::Empty => panic!("Should not be empty!"),
                    Link::More(val) => {
                        assert_eq!(val.elem, 22);
                    }
                }

                match node.right {
                    Link::Empty => panic!("Should not be empty!"),
                    Link::More(val) => {
                        assert_eq!(val.elem, 85);

                        match val.left {
                            Link::Empty => panic!("Should not be empty!"),
                            Link::More(nested) => {
                                assert_eq!(nested.elem, 80);
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn search() {
        let mut bst = BST::new();

        assert_eq!(bst.search(3), false);
        bst.insert(12);
        bst.insert(17);
        bst.insert(29);
        bst.insert(2);
        bst.insert(8);
        bst.insert(90);
        assert_eq!(bst.search(8), true);
    }
}
