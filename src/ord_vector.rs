use crate::{ReadGenericMap, WriteGenericMap};

pub struct VecMapImplAsOrdKey<K: Ord, V>(Vec<(K, V)>);
impl<K: Ord, V> ReadGenericMap<K, V> for VecMapImplAsOrdKey<K, V> {
    fn get(&self, key: &K) -> Option<&V> {
        match self.0.binary_search_by(|(k, _)| k.cmp(key)) {
            Ok(i) => Some(&self.0[i].1),
            _ => None
        }
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        match self.0.binary_search_by(|(k, _)| k.cmp(key)) {
            Ok(i) => Some(&mut self.0[i].1),
            _ => None
        }
    }

    fn get_key_value(&self, key: &K) -> Option<(&K, &V)> {
        match self.0.binary_search_by(|(k, _)| k.cmp(key)) {
            Ok(i) => Some({
                let (key,val) = &self.0[i];
                (key, val)
            }),
            _ => None
        }
    }
}

impl<K: Ord, V> WriteGenericMap<K, V> for VecMapImplAsOrdKey<K, V> {
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.0.binary_search_by(|(k, _)| k.cmp(&key)) {
            Ok(i) => {
                let mut tup = (key, value);
                std::mem::swap(&mut tup, &mut self.0[i]);
                return Some(tup.1)
            },
            Err(i) => self.0.insert(i, (key, value))
        };
        None
    }
    fn remove(&mut self, key: &K) -> Option<V> {
        match self.0.binary_search_by(|(k, _)| k.cmp(&key)) {
            Ok(i) => Some(self.0.remove(i).1),
            Err(_) => None
        }
    }
}
