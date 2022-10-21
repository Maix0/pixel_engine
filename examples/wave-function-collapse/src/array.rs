use std::{
    mem::{ManuallyDrop, MaybeUninit},
    ops::Index,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Array2D<T, const X: usize, const Y: usize>([[T; X]; Y]);

impl<T, const X: usize, const Y: usize> AsRef<[T]> for Array2D<T, X, Y> {
    fn as_ref(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.0.as_ptr() as *const T, X * Y) }
    }
}
impl<T, const X: usize, const Y: usize> AsMut<[T]> for Array2D<T, X, Y> {
    fn as_mut(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.0.as_mut_ptr() as *mut T, X * Y) }
    }
}

impl<T, const X: usize, const Y: usize> AsRef<[[T; X]; Y]> for Array2D<T, X, Y> {
    fn as_ref(&self) -> &[[T; X]; Y] {
        &self.0
    }
}
impl<T, const X: usize, const Y: usize> AsMut<[[T; X]; Y]> for Array2D<T, X, Y> {
    fn as_mut(&mut self) -> &mut [[T; X]; Y] {
        &mut self.0
    }
}

impl<T, const X: usize, const Y: usize> Array2D<T, X, Y> {
    pub fn as_ref(&self) -> Array2D<&T, X, Y> {
        let mut uninit: std::mem::MaybeUninit<Array2D<&T, X, Y>> = std::mem::MaybeUninit::uninit();
        let mut uninit_ptr = uninit.as_mut_ptr() as *mut &T;
        for y in 0..Y {
            for x in 0..X {
                unsafe {
                    uninit_ptr.write(&self[(x, y)]);
                    uninit_ptr = uninit_ptr.add(1);
                }
            }
        }
        unsafe { uninit.assume_init() }
    }

    pub fn as_mut(&self) -> Array2D<&mut T, X, Y> {
        let mut uninit: std::mem::MaybeUninit<Array2D<&mut T, X, Y>> =
            std::mem::MaybeUninit::uninit();
        let mut uninit_ptr = uninit.as_mut_ptr() as *mut &T;
        for y in 0..Y {
            for x in 0..X {
                unsafe {
                    uninit_ptr.write(&self[(x, y)]);
                    uninit_ptr = uninit_ptr.add(1);
                }
            }
        }
        unsafe { uninit.assume_init() }
    }
}

impl<T, const X: usize, const Y: usize> Array2D<T, X, Y> {
    pub fn into_inner(self) -> [[T; X]; Y] {
        self.0
    }

    pub fn new(array: [[T; X]; Y]) -> Self {
        Self(array)
    }
}

impl<T, const X: usize, const Y: usize> std::ops::Index<(usize, usize)> for Array2D<T, X, Y> {
    type Output = T;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        assert!(x < X, "Invalid X Index");
        assert!(y < Y, "Invalid Y Index");
        &self.0[y][x]
    }
}

impl<T, const X: usize, const Y: usize> std::ops::IndexMut<(usize, usize)> for Array2D<T, X, Y> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        assert!(x < X, "Invalid X Index");
        assert!(y < Y, "Invalid Y Index");
        &mut self.0[y][x]
    }
}

impl<T: Clone, const X: usize, const Y: usize> Array2D<&T, X, Y> {
    pub fn into_owned(&self) -> Array2D<T, X, Y> {
        Array2D(
            (*std::convert::AsRef::<[[&T; X]; Y]>::as_ref(self))
                .map(|inner| inner.map(|v| v.clone())),
        )
    }
}
impl<'a, T, const X: usize, const Y: usize> Array2D<&'a T, X, Y> {
    pub fn rotate_clockwise_ref_copy(&self) -> Array2D<&'a T, Y, X> {
        let mut uninit: std::mem::MaybeUninit<Array2D<&'a T, Y, X>> =
            std::mem::MaybeUninit::uninit();
        let mut uninit_ptr = uninit.as_mut_ptr() as *mut &'a T;
        for y in 0..Y {
            for x in 0..X {
                unsafe {
                    uninit_ptr.write(self[(X - 1 - x, y)]);
                    uninit_ptr = uninit_ptr.add(1);
                }
            }
        }

        unsafe { uninit.assume_init() }
    }

    pub fn flip_horizontal_ref_copy(&self) -> Array2D<&'a T, X, Y> {
        let mut uninit: std::mem::MaybeUninit<Array2D<&'a T, X, Y>> =
            std::mem::MaybeUninit::uninit();
        let uninit_ptr = uninit.as_mut_ptr() as *mut &'a T;
        for y in 0..Y {
            for x in 0..X {
                unsafe {
                    uninit_ptr
                        .add(y * X + (X - x - 1))
                        .write(self[(x, Y - y - 1)]);
                }
            }
        }

        unsafe { uninit.assume_init() }
    }
    pub fn flip_vertical_ref_copy(&self) -> Array2D<&'a T, X, Y> {
        let mut uninit: std::mem::MaybeUninit<Array2D<&'a T, X, Y>> =
            std::mem::MaybeUninit::uninit();
        let uninit_ptr = uninit.as_mut_ptr() as *mut &'a T;
        for y in 0..Y {
            let flipped_y = Y - y - 1;
            for x in 0..X {
                unsafe {
                    uninit_ptr
                        .add(flipped_y * X + (x))
                        .write(self[(x, Y - y - 1)]);
                }
            }
        }

        unsafe { uninit.assume_init() }
    }
}

impl<'a, T, const X: usize, const Y: usize> Array2D<T, X, Y> {
    pub fn rotate_clockwise_ref(&'a self) -> Array2D<&'a T, Y, X> {
        let mut uninit: std::mem::MaybeUninit<Array2D<&'a T, Y, X>> =
            std::mem::MaybeUninit::uninit();
        let mut uninit_ptr = uninit.as_mut_ptr() as *mut &'a T;
        for y in 0..Y {
            for x in 0..X {
                unsafe {
                    uninit_ptr.write(&self[(X - 1 - x, y)]);
                    uninit_ptr = uninit_ptr.add(1);
                }
            }
        }

        unsafe { uninit.assume_init() }
    }

    pub fn flip_horizontal_ref(&'a self) -> Array2D<&'a T, X, Y> {
        let mut uninit: std::mem::MaybeUninit<Array2D<&'a T, X, Y>> =
            std::mem::MaybeUninit::uninit();
        let uninit_ptr = uninit.as_mut_ptr() as *mut &'a T;
        for y in 0..Y {
            for x in 0..X {
                unsafe {
                    uninit_ptr
                        .add(y * X + (X - x - 1))
                        .write(&self[(x, Y - y - 1)]);
                }
            }
        }

        unsafe { uninit.assume_init() }
    }
    pub fn flip_vertical_ref(&'a self) -> Array2D<&'a T, X, Y> {
        let mut uninit: std::mem::MaybeUninit<Array2D<&'a T, X, Y>> =
            std::mem::MaybeUninit::uninit();
        let uninit_ptr = uninit.as_mut_ptr() as *mut &'a T;
        for y in 0..Y {
            let flipped_y = Y - y - 1;
            for x in 0..X {
                unsafe {
                    uninit_ptr
                        .add(flipped_y * X + (x))
                        .write(&self[(x, Y - y - 1)]);
                }
            }
        }

        unsafe { uninit.assume_init() }
    }
}

impl<T, const WIDTH: usize, const HEIGHT: usize> Array2D<T, WIDTH, HEIGHT> {
    pub fn from_fn(mut f: impl FnMut(usize, usize) -> T) -> Self {
        let mut index = 0;
        Array2D::new([[(); WIDTH]; HEIGHT]).map(|_| {
            let val = f(index % WIDTH, index / WIDTH);
            index += 1;
            val
        })
    }

    pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> Array2D<U, WIDTH, HEIGHT> {
        struct UninitArray2D<T, const WIDTH: usize, const HEIGHT: usize> {
            array: [[MaybeUninit<T>; WIDTH]; HEIGHT],
            index: usize,
        }

        impl<T, const WIDTH: usize, const HEIGHT: usize> UninitArray2D<T, WIDTH, HEIGHT> {
            fn new() -> Self {
                let array = unsafe {
                    MaybeUninit::<[[MaybeUninit<T>; WIDTH]; HEIGHT]>::uninit().assume_init()
                };
                Self { array, index: 0 }
            }
            pub unsafe fn write(&mut self, value: T) {
                self.array
                    .get_unchecked_mut(self.index / HEIGHT)
                    .get_unchecked_mut(self.index % WIDTH)
                    .write(value);
                self.index += 1;
            }
        }

        impl<T, const WIDTH: usize, const HEIGHT: usize> Drop for UninitArray2D<T, WIDTH, HEIGHT> {
            fn drop(&mut self) {
                let ptr: *mut MaybeUninit<T> = self.array.as_mut_ptr().cast();
                unsafe { std::slice::from_raw_parts_mut(ptr, self.index) }
                    .into_iter()
                    .for_each(|p| unsafe { p.assume_init_drop() });
            }
        }
        struct DrainedArray2D<T, const WIDTH: usize, const HEIGHT: usize> {
            array: std::mem::ManuallyDrop<Array2D<T, WIDTH, HEIGHT>>,
            index: usize,
        }

        impl<T, const WIDTH: usize, const HEIGHT: usize> DrainedArray2D<T, WIDTH, HEIGHT> {
            fn new(array: Array2D<T, WIDTH, HEIGHT>) -> Self {
                let array = ManuallyDrop::new(array);
                Self { array, index: 0 }
            }
            pub unsafe fn read(&mut self) -> T {
                let ptr = (*self.array).index((self.index % WIDTH, self.index / WIDTH));
                let data = std::ptr::read(ptr);
                self.index += 1;
                data
            }
        }

        impl<T, const WIDTH: usize, const HEIGHT: usize> Drop for DrainedArray2D<T, WIDTH, HEIGHT> {
            fn drop(&mut self) {
                let ptrs =
                    AsMut::<[[T; WIDTH]; HEIGHT]>::as_mut(&mut *self.array).as_mut_ptr_range();
                let ptr: *mut T = ptrs.start.cast();
                unsafe {
                    std::slice::from_raw_parts_mut(
                        ptr.add(self.index),
                        WIDTH * HEIGHT - self.index,
                    )
                    .into_iter()
                    .for_each(|p| core::ptr::drop_in_place(p));
                }
                :aq
                :a
                :q
                :
            }
        }

        let mut uninit: UninitArray2D<U, WIDTH, HEIGHT> = UninitArray2D::new();
        let mut this = DrainedArray2D::new(self);
        for _ in 0..HEIGHT {
            for _ in 0..WIDTH {
                unsafe {
                    uninit.write(f(this.read()));
                }
            }
        }

        Array2D::new(unsafe { std::mem::transmute_copy::<_, [[U; WIDTH]; HEIGHT]>(&uninit.array) })
    }
}
