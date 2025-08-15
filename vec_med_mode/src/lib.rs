//! This module provides functionality for calculating the median and mode of
//!  a list of integers.
use std::collections::{HashMap, HashSet};
use std::fmt;

/// Represents the median value of a set of integers.
///
/// The median may be an integer or a floating-point number:
/// - `Int(i32)` is used when the dataset has an odd number of elements,
///   so the median is a single middle value.
/// - `Float(f32)` is used when the dataset has an even number of elements,
///   and the median is the average of the two middle values.
#[derive(PartialEq, Debug)]
pub enum Median {
    Int(i32),
    Float(f32),
}

impl fmt::Display for Median {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Median::Int(v) => write!(f, "{v}"),
            Median::Float(v) => write!(f, "{v}"),
        }
    }
}

/// Returns the median value of a slice of integers.
///
/// For an empty slice, returns `None`.
/// For a slice with one element, returns that element as a `Median::Int`.
/// For multiple elements:
/// - If the number of elements is odd, returns the middle value as `Median::Int`.
/// - If even, returns the average of the two middle values as `Median::Float`.
///
/// ## Example
/// ```rust
/// use vec_med_mode::{median, Median};
/// let vec = vec![1,1,2,2,3,4];
/// assert_eq!(median(&vec).unwrap(), Median::Float(2.0));
/// ```
pub fn median(vector_slice: &[i32]) -> Option<Median> {
    let mut vector = vector_slice.to_owned();
    let length = vector.len();
    match length {
        0 => None,
        1 => Some(Median::Int(vector[0])),
        _ => compute_median(&mut vector, length),
    }
}

fn compute_median(vector: &mut [i32], length: usize) -> Option<Median> {
    vector.sort();
    let median = if length % 2 == 1 {
        // Odd length → middle element
        Median::Int(vector[length / 2])
    } else {
        // Even length → average of two middle elements
        let a = vector[length / 2 - 1];
        let b = vector[length / 2];
        Median::Float(((a + b) as f32) * 0.5)
    };

    Some(median)
}

/// Calculates the mode of a slice of integers.
///
/// ## Example
/// ```rust
/// use vec_med_mode::mode;
/// let vec = vec![1,1,2,2,3,4];
/// assert_eq!(mode(&vec).unwrap(), vec![1,2]);
/// ```
pub fn mode(vector_slice: &[i32]) -> Option<Vec<i32>> {
    let mut vector = vector_slice.to_owned();
    let length = vector.len();
    match length {
        0 => None,
        1 => Some(vector_slice.to_vec()),
        _ => compute_mode(&mut vector),
    }
}

fn compute_mode(vector: &mut [i32]) -> Option<Vec<i32>> {
    let mut num_to_count: HashMap<i32, u32> = HashMap::new();

    for num in vector.iter() {
        let count = num_to_count.entry(*num).or_default();
        *count += 1;
    }
    // Find the max count
    let max_count = num_to_count.values().copied().max()?;

    // Collect all numbers with the max count
    let mode_set: HashSet<i32> = num_to_count
        .iter()
        .filter(|(_, count)| **count == max_count)
        .map(|(num, _)| *num)
        .collect();
    let mut mode_vec: Vec<i32> = mode_set.into_iter().collect();
    mode_vec.sort();
    Some(mode_vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn median_odd_and_even_length_input() {
        let odd_length: Vec<i32> = vec![4, 5, 3, 1, 2];
        let even_length: Vec<i32> = vec![4, 2, 3, 1];
        assert_eq!(median(&odd_length).unwrap(), Median::Int(3));
        assert_eq!(median(&even_length).unwrap(), Median::Float(2.5));
    }

    #[test]
    fn median_empty_and_single_element_input() {
        let empty: Vec<i32> = vec![];
        let one_element: Vec<i32> = vec![5];
        assert_eq!(median(&empty), None);
        assert_eq!(median(&one_element).unwrap(), Median::Int(5));
    }

    #[test]
    fn unimodal_and_multimodal() {
        let one_mode_vec: Vec<i32> = vec![4, 1, 3, 3, 1, 3];
        let two_mode_vec: Vec<i32> = vec![4, 1, 3, 3, 1, 1, 1, 3, 3];
        assert_eq!(mode(&one_mode_vec).unwrap(), vec![3]);
        assert_eq!(mode(&two_mode_vec).unwrap(), vec![1, 3]);
    }
    #[test]
    fn mode_empty_and_single_element_input() {
        let empty: Vec<i32> = vec![];
        let one_element: Vec<i32> = vec![5];
        assert_eq!(mode(&empty), None);
        assert_eq!(mode(&one_element).unwrap(), vec![5]);
    }
}
