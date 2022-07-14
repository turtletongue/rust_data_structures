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

impl<T: Clone> Node<T> {
  fn new(value: T, next: OptionalNode<T>) -> Self {
    Self { value, next }
  }

  fn get_next(&self) -> OptionalNode<T> {
    if let Some(node) = &self.next {
      Some(Rc::clone(node))
    } else {
      None
    }
  }

  fn set_next(&mut self, next: OptionalNode<T>) {
    self.next = if let Some(next) = &next {
      Some(Rc::clone(next))
    } else {
      None
    };
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

    if self.is_empty() {
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

    if self.is_empty() {
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

  pub fn reverse(&mut self) {
    if self.is_empty() {
      return;
    }

    let mut current = Rc::clone(self.head.as_ref().unwrap());
    let mut previous = None;

    loop {
      current = {
        let next = current.borrow().get_next();

        current.borrow_mut().set_next(previous);

        if next.is_none() {
          break;
        }

        previous = Some(Rc::clone(&current));

        Rc::clone(next.as_ref().unwrap())
      };
    }

    let head = Rc::clone(self.head.as_ref().unwrap());
    let tail = Rc::clone(self.tail.as_ref().unwrap());

    self.tail = Some(head);
    self.head = Some(tail);
  }

  pub fn get_kth_from_end(&self, k: usize) -> Result<T, &'static str> {
    if k <= 0 {
      return Err("K must be greater than zero");
    }

    if k > self.size() {
      return Err("K must be less than or equal to size of the list");
    }

    let mut result = Rc::clone(self.head.as_ref().unwrap());
    let mut end = Rc::clone(&result);

    let distance = k - 1;

    for _ in 0..distance {
      end = {
        let end_borrowed = end.borrow();

        end_borrowed.get_next().unwrap()
      };
    }

    self.loop_items(&mut |item: &Rc<RefCell<Node<T>>>| {
      end = {
        let end_borrowed = end.borrow();

        if end_borrowed.next.is_none() {
          result = Rc::clone(item);

          return false;
        }

        end_borrowed.get_next().unwrap()
      };

      true
    });

    let result_borrowed = result.borrow();

    Ok(result_borrowed.value.clone())
  }

  pub fn size(&self) -> usize {
    self.size
  }

  pub fn is_empty(&self) -> bool {
    self.head.is_none()
  }

  fn loop_items<F>(&self, mut consumer: F) where F: FnMut(&Rc<RefCell<Node<T>>>) -> bool {
    if let Some(head) = &self.head {
      let mut current = Rc::clone(head);

      loop {
        current = {
          if !consumer(&current) {
            break;
          }

          let current_borrowed = current.borrow();

          if let Some(next) = &current_borrowed.next {
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

impl<T: Clone> Drop for LinkedList<T> {
  fn drop(&mut self) {
    if let Some(mut child) = self.head.take() {
      loop {
        child = {
          let mut child_borrowed = child.borrow_mut();

          if let Some(next) = child_borrowed.next.take() {
            next
          } else {
            break;
          }
        };
      }
    }
  }
}
