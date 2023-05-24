use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
};
mod hashmap;
mod btreemap;
mod ord_vector;
mod eq_vector;

/// Idea 1. Put everything into one interface
pub trait GenericMap<K, V> {
    fn get(&self, key: &K) -> Option<&V>;
    fn get_mut(&mut self, key: &K) -> Option<&mut V>;
    fn get_key_value(&self, key: &K) -> Option<(&K, &V)>;
    fn insert(&mut self, key: K, value: V) -> Option<V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}

/// Idea 2. separate getter/setter
/// This would work if you want to implement it for things like Box<[Value]>
/// I think you can even go as far as to seperate them into smaller pieces
pub trait ReadGenericMap<K, V> {
    fn get(&self, key: &K) -> Option<&V>;
    fn get_mut(&mut self, key: &K) -> Option<&mut V>;
    fn get_key_value(&self, key: &K) -> Option<(&K, &V)>;
}

pub trait WriteGenericMap<K, V> {
    fn insert(&mut self, key: K, value: V) -> Option<V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}
