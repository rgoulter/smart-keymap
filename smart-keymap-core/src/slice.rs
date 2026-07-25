use core::mem::MaybeUninit;

use serde::Deserialize;

/// A helper value type for Copy-able slices.
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(from = "heapless::Vec<T, N>")]
pub struct Slice<T: Copy, const N: usize> {
    len: usize,
    data: [MaybeUninit<T>; N],
}

impl<T: Copy, const N: usize> Slice<T, N> {
    /// Constructs [Slice] from a slice of items.
    pub const fn from_slice(slice: &[T]) -> Self {
        let mut data = [const { MaybeUninit::uninit() }; N];
        if slice.len() > N {
            panic!("Slice length exceeds the maximum size for N");
        }
        let mut i = 0;
        let len = slice.len();
        while i < len {
            let item = slice[i];
            data[i] = MaybeUninit::new(item);
            i += 1;
        }
        Slice { len, data }
    }

    /// A slice representation of the data.
    pub const fn as_slice(&self) -> &[T] {
        unsafe { core::slice::from_raw_parts(self.data.as_ptr() as *const T, self.len) }
    }
}

impl<T: Copy, const N: usize> core::ops::Deref for Slice<T, N> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T: Copy + PartialEq, const N: usize> core::cmp::PartialEq for Slice<T, N> {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<T: Copy, const N: usize> core::convert::From<&[T]> for Slice<T, N> {
    fn from(slice: &[T]) -> Self {
        Self::from_slice(slice)
    }
}

impl<T: Copy, const N: usize> core::convert::From<heapless::Vec<T, N>> for Slice<T, N> {
    fn from(v: heapless::Vec<T, N>) -> Self {
        Self::from_slice(v.as_slice())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slice_as_slice_equals() {
        const N: usize = 5;
        type S = Slice<u8, N>;
        let expected: &[u8] = &[1, 2];
        let s: S = Slice::from_slice(expected);

        assert_eq!(expected, s.as_slice());
    }
}
