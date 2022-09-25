#[derive(Clone, Copy, Debug)]
pub struct Array<const DIM: usize>(pub [f32; DIM]);

impl<const DIM: usize> Default for Array<DIM> {
    fn default() -> Self {
        Self([0.0; DIM])
    }
}

impl<const DIM: usize> std::ops::Add for Array<DIM> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Self::default();
        for i in 0..DIM {
            result.0[i] = self.0[i] + rhs.0[i];
        }
        result
    }
}

impl<const DIM: usize> std::ops::Sub for Array<DIM> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = Self::default();
        for i in 0..DIM {
            result.0[i] = self.0[i] - rhs.0[i];
        }
        result
    }
}
