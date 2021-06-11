/// Finds next lexographic permutation of given slice. If next larger permutation exists,
/// the slice would then have this and the return value would be true. If the slice
/// represents the largest lexicographic sequence, it updates the slice with smallest one
/// and returns false.
/// The type whose permutation is required must implement [Ord](std::cmp::Ord) trait.
pub fn next_permutation<T>(arr: &mut [T]) -> bool
where
    T: std::cmp::Ord,
{
    use std::cmp::Ordering;

    // find 1st pair (x, y) from back which satisfies x < y
    let last_ascending = match arr.windows(2).rposition(|w| w[0] < w[1]) {
        Some(i) => i,
        None => {
            arr.reverse();
            return false;
        }
    };

    // In the remaining later segment, find the one which is just
    // larger that the index found above.
    // SAFETY: unwrap_err whill not panic since binary search will
    // will never succeed since we never return equal ordering
    let swap_with = arr[last_ascending + 1..]
        .binary_search_by(|n| match arr[last_ascending].cmp(n) {
            Ordering::Equal => Ordering::Less,
            ord => ord,
        })
        .unwrap_err();
    arr.swap(last_ascending, last_ascending + swap_with);
    arr[last_ascending + 1..].reverse();
    true
}

#[test]
fn distinct() {
    let mut start = vec![1, 4, 6, 8];

    let next_state = vec![
        (vec![1, 4, 8, 6], true),
        (vec![1, 6, 4, 8], true),
        (vec![1, 6, 8, 4], true),
        (vec![1, 8, 4, 6], true),
        (vec![1, 8, 6, 4], true),
        (vec![4, 1, 6, 8], true),
        (vec![4, 1, 8, 6], true),
        (vec![4, 6, 1, 8], true),
        (vec![4, 6, 8, 1], true),
        (vec![4, 8, 1, 6], true),
        (vec![4, 8, 6, 1], true),
        (vec![6, 1, 4, 8], true),
        (vec![6, 1, 8, 4], true),
        (vec![6, 4, 1, 8], true),
        (vec![6, 4, 8, 1], true),
        (vec![6, 8, 1, 4], true),
        (vec![6, 8, 4, 1], true),
        (vec![8, 1, 4, 6], true),
        (vec![8, 1, 6, 4], true),
        (vec![8, 4, 1, 6], true),
        (vec![8, 4, 6, 1], true),
        (vec![8, 6, 1, 4], true),
        (vec![8, 6, 4, 1], true),
        (vec![1, 4, 6, 8], false),
    ];

    for (next_vec, ret) in next_state {
        assert_eq!(ret, next_permutation(&mut start));
        assert_eq!(next_vec, start);
    }
}

#[test]
fn non_distinct() {
    let mut start = vec![1, 4, 6, 6];

    let next_state = vec![
        (vec![1, 6, 4, 6], true),
        (vec![1, 6, 6, 4], true),
        (vec![4, 1, 6, 6], true),
        (vec![4, 6, 1, 6], true),
        (vec![4, 6, 6, 1], true),
        (vec![6, 1, 4, 6], true),
        (vec![6, 1, 6, 4], true),
        (vec![6, 4, 1, 6], true),
        (vec![6, 4, 6, 1], true),
        (vec![6, 6, 1, 4], true),
        (vec![6, 6, 4, 1], true),
        (vec![1, 4, 6, 6], false),
    ];

    for (next_vec, ret) in next_state {
        assert_eq!(ret, next_permutation(&mut start));
        assert_eq!(next_vec, start);
    }
}
