#[macro_export]
macro_rules! input_matrix {
    ($m: expr, $($val: expr), +) => {
        let rows = $m.rows();
        let cols = $m.cols();
        let mut iter = (0..rows).flat_map(move |a| (0..cols).map(move |b| (a, b)));
        $(
            {
                let (row, col) = iter.next().unwrap();
                $m[row][col] = $val;
            }
        )+
    };
}

#[macro_export]
macro_rules! matrix {
    ($type: ty; $length: ty) => {
        Matrix::<$type, $length, $length>::new();
    };
    ($type: ty; $row: ty, $col: ty ) => {
        Matrix::<$type, $row, $col>::new();
    };
    ($type: ty; $length: ty; $($val: expr),+ ) => {
        {
            let mut m = Matrix::<$type, $length, $length>::new();
            input_matrix!(m, $($val), +);
            m
        }
    };
    ($type: ty; $row: ty, $col: ty; $($val: expr), + ) => {
        {
            let mut m = Matrix::<$type, $row, $col>::new();
            input_matrix!(m, $($val), +);
            m
        }
    };
}

#[macro_export]
macro_rules! matrixf {
    ($dimension: ty; $($val: expr), +) => {
        {
            let mut m = Matrixf::<$dimension, $dimension>::new();
            input_matrix!(m, $($val), +);
            m
       }
    };

    ($row: ty, $col: ty; $($val: expr), +) => {
        {
            let mut m = Matrixf::<$row, $col>::new();
            input_matrix!(m, $($val), +);
            m
       }
    };
}

#[macro_export]
macro_rules! matrix1f {
    () => {
        Matrix1f::new()
    };
    ($($val: expr), +) => {
        {
            let mut m = Matrix1f::new();
            input_matrix!(m, $($val), +);
            m
        }
    }
}

#[macro_export]
macro_rules! matrix2f {
    () => {
        Matrix2f::new()
    };
    ($($val: expr), +) => {
        {
            let mut m = Matrix2f::new();
            input_matrix!(m, $($val), +);
            m
        }
    }
}

#[macro_export]
macro_rules! matrix3f {
    () => {
        Matrix3f::new()
    };
    ($($val: expr), +) => {
        {
            let mut m = Matrix3f::new();
            input_matrix!(m, $($val), +);
            m
        }
    }
}

#[macro_export]
macro_rules! matrix4f {
    () => {
        Matrix4f::new()
    };
    ($($val: expr), +) => {
        {
            let mut m = Matrix4f::new();
            input_matrix!(m, $($val), +);
            m
        }
    }
}
