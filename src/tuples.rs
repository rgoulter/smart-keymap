use core::fmt::Debug;
use core::ops::{Index, IndexMut};

use crate::key;

/// A tuple struct for 1 key.
#[derive(Debug)]
pub struct Keys1<
    K0: key::Key<Context = Ctx, Event = Ev, PressedKey = PK>,
    Ctx,
    Ev,
    PK,
    const M: usize = 2,
>(K0);

impl<
        K0: key::Key<Context = Ctx, Event = Ev, PressedKey = PK> + Copy,
        Ctx,
        Ev,
        PK,
        const M: usize,
    > Keys1<K0, Ctx, Ev, PK, M>
{
    /// Constructs a KeysN for the given tuple.
    pub const fn new((k0,): (K0,)) -> Self {
        Keys1(k0)
    }
}

impl<
        K0: key::Key<Context = Ctx, Event = Ev, PressedKey = PK> + 'static,
        Ctx,
        Ev,
        PK,
        const M: usize,
    > Index<usize> for Keys1<K0, Ctx, Ev, PK, M>
{
    type Output = dyn key::Key<Context = Ctx, Event = Ev, PressedKey = PK>;

    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.0,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl<
        K0: key::Key<Context = Ctx, Event = Ev, PressedKey = PK> + 'static,
        Ctx,
        Ev,
        PK,
        const M: usize,
    > IndexMut<usize> for Keys1<K0, Ctx, Ev, PK, M>
{
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match idx {
            0 => &mut self.0,
            _ => panic!("Index out of bounds"),
        }
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
                        K~I: crate::key::Key<Context = Ctx, Event = Ev, PressedKey = PK>,
                    )*
                Ctx,
                Ev,
                PK,
                const M: usize = 2,
                >(
                    #(
                        K~I,
                    )*
                );

                impl<
                    #(
                        K~I: crate::key::Key<Context = Ctx, Event = Ev, PressedKey = PK> + Copy,
                    )*
                Ctx,
                Ev,
                PK,
                const M: usize,
                > [<Keys $n>]<
                    #(K~I,)*
                Ctx, Ev, PK, M
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
                                (k~I),
                            )*
                        )
                    }
                }

                impl<
                    #(
                        K~I: crate::key::Key<Context = Ctx, Event = Ev, PressedKey = PK> + 'static,
                    )*
                Ctx,
                Ev,
                PK,
                const M: usize,
                > core::ops::Index<usize> for [<Keys $n>]<
                    #(K~I,)*
                Ctx, Ev, PK, M
                    >
                {
                    type Output = dyn crate::key::Key<Context = Ctx, Event = Ev, PressedKey = PK>;

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
                        K~I: crate::key::Key<Context = Ctx, Event = Ev, PressedKey = PK> + 'static,
                    )*
                Ctx,
                Ev,
                PK,
                const M: usize,
                > core::ops::IndexMut<usize> for [<Keys $n>]<
                    #(K~I,)*
                Ctx, Ev, PK, M
                    >
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
            });
        }
    };
}

pub(crate) use define_keys;

define_keys!(2);

define_keys!(4);
