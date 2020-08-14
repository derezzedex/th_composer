pub mod tools;

pub mod caesar;
pub mod reverse;

pub use reverse::Reversed;
pub use caesar::Caesar;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Technique{
    Reversed,
    Caesar,
}
