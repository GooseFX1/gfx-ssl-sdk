use std::ops::{Deref, Index, IndexMut};
use anchor_lang::prelude::*;
use std::mem::{forget, zeroed};
#[cfg(feature = "no-entrypoint")]
use std::fmt::{self, Debug};
use anchor_lang::prelude::borsh::{BorshDeserialize, BorshSerialize};

#[account]
#[derive(Copy)]
#[repr(C)]
pub struct StackVec5<T: BorshSerialize + BorshDeserialize + Default + Copy> {
    val: [T; 5],
    n: u64,
}

#[cfg(feature = "no-entrypoint")]
impl<T: BorshSerialize + BorshDeserialize + Default + Copy> Debug for StackVec5<T>
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

impl<T: BorshSerialize + BorshDeserialize + Default + Copy> Default for StackVec5<T>
where
    T: Default,
{
    fn default() -> Self {
        let mut ret = Self {
            val: unsafe { zeroed() },
            n: 0,
        };

        for i in 0..5 {
            ret.val[i as usize] = T::default();
        }

        ret
    }
}

impl<T: BorshSerialize + BorshDeserialize + Default + Copy> StackVec5<T>
where
    T: Copy,
{
    pub fn new() -> Self {
        Self {
            val: [unsafe { zeroed() }; 5],
            n: 0,
        }
    }

    pub fn push(&mut self, elem: T) -> Option<T> {
        if (self.n as usize) >= 5 {
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

        let mut v = unsafe { Vec::from_raw_parts(&mut self.val[0], self.n as usize, 5) };
        let ret = v.remove(idx);
        self.n -= 1;
        forget(v);

        ret
    }
}

impl<T: BorshSerialize + BorshDeserialize + Default + Copy> Index<usize> for StackVec5<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        if index >= (self.n as usize) {
            panic!("index out of bound")
        }

        &self.val[index]
    }
}

impl<T: BorshSerialize + BorshDeserialize + Default + Copy> IndexMut<usize> for StackVec5<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= (self.n as usize) {
            panic!("index out of bound")
        }

        &mut self.val[index]
    }
}

impl<T: BorshSerialize + BorshDeserialize + Default + Copy> Deref for StackVec5<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        &self.val[..self.n as usize]
    }
}
