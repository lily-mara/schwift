use std::borrow::Borrow;

#[cfg(test)]
mod test;

pub struct VecMap<K, V> {
    data: Vec<(K, V)>,
}

impl<K, V> VecMap<K, V>
    where K: PartialEq
{
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn insert(&mut self, key: K, value: V) {
        if let Some(idx) = self.find(&key) {
            self.data[idx].1 = value;
        } else {
            self.data.push((key, value));
        }
    }

    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
        where K: Borrow<Q>,
              Q: PartialEq
    {
        self.find(k).map(|idx| &self.data[idx].1)
    }

    pub fn get_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut V>
        where K: Borrow<Q>,
              Q: PartialEq
    {
        self.find(k).map(move |idx| &mut self.data[idx].1)
    }

    pub fn remove<Q: ?Sized>(&mut self, k: &Q) -> Option<V>
        where K: Borrow<Q>,
              Q: PartialEq
    {
        self.find(k).map(|idx| self.data.remove(idx).1)
    }

    fn find<Q: ?Sized>(&self, k: &Q) -> Option<usize>
        where K: Borrow<Q>,
              Q: PartialEq
    {
        for (i, &(ref key, _)) in self.data.iter().enumerate() {
            if k == key.borrow() {
                return Some(i);
            }
        }
        None
    }
}
