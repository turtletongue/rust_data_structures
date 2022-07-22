use std::{cell::RefCell, rc::Rc, cmp::Ordering};

type OptionalAVLNode<T> = Option<Rc<RefCell<AVLNode<T>>>>;
type OptionalAVLNodeRef<'a, T> = Option<&'a Rc<RefCell<AVLNode<T>>>>;

enum Rotation {
  Left,
  Right,
  LeftRight,
  RightLeft,
}

struct AVLNode<T> {
  value: T,
  left_child: OptionalAVLNode<T>,
  right_child: OptionalAVLNode<T>, 
  height: i32,
}

impl<T> AVLNode<T> {
  pub fn new(value: T) -> Self {
    Self {
      value,
      left_child: None,
      right_child: None,
      height: 0,
    }
  }

  pub fn rotation(&self) -> Option<Rotation> {
    if self.is_left_heavy() {
      let balance_factor = self.left_child.as_ref().unwrap().borrow().balance_factor();
      
      return if balance_factor > 0 { Some(Rotation::Right) } else { Some(Rotation::LeftRight) };
    }
    
    if self.is_right_heavy() {
      let balance_factor = self.right_child.as_ref().unwrap().borrow().balance_factor();
      
      return if balance_factor <= 0 { Some(Rotation::Left) } else { Some(Rotation::RightLeft) };
    }
    
    None
  }
  
  pub fn height(node: OptionalAVLNodeRef<T>) -> i32 {
    if node.is_none() {
      return -1;
    }
    
    node.unwrap().borrow().height
  }
  
  pub fn update_height(&mut self) {
    let left_height = AVLNode::height(self.left_child.as_ref());
    let right_height = AVLNode::height(self.right_child.as_ref());
    
    self.height = left_height.max(right_height) + 1
  }
  
  pub fn create_optional(value: T) -> OptionalAVLNode<T> {
    Some(Rc::new(RefCell::new(Self::new(value))))
  }
  
  fn is_left_heavy(&self) -> bool {
    self.balance_factor() > 1
  }
  
  fn is_right_heavy(&self) -> bool {
    self.balance_factor() < -1
  }

  fn balance_factor(&self) -> i32 {
    Self::height(self.left_child.as_ref()) - Self::height(self.right_child.as_ref())
  }
}

pub struct AVLTree<T> {
  root: OptionalAVLNode<T>,
}

impl<T: Ord> AVLTree<T> {
  pub fn new() -> Self {
    Self { root: None }
  }

  pub fn insert(&mut self, value: T) {
    self.root = Self::insert_to_free_parent(self.root.as_ref(), value);
  }

  fn insert_to_free_parent(root: OptionalAVLNodeRef<T>, value: T) -> OptionalAVLNode<T> {
    if root.is_none() {
      return AVLNode::create_optional(value);
    }

    let root = root.unwrap();

    {
      let mut borrowed_root = root.borrow_mut();

      let compared_value = borrowed_root.value.cmp(&value);

      let next = {
        match compared_value {
          Ordering::Greater => borrowed_root.left_child.as_ref(),
          Ordering::Less => borrowed_root.right_child.as_ref(),
          Ordering::Equal => return Some(Rc::clone(root)),
        }
      };

      let node = Self::insert_to_free_parent(next, value);

      if compared_value == Ordering::Greater {
        borrowed_root.left_child = node;
      } else {
        borrowed_root.right_child = node;
      }

      borrowed_root.update_height();
    };

    Some(Self::balance(root))
  }

  fn balance(root: &Rc<RefCell<AVLNode<T>>>) -> Rc<RefCell<AVLNode<T>>> {
    let rotation = root.borrow().rotation();
    
    match rotation {
      Some(Rotation::Left) => Self::rotate_left(root),
      Some(Rotation::Right) => Self::rotate_right(root),
      Some(Rotation::LeftRight) => Self::rotate_left_right(root),
      Some(Rotation::RightLeft) => Self::rotate_right_left(root),
      None => Rc::clone(root),
    }
  }

  fn rotate_left_right(root: &Rc<RefCell<AVLNode<T>>>) -> Rc<RefCell<AVLNode<T>>> {
    {
      let mut borrowed_root = root.borrow_mut();

      borrowed_root.left_child = Some(Self::rotate_left(borrowed_root.left_child.as_ref().unwrap()));
    };

    Self::rotate_right(root)
  }

  fn rotate_right_left(root: &Rc<RefCell<AVLNode<T>>>) -> Rc<RefCell<AVLNode<T>>> {
    {
      let mut borrowed_root = root.borrow_mut();

      borrowed_root.right_child = Some(Self::rotate_right(borrowed_root.right_child.as_ref().unwrap()));
    };

    Self::rotate_left(root)
  }

  fn rotate_left(root: &Rc<RefCell<AVLNode<T>>>) -> Rc<RefCell<AVLNode<T>>> {
    let new_root = {
      let mut borrowed_root = root.borrow_mut();

      let new_root = borrowed_root.right_child.take().unwrap();
    
      borrowed_root.right_child = {
        let mut borrowed_new_root = new_root.borrow_mut();
  
        borrowed_new_root.left_child.take()
      };
  
      new_root.borrow_mut().left_child = Some(Rc::clone(root));

      new_root
    };

    root.borrow_mut().update_height();
    new_root.borrow_mut().update_height();

    new_root
  }

  fn rotate_right(root: &Rc<RefCell<AVLNode<T>>>) -> Rc<RefCell<AVLNode<T>>> {
    let new_root = {
      let mut borrowed_root = root.borrow_mut();

      let new_root = borrowed_root.left_child.take().unwrap();
     
      borrowed_root.left_child = {
        let mut borrowed_new_root = new_root.borrow_mut();
    
        borrowed_new_root.right_child.take()
      };
  
      new_root.borrow_mut().right_child = Some(Rc::clone(root));

      new_root
    };

    root.borrow_mut().update_height();
    new_root.borrow_mut().update_height();

    new_root
  }
}