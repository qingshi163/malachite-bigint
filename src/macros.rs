macro_rules! apply_to_unsigneds {
    ($m:tt{_$(,$args:ident)*}) => {
        $m!(u8 $(,$args)*);
        $m!(u16 $(,$args)*);
        $m!(u32 $(,$args)*);
        $m!(u64 $(,$args)*);
        $m!(usize $(,$args)*);
        $m!(u128 $(,$args)*);
    };
    ($m:tt{$arg0:tt,_$(,$args:tt)*}) => {
        $m!($arg0, u8 $(,$args)*);
        $m!($arg0, u16 $(,$args)*);
        $m!($arg0, u32 $(,$args)*);
        $m!($arg0, u64 $(,$args)*);
        $m!($arg0, usize $(,$args)*);
        $m!($arg0, u128 $(,$args)*);
    }
}

macro_rules! apply_to_signeds {
    ($m:tt{_$(,$args:ident)*}) => {
        $m!(i8 $(,$args)*);
        $m!(i16 $(,$args)*);
        $m!(i32 $(,$args)*);
        $m!(i64 $(,$args)*);
        $m!(isize $(,$args)*);
        $m!(i128 $(,$args)*);
    };
    ($m:tt{$arg0:tt,_$(,$args:tt)*}) => {
        $m!($arg0, i8 $(,$args)*);
        $m!($arg0, i16 $(,$args)*);
        $m!($arg0, i32 $(,$args)*);
        $m!($arg0, i64 $(,$args)*);
        $m!($arg0, isize $(,$args)*);
        $m!($arg0, i128 $(,$args)*);
    }
}

macro_rules! apply_to_primitives {
    ($m:tt{_$(,$args:ident)*}) => {
        apply_to_signeds!($m{_$(,$args)*});
        apply_to_unsigneds!($m{_$(,$args)*});
    };
    ($m:tt{$arg0:tt,_$(,$args:tt)*}) => {
        apply_to_signeds!($m{$arg0, _$(,$args)*});
        apply_to_unsigneds!($m{$arg0, _$(,$args)*});
    };
}

macro_rules! forward_from {
    ($lhs:ty, $rhs:ty) => {
        impl From<$rhs> for $lhs {
            #[inline]
            fn from(value: $rhs) -> Self {
                Self(<_ as From<_>>::from(value))
            }
        }
    };
}

macro_rules! forward_try_from {
    ($lhs:ty, $rhs:ty) => {
        impl TryFrom<$rhs> for $lhs {
            type Error = TryFromBigIntError<()>;

            #[inline]
            fn try_from(value: $rhs) -> Result<Self, Self::Error> {
                <_ as TryFrom<_>>::try_from(value).map_err(|_| Self::Error::new(())).map(Self)
            }
        }
    };
}

macro_rules! forward_unary_op {
    ($struct:tt, $trait:tt, $fn:ident) => {
        impl $trait for $struct {
            type Output = $struct;

            #[inline]
            fn $fn(mut self) -> Self::Output {
                self.0 = $trait::$fn(self.0);
                self
            }
        }

        impl $trait for &$struct {
            type Output = $struct;

            #[inline]
            fn $fn(self) -> Self::Output {
                $struct($trait::$fn(&self.0))
            }
        }
    };
}

macro_rules! impl_binary_op {
    ($lhs:ty, $rhs:ty, $output:ty, $trait:tt, $fn:ident, $expr:expr) => {
        impl $trait<$rhs> for $lhs {
            type Output = $output;

            #[inline]
            fn $fn(self, rhs: $rhs) -> Self::Output {
                $expr(self, rhs)
            }
        }
    };
}

macro_rules! impl_assign_op {
    ($lhs:ty, $rhs:ty, $trait:tt, $fn:ident, $expr:expr) => {
        impl $trait<$rhs> for $lhs {
            #[inline]
            fn $fn(&mut self, rhs: $rhs) {
                $expr(self, rhs)
            }
        }
    };
}

macro_rules! forward_binary_self {
    ($struct:ty, $trait:tt, $fn:ident) => {
        impl_binary_op!(
            $struct,
            $struct,
            $struct,
            $trait,
            $fn,
            |lhs: $struct, rhs: $struct| { $trait::$fn(lhs.0, rhs.0).into() }
        );
        impl_binary_op!(
            &$struct,
            $struct,
            $struct,
            $trait,
            $fn,
            |lhs: &$struct, rhs: $struct| { $trait::$fn(&lhs.0, rhs.0).into() }
        );
        impl_binary_op!(
            $struct,
            &$struct,
            $struct,
            $trait,
            $fn,
            |lhs: $struct, rhs: &$struct| { $trait::$fn(lhs.0, &rhs.0).into() }
        );
        impl_binary_op!(
            &$struct,
            &$struct,
            $struct,
            $trait,
            $fn,
            |lhs: &$struct, rhs: &$struct| { $trait::$fn(&lhs.0, &rhs.0).into() }
        );
    };
}

macro_rules! forward_binary_right_primitive {
    ($lhs:ty, $rhs:ty, $trait:tt, $fn:ident) => {
        impl_binary_op!($lhs, $rhs, $lhs, $trait, $fn, |lhs: $lhs, rhs: $rhs| {
            $trait::$fn(lhs.0, rhs).into()
        });
        impl_binary_op!(&$lhs, $rhs, $lhs, $trait, $fn, |lhs: &$lhs, rhs: $rhs| {
            $trait::$fn(&lhs.0, rhs).into()
        });
        impl_binary_op!($lhs, &$rhs, $lhs, $trait, $fn, |lhs: $lhs, rhs: &$rhs| {
            $trait::$fn(lhs.0, *rhs).into()
        });
        impl_binary_op!(&$lhs, &$rhs, $lhs, $trait, $fn, |lhs: &$lhs, rhs: &$rhs| {
            $trait::$fn(&lhs.0, *rhs).into()
        });
    };
}

macro_rules! forward_binary_right_primitive_into {
    ($lhs:ty, $rhs:ty, $trait:tt, $fn:ident) => {
        impl_binary_op!($lhs, $rhs, $lhs, $trait, $fn, |lhs: $lhs, rhs: $rhs| {
            $trait::$fn(lhs.0, <$lhs>::from(rhs).0).into()
        });
        impl_binary_op!(&$lhs, $rhs, $lhs, $trait, $fn, |lhs: &$lhs, rhs: $rhs| {
            $trait::$fn(&lhs.0, <$lhs>::from(rhs).0).into()
        });
        impl_binary_op!($lhs, &$rhs, $lhs, $trait, $fn, |lhs: $lhs, rhs: &$rhs| {
            $trait::$fn(lhs.0, <$lhs>::from(*rhs).0).into()
        });
        impl_binary_op!(&$lhs, &$rhs, $lhs, $trait, $fn, |lhs: &$lhs, rhs: &$rhs| {
            $trait::$fn(&lhs.0, <$lhs>::from(*rhs).0).into()
        });
    };
}

macro_rules! forward_binary_left_primitive_into {
    ($lhs:ty, $rhs:ty, $trait:tt, $fn:ident) => {
        impl_binary_op!($lhs, $rhs, $rhs, $trait, $fn, |lhs: $lhs, rhs: $rhs| {
            $trait::$fn(<$rhs>::from(lhs).0, rhs.0).into()
        });
        impl_binary_op!(&$lhs, $rhs, $rhs, $trait, $fn, |lhs: &$lhs, rhs: $rhs| {
            $trait::$fn(<$rhs>::from(*lhs).0, rhs.0).into()
        });
        impl_binary_op!($lhs, &$rhs, $rhs, $trait, $fn, |lhs: $lhs, rhs: &$rhs| {
            $trait::$fn(<$rhs>::from(lhs).0, &rhs.0).into()
        });
        impl_binary_op!(&$lhs, &$rhs, $rhs, $trait, $fn, |lhs: &$lhs, rhs: &$rhs| {
            $trait::$fn(<$rhs>::from(*lhs).0, &rhs.0).into()
        });
    };
}

macro_rules! forward_assign_self {
    ($struct:ty, $trait:tt, $fn:ident) => {
        impl_assign_op!(
            $struct,
            $struct,
            $trait,
            $fn,
            |lhs: &mut $struct, rhs: $struct| { $trait::$fn(&mut lhs.0, rhs.0).into() }
        );
        impl_assign_op!(
            $struct,
            &$struct,
            $trait,
            $fn,
            |lhs: &mut $struct, rhs: &$struct| { $trait::$fn(&mut lhs.0, &rhs.0).into() }
        );
    };
}

macro_rules! forward_assign_primitive {
    ($lhs:ty, $rhs:ty, $trait:tt, $fn:ident) => {
        impl_assign_op!($lhs, $rhs, $trait, $fn, |lhs: &mut $lhs, rhs: $rhs| {
            $trait::$fn(&mut lhs.0, rhs).into()
        });
        impl_assign_op!($lhs, &$rhs, $trait, $fn, |lhs: &mut $lhs, rhs: &$rhs| {
            $trait::$fn(&mut lhs.0, *rhs).into()
        });
    };
}

macro_rules! forward_assign_primitive_into {
    ($lhs:ty, $rhs:ty, $trait:tt, $fn:ident) => {
        impl_assign_op!($lhs, $rhs, $trait, $fn, |lhs: &mut $lhs, rhs: $rhs| {
            $trait::$fn(&mut lhs.0, <$lhs>::from(rhs).0).into()
        });
        impl_assign_op!($lhs, &$rhs, $trait, $fn, |lhs: &mut $lhs, rhs: &$rhs| {
            $trait::$fn(&mut lhs.0, <$lhs>::from(*rhs).0).into()
        });
    };
}

macro_rules! forward_pow_primitive {
    ($lhs:ty, $rhs:ty) => {
        impl_binary_op!($lhs, $rhs, $lhs, Pow, pow, |lhs: $lhs, rhs: $rhs| {
            <_ as malachite::num::arithmetic::traits::Pow<u64>>::pow(lhs.0, rhs as _).into()
        });
        impl_binary_op!(&$lhs, $rhs, $lhs, Pow, pow, |lhs: &$lhs, rhs: $rhs| {
            <_ as malachite::num::arithmetic::traits::Pow<u64>>::pow(&lhs.0, rhs as _).into()
        });
        impl_binary_op!($lhs, &$rhs, $lhs, Pow, pow, |lhs: $lhs, rhs: &$rhs| {
            <_ as malachite::num::arithmetic::traits::Pow<u64>>::pow(lhs.0, *rhs as _).into()
        });
        impl_binary_op!(&$lhs, &$rhs, $lhs, Pow, pow, |lhs: &$lhs, rhs: &$rhs| {
            <_ as malachite::num::arithmetic::traits::Pow<u64>>::pow(&lhs.0, *rhs as _).into()
        });
    };
}
