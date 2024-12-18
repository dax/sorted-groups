//! `sorted-groups` implement a data structure to store elements in sorted groups while maintaining the order of elements in each group.
//!
//! # Usage
//!
//! First, add the `sorted_groups` crate as a dependency:
//! ```sh
//! cargo add sorted_groups
//! ```
//!
//! ```
//! use sorted_groups::SortedGroups;
//!
//! #[derive(PartialEq, Eq, Ord, Debug)]
//! struct Element {
//!    group: i32,
//!    value: i32,
//! }
//!
//! impl PartialOrd for Element {
//!    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//!        Some(self.cmp(other))
//!    }
//! }
//!
//! // Elements will be grouped by the `group` field
//! let sorted_groups = SortedGroups::<i32, Element>::new(vec![
//!    Element { group: 1, value: 1 },
//!    Element { group: 1, value: 2 },
//!    Element { group: 2, value: 1 },
//! ], |e| e.group);
//!
//! // `len` returns the total number of elements
//! assert_eq!(sorted_groups.len(), 3);
//! // `groups_len` returns the number of groups
//! assert_eq!(sorted_groups.groups_len(), 2);
//! // `iter` returns an iterator over groups and elements
//! let mut iter = sorted_groups.iter();
//! assert_eq!(iter.next(), Some((&1, &Element { group: 1, value: 1 })));
//! assert_eq!(iter.next(), Some((&1, &Element { group: 1, value: 2 })));
//! assert_eq!(iter.next(), Some((&2, &Element { group: 2, value: 1 })));
//! assert_eq!(iter.next(), None);
//! ```
//!
use std::collections::{btree_map::BTreeMap, btree_set, BTreeSet};

#[derive(Clone, Debug)]
pub struct SortedGroups<G, E>
where
    G: Ord,
    E: Ord,
{
    groups: BTreeMap<G, BTreeSet<E>>,
}

impl<G, E> SortedGroups<G, E>
where
    G: Ord,
    E: Ord,
{
    pub fn new(
        elements: impl IntoIterator<Item = E>,
        group_from_element: impl Fn(&E) -> G,
    ) -> Self {
        let mut groups = BTreeMap::<G, BTreeSet<E>>::new();
        for element in elements {
            groups
                .entry(group_from_element(&element))
                .or_default()
                .insert(element);
        }
        Self { groups }
    }

    pub fn len(&self) -> usize {
        self.groups.values().map(|v| v.len()).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, index: usize) -> Option<(&G, &E)> {
        self.iter().nth(index)
    }

    pub fn groups_len(&self) -> usize {
        self.groups.len()
    }

    pub fn iter_groups(&self) -> impl Iterator<Item = (&G, &BTreeSet<E>)> {
        self.groups.iter()
    }
}

pub struct SortedGroupsIter<'a, G, E> {
    // Iterator over groups
    groups_iter: std::collections::btree_map::Iter<'a, G, BTreeSet<E>>,
    // Current group and its iterator
    current_group: Option<(&'a G, btree_set::Iter<'a, E>)>,
}

impl<G, E> SortedGroups<G, E>
where
    G: Ord,
    E: Ord,
{
    pub fn iter(&self) -> SortedGroupsIter<'_, G, E> {
        let mut groups_iter = self.groups.iter();
        let current_group = groups_iter.next().map(|(g, v)| (g, v.iter()));

        SortedGroupsIter {
            groups_iter,
            current_group,
        }
    }
}

impl<'a, G, E> Iterator for SortedGroupsIter<'a, G, E>
where
    G: Ord,
    E: Ord,
{
    type Item = (&'a G, &'a E);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match &mut self.current_group {
                Some((group, iter)) => {
                    if let Some(element) = iter.next() {
                        return Some((*group, element));
                    } else {
                        // Current group is exhausted, move to next group
                        self.current_group = self.groups_iter.next().map(|(g, v)| (g, v.iter()));
                    }
                }
                None => return None,
            }
        }
    }
}

// Implement IntoIterator for reference
impl<'a, G, E> IntoIterator for &'a SortedGroups<G, E>
where
    G: Ord,
    E: Ord,
{
    type Item = (&'a G, &'a E);
    type IntoIter = SortedGroupsIter<'a, G, E>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<G, E> PartialEq for SortedGroups<G, E>
where
    G: Ord,
    E: Ord,
{
    fn eq(&self, other: &Self) -> bool {
        self.groups.eq(&other.groups)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(PartialEq, Eq, Ord, Debug)]
    struct Element {
        group: i32,
        value: i32,
    }

    impl PartialOrd for Element {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    #[test]
    fn test_empty_sorted_groups() {
        let sorted_groups = SortedGroups::<i32, Element>::new(vec![].into_iter(), |e| e.group);
        assert_eq!(sorted_groups.len(), 0);
    }

    #[test]
    fn test_insert_sorted_groups() {
        let sorted_groups = SortedGroups::<i32, Element>::new(
            vec![
                Element { group: 1, value: 1 },
                Element { group: 1, value: 2 },
                Element { group: 2, value: 1 },
            ],
            |e| e.group,
        );

        assert_eq!(sorted_groups.len(), 3);
        assert_eq!(sorted_groups.groups_len(), 2);
        let mut iter = sorted_groups.iter();
        assert_eq!(iter.next(), Some((&1, &Element { group: 1, value: 1 })));
        assert_eq!(iter.next(), Some((&1, &Element { group: 1, value: 2 })));
        assert_eq!(iter.next(), Some((&2, &Element { group: 2, value: 1 })));
        assert_eq!(iter.next(), None);
    }
}
