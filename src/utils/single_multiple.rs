use serde::Serialize;

/// Representation of something that is either a single value - rendered as the value itself - or
/// multiple values - rendered as a JSON array.
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum SingleOrMultiple<T>
where
    T: Serialize,
{
    Single(T),
    Multiple(Vec<T>),
}

impl<T> SingleOrMultiple<T>
where
    T: Serialize,
{
    /// Insert a new value into this.
    /// This always returns `Self::Multiple` with the new value at the end.
    #[must_use]
    pub fn insert(self, value: T) -> Self {
        match self {
            Self::Single(old) => Self::Multiple(vec![old, value]),
            Self::Multiple(mut values) => {
                values.push(value);

                Self::Multiple(values)
            },
        }
    }

    /// Generate an iterator for iterating over the values.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        SingleOrMultipleIter {
            value:   self,
            current: 0,
        }
    }
}

/// Iterator over the values in a `SingleOrMultiple`.
pub struct SingleOrMultipleIter<'a, T>
where
    T: Serialize,
{
    value:   &'a SingleOrMultiple<T>,
    current: usize,
}

impl<'a, T> Iterator for SingleOrMultipleIter<'a, T>
where
    T: Serialize,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        self.current += 1;

        match self.value {
            SingleOrMultiple::Single(value) if current == 0 => Some(value),
            SingleOrMultiple::Multiple(values) if current < values.len() => values.get(current),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use assert2::{check, let_assert};

    use super::*;

    #[test]
    fn insert_into_single() {
        let mut sut = SingleOrMultiple::Single(1);
        sut = sut.insert(2);

        let_assert!(SingleOrMultiple::Multiple(values) = sut);
        check!(values == vec![1, 2]);
    }

    #[test]
    fn insert_into_multiple() {
        let mut sut = SingleOrMultiple::Multiple(vec![0, 1]);
        sut = sut.insert(2);

        let_assert!(SingleOrMultiple::Multiple(values) = sut);
        check!(values == vec![0, 1, 2]);
    }

    #[test]
    fn iterate_over_single() {
        let sut = SingleOrMultiple::Single(1);

        let values: Vec<&u32> = sut.iter().collect();

        check!(values.len() == 1);
        check!(values[0] == &1);
    }

    #[test]
    fn iterate_over_multiple() {
        let sut = SingleOrMultiple::Multiple(vec![1, 2, 3]);

        let values: Vec<&u32> = sut.iter().collect();

        check!(values.len() == 3);
        check!(values[0] == &1);
        check!(values[1] == &2);
        check!(values[2] == &3);
    }
}
