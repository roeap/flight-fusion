pub mod lru;

use std::borrow::Borrow;
use std::hash::BuildHasher;
use std::hash::Hash;

use crate::Meter;

/// A trait for a cache.
pub trait Cache<K, V, S, M>
where
    K: Eq + Hash,
    S: BuildHasher,
    M: Meter<K, V>,
{
    /// Creates an empty cache that can hold at most `capacity` as measured by `meter` with the
    /// given hash builder.
    fn with_meter_and_hasher(cap: u64, meter: M, hash_builder: S) -> Self;

    /// Returns a reference to the value corresponding to the given key in the cache, if
    /// any.
    fn get<'a, Q>(&'a mut self, k: &Q) -> Option<&'a V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized;

    /// Returns a mutable reference to the value corresponding to the given key in the cache, if
    /// any.
    fn get_mut<'a, Q>(&'a mut self, k: &Q) -> Option<&'a mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized;

    /// Returns a reference to the value corresponding to the key in the cache or `None` if it is
    /// not present in the cache. Unlike `get`, `peek` does not update the Cache state so the key's
    /// position will be unchanged.
    fn peek<'a, Q>(&'a self, k: &Q) -> Option<&'a V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized;

    /// Returns a mutable reference to the value corresponding to the key in the cache or `None`
    /// if it is not present in the cache. Unlike `get_mut`, `peek_mut` does not update the Cache
    /// state so the key's position will be unchanged.
    fn peek_mut<'a, Q>(&'a mut self, k: &Q) -> Option<&'a mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized;

    /// Returns the value corresponding to the item by policy or `None` if the
    /// cache is empty. Like `peek`, `peek_by_policy` does not update the Cache state so the item's
    /// position will be unchanged.
    ///
    // TODO: change to fn peek_by_policy<'a>(&self) -> Option<(&'a K, &'a V)>;
    fn peek_by_policy(&self) -> Option<(&K, &V)>;

    /// Inserts a key-value pair into the cache. If the key already existed, the old value is
    /// returned.
    fn put(&mut self, k: K, v: V) -> Option<V>;

    /// Removes the given key from the cache and returns its corresponding value.
    fn pop<Q>(&mut self, k: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized;

    /// Removes and returns the key-value pair as a tuple by policy (Lru, Lfu, etc.).
    fn pop_by_policy(&mut self) -> Option<(K, V)>;

    /// Checks if the map contains the given key.
    fn contains<Q>(&self, k: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized;

    /// Returns the number of key-value pairs in the cache.
    fn len(&self) -> usize;

    /// Returns `true` if the cache contains no key-value pairs.
    fn is_empty(&self) -> bool;

    /// Returns the maximum size of the key-value pairs the cache can hold, as measured by the
    /// `Meter` used by the cache.
    fn capacity(&self) -> u64;

    /// Sets the size of the key-value pairs the cache can hold, as measured by the `Meter` used by
    /// the cache.
    fn set_capacity(&mut self, cap: u64);

    /// Returns the size of all the key-value pairs in the cache, as measured by the `Meter` used
    /// by the cache.
    fn size(&self) -> u64;

    /// Removes all key-value pairs from the cache.
    fn clear(&mut self);
}
