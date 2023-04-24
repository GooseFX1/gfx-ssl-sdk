use bytemuck::{Pod, Zeroable};
use std::{
    fmt::{self, Debug},
    mem::{forget, zeroed},
    ops::{Deref, Index, IndexMut},
};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct StackVec<T, const N: usize> {
    val: [T; N],
    n: u64,
}

impl<T, const N: usize> Debug for StackVec<T, N>
where
    T: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for i in 0..self.n as usize {
            if i + 1 != self.n as usize {
                write!(f, "{:?}, ", self.val[i])?;
            } else {
                write!(f, "{:?}", self.val[i])?;
            }
        }
        write!(f, "]")
    }
}

impl<T, const N: usize> Default for StackVec<T, N>
where
    T: Default,
{
    fn default() -> Self {
        let mut ret = Self {
            val: unsafe { zeroed() },
            n: 0,
        };

        for i in 0..N {
            ret.val[i as usize] = T::default();
        }

        ret
    }
}

impl<T, const N: usize> StackVec<T, N>
where
    T: Copy,
{
    pub fn new() -> Self {
        StackVec {
            val: [unsafe { zeroed() }; N],
            n: 0,
        }
    }

    pub fn push(&mut self, elem: T) -> Option<T> {
        if (self.n as usize) >= N {
            Some(elem)
        } else {
            self.val[self.n as usize] = elem;
            self.n += 1;
            None
        }
    }

    pub fn len(&self) -> usize {
        self.n as usize
    }

    pub fn remove(&mut self, idx: usize) -> T {
        if idx >= (self.n as usize) {
            panic!("index out of bound")
        }

        let mut v = unsafe { Vec::from_raw_parts(&mut self.val[0], self.n as usize, N) };
        let ret = v.remove(idx);
        self.n -= 1;
        forget(v);

        ret
    }
}

impl<T, const N: usize> Index<usize> for StackVec<T, N> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        if index >= (self.n as usize) {
            panic!("index out of bound")
        }

        &self.val[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for StackVec<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= (self.n as usize) {
            panic!("index out of bound")
        }

        &mut self.val[index]
    }
}

impl<T, const N: usize> Deref for StackVec<T, N> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        &self.val[..self.n as usize]
    }
}

unsafe impl<T, const N: usize> Pod for StackVec<T, N> where T: Pod {}
unsafe impl<T, const N: usize> Zeroable for StackVec<T, N> where T: Zeroable {}
