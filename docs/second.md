# An Ok Singly-Linked Stack

[这里是原文](https://rust-unofficial.github.io/too-many-lists/second.html)

```rust
/// Link<T> 是一个 Option<Box<Node<T>>> 类型的别名，表示一个指向节点的指针
type Link<T> = Option<Box<Node<T>>>;

/// Node 结构体表示列表中的一个节点
struct Node<T> {
    /// elem 是一个泛型 T，表示节点中的值
    elem: T,
    /// next 是一个 Link<T>，指向下一个节点
    next: Link<T>,
}

/// List 结构体表示一个链表
pub struct List<T> {
    /// head 是一个 Link<T>，指向列表的第一个节点
    head: Link<T>,
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> List<T> {
    /// new 方法创建一个空列表
    pub fn new() -> Self {
        List { head: None }
    }

    /// push 方法将一个元素插入列表的头部
    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            // Option 的 take 方法将 Some(T) 变为 None 并返回 T，和 mem::replace 一样
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    /// pop 方法返回 Option<T>，允许我们在列表为空时返回 None
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    /// peek 方法返回一个引用，允许我们查看列表的第一个元素
    pub fn peek(&self) -> Option<&T> {
        // Option 的 as_ref 方法将 &Option<T> 变为 Option<&T>
        self.head.as_ref().map(|node| &node.elem)
    }

    /// peek_mut 方法返回一个可变引用，允许我们修改节点的值
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
}

impl<T> Drop for List<T> {
    /// 实现 Drop trait 以确保我们在列表被丢弃时释放所有节点
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter {
            // as_deref 方法将 Option<T> 变为 Option<&T>
            // 相当于 self.head.as_ref().map(|node| &**node)，先 match 再解引用
            next: self.head.as_deref(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            // as_deref 方法将 Option<T> 变为 Option<&T>
            // 相当于 self.head.as_ref().map(|node| &**node)，先 match 再解引用
            next: self.head.as_deref_mut(),
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use crate::second::List;

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

    #[test]
    fn peek() {
        let mut ls = List::new();
        assert_eq!(ls.peek(), None);
        assert_eq!(ls.peek_mut(), None);

        ls.push(1);
        ls.push(2);
        ls.push(3);

        assert_eq!(ls.peek(), Some(&3));
        assert_eq!(ls.peek_mut(), Some(&mut 3));

        // 这里 v 的类型是 &mut i32
        ls.peek_mut().map(|v| {
            *v = 4;
        });

        assert_eq!(ls.peek(), Some(&4));
        assert_eq!(ls.pop(), Some(4));
    }

    #[test]
    fn into_iter() {
        let mut ls = List::new();
        ls.push(1);
        ls.push(2);
        ls.push(3);

        let mut iter = ls.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut ls = List::new();
        ls.push(1);
        ls.push(2);
        ls.push(3);

        let mut iter = ls.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn iter_mut() {
        let mut ls = List::new();
        ls.push(1);
        ls.push(2);
        ls.push(3);

        let mut iter = ls.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), None);

        let mut iter = ls.iter_mut();
        iter.next().map(|v| {
            *v = *v * 2;
        });
        assert_eq!(ls.pop(), Some(6));
    }
}
```
