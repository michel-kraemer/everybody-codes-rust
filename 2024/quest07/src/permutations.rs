/// Returns an iterator over all possible permutations of a given set of items.
/// The permutations will be returned in lexicographic order, automatically
/// skipping duplicates.
///
/// See https://en.wikipedia.org/wiki/Permutation#Generation_in_lexicographic_order
///
/// Example:
///
/// ```
/// let p = permutations_lexicographic(&['A', 'B', 'A']).collect::<Vec<_>>();
/// assert_eq!(p, vec![vec!['A', 'A', 'B'], vec!['A', 'B', 'A'], vec!['B', 'A', 'A']]);
/// ```
pub fn permutations_lexicographic<T>(a: &[T]) -> PermutationsIterator<T>
where
    T: Clone + Ord,
{
    let mut v = a.to_vec();
    v.sort_unstable();
    PermutationsIterator { next_item: Some(v) }
}

pub struct PermutationsIterator<T>
where
    T: Clone + Ord,
{
    next_item: Option<Vec<T>>,
}

impl<T> Iterator for PermutationsIterator<T>
where
    T: Clone + Ord,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let r = self.next_item.take()?;

        if r.len() < 2 {
            // the list has only one element
            return Some(r);
        }

        let mut k = r.len() - 2;
        while r[k] >= r[k + 1] {
            if k == 0 {
                // Return the last possible permutation. Every subsequent
                // call to next() will return None.
                return Some(r);
            }
            k -= 1;
        }

        let mut l = k + 1;
        while l + 1 < r.len() && r[k] < r[l + 1] {
            l += 1;
        }

        // produce next permutation
        let mut nr = r.clone();
        nr.swap(k, l);

        let mut i = k + 1;
        let mut j = nr.len() - 1;
        while i < j {
            nr.swap(i, j);
            i += 1;
            j -= 1;
        }

        self.next_item = Some(nr);

        Some(r)
    }
}
