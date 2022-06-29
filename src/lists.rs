use std::{fmt::Display, rc::Rc, cell::RefCell};

const NOT_FOUND_INDEX: isize = -1;

type OptionalNode<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T: Clone> {
  value: T,
  next: OptionalNode<T>
}

pub struct LinkedList<T: Clone> {
  head: OptionalNode<T>,
  tail: OptionalNode<T>,
  size: usize
}

impl <T: Clone> Node<T> {
  fn new(value: T, next: OptionalNode<T>) -> Self {
    Self { value, next }
  }
}

impl <T: PartialEq + Clone> LinkedList<T> {
  pub fn new() -> Self {
    Self {
      head: None,
      tail: None,
      size: 0,
    }
  }

  pub fn add_last(&mut self, value: T) {
    let node = Rc::new(RefCell::new(Node::new(value, None)));

    self.size += 1;

    if let None = &self.tail {
      self.tail = Some(Rc::clone(&node));
      self.head = Some(node);

      return;
    }

    self.tail = {
      let mut tail_borrowed = self.tail.as_ref().unwrap().borrow_mut();

      tail_borrowed.next = Some(node);

      Some(Rc::clone(tail_borrowed.next.as_ref().unwrap()))
    };
  }

  pub fn add_first(&mut self, value: T) {
    self.size += 1;

    let head = if let Some(head) = &self.head {
      Some(Rc::clone(head))
    } else {
      None
    };

    let node = Rc::new(RefCell::new(Node::new(value, head)));

    if let None = &self.tail {
      self.tail = Some(Rc::clone(&node));
    }

    self.head = Some(node);
  }

  pub fn remove_last(&mut self) -> Result<(), &'static str> {
    if self.size() == 1 {
      self.head = None;
      self.tail = None;

      self.size -= 1;

      return Ok(())
    }

    if let None = &self.head {
      return Err("List is empty");
    }

    let mut node_before_last = Rc::clone(&self.head.as_ref().unwrap());

    self.loop_items(&mut |item: &Rc<RefCell<Node<T>>>| {
      let item_borrowed = item.borrow();
      let next = item_borrowed.next.as_ref().unwrap();

      if let None = &next.borrow().next {
        node_before_last = Rc::clone(item);

        return false;
      };

      true
    });

    node_before_last.borrow_mut().next = None;
    self.size -= 1;

    Ok(())
  }

  pub fn remove_first(&mut self) -> Result<(), &'static str> {
    if self.size() == 1 {
      self.head = None;
      self.tail = None;

      self.size -= 1;

      return Ok(())
    }

    if let None = &self.head {
      return Err("List is empty");
    }

    self.head = {
      let borrowed_head = self.head.as_ref().unwrap().borrow();

      Some(Rc::clone(borrowed_head.next.as_ref().unwrap()))
    };

    self.size -= 1;

    Ok(())
  }

  pub fn contains(&self, value: T) -> bool {
    self.index_of(value) > NOT_FOUND_INDEX
  }

  pub fn index_of(&self, value: T) -> isize {
    let mut index = 0;
    let mut is_found = false;

    self.loop_items(&mut |item: &Rc<RefCell<Node<T>>>| {
      if item.borrow().value == value {
        is_found = true;

        return false;
      }

      index += 1;

      true
    });

    if is_found {
      index
    } else {
      NOT_FOUND_INDEX
    }
  }

  pub fn to_vec(&self) -> Vec<T> {
    let mut result = Vec::with_capacity(self.size);

    self.loop_items(&mut |item: &Rc<RefCell<Node<T>>>| {
      result.push(item.borrow().value.clone());

      true
    });

    result
  }

  pub fn size(&self) -> usize {
    self.size
  }

  fn loop_items<F>(&self, mut consumer: F) where F: FnMut(&Rc<RefCell<Node<T>>>) -> bool {
    if let Some(head) = &self.head {
      let mut current_node = Rc::clone(head);

      loop {
        current_node = {
          if !consumer(&current_node) {
            break;
          }

          let current_node_borrowed = current_node.borrow();

          if let Some(next) = &current_node_borrowed.next {
            Rc::clone(&next)
          } else {
            break;
          }
        }
      }
    }
  }
}

impl<T: Display + Clone + PartialEq> Display for LinkedList<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut formatted_items = String::new();

    self.loop_items(&mut |item: &Rc<RefCell<Node<T>>>| {
      let delimeter = if let Some(_) = item.borrow().next { ", " } else { "" };

      formatted_items.push_str(&(item.borrow().value.to_string() + delimeter));

      true
    });

    write!(f, "[{}]", formatted_items)?;

    Ok(())
  }
}