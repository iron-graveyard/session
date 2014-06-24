pub mod store;

pub trait SessionStore<K, V>: Send + Share {
    fn set_key(&mut self, K);
    fn insert(&self, V);
    fn find(&self) -> Option<V>;
    fn swap(&self, V) -> Option<V>;
    fn upsert(&self, V, |&mut V|) -> V;
    fn remove(&self) -> bool;
}
