use core::fmt::Debug;
use core::ops::{Index, IndexMut};

use crate::key;

use key::dynamic;

/// A trait for resetting all keys in a tuple struct.
pub trait KeysReset {
    /// Reset all keys.
    fn reset(&mut self);
}

/// A tuple struct for 1 key.
#[derive(Debug)]
pub struct Keys1<
    K0: key::Key,
    Ctx: key::Context<Event = Ev> + Debug,
    Ev: Copy + Debug,
    const M: usize = 2,
>(dynamic::DynamicKey<K0, Ctx, Ev>);

impl<K0: key::Key, Ctx: key::Context<Event = Ev> + Debug, Ev: Copy + Debug, const M: usize>
    Keys1<K0, Ctx, Ev, M>
{
    /// Constructs a KeysN for the given tuple.
    pub const fn new((k0,): (K0,)) -> Self {
        Keys1(dynamic::DynamicKey::new(k0))
    }
}

impl<
        K0: key::Key + 'static,
        Ctx: key::Context<Event = Ev> + Debug + 'static,
        Ev: Copy + Debug + 'static,
        const M: usize,
    > Index<usize> for Keys1<K0, Ctx, Ev, M>
where
    <K0 as key::Key>::Event: TryFrom<Ev>,
    Ev: From<<K0 as key::Key>::Event>,
    <K0 as key::Key>::Context: From<Ctx>,
{
    type Output = dyn dynamic::Key<Ev, M, Context = Ctx>;

    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.0,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl<
        K0: crate::key::Key + 'static,
        Ctx: crate::key::Context<Event = Ev> + Debug + 'static,
        Ev: Copy + Debug + 'static,
        const M: usize,
    > IndexMut<usize> for Keys1<K0, Ctx, Ev, M>
where
    <K0 as crate::key::Key>::Event: TryFrom<Ev>,
    Ev: From<<K0 as crate::key::Key>::Event>,
    <K0 as crate::key::Key>::Context: From<Ctx>,
{
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match idx {
            0 => &mut self.0,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl<
        K0: crate::key::Key + 'static,
        Ctx: crate::key::Context<Event = Ev> + Debug + 'static,
        Ev: Copy + Debug + 'static,
        const M: usize,
    > KeysReset for Keys1<K0, Ctx, Ev, M>
where
    <K0 as crate::key::Key>::Event: TryFrom<Ev>,
    Ev: From<<K0 as crate::key::Key>::Event>,
    <K0 as crate::key::Key>::Context: From<Ctx>,
{
    fn reset(&mut self) {
        <dynamic::DynamicKey<K0, Ctx, Ev> as dynamic::Key<Ev, M>>::reset(&mut self.0)
    }
}

// Use seq_macro's seq! to generate Keys2, Keys3, etc.

macro_rules! define_keys {
    ($n:expr) => {
        paste::paste! {
            seq_macro::seq!(I in 0..$n {
                /// A tuple struct for some number of keys.
                #[derive(core::fmt::Debug)]
                pub struct [<Keys $n>]<
                    #(
                        K~I: crate::key::Key,
                    )*
                Ctx: crate::key::Context<Event = Ev> + core::fmt::Debug,
                Ev: Copy + core::fmt::Debug,
                const M: usize = 2,
                >(
                    #(
                        crate::key::dynamic::DynamicKey<K~I, Ctx, Ev>,
                    )*
                );

                impl<
                    #(
                        K~I: crate::key::Key,
                    )*
                Ctx: crate::key::Context<Event = Ev> + core::fmt::Debug,
                Ev: Copy + core::fmt::Debug,
                const M: usize,
                > [<Keys $n>]<
                    #(K~I,)*
                Ctx, Ev, M
                    >
                {
                    /// Constructs a KeysN tuple struct with the given tuple.
                    pub const fn new((
                        #(k~I,)*
                    ): (
                        #(K~I,)*
                    )) -> Self {
                        [<Keys $n>](
                            #(
                                crate::key::dynamic::DynamicKey::new(k~I),
                            )*
                        )
                    }
                }

                impl<
                    #(
                        K~I: crate::key::Key + 'static,
                    )*
                Ctx: crate::key::Context<Event = Ev> + core::fmt::Debug + 'static,
                Ev: Copy + core::fmt::Debug + 'static,
                const M: usize,
                > core::ops::Index<usize> for [<Keys $n>]<
                    #(K~I,)*
                Ctx, Ev, M
                    >
                where
                    #(
                    <K~I as crate::key::Key>::Event: TryFrom<Ev>,
                    Ev: From<<K~I as crate::key::Key>::Event>,
                    <K~I as crate::key::Key>::Context: From<Ctx>,
                )*
                {
                    type Output = dyn crate::key::dynamic::Key<Ev, M, Context = Ctx>;

                    fn index(&self, idx: usize) -> &Self::Output {
                        match idx {
                            #(
                                I => &self.I,
                            )*
                            _ => panic!("Index out of bounds"),
                        }
                    }
                }

                impl<
                    #(
                        K~I: crate::key::Key + 'static,
                    )*
                Ctx: crate::key::Context<Event = Ev> + core::fmt::Debug + 'static,
                Ev: Copy + core::fmt::Debug + 'static,
                const M: usize,
                > core::ops::IndexMut<usize> for [<Keys $n>]<
                    #(K~I,)*
                Ctx, Ev, M
                    >
                where
                    #(
                    <K~I as crate::key::Key>::Event: TryFrom<Ev>,
                    Ev: From<<K~I as crate::key::Key>::Event>,
                    <K~I as crate::key::Key>::Context: From<Ctx>,
                )*
                {
                    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
                        match idx {
                            #(
                                I => &mut self.I,
                            )*
                            _ => panic!("Index out of bounds"),
                        }
                    }
                }

                impl<
                    #(
                        K~I: crate::key::Key + 'static,
                    )*
                Ctx: crate::key::Context<Event = Ev> + core::fmt::Debug + 'static,
                Ev: Copy + core::fmt::Debug + 'static,
                const M: usize,
                > crate::tuples::KeysReset for [<Keys $n>]<
                    #(K~I,)*
                Ctx, Ev, M
                    >
                where
                    #(
                    <K~I as crate::key::Key>::Event: TryFrom<Ev>,
                    Ev: From<<K~I as crate::key::Key>::Event>,
                    <K~I as crate::key::Key>::Context: From<Ctx>,
                )*
                {
                    fn reset(&mut self) {
                        #(
                        <crate::key::dynamic::DynamicKey<K~I, Ctx, Ev> as crate::key::dynamic::Key<Ev, M>>::reset(&mut self.I);
                        )*
                    }
                }
            });
        }
    };
}

pub(crate) use define_keys;

define_keys!(2);

define_keys!(4);
