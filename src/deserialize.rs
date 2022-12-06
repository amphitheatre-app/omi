use crate::Statement;

pub trait Queryable<T> {
    fn get() -> Statement<T>;
    fn find() -> Statement<T>;
}