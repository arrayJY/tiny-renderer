mod ops;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct Vector<const N: usize>(pub [f32; N]);

impl<const N: usize> Vector<N> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn data_iter(&self) -> impl Iterator<Item = &f32> + Clone {
        self.0.iter()
    }

    pub fn data_iter_mut(&mut self) -> impl Iterator<Item = &mut f32> {
        self.0.iter_mut()
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.data_iter()
            .zip(other.data_iter())
            .map(|(&a, &b)| a * b)
            .sum()
    }

    pub fn norm(&self) -> f32 {
        self.data_iter().map(|v| v.powi(2)).sum::<f32>().sqrt()
    }

    pub fn normalize(&mut self) {
        let norm = self.norm();
        self.data_iter_mut().for_each(|v| *v /= norm);
    }

    pub fn normalized(mut self) -> Self {
        self.normalize();
        self
    }

    pub fn cwise_product(&self, other: &Self) -> Self {
        let mut m = Self::new();
        m.data_iter_mut()
            .zip(self.data_iter().zip(other.data_iter()))
            .for_each(|(l, (&r1, &r2))| *l = r1 * r2);
        m
    }

    pub fn clamp_max(self, v: f32) -> Self {
        let mut m = Self::new();
        m.data_iter_mut()
            .zip(self.data_iter())
            .for_each(|(l, x)| *l = x.min(v));
        m
    }
}

impl<const N: usize> Default for Vector<N> {
    fn default() -> Self {
        Self([f32::default(); N])
    }
}

impl<const N: usize> From<f32> for Vector<N> {
    fn from(v: f32) -> Self {
        Self([v; N])
    }
}

impl<const N: usize> From<&[f32; N]> for Vector<N> {
    fn from(slice: &[f32; N]) -> Self {
        Self(slice.clone())
    }
}

impl<const N: usize, const M: usize> From<&Vector<M>> for Vector<N> {
    fn from(other: &Vector<M>) -> Self {
        let mut r = Self::new();
        r.0.iter_mut()
            .zip(other.0.iter())
            .for_each(|(l, r)| *l = *r);
        r
    }
}

pub type Vector3 = Vector<3>;
pub type Vector4 = Vector<4>;

macro_rules! def_getter {
    ($func: ident, $func_mut: ident, $index: expr) => {
        pub fn $func(&self) -> f32 {
            unsafe { *self.0.get_unchecked($index) }
        }
        pub fn $func_mut(&mut self) -> &mut f32 {
            unsafe { self.0.get_unchecked_mut($index) }
        }
    };
}

impl Vector3 {
    def_getter!(x, x_mut, 0);
    def_getter!(y, y_mut, 1);
    def_getter!(z, z_mut, 2);
    pub fn cross(&self, rhs: &Self) -> Self {
        let mut m = Self::new();
        *m.x_mut() = self.y() * rhs.z() - self.z() * rhs.y();
        *m.y_mut() = self.z() * rhs.x() - self.x() * rhs.z();
        *m.z_mut() = self.x() * rhs.y() - self.y() * rhs.x();
        m
    }
}

impl Vector4 {
    def_getter!(x, x_mut, 0);
    def_getter!(y, y_mut, 1);
    def_getter!(z, z_mut, 2);
    def_getter!(w, w_mut, 3);

    pub fn point_from(v: &Vector3) -> Self {
        let mut r = Self::new();
        *r.x_mut() = v.x();
        *r.y_mut() = v.y();
        *r.z_mut() = v.z();
        *r.w_mut() = 1.0;
        r
    }

    pub fn vector_from(v: &Vector3) -> Self {
        let mut r = Self::new();
        *r.x_mut() = v.x();
        *r.y_mut() = v.y();
        *r.z_mut() = v.z();
        *r.w_mut() = 0.0;
        r
    }
}

macro_rules! def_vector_func {
    ($func: ident, $n: expr) => {
        pub const fn $func(data: [f32; $n]) -> Vector<$n> {
            Vector::<$n>(data)
        }
    };
}

def_vector_func!(vector3, 3);
def_vector_func!(vector4, 4);
