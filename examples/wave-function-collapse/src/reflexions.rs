use crate::array;

pub struct Rotator<'a, T, const SIZE: usize> {
    pub(crate) rotations: [array::Array2D<&'a T, SIZE, SIZE>; 4],
}

impl<'a, T, const SIZE: usize> Rotator<'a, T, SIZE> {
    pub fn new(array: array::Array2D<&'a T, SIZE, SIZE>) -> Self {
        let first = array;
        let second = first.rotate_clockwise_ref_copy();
        let third = second.rotate_clockwise_ref_copy();
        let fourth = third.rotate_clockwise_ref_copy();

        Self {
            rotations: [first, second, third, fourth],
        }
    }
}

/*
* |BASE| V  |
* +----+----+
* |XXGG|GGXX|
* |@@##|##@@|
* +----+----+
* |@@##|##@@|
* |XXGG|GGXX|
* +----+----+
* | H  | VH |
*/
pub struct Reflextions<'a, T, const SIZE: usize> {
    pub(crate) reflextions: [array::Array2D<&'a T, SIZE, SIZE>; 4],
}

impl<'a, T, const SIZE: usize> Reflextions<'a, T, SIZE> {
    pub fn new(array: array::Array2D<&'a T, SIZE, SIZE>) -> Self {
        let unfliped = array;
        let flipped_h = unfliped.flip_horizontal_ref_copy();
        let flipped_v = unfliped.flip_vertical_ref_copy();
        let flipped_vh = flipped_v.flip_horizontal_ref_copy();

        Self {
            reflextions: [unfliped, flipped_h, flipped_v, flipped_vh],
        }
    }
}
