type Key = usize;

struct Entry {
  key: Key,
  value: String
}

impl Entry {
  pub fn new(key: Key, value: String) -> Self {
    Self { key, value }
  }
}

pub struct HashTable {
  items: Vec<Option<Vec<Entry>>>,
  size: usize
}

impl HashTable {
  pub fn new(size: usize) -> Self {
    let mut items = Vec::with_capacity(size);

    for _ in 0..size {
      items.push(None);
    }

    Self { items, size }
  }

  pub fn put(&mut self, key: Key, value: String) {
    let hash = self.hash(key);

    if let None = self.items[hash] {
      self.items[hash] = Some(Vec::new());
    }

    let list = self.items[hash].as_mut().unwrap();

    let item = list.iter_mut().find(|entry| entry.key == key);

    if let Some(entry) = item {
      entry.value = value;
    } else {
      list.push(Entry::new(key, value));
    }
  }

  pub fn get(&self, key: Key) -> Option<String> {
    let hash = self.hash(key);

    let list = self.items[hash].as_ref();

    if let None = list {
      return None;
    }

    let item = list.unwrap().iter().find(|entry| entry.key == key);

    if let None = item {
      return None;
    }

    Some(item.unwrap().value.clone())
  }

  pub fn remove(&mut self, key: Key) {
    let hash = self.hash(key);

    let list = self.items[hash].as_mut();

    if let None = list {
      return;
    }

    list.unwrap().retain(|entry| entry.key != key);
  }

  fn hash(&self, key: Key) -> usize {
    key % self.size
  }
}