use std::{collections::HashMap, hash::Hash};

use crate::{ReadGenericMap, WriteGenericMap, GenericMap};

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
    fn get(&self, key: &K) -> Option<&V> {
        self.0.get(key)
    }
    fn get_key_value(&self, key: &K) -> Option<(&K, &V)> {
        self.0.get_key_value(key)
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.0.insert(key, value)
    }
    fn remove(&mut self, key: &K) -> Option<V> {
        self.0.remove(key)
    }
}
