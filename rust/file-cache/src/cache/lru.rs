//! A cache that holds a limited number of key-value pairs. When the
//! capacity of the cache is exceeded, the least-recently-used
//! (where "used" means a look-up or putting the pair into the cache)
//! pair is automatically removed.
//!
//! # Examples
//!
//! ```rust
//! use file_cache::{Cache, LruCache};
//!
//! let mut cache = LruCache::new(2);
//!
//! cache.put(1, 10);
//! cache.put(2, 20);
//! cache.put(3, 30);
//! assert!(cache.get_mut(&1).is_none());
//! assert_eq!(*cache.get_mut(&2).unwrap(), 20);
//! assert_eq!(*cache.get_mut(&3).unwrap(), 30);
//!
//! cache.put(2, 22);
//! assert_eq!(*cache.get_mut(&2).unwrap(), 22);
//!
//! cache.put(6, 60);
//! assert!(cache.get_mut(&3).is_none());
//!
//! cache.set_capacity(1);
//! assert!(cache.get_mut(&2).is_none());
//! ```
//!
//! The cache can also be limited by an arbitrary metric calculated from its key-value pairs, see
//! [`LruCache::with_meter`][with_meter] for more information. If the `heapsize` feature is enabled,
//! this crate provides one such alternate metric&mdash;`HeapSize`. Custom metrics can be written by
//! implementing the [`Meter`][meter] trait.
//!
//! [with_meter]: struct.LruCache.html#method.with_meter
//! [meter]: trait.Meter.html
use std::borrow::Borrow;
use std::fmt;
use std::hash::BuildHasher;
use std::hash::Hash;

use ritelinked::linked_hash_map;
use ritelinked::DefaultHashBuilder;
use ritelinked::LinkedHashMap;

use crate::cache::Cache;
use crate::meter::count::Count;
use crate::meter::count::CountableMeter;

/// An LRU cache.
#[derive(Clone)]
pub struct LruCache<
    K: Eq + Hash,
    V,
    S: BuildHasher = DefaultHashBuilder,
    M: CountableMeter<K, V> = Count,
> {
    map: LinkedHashMap<K, V, S>,
    current_measure: M::Measure,
    max_capacity: u64,
    meter: M,
}

impl<K: Eq + Hash, V> LruCache<K, V> {
    /// Creates an empty cache that can hold at most `capacity` items.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use file_cache::{Cache, LruCache};
    /// let mut cache: LruCache<i32, &str> = LruCache::new(10);
    /// ```
    pub fn new(capacity: u64) -> Self {
        LruCache {
            map: LinkedHashMap::new(),
            current_measure: (),
            max_capacity: capacity,
            meter: Count,
        }
    }
}

impl<K: Eq + Hash, V, M: CountableMeter<K, V>> LruCache<K, V, DefaultHashBuilder, M> {
    /// Creates an empty cache that can hold at most `capacity` as measured by `meter`.
    ///
    /// You can implement the [`Meter`][meter] trait to allow custom metrics.
    ///
    /// [meter]: trait.Meter.html
    ///
    /// # Examples
    ///
    /// ```rust
    /// use file_cache::{Cache, LruCache, Meter};
    /// use std::borrow::Borrow;
    ///
    /// /// Measure Vec items by their length
    /// struct VecLen;
    ///
    /// impl<K, T> Meter<K, Vec<T>> for VecLen {
    ///     // Use `Measure = usize` or implement `CountableMeter` as well.
    ///     type Measure = usize;
    ///     fn measure<Q: ?Sized>(&self, _: &Q, v: &Vec<T>) -> usize
    ///         where K: Borrow<Q>
    ///     {
    ///         v.len()
    ///     }
    /// }
    ///
    /// let mut cache = LruCache::with_meter(5, VecLen);
    /// cache.put(1, vec![1, 2]);
    /// assert_eq!(cache.size(), 2);
    /// cache.put(2, vec![3, 4]);
    /// cache.put(3, vec![5, 6]);
    /// assert_eq!(cache.size(), 4);
    /// assert_eq!(cache.len(), 2);
    /// ```
    pub fn with_meter(capacity: u64, meter: M) -> LruCache<K, V, DefaultHashBuilder, M> {
        LruCache {
            map: LinkedHashMap::new(),
            current_measure: Default::default(),
            max_capacity: capacity,
            meter,
        }
    }
}

impl<K: Eq + Hash, V, S: BuildHasher> LruCache<K, V, S, Count> {
    /// Creates an empty cache that can hold at most `capacity` items with the given hash builder.
    pub fn with_hasher(capacity: u64, hash_builder: S) -> LruCache<K, V, S, Count> {
        LruCache {
            map: LinkedHashMap::with_hasher(hash_builder),
            current_measure: (),
            max_capacity: capacity,
            meter: Count,
        }
    }
}

impl<K: Eq + Hash, V, S: BuildHasher, M: CountableMeter<K, V>> Cache<K, V, S, M>
    for LruCache<K, V, S, M>
{
    /// Creates an empty cache that can hold at most `capacity` as measured by `meter` with the
    /// given hash builder.
    fn with_meter_and_hasher(capacity: u64, meter: M, hash_builder: S) -> Self {
        LruCache {
            map: LinkedHashMap::with_hasher(hash_builder),
            current_measure: Default::default(),
            max_capacity: capacity,
            meter,
        }
    }

    /// Returns a reference to the value corresponding to the given key in the cache, if
    /// any.
    ///
    /// Note that this method is not available for cache objects using `Meter` implementations
    /// other than `Count`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use file_cache::{Cache, LruCache};
    ///
    /// let mut cache = LruCache::new(2);
    ///
    /// cache.put(1, "a");
    /// cache.put(2, "b");
    /// cache.put(2, "c");
    /// cache.put(3, "d");
    ///
    /// assert_eq!(cache.get(&1), None);
    /// assert_eq!(cache.get(&2), Some(&"c"));
    /// ```
    fn get<Q>(&mut self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.map.get_refresh(k).map(|v| v as &V)
    }

    /// Returns a mutable reference to the value corresponding to the given key in the cache, if
    /// any.
    ///
    /// Note that this method is not available for cache objects using `Meter` implementations
    /// other than `Count`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use file_cache::{Cache, LruCache};
    ///
    /// let mut cache = LruCache::new(2);
    ///
    /// cache.put(1, "a");
    /// cache.put(2, "b");
    /// cache.put(2, "c");
    /// cache.put(3, "d");
    ///
    /// assert_eq!(cache.get_mut(&1), None);
    /// assert_eq!(cache.get_mut(&2), Some(&mut "c"));
    /// ```
    fn get_mut<Q>(&mut self, k: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.map.get_refresh(k)
    }

    /// Returns a reference to the value corresponding to the key in the cache or `None` if it is
    /// not present in the cache. Unlike `get`, `peek` does not update the LRU list so the key's
    /// position will be unchanged.
    ///
    /// # Example
    ///
    /// ```rust
    /// use file_cache::{Cache, LruCache};
    /// let mut cache = LruCache::new(2);
    ///
    /// cache.put(1, "a");
    /// cache.put(2, "b");
    ///
    /// assert_eq!(cache.peek(&1), Some(&"a"));
    /// assert_eq!(cache.peek(&2), Some(&"b"));
    /// ```
    fn peek<'a, Q>(&'a self, k: &Q) -> Option<&'a V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.map.get(k)
    }

    /// Returns a mutable reference to the value corresponding to the key in the cache or `None`
    /// if it is not present in the cache. Unlike `get_mut`, `peek_mut` does not update the LRU
    /// list so the key's position will be unchanged.
    ///
    /// # Example
    ///
    /// ```rust
    /// use file_cache::{Cache, LruCache};
    /// let mut cache = LruCache::new(2);
    ///
    /// cache.put(1, "a");
    /// cache.put(2, "b");
    ///
    /// assert_eq!(cache.peek_mut(&1), Some(&mut "a"));
    /// assert_eq!(cache.peek_mut(&2), Some(&mut "b"));
    /// ```
    fn peek_mut<'a, Q>(&'a mut self, k: &Q) -> Option<&'a mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.map.get_mut(k)
    }

    /// Returns the value corresponding to the least recently used item or `None` if the
    /// cache is empty. Like `peek`, `peek_by_policy` does not update the LRU list so the item's
    /// position will be unchanged.
    ///
    /// # Example
    ///
    /// ```rust
    /// use file_cache::{Cache, LruCache};
    /// let mut cache = LruCache::new(2);
    ///
    /// cache.put(1, "a");
    /// cache.put(2, "b");
    ///
    /// assert_eq!(cache.peek_by_policy(), Some((&1, &"a")));
    /// ```
    fn peek_by_policy(&self) -> Option<(&K, &V)> {
        self.map.front()
    }

    /// Checks if the map contains the given key.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use file_cache::{Cache, LruCache};
    ///
    /// let mut cache = LruCache::new(1);
    ///
    /// cache.put(1, "a");
    /// assert!(cache.contains(&1));
    /// ```
    fn contains<Q: ?Sized>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.map.contains_key(key)
    }

    /// Inserts a key-value pair into the cache. If the key already existed, the old value is
    /// returned.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use file_cache::{Cache, LruCache};
    ///
    /// let mut cache = LruCache::new(2);
    ///
    /// cache.put(1, "a");
    /// cache.put(2, "b");
    /// assert_eq!(cache.get_mut(&1), Some(&mut "a"));
    /// assert_eq!(cache.get_mut(&2), Some(&mut "b"));
    /// ```
    fn put(&mut self, k: K, v: V) -> Option<V> {
        let new_size = self.meter.measure(&k, &v);
        self.current_measure = self.meter.add(self.current_measure, new_size);
        if let Some(old) = self.map.get(&k) {
            self.current_measure = self
                .meter
                .sub(self.current_measure, self.meter.measure(&k, old));
        }
        let old_val = self.map.insert(k, v);
        while self.size() > self.capacity() {
            self.pop_by_policy();
        }
        old_val
    }

    /// Removes the given key from the cache and returns its corresponding value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use file_cache::{Cache, LruCache};
    ///
    /// let mut cache = LruCache::new(2);
    ///
    /// cache.put(2, "a");
    ///
    /// assert_eq!(cache.pop(&1), None);
    /// assert_eq!(cache.pop(&2), Some("a"));
    /// assert_eq!(cache.pop(&2), None);
    /// assert_eq!(cache.len(), 0);
    /// ```
    fn pop<Q>(&mut self, k: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.map.remove(k).map(|v| {
            self.current_measure = self
                .meter
                .sub(self.current_measure, self.meter.measure(k, &v));
            v
        })
    }

    /// Removes and returns the least recently used key-value pair as a tuple.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use file_cache::{Cache, LruCache};
    ///
    /// let mut cache = LruCache::new(2);
    ///
    /// cache.put(1, "a");
    /// cache.put(2, "b");
    ///
    /// assert_eq!(cache.pop_by_policy(), Some((1, "a")));
    /// assert_eq!(cache.len(), 1);
    /// ```
    #[inline]
    fn pop_by_policy(&mut self) -> Option<(K, V)> {
        self.map.pop_front().map(|(k, v)| {
            self.current_measure = self
                .meter
                .sub(self.current_measure, self.meter.measure(&k, &v));
            (k, v)
        })
    }

    /// Sets the size of the key-value pairs the cache can hold, as measured by the `Meter` used by
    /// the cache.
    ///
    /// Removes least-recently-used key-value pairs if necessary.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use file_cache::{Cache, LruCache};
    ///
    /// let mut cache = LruCache::new(2);
    ///
    /// cache.put(1, "a");
    /// cache.put(2, "b");
    /// cache.put(3, "c");
    ///
    /// assert_eq!(cache.get_mut(&1), None);
    /// assert_eq!(cache.get_mut(&2), Some(&mut "b"));
    /// assert_eq!(cache.get_mut(&3), Some(&mut "c"));
    ///
    /// cache.set_capacity(3);
    /// cache.put(1, "a");
    /// cache.put(2, "b");
    ///
    /// assert_eq!(cache.get_mut(&1), Some(&mut "a"));
    /// assert_eq!(cache.get_mut(&2), Some(&mut "b"));
    /// assert_eq!(cache.get_mut(&3), Some(&mut "c"));
    ///
    /// cache.set_capacity(1);
    ///
    /// assert_eq!(cache.get_mut(&1), None);
    /// assert_eq!(cache.get_mut(&2), None);
    /// assert_eq!(cache.get_mut(&3), Some(&mut "c"));
    /// ```
    fn set_capacity(&mut self, capacity: u64) {
        while self.size() > capacity {
            self.pop_by_policy();
        }
        self.max_capacity = capacity;
    }

    /// Returns the number of key-value pairs in the cache.
    fn len(&self) -> usize {
        self.map.len()
    }

    /// Returns the size of all the key-value pairs in the cache, as measured by the `Meter` used
    /// by the cache.
    fn size(&self) -> u64 {
        self.meter
            .size(self.current_measure)
            .unwrap_or_else(|| self.map.len() as u64)
    }

    /// Returns the maximum size of the key-value pairs the cache can hold, as measured by the
    /// `Meter` used by the cache.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use file_cache::{Cache, LruCache};
    /// let mut cache: LruCache<i32, &str> = LruCache::new(2);
    /// assert_eq!(cache.capacity(), 2);
    /// ```
    fn capacity(&self) -> u64 {
        self.max_capacity
    }

    /// Returns `true` if the cache contains no key-value pairs.
    fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Removes all key-value pairs from the cache.
    fn clear(&mut self) {
        self.map.clear();
        self.current_measure = Default::default();
    }
}

impl<K: Eq + Hash, V, S: BuildHasher, M: CountableMeter<K, V>> LruCache<K, V, S, M> {
    /// Returns an iterator over the cache's key-value pairs in least- to most-recently-used order.
    ///
    /// Accessing the cache through the iterator does _not_ affect the cache's LRU state.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use file_cache::{Cache, LruCache};
    ///
    /// let mut cache = LruCache::new(2);
    ///
    /// cache.put(1, 10);
    /// cache.put(2, 20);
    /// cache.put(3, 30);
    ///
    /// let kvs: Vec<_> = cache.iter().collect();
    /// assert_eq!(kvs, [(&2, &20), (&3, &30)]);
    /// ```
    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter(self.map.iter())
    }

    /// Returns an iterator over the cache's key-value pairs in least- to most-recently-used order,
    /// with mutable references to the values.
    ///
    /// Accessing the cache through the iterator does _not_ affect the cache's LRU state.
    /// Note that this method is not available for cache objects using `Meter` implementations.
    /// other than `Count`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use file_cache::{Cache, LruCache};
    ///
    /// let mut cache = LruCache::new(2);
    ///
    /// cache.put(1, 10);
    /// cache.put(2, 20);
    /// cache.put(3, 30);
    ///
    /// let mut n = 2;
    ///
    /// for (k, v) in cache.iter_mut() {
    ///     assert_eq!(*k, n);
    ///     assert_eq!(*v, n * 10);
    ///     *v *= 10;
    ///     n += 1;
    /// }
    ///
    /// assert_eq!(n, 4);
    /// assert_eq!(cache.get_mut(&2), Some(&mut 200));
    /// assert_eq!(cache.get_mut(&3), Some(&mut 300));
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        IterMut(self.map.iter_mut())
    }
}

impl<K: Eq + Hash, V, S: BuildHasher, M: CountableMeter<K, V>> Extend<(K, V)>
    for LruCache<K, V, S, M>
{
    fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, iter: I) {
        for (k, v) in iter {
            self.put(k, v);
        }
    }
}

impl<K: fmt::Debug + Eq + Hash, V: fmt::Debug, S: BuildHasher, M: CountableMeter<K, V>> fmt::Debug
    for LruCache<K, V, S, M>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter().rev()).finish()
    }
}

impl<K: Eq + Hash, V, S: BuildHasher, M: CountableMeter<K, V>> IntoIterator
    for LruCache<K, V, S, M>
{
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> IntoIter<K, V> {
        IntoIter(self.map.into_iter())
    }
}

impl<'a, K: Eq + Hash, V, S: BuildHasher, M: CountableMeter<K, V>> IntoIterator
    for &'a LruCache<K, V, S, M>
{
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;
    fn into_iter(self) -> Iter<'a, K, V> {
        self.iter()
    }
}

impl<'a, K: Eq + Hash, V, S: BuildHasher, M: CountableMeter<K, V>> IntoIterator
    for &'a mut LruCache<K, V, S, M>
{
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;
    fn into_iter(self) -> IterMut<'a, K, V> {
        self.iter_mut()
    }
}

/// An iterator over a cache's key-value pairs in least- to most-recently-used order.
///
/// # Examples
///
/// ```rust
/// use file_cache::{Cache, LruCache};
///
/// let mut cache = LruCache::new(2);
///
/// cache.put(1, 10);
/// cache.put(2, 20);
/// cache.put(3, 30);
///
/// let mut n = 2;
///
/// for (k, v) in cache {
///     assert_eq!(k, n);
///     assert_eq!(v, n * 10);
///     n += 1;
/// }
///
/// assert_eq!(n, 4);
/// ```
pub struct IntoIter<K, V>(linked_hash_map::IntoIter<K, V>);

impl<K, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<(K, V)> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<K, V> DoubleEndedIterator for IntoIter<K, V> {
    fn next_back(&mut self) -> Option<(K, V)> {
        self.0.next_back()
    }
}

impl<K, V> ExactSizeIterator for IntoIter<K, V> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

/// An iterator over a cache's key-value pairs in least- to most-recently-used order.
///
/// Accessing a cache through the iterator does _not_ affect the cache's LRU state.
pub struct Iter<'a, K, V>(linked_hash_map::Iter<'a, K, V>);

impl<'a, K, V> Clone for Iter<'a, K, V> {
    fn clone(&self) -> Iter<'a, K, V> {
        Iter(self.0.clone())
    }
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);
    fn next(&mut self) -> Option<(&'a K, &'a V)> {
        self.0.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a, K, V> DoubleEndedIterator for Iter<'a, K, V> {
    fn next_back(&mut self) -> Option<(&'a K, &'a V)> {
        self.0.next_back()
    }
}

impl<'a, K, V> ExactSizeIterator for Iter<'a, K, V> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

/// An iterator over a cache's key-value pairs in least- to most-recently-used order with mutable
/// references to the values.
///
/// Accessing a cache through the iterator does _not_ affect the cache's LRU state.
pub struct IterMut<'a, K, V>(linked_hash_map::IterMut<'a, K, V>);

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);
    fn next(&mut self) -> Option<(&'a K, &'a mut V)> {
        self.0.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a, K, V> DoubleEndedIterator for IterMut<'a, K, V> {
    fn next_back(&mut self) -> Option<(&'a K, &'a mut V)> {
        self.0.next_back()
    }
}

impl<'a, K, V> ExactSizeIterator for IterMut<'a, K, V> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    use crate::Cache;
    use crate::LruCache;
    use crate::Meter;

    #[test]
    fn test_put_and_get() {
        let mut cache = LruCache::new(2);
        cache.put(1, 10);
        cache.put(2, 20);
        assert_eq!(cache.get_mut(&1), Some(&mut 10));
        assert_eq!(cache.get_mut(&2), Some(&mut 20));
        assert_eq!(cache.len(), 2);
        assert_eq!(cache.size(), 2);
    }

    #[test]
    fn test_put_update() {
        let mut cache = LruCache::new(1);
        cache.put("1", 10);
        cache.put("1", 19);
        assert_eq!(cache.get_mut("1"), Some(&mut 19));
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_contains() {
        let mut cache = LruCache::new(1);
        cache.put("1", 10);
        assert!(cache.contains("1"));
    }

    #[test]
    fn test_expire_lru() {
        let mut cache = LruCache::new(2);
        cache.put("foo1", "bar1");
        cache.put("foo2", "bar2");
        cache.put("foo3", "bar3");
        assert!(cache.get_mut("foo1").is_none());
        cache.put("foo2", "bar2update");
        cache.put("foo4", "bar4");
        assert!(cache.get_mut("foo3").is_none());
    }

    #[test]
    fn test_pop() {
        let mut cache = LruCache::new(2);
        cache.put(1, 10);
        cache.put(2, 20);
        assert_eq!(cache.len(), 2);
        let opt1 = cache.pop(&1);
        assert!(opt1.is_some());
        assert_eq!(opt1.unwrap(), 10);
        assert!(cache.get_mut(&1).is_none());
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_change_capacity() {
        let mut cache = LruCache::new(2);
        assert_eq!(cache.capacity(), 2);
        cache.put(1, 10);
        cache.put(2, 20);
        cache.set_capacity(1);
        assert!(cache.get_mut(&1).is_none());
        assert_eq!(cache.capacity(), 1);
    }

    #[test]
    fn test_debug() {
        let mut cache = LruCache::new(3);
        cache.put(1, 10);
        cache.put(2, 20);
        cache.put(3, 30);
        assert_eq!(format!("{:?}", cache), "{3: 30, 2: 20, 1: 10}");
        cache.put(2, 22);
        assert_eq!(format!("{:?}", cache), "{2: 22, 3: 30, 1: 10}");
        cache.put(6, 60);
        assert_eq!(format!("{:?}", cache), "{6: 60, 2: 22, 3: 30}");
        cache.get_mut(&3);
        assert_eq!(format!("{:?}", cache), "{3: 30, 6: 60, 2: 22}");
        cache.set_capacity(2);
        assert_eq!(format!("{:?}", cache), "{3: 30, 6: 60}");
    }

    #[test]
    fn test_remove() {
        let mut cache = LruCache::new(3);
        cache.put(1, 10);
        cache.put(2, 20);
        cache.put(3, 30);
        cache.put(4, 40);
        cache.put(5, 50);
        cache.pop(&3);
        cache.pop(&4);
        assert!(cache.get_mut(&3).is_none());
        assert!(cache.get_mut(&4).is_none());
        cache.put(6, 60);
        cache.put(7, 70);
        cache.put(8, 80);
        assert!(cache.get_mut(&5).is_none());
        assert_eq!(cache.get_mut(&6), Some(&mut 60));
        assert_eq!(cache.get_mut(&7), Some(&mut 70));
        assert_eq!(cache.get_mut(&8), Some(&mut 80));
    }

    #[test]
    fn test_clear() {
        let mut cache = LruCache::new(2);
        cache.put(1, 10);
        cache.put(2, 20);
        cache.clear();
        assert!(cache.get_mut(&1).is_none());
        assert!(cache.get_mut(&2).is_none());
        assert_eq!(format!("{:?}", cache), "{}");
    }

    #[test]
    fn test_iter() {
        let mut cache = LruCache::new(3);
        cache.put(1, 10);
        cache.put(2, 20);
        cache.put(3, 30);
        cache.put(4, 40);
        cache.put(5, 50);
        assert_eq!(
            cache.iter().collect::<Vec<_>>(),
            [(&3, &30), (&4, &40), (&5, &50)]
        );
        assert_eq!(
            cache.iter_mut().collect::<Vec<_>>(),
            [(&3, &mut 30), (&4, &mut 40), (&5, &mut 50)]
        );
        assert_eq!(
            cache.iter().rev().collect::<Vec<_>>(),
            [(&5, &50), (&4, &40), (&3, &30)]
        );
        assert_eq!(
            cache.iter_mut().rev().collect::<Vec<_>>(),
            [(&5, &mut 50), (&4, &mut 40), (&3, &mut 30)]
        );
    }

    struct VecLen;

    impl<K, T> Meter<K, Vec<T>> for VecLen {
        type Measure = usize;
        fn measure<Q: ?Sized>(&self, _: &Q, v: &Vec<T>) -> usize
        where
            K: Borrow<Q>,
        {
            v.len()
        }
    }

    #[test]
    fn test_metered_cache() {
        let mut cache = LruCache::with_meter(5, VecLen);
        cache.put("foo1", vec![1, 2]);
        assert_eq!(cache.size(), 2);
        cache.put("foo2", vec![3, 4]);
        cache.put("foo3", vec![5, 6]);
        assert_eq!(cache.size(), 4);
        assert!(!cache.contains("foo1"));
        cache.put("foo2", vec![7, 8]);
        cache.put("foo4", vec![9, 10]);
        assert_eq!(cache.size(), 4);
        assert!(!cache.contains("foo3"));
        assert_eq!(cache.get("foo2"), Some(&vec![7, 8]));
    }

    #[test]
    fn test_metered_cache_reinsert_larger() {
        let mut cache = LruCache::with_meter(5, VecLen);
        cache.put("foo1", vec![1, 2]);
        cache.put("foo2", vec![3, 4]);
        assert_eq!(cache.size(), 4);
        cache.put("foo2", vec![5, 6, 7, 8]);
        assert_eq!(cache.size(), 4);
        assert!(!cache.contains("foo1"));
    }

    #[test]
    fn test_metered_cache_oversize() {
        let mut cache = LruCache::with_meter(2, VecLen);
        cache.put("foo1", vec![1, 2]);
        cache.put("foo2", vec![3, 4, 5, 6]);
        assert_eq!(cache.size(), 0);
        assert!(!cache.contains("foo1"));
        assert!(!cache.contains("foo2"));
    }
}
