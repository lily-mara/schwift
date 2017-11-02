use std::borrow::Borrow;

#[cfg(test)]
mod test;

struct Entry<K, V> {
    key: K,
    value: V,
}

pub struct VecMap<K, V> {
    data: Vec<Entry<K, V>>,
}

impl<K, V> VecMap<K, V>
where
    K: PartialEq,
{
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn insert(&mut self, key: K, value: V) {
        if let Some(idx) = self.find(&key) {
            self.data[idx].value = value;
        } else {
            self.data.push(Entry { key, value });
        }
    }

    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: PartialEq,
    {
        self.find(k).map(|idx| &self.data[idx].value)
    }

    pub fn get_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: PartialEq,
    {
        self.find(k).map(move |idx| &mut self.data[idx].value)
    }

    pub fn remove<Q: ?Sized>(&mut self, k: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: PartialEq,
    {
        self.find(k).map(|idx| self.data.remove(idx).value)
    }

    fn find<Q: ?Sized>(&self, k: &Q) -> Option<usize>
    where
        K: Borrow<Q>,
        Q: PartialEq,
    {
        for (i, entry) in self.data.iter().enumerate() {
            if k == entry.key.borrow() {
                return Some(i);
            }
        }
        None
    }
}
