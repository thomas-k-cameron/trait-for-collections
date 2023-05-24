use std::collections::BTreeMap;

use crate::{ReadGenericMap, WriteGenericMap, GenericMap};

pub struct BTreeMapImpl<K, V>(BTreeMap<K, V>);
impl<K, V> GenericMap<K, V> for BTreeMapImpl<K, V>
where
    K: Ord,
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

impl<K, V> WriteGenericMap<K, V> for BTreeMapImpl<K, V>
where
    K: Ord,
{
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.0.insert(key, value)
    }
    fn remove(&mut self, key: &K) -> Option<V> {
        self.0.remove(key)
    }
}

impl<K, V> ReadGenericMap<K, V> for BTreeMapImpl<K, V>
where
    K: Ord,
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
}
