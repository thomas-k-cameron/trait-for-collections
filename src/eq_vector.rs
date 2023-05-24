use crate::{ReadGenericMap, WriteGenericMap};

pub struct VecMapImplAsEqKey<K, V>(Vec<(K, V)>);
impl<K: Eq, V> ReadGenericMap<K, V> for VecMapImplAsEqKey<K, V> {
    fn get(&self, key: &K) -> Option<&V> {
        for (key2, val) in self.0.iter() {
            if key2 == key {
                return Some(val);
            }
        }

        None
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        for (key2, val) in self.0.iter_mut() {
            if key2 == key {
                return Some(val);
            }
        }

        None
    }

    fn get_key_value(&self, key: &K) -> Option<(&K, &V)> {
        for (key2, val) in self.0.iter() {
            if key2 == key {
                return Some((key2, val));
            }
        }

        None
    }
}

impl<K: Eq, V> WriteGenericMap<K, V> for VecMapImplAsEqKey<K, V> {
    fn insert(&mut self, key: K, mut value: V) -> Option<V> {
        for (key2, val) in self.0.iter_mut() {
            if key2 == &key {
                std::mem::swap(val, &mut value);
                return Some(value);
            }
        }
        None
    }
    fn remove(&mut self, key: &K) -> Option<V> {
        let mut idx = None;
        for (idx2, (key2, val)) in self.0.iter_mut().enumerate() {
            if key2 == key {
                idx = Some(idx2);
                break;
            }
        }
        match idx {
            Some(idx) => {
                Some(self.0.remove(idx).1)
            }
            _ => None
        }
        
    }
}
