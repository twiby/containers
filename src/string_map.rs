use std::ops::Index;
use std::ops::IndexMut;

/// This is meant to replace a HashMap<String, T> in every way
/// It is a horrible idea and performs much worse in almost every case
pub struct StringMap<T> {
    keys: Vec<String>,
    values: Vec<T>,
}

impl<T> Default for StringMap<T> {
    fn default() -> Self {
        Self {
            keys: vec![],
            values: vec![],
        }
    }
}

impl<T> StringMap<T> {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn keys(&self) -> &[String] {
        &self.keys
    }

    #[inline]
    pub fn values(&self) -> &[T] {
        &self.values
    }

    #[inline]
    pub fn len(&self) -> usize {
        debug_assert_eq!(self.keys.len(), self.values.len());
        self.keys.len()
    }

    #[inline]
    pub fn key_idx(&self, s: &str) -> KeyIndex {
        self.keys.binary_search_by(|string| string.as_str().cmp(s))
    }

    #[inline]
    pub fn contains_key(&self, key: &str) -> bool {
        self.key_idx(key).is_present()
    }

    #[inline]
    pub fn insert(&mut self, key: String, mut value: T) -> Option<T> {
        match self.key_idx(&key) {
            Ok(n) => {
                // std::mem::swap(&mut self.keys[n], &mut key);
                self.keys[n] = key;
                std::mem::swap(&mut self.values[n], &mut value);
                Some(value)
            }
            Err(n) => {
                self.keys.insert(n, key);
                self.values.insert(n, value);
                None
            }
        }
    }

    #[inline]
    pub fn remove(&mut self, key: &str) -> Option<T> {
        self.key_idx(&key).ok().map(|n| {
            self.keys.remove(n);
            self.values.remove(n)
        })
    }

    #[inline]
    pub fn get(&self, key: &str) -> Option<&T> {
        self.key_idx(key).ok().map(|n| &self.values[n])
    }

    #[inline]
    pub fn get_mut(&mut self, key: &str) -> Option<&mut T> {
        self.key_idx(key).ok().map(|n| &mut self.values[n])
    }

    #[inline]
    pub fn items(&self) -> impl Iterator<Item = (&str, &T)> {
        debug_assert_eq!(self.keys.len(), self.values.len());
        self.keys.iter().map(|s| s.as_str()).zip(self.values.iter())
    }
}

impl<T> Index<&str> for StringMap<T> {
    type Output = T;

    fn index(&self, n: &str) -> &T {
        self.get(n).unwrap()
    }
}

impl<T> IndexMut<&str> for StringMap<T> {
    fn index_mut(&mut self, n: &str) -> &mut T {
        self.get_mut(n).unwrap()
    }
}

type KeyIndex = Result<usize, usize>;
trait KeyIndexProps {
    fn is_present(&self) -> bool;
}
impl KeyIndexProps for KeyIndex {
    fn is_present(&self) -> bool {
        self.is_ok()
    }
}

#[cfg(test)]
mod tests {
    use crate::StringMap;

    fn test_key_value(map: &mut StringMap<u32>, key: &str, value: u32) {
        assert!(map.contains_key(key));
        assert_eq!(map[key], value);
        map[key] = value + 1;
        assert_eq!(map[key], value + 1);
        map[key] = value;
        assert_eq!(map[key], value);
    }

    #[test]
    fn test_map() {
        let mut map = StringMap::<u32>::default();

        map.insert("test".to_string(), 10);
        test_key_value(&mut map, "test", 10);

        map.insert("test2".to_string(), 20);
        test_key_value(&mut map, "test", 10);
        test_key_value(&mut map, "test2", 20);

        map.insert("atest".to_string(), 30);
        test_key_value(&mut map, "test", 10);
        test_key_value(&mut map, "test2", 20);
        test_key_value(&mut map, "atest", 30);

        let prev = map.insert("test".to_string(), 5);
        assert_eq!(prev, Some(10));
        test_key_value(&mut map, "test", 5);
        test_key_value(&mut map, "test2", 20);
        test_key_value(&mut map, "atest", 30);

        map.insert("btest".to_string(), 100);
        test_key_value(&mut map, "test", 5);
        test_key_value(&mut map, "test2", 20);
        test_key_value(&mut map, "atest", 30);
        test_key_value(&mut map, "btest", 100);

        assert_eq!(map.get("prout"), None);
        assert_eq!(map.get("tes"), None);
        assert_eq!(map.get("tesp"), None);
        assert_eq!(map.get("test "), None);

        assert_eq!(map.keys(), &vec!["atest", "btest", "test", "test2"]);
        assert_eq!(map.values(), &vec![30, 100, 5, 20]);
        assert_eq!(
            map.items().collect::<Vec<_>>(),
            vec![
                ("atest", &30),
                ("btest", &100),
                ("test", &5),
                ("test2", &20)
            ]
        );

        assert_eq!(map.remove("test"), Some(5));
        assert_eq!(map.keys(), &vec!["atest", "btest", "test2"]);
        assert_eq!(map.values(), &vec![30, 100, 20]);
        assert_eq!(
            map.items().collect::<Vec<_>>(),
            vec![("atest", &30), ("btest", &100), ("test2", &20)]
        );
    }
}
