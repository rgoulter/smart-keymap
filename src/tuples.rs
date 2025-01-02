use core::fmt::Debug;
use core::ops::{Index, IndexMut};

use crate::key;

use key::{composite, dynamic};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Keys1<
    K0: key::Key,
    Ctx: key::Context<Event = Ev> + Debug = composite::Context<0, composite::DefaultNestableKey>,
    Ev: Copy + Debug + Ord = composite::Event,
    const N: usize = 2,
>(dynamic::DynamicKey<K0, Ctx, Ev>);

impl<
        K0: key::Key + Copy,
        Ctx: key::Context<Event = Ev> + Debug,
        Ev: Copy + Debug + Ord,
        const N: usize,
    > Keys1<K0, Ctx, Ev, N>
{
    pub const fn new((k0,): (K0,)) -> Self {
        Keys1(dynamic::DynamicKey::new(k0))
    }
}

impl<
        K0: key::Key + 'static,
        Ctx: key::Context<Event = Ev> + Debug + 'static,
        Ev: Copy + Debug + Ord + 'static,
        const N: usize,
    > Index<usize> for Keys1<K0, Ctx, Ev, N>
where
    key::Event<<K0 as key::Key>::Event>: TryFrom<key::Event<Ev>>,
    key::ScheduledEvent<Ev>: From<key::ScheduledEvent<<K0 as key::Key>::Event>>,
    <K0 as key::Key>::Context: From<Ctx>,
{
    type Output = dyn dynamic::Key<Ev, N, Context = Ctx>;

    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.0,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl<
        K0: key::Key + 'static,
        Ctx: key::Context<Event = Ev> + Debug + 'static,
        Ev: Copy + Debug + Ord + 'static,
        const N: usize,
    > IndexMut<usize> for Keys1<K0, Ctx, Ev, N>
where
    key::Event<<K0 as key::Key>::Event>: TryFrom<key::Event<Ev>>,
    key::ScheduledEvent<Ev>: From<key::ScheduledEvent<<K0 as key::Key>::Event>>,
    <K0 as key::Key>::Context: From<Ctx>,
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
pub struct Keys2<
    K0: key::Key,
    K1: key::Key,
    Ctx: key::Context<Event = Ev> + Debug = composite::Context<0, composite::DefaultNestableKey>,
    Ev: Copy + Debug + Ord = composite::Event,
    const N: usize = 2,
>(
    dynamic::DynamicKey<K0, Ctx, Ev>,
    dynamic::DynamicKey<K1, Ctx, Ev>,
);

impl<
        K0: key::Key + Copy,
        K1: key::Key + Copy,
        Ctx: key::Context<Event = Ev> + Debug,
        Ev: Copy + Debug + Ord,
        const N: usize,
    > Keys2<K0, K1, Ctx, Ev, N>
{
    pub const fn new((k0, k1): (K0, K1)) -> Self {
        Keys2(dynamic::DynamicKey::new(k0), dynamic::DynamicKey::new(k1))
    }
}

impl<
        K0: key::Key + 'static,
        K1: key::Key + 'static,
        Ctx: key::Context<Event = Ev> + Debug + 'static,
        Ev: Copy + Debug + Ord + 'static,
        const N: usize,
    > Index<usize> for Keys2<K0, K1, Ctx, Ev, N>
where
    key::Event<<K0 as key::Key>::Event>: TryFrom<key::Event<Ev>>,
    key::ScheduledEvent<Ev>: From<key::ScheduledEvent<<K0 as key::Key>::Event>>,
    <K0 as key::Key>::Context: From<Ctx>,
    key::Event<<K1 as key::Key>::Event>: TryFrom<key::Event<Ev>>,
    key::ScheduledEvent<Ev>: From<key::ScheduledEvent<<K1 as key::Key>::Event>>,
    <K1 as key::Key>::Context: From<Ctx>,
{
    type Output = dyn dynamic::Key<Ev, N, Context = Ctx>;

    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.0,
            1 => &self.1,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl<
        K0: key::Key + 'static,
        K1: key::Key + 'static,
        Ctx: key::Context<Event = Ev> + Debug + 'static,
        Ev: Copy + Debug + Ord + 'static,
        const N: usize,
    > IndexMut<usize> for Keys2<K0, K1, Ctx, Ev, N>
where
    key::Event<<K0 as key::Key>::Event>: TryFrom<key::Event<Ev>>,
    key::ScheduledEvent<Ev>: From<key::ScheduledEvent<<K0 as key::Key>::Event>>,
    <K0 as key::Key>::Context: From<Ctx>,
    key::Event<<K1 as key::Key>::Event>: TryFrom<key::Event<Ev>>,
    key::ScheduledEvent<Ev>: From<key::ScheduledEvent<<K1 as key::Key>::Event>>,
    <K1 as key::Key>::Context: From<Ctx>,
{
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match idx {
            0 => &mut self.0,
            1 => &mut self.1,
            _ => panic!("Index out of bounds"),
        }
    }
}
