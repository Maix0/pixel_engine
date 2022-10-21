use crate::array::Array2D;

struct ReflexionAndRotation<'a, PatternItem, const SIZE: usize> {
    rotations: crate::reflexions::Rotator<'a, PatternItem, SIZE>,
    reflexions: crate::reflexions::Reflextions<'a, PatternItem, SIZE>,
}

impl<'a, PatternItem, const SIZE: usize> ReflexionAndRotation<'a, PatternItem, SIZE> {
    fn from(array: Array2D<&'a PatternItem, SIZE, SIZE>) -> Self {
        Self {
            rotations: crate::reflexions::Rotator::new(array),
            reflexions: crate::reflexions::Reflextions::new(array),
        }
    }
}

pub struct Wfc<'pattern, PatternItem, const SUBPAT_SIZE: usize = 2> {
    patterns: Box<[ReflexionAndRotation<'pattern, PatternItem, SUBPAT_SIZE>]>,
}

impl<'pattern, PatternItem: WfcItem, const SUBPAT_SIZE: usize>
    Wfc<'pattern, PatternItem, SUBPAT_SIZE>
{
    pub fn run<
        const IN_WIDTH: usize,
        const IN_HEIGHT: usize,
        const OUT_WIDTH: usize,
        const OUT_HEIGHT: usize,
    >(
        base_pattern: &'pattern Array2D<PatternItem, IN_WIDTH, IN_HEIGHT>,
    ) -> Array2D<PatternItem, OUT_WIDTH, OUT_HEIGHT> {
        fn binom(n: usize, k: usize) -> usize {
            let mut res = 1;
            for i in 0..k {
                res = (res * (n - i)) / (i + 1);
            }
            res
        }

        let mut patterns =
            Vec::with_capacity(binom(SUBPAT_SIZE, IN_WIDTH) * binom(SUBPAT_SIZE, IN_HEIGHT));
        for y in 0..(IN_HEIGHT - SUBPAT_SIZE) {
            for x in 0..(IN_WIDTH - SUBPAT_SIZE) {
                patterns.push(ReflexionAndRotation::from(Array2D::<
                    &'pattern PatternItem,
                    SUBPAT_SIZE,
                    SUBPAT_SIZE,
                >::from_fn(
                    |i, j| &base_pattern[(x + i, y + j)],
                )));
            }
        }
        Self {
            patterns: patterns.into_boxed_slice(),
        };

        let out: Array2D<Item<PatternItem>, OUT_WIDTH, OUT_HEIGHT> =
            Array2D::from_fn(|_, _| Item::Unfilled(PatternItem::create_bitset()));

        todo!()
    }
}

pub trait WfcItem: Clone {
    type BitSet: bitset_core::BitSet;
    fn bit_length() -> usize;
    fn create_bitset() -> Self::BitSet;
}

enum Item<T: WfcItem> {
    Filled(T),
    Unfilled(<T as WfcItem>::BitSet),
}
