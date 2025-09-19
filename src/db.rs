use alloc::string::String;
use heapless::index_map::FnvIndexMap as HashMap;

const MAX_SIZE: usize = 1024;

pub struct Seidr<T> {
    data: HashMap<String, T, MAX_SIZE>,
}

impl<T> Default for Seidr<T> {
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}

pub trait KeyValueStore<T> {
    fn insert(&mut self, key: String, value: T) -> Result<Option<T>, String>;
    fn get(&self, key: &str) -> Option<&T>;
    fn remove(&mut self, key: &str) -> Option<T>;
}

impl<T> KeyValueStore<T> for Seidr<T> {
    fn insert(&mut self, key: String, value: T) -> Result<Option<T>, String> {
        self.data
            .insert(key, value)
            .map_err(|_| String::from("Max capacity reached"))
    }

    fn get(&self, key: &str) -> Option<&T> {
        self.data.get(key)
    }

    fn remove(&mut self, key: &str) -> Option<T> {
        self.data.remove(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut db = Seidr::default();
        db.insert("foo".to_owned(), "bar").unwrap();
        assert_eq!(db.get("foo"), Some(&"bar"));
    }

    #[test]
    fn test_remove() {
        let mut db = Seidr::default();
        db.insert("foo".to_owned(), "bar").unwrap();
        assert_eq!(db.remove("foo"), Some("bar"));
        assert_eq!(db.get("foo"), None);
    }
}
