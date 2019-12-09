#![allow(soft_unstable)]
#![cfg_attr(feature = "nightly", feature(test))]
#[cfg(feature = "nightly")]
extern crate test;

use std::collections::HashMap;
use std::hash::Hash;
use vector_map::VecMap;

#[derive(Clone, Debug)]
enum MapSupport<K, V>
where
    K: PartialEq + Eq + Hash,
{
    Vector(VecMap<K, V>),
    Hash(HashMap<K, V>),
}

impl<K, V> MapSupport<K, V>
where
    K: PartialEq + Eq + Hash,
{
    fn convert_to_vec(self) -> Self {
        if let MapSupport::Hash(values) = self {
            MapSupport::Vector(values.into_iter().collect())
        } else {self}
    }

    fn convert_to_hashmap(self) -> Self {
        if let MapSupport::Vector(values) = self {
            MapSupport::Hash(values.into_iter().collect())
        } else {self}
    }

    pub fn iter<'l>(&'l self) -> Box<dyn Iterator<Item = (&'l K, &'l V)> + 'l> {
        match self {
            MapSupport::Vector(values) => values.iter(),
            MapSupport::Hash(values) => Box::new(values.iter()),
        }
    }

    pub fn iter_mut<'l>(&'l mut self) -> Box<dyn Iterator<Item = (&'l K, &'l mut V)> + 'l> {
        match self {
            MapSupport::Vector(values) => values.iter_mut(),
            MapSupport::Hash(values) => Box::new(values.iter_mut()),
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self {
            MapSupport::Vector(values) => values.insert(key, value),
            MapSupport::Hash(values) => values.insert(key, value),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            MapSupport::Vector(values) => {values.inner().len()},
            MapSupport::Hash(values) => {values.len()},
        }
    }
}

impl<K, V> IntoIterator for MapSupport<K, V>
where
    K: PartialEq + Eq + Hash + 'static,
    V: 'static,
{
    type Item = (K, V);
    type IntoIter = Box<dyn Iterator<Item = (K, V)>>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            MapSupport::Vector(values) => Box::new(values.into_iter()),
            MapSupport::Hash(values) => Box::new(values.into_iter()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct OptiMap<K, V>
where
    K: PartialEq + Eq + Hash,
{
    inner: MapSupport<K, V>,
    switch_size: usize,
}

impl<K, V> OptiMap<K, V>
where
    K: PartialEq + Eq + Hash,
{
    pub fn new() -> Self {
        OptiMap {
            inner: MapSupport::Vector(Default::default()),
            switch_size: 128,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let switch_size = 128;
        OptiMap {
            inner: if capacity < switch_size {
                MapSupport::Vector(VecMap::with_capacity(capacity))
            } else {
                MapSupport::Hash(HashMap::with_capacity(capacity))
            },
            switch_size,
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.len() == self.switch_size {
            let mut convertible: std::mem::MaybeUninit<MapSupport<K, V>> = unsafe{ std::mem::MaybeUninit::uninit()};
            std::mem::swap(unsafe {&mut *convertible.as_mut_ptr()}, &mut self.inner);
            let mut convertible = unsafe {std::mem::MaybeUninit::new(convertible.assume_init().convert_to_hashmap())};
            std::mem::swap(unsafe {&mut *convertible.as_mut_ptr()}, &mut self.inner);
        }
        self.inner.insert(key, value)
    }
}

#[cfg(feature = "nightly")]
mod bench {
    macro_rules! bench_module {
        ($name: ident, $type: path) => {
            mod $name {
                use $type as Map;
                mod insert {
                    use super::Map;
                    #[bench]
                    fn smalltype_00_016(b: &mut test::Bencher) {
                        b.iter(|| {
                            let mut map = Map::new();
                            for i in 0usize..00_064 {
                                map.insert(i, i);
                            }
                        })
                    }
                    #[bench]
                    fn smalltype_00_064(b: &mut test::Bencher) {
                        b.iter(|| {
                            let mut map = Map::new();
                            for i in 0usize..00_064 {
                                map.insert(i, i);
                            }
                        })
                    }
                    #[bench]
                    fn smalltype_00_128(b: &mut test::Bencher) {
                        b.iter(|| {
                            let mut map = Map::new();
                            for i in 0usize..00_128 {
                                map.insert(i, i);
                            }
                        })
                    }
                    #[bench]
                    fn smalltype_00_129(b: &mut test::Bencher) {
                        b.iter(|| {
                            let mut map = Map::new();
                            for i in 0usize..00_129 {
                                map.insert(i, i);
                            }
                        })
                    }
                    #[bench]
                    fn smalltype_00_256(b: &mut test::Bencher) {
                        b.iter(|| {
                            let mut map = Map::new();
                            for i in 0usize..00_256 {
                                map.insert(i, i);
                            }
                        })
                    }
                    #[bench]
                    fn smalltype_01_024(b: &mut test::Bencher) {
                        b.iter(|| {
                            let mut map = Map::new();
                            for i in 0usize..01_024 {
                                map.insert(i, i);
                            }
                        })
                    }
                    #[bench]
                    fn smalltype_16_000(b: &mut test::Bencher) {
                        b.iter(|| {
                            let mut map = Map::new();
                            for i in 0usize..16_000 {
                                map.insert(i, i);
                            }
                        })
                    }
                    #[bench]
                    fn bigtype_00_016(b: &mut test::Bencher) {
                        b.iter(|| {
                            let mut map = Map::new();
                            for i in 0usize..00_064 {
                                map.insert(i, (i, i, i, i, i, i, i));
                            }
                        })
                    }
                    #[bench]
                    fn bigtype_00_064(b: &mut test::Bencher) {
                        b.iter(|| {
                            let mut map = Map::new();
                            for i in 0usize..00_064 {
                                map.insert(i, (i, i, i, i, i, i, i));
                            }
                        })
                    }
                    #[bench]
                    fn bigtype_00_128(b: &mut test::Bencher) {
                        b.iter(|| {
                            let mut map = Map::new();
                            for i in 0usize..00_128 {
                                map.insert(i, (i, i, i, i, i, i, i));
                            }
                        })
                    }
                    #[bench]
                    fn bigtype_00_129(b: &mut test::Bencher) {
                        b.iter(|| {
                            let mut map = Map::new();
                            for i in 0usize..00_129 {
                                map.insert(i, (i, i, i, i, i, i, i));
                            }
                        })
                    }
                    #[bench]
                    fn bigtype_00_256(b: &mut test::Bencher) {
                        b.iter(|| {
                            let mut map = Map::new();
                            for i in 0usize..00_256 {
                                map.insert(i, (i, i, i, i, i, i, i));
                            }
                        })
                    }
                    #[bench]
                    fn bigtype_01_024(b: &mut test::Bencher) {
                        b.iter(|| {
                            let mut map = Map::new();
                            for i in 0usize..01_024 {
                                map.insert(i, (i, i, i, i, i, i, i));
                            }
                        })
                    }
                    #[bench]
                    fn bigtype_16_000(b: &mut test::Bencher) {
                        b.iter(|| {
                            let mut map = Map::new();
                            for i in 0usize..16_000 {
                                map.insert(i, (i, i, i, i, i, i, i));
                            }
                        })
                    }
                }
            }
        };
    }

    bench_module!(vecmap, vector_map::VecMap);
    bench_module!(hashmap, std::collections::HashMap);
    bench_module!(optimap, crate::OptiMap);
}