use std::hash::Hash;

pub trait Edge {
    type Value: Hash + Eq;

    fn source(&self) -> &Self::Value;
    fn destination(&self) -> &Self::Value;

    // Define a default constructor for edges (common trait method)
    fn new(src: Self::Value, dest: Self::Value) -> Self;
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct UnweightedEdge<V> {
    src: V,
    dest: V,
}

impl<V> Edge for UnweightedEdge<V>
where
    V: Hash + Eq,
{
    type Value = V;

    fn source(&self) -> &Self::Value {
        &self.src
    }

    fn destination(&self) -> &Self::Value {
        &self.dest
    }

    fn new(src: Self::Value, dest: Self::Value) -> Self {
        Self { src, dest }
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct WeightedEdge<V, W> {
    src: V,
    dest: V,
    weight: W,
}

impl<V, W> WeightedEdge<V, W>
where
    V: Hash + Eq,
    W: Copy,
{
    pub fn new_with_weight(src: V, dest: V, weight: W) -> Self {
        Self { src, dest, weight }
    }
}

impl<V, W> Edge for WeightedEdge<V, W>
where
    V: Hash + Eq,
    W: Copy + Default,
{
    type Value = V;

    fn source(&self) -> &Self::Value {
        &self.src
    }

    fn destination(&self) -> &Self::Value {
        &self.dest
    }

    fn new(src: Self::Value, dest: Self::Value) -> Self {
        Self {
            src,
            dest,
            weight: W::default()
        }
    }
}
