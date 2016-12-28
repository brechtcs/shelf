use std::io::prelude::*;
use std::fs;

pub struct Shelf {
  dir: String
}

impl Shelf {
  pub fn new(path: &str) -> Shelf {
    fs::DirBuilder::new()
      .recursive(true)
      .create(path)
      .unwrap();

    Shelf {
      dir: path.to_string()
    }
  }

  pub fn set(&self, key: &str, value: &str) -> () {
    let mut path = String::new();
    path.push_str(&self.dir);
    path.push_str("/");
    path.push_str(key);

    let mut entry = fs::File::create(path).unwrap();
    entry.write(value.as_bytes()).unwrap();
  }

  pub fn get(&self, key: &str) -> Option<String> {
    let mut value = String::new();
    let mut path = String::new();
    path.push_str(&self.dir);
    path.push_str("/");
    path.push_str(key);

    match fs::File::open(path) {
      Err(_) => None,
      Ok(mut entry) => {
        entry.read_to_string(&mut value).unwrap();
        Some(value)
      }
    }
  }

  pub fn keys(&self) -> Vec<String> {
    let mut keys = Vec::new();

    for entry in fs::read_dir(&self.dir).unwrap() {
      let key = entry.unwrap().file_name().into_string().unwrap();
      keys.push(key);
    }
    keys
  }

  pub fn values(&self) -> Vec<String> {
    let mut values = Vec::new();

    for entry in fs::read_dir(&self.dir).unwrap() {
      match self.get(&entry.unwrap().file_name().into_string().unwrap()) {
        Some(value) => values.push(value),
        None => ()
      }
    }
    values
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn basic() {
    let shelf = Shelf::new("/tmp/nested/folders");
    shelf.set("test", "Some data");
    shelf.set("other", "Some more");

    assert_eq!("Some data", shelf.get("test").unwrap());
    assert_eq!("Some more", shelf.get("other").unwrap());
    assert_eq!(2, shelf.keys().len());
    assert_eq!(2, shelf.values().len());
  }
}
