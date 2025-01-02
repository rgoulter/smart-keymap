use core::fmt::Debug;
use core::ops::{Index, IndexMut};

use crate::key;

use key::{composite, dynamic, simple};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Keys1<K0: key::Key, const L: key::layered::LayerIndex = 0>(
    dynamic::DynamicKey<K0, composite::Context<L, composite::DefaultNestableKey>, composite::Event>,
    key::layered::LayerIndex,
);

impl<K0: key::Key + Copy, const L: key::layered::LayerIndex> Keys1<K0, L> {
    pub const fn new((k0,): (K0,)) -> Self {
        Keys1(dynamic::DynamicKey::new(k0), L)
    }
}

impl<K0: key::Key + 'static, const L: key::layered::LayerIndex> Index<usize> for Keys1<K0, L>
where
    key::Event<<K0 as key::Key>::Event>: TryFrom<key::Event<composite::Event>>,
    key::ScheduledEvent<composite::Event>: From<key::ScheduledEvent<<K0 as key::Key>::Event>>,
    <K0 as key::Key>::Context: From<composite::Context<L, simple::Key>>,
{
    type Output = dyn dynamic::Key<
        key::composite::Event,
        Context = key::composite::Context<L, key::composite::DefaultNestableKey>,
    >;

    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.0,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl<K0: key::Key + 'static, const L: key::layered::LayerIndex> IndexMut<usize> for Keys1<K0, L>
where
    key::Event<<K0 as key::Key>::Event>: TryFrom<key::Event<composite::Event>>,
    key::ScheduledEvent<composite::Event>: From<key::ScheduledEvent<<K0 as key::Key>::Event>>,
    <K0 as key::Key>::Context: From<composite::Context<L, simple::Key>>,
{
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match idx {
            0 => &mut self.0,
            _ => panic!("Index out of bounds"),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Keys2<K0: key::Key, K1: key::Key, const L: key::layered::LayerIndex = 0>(
    dynamic::DynamicKey<K0, composite::Context<L, composite::DefaultNestableKey>, composite::Event>,
    dynamic::DynamicKey<K1, composite::Context<L, composite::DefaultNestableKey>, composite::Event>,
    key::layered::LayerIndex,
);

impl<K0: key::Key + Copy, K1: key::Key + Copy, const L: key::layered::LayerIndex> Keys2<K0, K1, L> {
    pub const fn new((k0, k1): (K0, K1)) -> Self {
        Keys2(
            dynamic::DynamicKey::new(k0),
            dynamic::DynamicKey::new(k1),
            L,
        )
    }
}

impl<K0: key::Key + 'static, K1: key::Key + 'static, const L: key::layered::LayerIndex> Index<usize>
    for Keys2<K0, K1, L>
where
    key::Event<<K0 as key::Key>::Event>: TryFrom<key::Event<composite::Event>>,
    key::ScheduledEvent<composite::Event>: From<key::ScheduledEvent<<K0 as key::Key>::Event>>,
    <K0 as key::Key>::Context: From<composite::Context<L, simple::Key>>,
    key::Event<<K1 as key::Key>::Event>: TryFrom<key::Event<composite::Event>>,
    key::ScheduledEvent<composite::Event>: From<key::ScheduledEvent<<K1 as key::Key>::Event>>,
    <K1 as key::Key>::Context: From<composite::Context<L, simple::Key>>,
{
    type Output = dyn dynamic::Key<
        key::composite::Event,
        Context = key::composite::Context<L, key::composite::DefaultNestableKey>,
    >;

    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.0,
            1 => &self.1,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl<K0: key::Key + 'static, K1: key::Key + 'static, const L: key::layered::LayerIndex>
    IndexMut<usize> for Keys2<K0, K1, L>
where
    key::Event<<K0 as key::Key>::Event>: TryFrom<key::Event<composite::Event>>,
    key::ScheduledEvent<composite::Event>: From<key::ScheduledEvent<<K0 as key::Key>::Event>>,
    <K0 as key::Key>::Context: From<composite::Context<L, simple::Key>>,
    key::Event<<K1 as key::Key>::Event>: TryFrom<key::Event<composite::Event>>,
    key::ScheduledEvent<composite::Event>: From<key::ScheduledEvent<<K1 as key::Key>::Event>>,
    <K1 as key::Key>::Context: From<composite::Context<L, simple::Key>>,
{
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match idx {
            0 => &mut self.0,
            1 => &mut self.1,
            _ => panic!("Index out of bounds"),
        }
    }
}
