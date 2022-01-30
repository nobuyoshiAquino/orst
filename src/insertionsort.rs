use super::Sorter;

pub struct InsertionSort {
    smart: bool,
}

impl Sorter for InsertionSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        // [sorted | not sorted]
        for unsorted in 1..slice.len() {
            // slice[unsorted..] is not sorted
            if !self.smart {
                // take slice[unsorted] and place in sorted location
                // in slice[..=unsorted]
                // [ 1 3 4 | 2]
                // [ 1 3 4 2 | ]
                // [ 1 3 2 4 | ]
                // [ 1 2 3 4 | ]
                let mut i = unsorted;
                while i > 0 && slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    i -= 1;
                }
            } else {
                // use binary search to find index
                let i = match slice[..unsorted].binary_search(&slice[unsorted]) {
                    Ok(i) => i,  // [a, c, e].binary_search(c) => Ok(1)
                    Err(i) => i, // [a, c, e].binary_search(b) => Err(1)
                };

                slice[i..=unsorted].rotate_right(1);
            }
        }
    }
}

#[test]
fn it_works() {
    let mut things = vec![4, 2, 5, 3, 1];
    InsertionSort { smart: false }.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}

#[test]
fn it_works_smart() {
    let mut things = vec![4, 2, 5, 3, 1];
    InsertionSort { smart: true }.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}
