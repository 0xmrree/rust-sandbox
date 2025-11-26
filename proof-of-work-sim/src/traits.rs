/// Trait for types that can be hashed
pub trait Hashable {
    fn hash(&self) -> String;
}

/// Trait for types that can be validated
pub trait Validatable {
    fn is_valid(&self) -> bool;
}
