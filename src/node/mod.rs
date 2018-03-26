
pub trait Node<T> {
    fn new(dat: T) -> Self
    where
        Self: Sized;
}
