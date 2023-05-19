use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
};

fn main() {
    println!("Hello, world!");
}


/// Idea 1. Put everything into one interface
pub trait GenericMap<K, V> {
    fn insert(&mut self, key: K, value: V) -> Option<V>;
    fn get_mut(&mut self, key: &K) -> Option<&mut V>;
}

/// Idea 2. separate getter/setter
/// This would work if you want to implement it for things like Box<[Value]>
pub trait ReadGenericMap<K, V> {
    fn get(&self, key: &K) -> Option<&V>;
    fn get_mut(&mut self, key: &K) -> Option<&mut V>;
    fn get_key_value(&self, key: &K) -> Option<(&K, &V)>;
}

pub trait WriteGenericMap<K, V> {
    fn insert(&mut self, key: K, value: V) -> Option<V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}

struct HashMapImpl<K, V>(HashMap<K, V>);
impl<K, V> ReadGenericMap<K, V> for HashMapImpl<K, V>
where
    K: Hash + Eq,
{
    fn get(&self, key: &K) -> Option<&V> {
        self.0.get(key)
    }
    fn get_key_value(&self, key: &K) -> Option<(&K, &V)> {
        self.0.get_key_value(key)
    }
    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.0.get_mut(key)
    }
}
impl<K, V> WriteGenericMap<K, V> for HashMapImpl<K, V>
where
    K: Hash + Eq,
{
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.0.insert(key, value)
    }
    fn remove(&mut self, key: &K) -> Option<V> {
        self.0.remove(key)
    }
}
impl<K, V> GenericMap<K, V> for HashMapImpl<K, V>
where
    K: Hash + Eq + PartialEq,
{
    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.0.get_mut(key)
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.0.insert(key, value)
    }
}

struct BTreeMapImpl<K, V>(BTreeMap<K, V>);
impl<K, V> GenericMap<K, V> for BTreeMapImpl<K, V>
where
    K: Ord,
{
    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.0.get_mut(key)
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.0.insert(key, value)
    }
}

struct OrdKeyValVec<K, V>(Vec<(K, V)>);
impl<K: Ord, V> GenericMap<K, V> for OrdKeyValVec<K, V> {
    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let idx = match self.0.binary_search_by(|(key2, _)| key.cmp(key2)) {
            Ok(i) => i,
            Err(i) => i,
        };
        if let Some(val) = self.0.get_mut(idx) {
            Some(&mut val.1)
        } else {
            None
        }
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.0.push((key, value));
        None
    }
}

struct VecMapImplAsEqKey<K, V>(Vec<(K, V)>);
impl<K: Eq, V> GenericMap<K, V> for VecMapImplAsEqKey<K, V> {
    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        for (key2, val) in self.0.iter_mut() {
            if key2 == key {
                return Some(val);
            }
        }

        None
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.0.push((key, value));
        None
    }
}
