use anchor_lang::prelude::*;
use std::ops::{Deref, Index, IndexMut};
use std::mem::{forget, zeroed};
#[cfg(feature = "no-entrypoint")]
use std::fmt::{self, Debug};
use std::io::Write;
use anchor_lang::prelude::borsh::{BorshDeserialize, BorshSerialize};

#[repr(C)]
pub struct StackVec<T, const N: usize>
    where T: BorshSerialize + BorshDeserialize + Default + Copy,
{
    val: [T; N],
    n: u64,
}

impl<T, const N: usize> BorshSerialize for StackVec<T, N>
    where T: BorshSerialize + BorshDeserialize + Default + Copy,
{
    #[inline]
    fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        if N == 0 {
            writer.write_all(self.n.to_le_bytes().as_slice())?;
            return Ok(());
        } else if let Some(u8_slice) = T::u8_slice(&self.val) {
            writer.write_all(u8_slice)?;
        } else {
            for el in self.val.iter() {
                el.serialize(writer)?;
            }
        }
        writer.write_all(self.n.to_le_bytes().as_slice())?;
        Ok(())
    }
}

impl<T> BorshDeserialize for StackVec<T, 5>
    where T: BorshSerialize + BorshDeserialize + Default + Copy,
{
    fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        todo!()
    }
}

#[cfg(feature = "no-entrypoint")]
impl<T, const N: usize> Debug for StackVec<T, N>
where
    T: BorshSerialize + BorshDeserialize + Default + Copy + Debug,
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
    T: BorshSerialize + BorshDeserialize + Copy + Default,
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
    T: BorshSerialize + BorshDeserialize + Default + Copy,
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

impl<T, const N: usize> Index<usize> for StackVec<T, N>
    where T: BorshSerialize + BorshDeserialize + Default + Copy,
{
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        if index >= (self.n as usize) {
            panic!("index out of bound")
        }

        &self.val[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for StackVec<T, N>
    where T: BorshSerialize + BorshDeserialize + Default + Copy,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= (self.n as usize) {
            panic!("index out of bound")
        }

        &mut self.val[index]
    }
}

impl<T, const N: usize> Deref for StackVec<T, N>
    where T: BorshSerialize + BorshDeserialize + Default + Copy,
{
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        &self.val[..self.n as usize]
    }
}
