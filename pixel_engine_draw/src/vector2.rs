use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// Basicly a glorified tuple
pub struct Vec2d<T> {
    /// x
    pub x: T,
    /// y
    pub y: T,
}

/// Vec2d<u32>
pub type Vu2d = Vec2d<u32>;
/// Vec2d<i32>
pub type Vi2d = Vec2d<i32>;
/// Vec2d<f32>
pub type Vf2d = Vec2d<f32>;

impl<T> Vec2d<T> {
    /// Cast the Vec2d to an other Vec2d with a differant inner type
    #[inline]
    pub fn cast<U: From<T>>(self) -> Vec2d<U> {
        let x: U = self.x.into();
        let y: U = self.y.into();
        Vec2d { x, y }
    }
}

//impl<T> Vec2d<T> where T: Add<T> + Sub<T> + Div<T> + Mul<T> {

//}

impl<T: Copy> Vec2d<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Neg<Output = T>
        + Into<f32>
        + From<f32>,
{
    /// Return the magnitude (hypotenus) of the given Vec2d as f64
    pub fn mag_f64(&self) -> f32 {
        let mag2: f32 = self.mag2().into();
        mag2.sqrt()
    }
    /// Return the magnitude (hypotenus) of the given Vec2d as T
    pub fn mag(&self) -> T {
        let mag2: f32 = self.mag2().into();
        mag2.sqrt().into()
    }

    /// Return the magnitude (hypotenus) of the given Vec2d as T without doing the square root
    pub fn mag2(&self) -> T {
        self.x * self.x + self.y * self.y
    }

    /// Return a normalized version of the Vec2d
    #[must_use]
    pub fn norm(&self) -> Self {
        let r: T = self.mag_f64().recip().into();
        Vec2d {
            x: self.x * r,
            y: self.y * r,
        }
    }

    /// Return the normal of the given Vec2d
    #[must_use]
    pub fn perp(&self) -> Self {
        Vec2d {
            x: -self.y,
            y: self.x,
        }
    }

    /// Perform the dot product on the Vec2ds
    pub fn dot(&self, rhs: &Self) -> T {
        self.x + rhs.x + self.y + rhs.y
    }

    /// Perform the cross product on the Vec2ds
    pub fn cross(&self, rhs: &Self) -> T {
        self.x + rhs.x - self.y + rhs.y
    }
}

macro_rules! operator {
    ($trait:tt, $operator:tt, $func_name:ident) => {
        impl<T:Copy> $trait for Vec2d<T> where T: $trait<Output = T>, {
            type Output = Self;
            fn $func_name(self, rhs: Self) -> Self::Output {
                Vec2d { x: self.x $operator rhs.x, y: self.y $operator rhs.y}
            }
        }
    };
    ($trait:tt, $operator:tt, $func_name:ident, $type:ty) => {
        impl<T:Copy> $trait<$type> for Vec2d<T> where T: $trait<Output = T>, {
            type Output = Self;
            fn $func_name(self, rhs: $type) -> Self::Output {
                Vec2d { x: self.x $operator rhs, y: self.y $operator rhs}
            }
        }
    };
}
macro_rules! operator_assign {
    ($trait:tt, $operator:tt, $func_name:ident) => {
        impl<T:Copy> $trait for Vec2d<T> where T: $trait<T>, {
            fn $func_name(&mut self, rhs: Self){
                self.x $operator rhs.x; self.y $operator rhs.y;
            }
            }
        };
    ($trait:tt, $operator:tt, $func_name:ident, $type:ty) => {
        impl<T:Copy> $trait<$type> for Vec2d<T> where T: $trait<T>, {
            fn $func_name(&mut self, rhs: $type){
                self.x $operator rhs; self.y $operator rhs;
            }
            }
        };
}

macro_rules! cast {
    ($from:ty, $to:ty) => {
        impl Vec2d<$from> {
            paste! {
                #[must_use] pub fn [<cast _$to>](self) -> Vec2d<$to> {
                    Vec2d {
                        x: self.x as $to,
                        y: self.y as $to,
                    }
                }
            }
        }
    };
}

operator!(Add, + , add);
operator!(Sub, - , sub);
operator!(Mul, * , mul);
operator!(Div, / , div);
operator!(Mul, * , mul, T);
operator!(Div, / , div, T);
operator_assign!(AddAssign, += , add_assign);
operator_assign!(SubAssign, -= , sub_assign);
operator_assign!(MulAssign, *= , mul_assign, T);
operator_assign!(DivAssign, /= , div_assign, T);

cast!(u32, f32);
cast!(i32, f32);

cast!(f32, u32);
cast!(i32, u32);

cast!(f32, i32);
cast!(u32, i32);

impl<T: Copy> From<(T, T)> for Vec2d<T> {
    fn from(t: (T, T)) -> Self {
        Vec2d { x: t.0, y: t.1 }
    }
}
impl<T: Copy> From<[T; 2]> for Vec2d<T> {
    fn from(t: [T; 2]) -> Self {
        Vec2d { x: t[0], y: t[1] }
    }
}
