/// Trait for buffering stream data into a Vec.
pub trait Buffer: Sized {
    type Item;
    /// Consumes the stream and collects its items into a Vec.
    fn into_buffer(self) -> Vec<Self::Item>;
}

// Blanket implementation for all Iterators.
impl<I> Buffer for I
where
    I: Iterator,
{
    type Item = I::Item;
    fn into_buffer(self) -> Vec<Self::Item> {
        self.collect()
    }
}
