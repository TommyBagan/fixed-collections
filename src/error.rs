use std::{error::Error, fmt::{Display}};

#[derive(Debug)]
pub struct EmptyCollectionError;

impl Display for EmptyCollectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EmptyCollectionError")
    }
}

impl Error for EmptyCollectionError {}

#[derive(Debug)]
pub struct FullCollectionError;

impl Display for FullCollectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FullCollectionError")
    }
}

impl Error for FullCollectionError {}