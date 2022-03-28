macro_rules! impl_scalar_ops {
    ($trait: ident, $func: ident, $op: tt, $type: ident) => {
        impl<const N: usize> $trait<f32> for $type<N> {
            type Output = $type<N>;
            fn $func(self, rhs: f32) -> Self::Output {
                let mut m = Self::Output::new();
                m.data_iter_mut().for_each(|v| *v = *v $op rhs);
                m
            }
        }

        impl<const N: usize> $trait<f32> for &$type<N> {
            type Output = $type<N>;
            fn $func(self, rhs: f32) -> Self::Output {
                let mut m = Self::Output::new();
                m.data_iter_mut().for_each(|v| *v = *v $op rhs);
                m
            }
        }
    };
}

macro_rules! impl_basic_ops {
    ($trait: ident, $func: ident, $op: tt, $type: ident) => {
        impl<const N: usize> $trait<$type<N>> for $type<N> {
            type Output = $type<N>;
            fn $func(self, rhs: $type<N>) -> Self::Output {
                let mut m = Self::Output::new();
                m.data_iter_mut()
                    .zip(self.data_iter().zip(rhs.data_iter()))
                    .for_each(|(l, (r1, r2))| *l = r1 $op r2);
                m
            }
        }

        impl<'a, 'b, const N: usize> $trait<&'a $type<N>> for &'b $type<N>
        where
            'a: 'b,
        {
            type Output = $type<N>;
            fn $func(self, rhs: &'a $type<N>) -> Self::Output {
                let mut m = Self::Output::new();
                m.data_iter_mut()
                    .zip(self.data_iter().zip(rhs.data_iter()))
                    .for_each(|(l, (r1, r2))| *l = r1 $op r2);
                m
            }
        }
    };
}

macro_rules! impl_assign_ops {
    ($trait: ident, $func: ident, $op: tt, $type: ident) => {
        impl<const N: usize> $trait for $type<N> {
            fn $func(&mut self, rhs: Self) {
                self.data_iter_mut()
                    .zip(rhs.data_iter())
                    .for_each(|(l, r)| {
                        *l $op r;
                    })
            }
        }

        impl<'a, 'b, const N: usize> $trait<&'a $type<N>> for &'b mut $type<N>
        where
            'a: 'b,
        {
            fn $func(&mut self, rhs: &'a $type<N>) {
                self.data_iter_mut()
                    .zip(rhs.data_iter())
                    .for_each(|(l, r)| {
                        *l $op r;
                    });
            }
        }
    };
}