pub type Result<T, E = OmiError> = std::result::Result<T, E>;

#[derive(Debug, PartialEq)]
pub enum OmiError {
    DatabaseError,
    NotFoundError,
}
