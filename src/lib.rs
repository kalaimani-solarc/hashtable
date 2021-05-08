use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::option::Option::Some;
use std::fmt::Display;

#[derive(Debug)]
pub struct HashTable<K, V> where K: Eq + Hash + Display {
    buckets: Vec<Bucket<K, V>>,
}

#[derive(Debug)]
struct Bucket<K, V> where K: Eq + Hash + Display {
    head: Link<K, V>,
    len: usize,
}

type Link<K, V> = Option<Box<Node<K, V>>>;

#[derive(Debug)]
struct Node<K, V> where K: Eq + Hash + Display {
    key: K,
    value: V,
    next: Link<K, V>,
}

const BUCKET_SIZE: usize = 8;

impl<K, V> HashTable<K, V> where K: Eq + Hash + Display {
    pub fn new() -> Self {
        let mut buckets = Vec::with_capacity(BUCKET_SIZE);

        for i in 0..BUCKET_SIZE {
            buckets.insert(i, Bucket { head: None, len: 0 })
        }

        HashTable { buckets }
    }

    pub fn len(&self) -> usize {
        self.buckets.iter().map(|bucket| bucket.len).sum()
    }

    pub fn insert(&mut self, key: K, value: V) {
        let index = Self::hash(&key);
        self.buckets[index].insert(key, value);
    }

    pub fn remove(&mut self, key: &K) {
        let index = Self::hash(key);
        self.buckets[index].remove(key);
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let index = Self::hash(key);
        self.buckets[index].get(key)
    }

    fn hash(key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let index = hasher.finish() % BUCKET_SIZE as u64;
        index as usize
    }
}

impl<K, V> Bucket<K, V> where K: Hash + Eq + Display {
    pub fn insert(&mut self, key: K, value: V) {
        let mut current = &mut self.head;

        loop {
            match current {
                None => break,
                Some(node) if node.key == key => {
                    break;
                }
                Some(node) => {
                    current = &mut node.next;
                }
            }
        };

        if let Some(node) = current {
            node.value = value
        } else {
            let head = self.head.take();
            self.head = Some(Box::new(Node{ key, value, next: head}));
            self.len += 1;
        }
    }

    pub fn remove(&mut self, key: &K) {
        let mut current = &mut self.head;
        loop {
            match current {
                None => return,
                Some(node) if &node.key == key => {
                    *current = node.next.take();
                    self.len -= 1;
                    return;
                }
                Some(node) => {
                    current = &mut node.next;
                }
            }
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let mut current = &self.head;
        loop {
            match current {
                None => return None,
                Some(node) if &node.key == key => {
                    return Some(&node.value);
                }
                Some(node) => {
                    current = &node.next;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::HashTable;

    #[test]
    fn it_works() {
        let mut hashtable = HashTable::new();
        hashtable.insert("Horse".to_string(), 11);
        hashtable.insert("Monkey".to_string(), 22);
        hashtable.insert("Elephant".to_string(), 33);
        hashtable.insert("Lion".to_string(), 44);

        assert_eq!(Some(&11), hashtable.get(&"Horse".to_string()));
        assert_eq!(Some(&22), hashtable.get(&"Monkey".to_string()));
        assert_eq!(Some(&33), hashtable.get(&"Elephant".to_string()));
        assert_eq!(Some(&44), hashtable.get(&"Lion".to_string()));
        assert_eq!(None, hashtable.get(&"Tiger".to_string()));

        hashtable.remove(&"Lion".to_string());
        assert_eq!(None, hashtable.get(&"Lion".to_string()));
        assert_eq!(3, hashtable.len());
    }
}
