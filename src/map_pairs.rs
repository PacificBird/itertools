pub fn map_pairs<I, F, T, U>(iter: I, f: F) -> MapPairs<I, F>
where
    I: Iterator<Item = T>,
    T: Clone,
    F: FnMut(T, T) -> U,
{
    MapPairs::new(iter, f)
}

/// An iterator to apply a binary mapping function on consecutive pairs of elements.
///
/// See [`.map_pairs()`](crate::Itertools::map_pairs) for more information.
#[must_use = "iterators are lazy and do nothing unless consumed"]
#[derive(Debug)]
pub struct MapPairs<I: Iterator, F> {
    iter: I,
    current: Option<I::Item>,
    f: F,
}

impl<I: Iterator, F> MapPairs<I, F> {
    fn new(mut iter: I, f: F) -> Self {
        let current = iter.next();
        MapPairs { iter, current, f }
    }
}

impl<B, I: Iterator, F> Iterator for MapPairs<I, F>
where
    F: FnMut(I::Item, I::Item) -> B,
    I::Item: Clone,
{
    type Item = B;

    #[inline]
    fn next(&mut self) -> Option<B> {
        let next = self.iter.next();
        let mapped = Option::zip(next.clone(), self.current.clone())
            .map(|(next, current)| (&mut self.f)(next, current));
        self.current = next;
        mapped
    }
}
