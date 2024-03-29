mod ops;
mod transform;
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<const N: usize>(pub [[f32; N]; N]);

impl<const N: usize> Default for Matrix<N> {
    fn default() -> Self {
        Self([[f32::default(); N]; N])
    }
}

impl<const N: usize> From<[[f32; N]; N]> for Matrix<N> {
    fn from(data: [[f32; N]; N]) -> Self {
        Self(data)
    }
}



impl<const N: usize, const M: usize> From<&Matrix<M>> for Matrix<N> {
    fn from(other: &Matrix<M>) -> Self {
        let mut m = Self::new();
        let s = if N < M { N } else { M };
        for (i, j) in Self::index_iter_with_size(s) {
            unsafe {
                let v = other.get_unchecked(i, j);
                m.set_unchecked(i, j, v);
            }
        }
        m
    }
}

impl<const N: usize> Matrix<N> {
    pub fn new() -> Self {
        Self::default()
    }

    pub const fn dimension() -> usize {
        N
    }

    pub unsafe fn get_unchecked(&self, i: usize, j: usize) -> f32 {
        *self.0.get_unchecked(i).get_unchecked(j)
    }

    pub unsafe fn get_unchecked_mut(&mut self, i: usize, j: usize) -> &mut f32 {
        self.0.get_unchecked_mut(i).get_unchecked_mut(j)
    }

    pub unsafe fn set_unchecked(&mut self, i: usize, j: usize, value: f32) {
        *self.get_unchecked_mut(i, j) = value;
    }

    // Return the transposed matrix.
    // It won't change self.
    pub fn transpose(&self) -> Matrix<N> {
        let mut m = Matrix::<N>::new();
        for (i, j) in Self::index_iter_with_size(N) {
            unsafe {
                let v = self.get_unchecked(i, j);
                m.set_unchecked(j, i, v);
            }
        }
        m
    }
    pub fn index_iter_with_size(n: usize) -> impl Iterator<Item = (usize, usize)> {
        (0..n).flat_map(move |a| (0..n).map(move |b| (a, b)))
    }
    pub fn index_iter() -> impl Iterator<Item = (usize, usize)> {
        (0..N).flat_map(move |a| (0..N).map(move |b| (a, b)))
    }

    pub fn cwise_product(&self, other: &Self) -> Self {
        let mut m = Self::new();
        m.data_iter_mut()
            .zip(self.data_iter().zip(other.data_iter()))
            .for_each(|(l, (&r1, &r2))| *l = r1 * r2);
        m
    }

    pub fn data_iter(&self) -> impl Iterator<Item = &f32> + Clone {
        self.0.iter().flat_map(|v| v.iter())
    }

    pub fn data_iter_mut(&mut self) -> impl Iterator<Item = &mut f32> {
        self.0.iter_mut().flat_map(|v| v.iter_mut())
    }

    pub fn unit() -> Self {
        let mut m = Self::new();
        m.data_iter_mut().for_each(|v| *v = 1.0);
        m
    }
}

impl<const N: usize> Matrix<N> {
    pub fn inverse_matrix(&self) -> Self {
        let det = self.determinant();
        (self.cofactor().transpose()) * (1.0 / det)
    }

    fn minor_iter<'a>(
        iter: impl Iterator<Item = &'a f32> + Clone,
        i: usize,
        n: usize,
    ) -> impl Iterator<Item = &'a f32> + Clone {
        iter.enumerate().filter_map(move |(idx, v)| {
            if idx / n != i / n && idx % n != i % n {
                Some(v)
            } else {
                None
            }
        })
    }

    fn cofactor(&self) -> Self {
        let mut m = Self::new();
        for i in 0..N {
            let minor = Self::minor_iter(self.data_iter(), i, N);
            let det: f32 = Self::determinant_iter(minor, N - 1);
            let v = if i % 2 == 0 { det } else { -det };
            unsafe {
                m.set_unchecked(i, 0, v);
            }
        }
        m
    }

    pub fn determinant(&self) -> f32 {
        let iter = self.data_iter();
        Self::determinant_iter(iter, N)
    }

    fn determinant_iter<'a>(mut iter: impl Iterator<Item = &'a f32> + Clone, size: usize) -> f32 {
        match size {
            1 => *iter.next().unwrap(),
            2 => {
                let a = *iter.next().unwrap();
                let b = *iter.next().unwrap();
                let c = *iter.next().unwrap();
                let d = *iter.next().unwrap();
                a * d - b * c
            }
            _ => {
                let mut det = 0.0;
                let data_iter = iter.clone().take(size);
                for (i, v) in (0..size).zip(data_iter) {
                    let it = iter.clone();
                    let minor_iter = Self::minor_iter(it, i, size);
                    let minor_det = Self::determinant_iter(minor_iter, size - 1);
                    let d = minor_det * *v;
                    det += if i % 2 == 0 { d } else { -d };
                }
                det
            }
        }
    }
}

pub type Matrix3 = Matrix<3>;
pub type Matrix4 = Matrix<4>;

macro_rules! def_matrix_func {
    ($func: ident, $n: expr) => {
        pub fn $func(data: [[f32; $n]; $n]) -> Matrix<$n> {
            Matrix::<$n>(data)
        }
    };
}

def_matrix_func!(matrix3, 3);
def_matrix_func!(matrix4, 4);
