use parking_lot::RwLock;
use std::borrow::Borrow;
use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Weak};
use xmodits_lib::dsp::SampleBuffer;

pub struct Cache<K, V> {
    cache: RwLock<HashMap<K, Weak<V>>>,
}

impl<K, V> Cache<K, V>
where
    K: Hash + Eq,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get<Q>(&self, id: &Q) -> Option<Arc<V>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.cache.read().get(id)?.upgrade()
    }

    #[must_use = "Cache will be immediately invalidated as this is the only owning reference."]
    pub fn add(&self, id: K, sample: V) -> Arc<V>
    where
        K: Into<K>,
        V: Into<Arc<V>>,
    {
        let sample = sample.into();

        self.cache
            .write()
            .insert(id.into(), Arc::downgrade(&sample));

        sample
    }
}

impl <K, V>Default for Cache<K, V> {
    fn default() -> Self {
        Self { cache: Default::default() }
    }
}

