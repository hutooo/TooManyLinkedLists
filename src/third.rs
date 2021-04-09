use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        return List { head: None };
    }
    pub fn append(&self, elem: T) -> Self {
        return List {
            head: Some(Rc::new(Node {
                elem: elem,
                next: self.head.clone(),
            })),
        };
    }
    pub fn tail(&self) -> Self {
        return List {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        };
    }
    pub fn head(&self) -> Option<&T> {
        return self.head.as_ref().map(|node| &node.elem);
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        return Iter {
            next: self.head.as_deref(),
        };
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        return self.next.map(|node| {
            self.next = node.next.as_deref();
            return &node.elem;
        });
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn xxx() {
        let mut s1 = String::from("hello");
        let s2 = &mut s1;
        s2.push_str("abc");
        let s3 = &s2[..];
        print!("{}, {}\n", s2, s3);
    }

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.append(1).append(2).append(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn iter() {
        let list = List::new().append(1).append(2).append(3);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }
}
