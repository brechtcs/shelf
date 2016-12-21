use std::io::prelude::*;
use std::fs;

pub struct Shelf {
  dir: String
}

impl Shelf {
  fn new(path: &str) -> Shelf {
    fs::DirBuilder::new()
      .recursive(true)
      .create(path)
      .unwrap();

    Shelf {
      dir: path.to_string()
    }
  }

  fn set(&self, key: &str, value: &str) -> () {
    let mut path = String::new();
    path.push_str(&self.dir);
    path.push_str("/");
    path.push_str(key);

    let mut entry = fs::File::create(path).unwrap();
    entry.write(value.as_bytes()).unwrap();
  }

  fn get(&self, key: &str) -> String {
    let mut value = String::new();
    let mut path = String::new();
    path.push_str(&self.dir);
    path.push_str("/");
    path.push_str(key);

    match fs::File::open(path) {
      Err(_) => value,
      Ok(mut entry) => {
        entry.read_to_string(&mut value).unwrap();
        value
      }
    }
  }

  fn keys(&self) -> Vec<String> {
    let mut keys = Vec::new();

    for entry in fs::read_dir(&self.dir).unwrap() {
      let key = entry.unwrap().file_name().into_string().unwrap();
      keys.push(key);
    }
    keys
  }

  fn values(&self) -> Vec<String> {
    let mut values = Vec::new();

    for entry in fs::read_dir(&self.dir).unwrap() {
      let value = self.get(&entry.unwrap().file_name().into_string().unwrap());
      values.push(value);
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

    assert_eq!("Some data", shelf.get("test"));
    assert_eq!("Some more", shelf.get("other"));
    assert_eq!(2, shelf.keys().len());
    assert_eq!(2, shelf.values().len());
  }
}
