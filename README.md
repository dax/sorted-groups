# sorted-groups

[![Crates.io Version](https://badgers.space/crates/version/sorted-groups)](https://crates.io/crates/sorted-groups)
[![Docs.rs Latest](https://badgers.space/badge/docs.rs/latest/blue)](https://docs.rs/sorted-groups)
[![Build Status](https://badgers.space/github/checks/dax/sorted-groups?label=build)](https://github.com/dax/sorted-groups/actions/workflows/build.yaml)

`sorted-groups` implement a data structure to store elements in sorted groups while maintaining the order of elements in each group.

## Usage

First, add the `sorted_groups` crate as a dependency:

```sh
cargo add sorted_groups
```

```rust
use sorted_groups::SortedGroups;

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

// Elements will be grouped by the `group` field
let mut sorted_groups = SortedGroups::<i32, Element, _>::new(|e| e.group);
sorted_groups.insert(Element { group: 1, value: 1 });
sorted_groups.insert(Element { group: 1, value: 2 });
sorted_groups.insert(Element { group: 2, value: 3 });

// `len` returns the total number of elements
assert_eq!(sorted_groups.len(), 3);
// `groups_len` returns the number of groups
assert_eq!(sorted_groups.groups_len(), 2);
// `iter` returns an iterator over groups and elements
let mut iter = sorted_groups.iter();
assert_eq!(iter.next(), Some((&1, &Element { group: 1, value: 1 })));
assert_eq!(iter.next(), Some((&1, &Element { group: 1, value: 2 })));
assert_eq!(iter.next(), Some((&2, &Element { group: 2, value: 3 })));
assert_eq!(iter.next(), None);
```

## License

This project is distributed under the terms of the Apache License (Version 2.0).

See [LICENSE](LICENSE)

