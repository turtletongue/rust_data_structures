use std::{cell::RefCell, rc::Rc, cmp::{Ord, Ordering}};

type OptionalNode<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T> {
  value: T,
  left_child: OptionalNode<T>,
  right_child: OptionalNode<T>,
}

impl<T> Node<T> {
  pub fn new(value: T) -> Self {
    Self {
      value,
      left_child: None,
      right_child: None,
    }
  }
}

#[derive(Debug)]
pub struct BinarySearchTree<T> {
  root: OptionalNode<T>,
}

impl<T: Ord> BinarySearchTree<T> {
  pub fn new() -> Self {
    Self {
      root: None,
    }
  }

  pub fn insert(&mut self, value: T) {
    let parent = self.find_free_parent(&value);

    if let None = parent {
      if let None = self.root {
        self.root = Self::create_optional_node(value);
      }

      return;
    }
    
    let mut parent_borrowed = parent.as_ref().unwrap().borrow_mut();
    let value_order = parent_borrowed.value.cmp(&value);

    let node = Self::create_optional_node(value);
    
    if value_order == Ordering::Greater {
      parent_borrowed.left_child = node;
    } else {
      parent_borrowed.right_child = node;
    };
  }

  pub fn find(&self, value: T) -> bool {
    self.root.is_some() && self.find_free_parent(&value).is_none()
  }

  fn find_free_parent(&self, value: &T) -> OptionalNode<T> {
    if let None = self.root {
      return None;
    }

    let mut current = Rc::clone(self.root.as_ref().unwrap());

    loop {
      current = {
        let node_borrowed = current.borrow();

        let next = match node_borrowed.value.cmp(&value) {
          Ordering::Greater => node_borrowed.left_child.as_ref(),
          Ordering::Less => node_borrowed.right_child.as_ref(),
          Ordering::Equal => return None,
        };

        if let None = next {
          break;
        }

        Rc::clone(next.unwrap())
      };
    }

    Some(current)
  }

  fn create_optional_node(value: T) -> OptionalNode<T> {
    Some(Rc::new(RefCell::new(Node::new(value))))
  }
}