use std::borrow::Borrow;

use super::Meter;
pub struct BytesMeter;

impl<K> Meter<K, Vec<u8>> for BytesMeter {
    type Measure = usize;
    fn measure<Q: ?Sized>(&self, _: &Q, v: &Vec<u8>) -> usize
    where
        K: Borrow<Q>,
    {
        v.len()
    }
}
