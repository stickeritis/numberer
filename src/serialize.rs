use crate::Numberer;
use serde_derive::{Deserialize, Serialize};
use std::hash::Hash;

/// Serialized representation for `Numberer`.
///
/// This exists because we do not need to serialize both the
/// `values` and `numbers` fields of `Numberer`, since they
/// represent the same information.
#[derive(Eq, PartialEq, Serialize, Deserialize)]
pub struct SerializedNumberer<T> {
    values: Vec<T>,
    start_at: usize,
}

impl<T> From<Numberer<T>> for SerializedNumberer<T>
where
    T: Clone + Eq + Hash,
{
    fn from(numberer: Numberer<T>) -> Self {
        SerializedNumberer {
            values: numberer.values,
            start_at: numberer.start_at,
        }
    }
}

impl<T> From<SerializedNumberer<T>> for Numberer<T>
where
    T: Clone + Eq + Hash,
{
    fn from(serialized_numberer: SerializedNumberer<T>) -> Self {
        let numbers = serialized_numberer
            .values
            .iter()
            .enumerate()
            .map(|(idx, val)| (val.clone(), idx + serialized_numberer.start_at))
            .collect();

        Numberer {
            numbers,
            values: serialized_numberer.values,
            start_at: serialized_numberer.start_at,
        }
    }
}
