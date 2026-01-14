use std::ops::{Add, Sub, Mul};
use std::fmt;

pub struct Matrix<T, const ROWS: usize, const COLUMNS: usize> {
    pub data: [[T; COLUMNS]; ROWS],
}

impl<T, const R: usize, const C: usize> Matrix<T, R, C>
where
    T: Copy + Default,
{
    #[inline]
    pub fn zero() -> Self {
        Self {
            data: [[T::default(); C]; R],
        }
    }

    #[inline]
    pub fn from_array(data: [[T; C]; R]) -> Self {
        Self { data }
    }

    #[inline]
    pub const fn rows(&self) -> usize {
        R
    }

    #[inline]
    pub const fn columns(&self) -> usize {
        C
    }
}

/// From array trait
impl<T, const R: usize, const C: usize>
    From<[[T; C]; R]> for Matrix<T, R, C>
where
    T: Copy,
{
    fn from(data: [[T; C]; R]) -> Self {
        Self { data }
    }
}

/// Debug display ---
impl<T: fmt::Display, const R: usize, const C: usize> fmt::Display for Matrix<T, R, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..R {
            writeln!(f, "{:?}", &self.data[i])?;
        }
        Ok(())
    }
}

/// Elementwise addition
impl<'a, 'b, T, const R: usize, const C: usize>
    Add<&'b Matrix<T, R, C>> for &'a Matrix<T, R, C>
where
    T: Copy + Add<Output = T> + Default,
{
    type Output = Matrix<T, R, C>;

    #[inline]
    fn add(self, rhs: &'b Matrix<T, R, C>) -> Self {
        let mut result = Matrix::<T, R, C>::zero();
        for i in 0..R {
            for j in 0..C {
                result.data[i][j] = self.data[i][j] + rhs.data[i][j];
            }
        }
        result
    }
}

/// Elementwise subtraction
impl<'a, 'b, T, const R: usize, const C: usize>
    Sub<&'b Matrix<T, R, C>> for &'a Matrix<T, R, C>
where
    T: Copy + Sub<Output = T> + Default,
{
    type Output = Matrix<T, R, C>;

    #[inline]
    fn sub(self, rhs: &'b Matrix<T, R, C>) -> Self {
        let mut result = Matrix::<T, R, C>::zero();
        for i in 0..R {
            for j in 0..C {
                result.data[i][j] = self.data[i][j] - rhs.data[i][j];
            }
        }
        result
    }
}

/// Scalar multiplication
impl<'a, T, const R: usize, const C: usize>
    Mul<T> for &'a Matrix<T, R, C>
where
    T: Copy + Mul<Output = T> + Default,
{
    type Output = Matrix<T, R, C>;

    #[inline]
    fn mul(self, rhs: T) -> Self {
        let mut result = Matrix::<T, R, C>::zero();
        for i in 0..R {
            for j in 0..C {
                result.data[i][j] = self.data[i][j] * rhs;
            }
        }
        result
    }
}

/// Matrix multiplication
impl<'a, 'b, T, const R: usize, const C: usize, const K: usize>
    Mul<&'b Matrix<T, C, K>> for &'a Matrix<T, R, C>
where
    T: Copy + Add<Output = T> + Mul<Output = T> + Default,
{
    type Output = Matrix<T, R, K>;

    #[inline]
    fn mul(self, rhs: &'b Matrix<T, C, K>) -> Self::Output {
        let mut result = Matrix::<T, R, K>::zero();
        for i in 0..R {
            for j in 0..K {
                for k in 0..C {
                    result.data[i][j] = result.data[i][j] + self.data[i][k] * rhs.data[k][j];
                }
            }
        }
        result
    }
}