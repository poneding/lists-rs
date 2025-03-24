use std::mem;

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

pub struct List {
    head: Link,
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            // mem::replace 返回 self.head 的值并将其本身设置为 Link::Empty
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::first::List;

    #[test]
    fn basics() {
        let mut ls = List::new();

        ls.push(1);
        ls.push(2);
        ls.push(3);

        assert_eq!(ls.pop(), Some(3));
        assert_eq!(ls.pop(), Some(2));

        ls.push(4);
        ls.push(5);

        assert_eq!(ls.pop(), Some(5));
        assert_eq!(ls.pop(), Some(4));

        assert_eq!(ls.pop(), Some(1));

        assert_eq!(ls.pop(), None);
    }
}
