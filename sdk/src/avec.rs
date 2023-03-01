use std::ops::{Deref, DerefMut};

#[derive(Clone, Copy, Debug)]
#[repr(C, align(8))]
pub struct A8Bytes<const N: usize>(pub [u8; N]);

impl<const N: usize> Default for A8Bytes<N> {
    fn default() -> Self {
        Self([0; N])
    }
}

impl<const N: usize> Deref for A8Bytes<N> {
    type Target = [u8; N];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> DerefMut for A8Bytes<N> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const N: usize> AsRef<[u8]> for A8Bytes<N> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl<const N: usize> AsMut<[u8]> for A8Bytes<N> {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        self.0.as_mut()
    }
}

impl<const N: usize> TryFrom<Vec<u8>> for A8Bytes<N> {
    type Error = <[u8; N] as TryFrom<Vec<u8>>>::Error;
    #[inline]
    fn try_from(v: Vec<u8>) -> Result<Self, Self::Error> {
        let s: [u8; N] = v.try_into()?;
        Ok(Self(s))
    }
}
