use crate::types::{Matrix, Vector};
use core::ops::*;

#[derive(Clone, Copy)]
pub struct TransposedMatrixView<'a, T, const M: usize, const N: usize> {
	pub(crate) matrix: &'a Matrix<T, N, M>,
}

impl<'a, T: 'a, const M: usize, const N: usize> TransposedMatrixView<'a, T, M, N> {
	pub fn get(&self, index: usize) -> VectorView<'a, T, M, N> {
		debug_assert!(index <= M);
		VectorView {
			row: index,
			matrix: self.matrix,
		}
	}
}

impl<'a, T: 'a + Clone, const M: usize, const N: usize> TransposedMatrixView<'a, T, M, N> {
	pub fn materialize(self) -> Matrix<T, M, N> {
		self.into_iter()
			.map(IntoIterator::into_iter)
			.map(Iterator::cloned)
			.map(Iterator::collect)
			.collect()
	}
}

impl<'a, T, const M: usize, const N: usize> IntoIterator for TransposedMatrixView<'a, T, M, N> {
	type Item = VectorView<'a, T, M, N>;
	type IntoIter = TransIter<'a, T, M, N>;

	fn into_iter(self) -> Self::IntoIter {
		TransIter {
			pos: 0,
			trans: self,
		}
	}
}

pub struct TransIter<'a, T, const M: usize, const N: usize> {
	pos: usize,
	trans: TransposedMatrixView<'a, T, M, N>,
}

impl<'a, T, const M: usize, const N: usize> Iterator for TransIter<'a, T, M, N> {
	type Item = VectorView<'a, T, M, N>;
	fn next(&mut self) -> Option<VectorView<'a, T, M, N>> {
		if self.pos == M {
			None
		} else {
			let out = self.trans.get(self.pos);
			self.pos += 1;
			Some(out)
		}
	}
}

#[derive(Copy, Clone)]
pub struct VectorView<'a, T, const M: usize, const N: usize> {
	row: usize,
	matrix: &'a Matrix<T, N, M>,
}

impl<'a, T, const M: usize, const N: usize> IntoIterator for VectorView<'a, T, M, N> {
	type Item = &'a T;
	type IntoIter = ViewIter<'a, T, M, N>;

	fn into_iter(self) -> Self::IntoIter { ViewIter { pos: 0, view: self } }
}

pub struct ViewIter<'a, T, const M: usize, const N: usize> {
	pos: usize,
	view: VectorView<'a, T, M, N>,
}

impl<'a, T, const M: usize, const N: usize> Iterator for ViewIter<'a, T, M, N> {
	type Item = &'a T;
	fn next(&mut self) -> Option<&'a T> {
		debug_assert_eq!(N, self.view.matrix.inner.len());
		if self.pos == N {
			None
		} else {
			let out = Some(&self.view.matrix[self.pos][self.view.row]);
			self.pos += 1;
			out
		}
	}
}

impl<'a, 'b, T, const M: usize, const N: usize> Mul<&'b Vector<T, M>> for VectorView<'a, T, M, N>
where
	&'a T: Mul<&'b T, Output = T>,
{
	type Output = Vector<T, M>;
	fn mul(self, other: &'b Vector<T, M>) -> Vector<T, M> {
		self.into_iter()
			.zip(other.into_iter())
			.map(|(s, o)| s * o)
			.collect()
	}
}

impl<'a, 'b, T, const M: usize, const N: usize> Mul<VectorView<'b, T, M, N>> for &'a Vector<T, N>
where
	&'a T: Mul<&'b T, Output = T>,
{
	type Output = Vector<T, N>;
	fn mul(self, other: VectorView<'b, T, M, N>) -> Vector<T, N> {
		self.into_iter()
			.zip(other.into_iter())
			.map(|(s, o)| s * o)
			.collect()
	}
}

#[cfg(test)]
use crate::base::TESTLEN;
