use std::borrow::Borrow;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;

use serde_derive::{Deserialize, Serialize};

/// Serialized representation for `Numberer`.
///
/// This exists because we do not need to serialize both the
/// `values` and `numbers` fields of `Numberer`, since they
/// represent the same information.
#[derive(Eq, PartialEq, Serialize, Deserialize)]
struct SerializedNumberer<T> {
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

/// Numberer for categorical values, such as features or class labels.
#[serde(from = "SerializedNumberer<T>", into = "SerializedNumberer<T>")]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Numberer<T>
where
    T: Clone + Eq + Hash,
{
    values: Vec<T>,
    numbers: HashMap<T, usize>,
    start_at: usize,
}

impl<T> Numberer<T>
where
    T: Clone + Eq + Hash,
{
    pub fn new(start_at: usize) -> Self {
        Numberer {
            values: Vec::new(),
            numbers: HashMap::new(),
            start_at,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn len(&self) -> usize {
        self.values.len() + self.start_at
    }

    /// Add an value. If the value has already been encountered before,
    /// the corresponding number is returned.
    pub fn add(&mut self, value: T) -> usize {
        match self.numbers.entry(value.clone()) {
            Entry::Occupied(e) => *e.get(),
            Entry::Vacant(e) => {
                let number = self.values.len() + self.start_at;
                self.values.push(value);
                e.insert(number);
                number
            }
        }
    }

    /// Return the number for a value.
    pub fn number<Q>(&self, item: &Q) -> Option<usize>
    where
        T: Borrow<Q>,
        Q: ?Sized + Eq + Hash,
    {
        self.numbers.get(item).cloned()
    }

    /// Return the value for a number.
    pub fn value(&self, number: usize) -> Option<&T> {
        self.values.get(number - self.start_at)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::Numberer;

    #[test]
    fn serialization_roundtrip_results_in_same_numberer() {
        let mut numberer = Numberer::new(13);
        numberer.add("hello".to_string());
        numberer.add("world".to_string());
        numberer.add("!".to_string());

        let mut serialized = Vec::new();
        serde_cbor::to_writer(&mut serialized, &numberer).unwrap();

        let serialized = Cursor::new(serialized);
        let numberer_deserialized: Numberer<String> = serde_cbor::from_reader(serialized).unwrap();

        assert_eq!(numberer, numberer_deserialized);
    }
}
