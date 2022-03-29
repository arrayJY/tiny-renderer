mod ops;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct VectorNew<const N: usize>(pub [f32; N]);

impl<const N: usize> VectorNew<N> {
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
        self.data_iter().map(|v| v * v).sum::<f32>().sqrt()
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
}

impl<const N: usize> Default for VectorNew<N> {
    fn default() -> Self {
        Self([f32::default(); N])
    }
}

impl<const N: usize, const M: usize> From<&VectorNew<M>> for VectorNew<N> {
    fn from(other: &VectorNew<M>) -> Self {
        let mut r = Self::new();
        r.0.iter_mut()
            .zip(other.0.iter())
            .for_each(|(l, r)| *l = *r);
        r
    }
}

pub type VectorNew3 = VectorNew<3>;
pub type VectorNew4 = VectorNew<4>;

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

impl VectorNew3 {
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

impl VectorNew4 {
    def_getter!(x, x_mut, 0);
    def_getter!(y, y_mut, 1);
    def_getter!(z, z_mut, 2);
    def_getter!(w, w_mut, 3);

    pub fn point_from(v: &VectorNew3) -> Self {
        let mut r = Self::new();
        *r.x_mut() = v.x();
        *r.y_mut() = v.y();
        *r.z_mut() = v.z();
        *r.w_mut() = 1.0;
        r
    }

    pub fn vector_from(v: &VectorNew3) -> Self {
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
        pub fn $func(data: [f32; $n]) -> VectorNew<$n> {
            VectorNew::<$n>(data)
        }
    };
}

def_vector_func!(vector3, 3);
def_vector_func!(vector4, 4);
