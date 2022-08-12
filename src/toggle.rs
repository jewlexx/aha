pub trait Toggle {
    fn flip(&self) -> Self;

    fn toggle(&mut self);
}

impl Toggle for bool {
    #[inline]
    fn flip(&self) -> bool {
        !self
    }

    #[inline]
    fn toggle(&mut self) {
        *self = self.flip();
    }
}
