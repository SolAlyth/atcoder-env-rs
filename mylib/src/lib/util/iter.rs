/// # Usage
///
/// ```no_run
/// use crate::lib::util::iter::*;
///
/// impl GenericIterable for T {
///     /* coding here... */
/// }
///
/// impl IntoIterator for T {
///     type Item = (usize, <Self as GenericIterable>::Item);
///     type IntoIter = GenericIter<Self>;
///     fn into_iter(self) -> Self::IntoIter { self.into() }
/// }
/// ```

pub struct GenericIter<T: GenericIterable>(T, usize);

pub trait GenericIterable: Sized {
    type Item;
    fn giter_next(&mut self, i: usize) -> Option<Self::Item>;
}

impl<T: GenericIterable> From<T> for GenericIter<T> {
    fn from(value: T) -> Self { GenericIter(value, 0) }
}

impl<T: GenericIterable> Iterator for GenericIter<T> {
    type Item = (usize, T::Item);
    fn next(&mut self) -> Option<Self::Item> { let tmp = self.0.giter_next(self.1); self.1 += 1; tmp.map(|v| (self.1, v)) }
}
