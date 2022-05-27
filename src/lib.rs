// Insertion sort
// adapted from C code in: Skiena - Algorithm design manual
fn insertion_sort<T: std::cmp::PartialOrd>(list: &mut [T]) {
    let length = list.len();
    for i in 1..length {
        let mut j = i;
        while j > 0 && list[j] < list[j - 1] {
            list.swap(j, j - 1);
            j = j - 1;
        }
    }
}

// Bubble sort
// adapted from pseudocode code in: Skiena - Algorithm design manual
fn bubble_sort<T: std::cmp::PartialOrd>(list: &mut [T]) {
    let length = list.len();
    for i in (1..length).rev() {
        for j in 0..=i - 1 {
            if list[j] > list[j + 1] {
                list.swap(j, j + 1);
            }
        }
        // at this point list[k] <= list[i] <= list[l], for k < i < l;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Insertion sort
    #[test]
    fn sort_empty_list() {
        let mut list: [i32; 0] = [];
        insertion_sort(&mut list);
        assert_eq!(list, []);
    }

    #[test]
    fn sort_singleton_list() {
        let mut list = [1];
        insertion_sort(&mut list);
        assert_eq!(list, [1]);
    }

    #[test]
    fn sort_ordered() {
        let mut list = [1, 2, 3, 4];
        insertion_sort(&mut list);
        assert_eq!(list, [1, 2, 3, 4]);
    }

    #[test]
    fn sort_reversed() {
        let mut list = [4, 3, 2, 1];
        insertion_sort(&mut list);
        assert_eq!(list, [1, 2, 3, 4]);
    }

    #[test]
    fn sort_1() {
        let mut list = [1, 3, 4, 2];
        insertion_sort(&mut list);
        assert_eq!(list, [1, 2, 3, 4]);
    }

    #[test]
    fn sort_2() {
        let mut list = [6, 3, 4, 2, 7, 5, 1];
        insertion_sort(&mut list);
        assert_eq!(list, [1, 2, 3, 4, 5, 6, 7]);
    }

    // TODO: unify tests
    // Bubble sort
    #[test]
    fn bubble_sort_empty_list() {
        let mut list: [i32; 0] = [];
        bubble_sort(&mut list);
        assert_eq!(list, []);
    }

    #[test]
    fn bubble_sort_singleton_list() {
        let mut list = [1];
        bubble_sort(&mut list);
        assert_eq!(list, [1]);
    }

    #[test]
    fn bubble_sort_ordered() {
        let mut list = [1, 2, 3, 4];
        bubble_sort(&mut list);
        assert_eq!(list, [1, 2, 3, 4]);
    }

    #[test]
    fn bubble_sort_reversed() {
        let mut list = [4, 3, 2, 1];
        bubble_sort(&mut list);
        assert_eq!(list, [1, 2, 3, 4]);
    }

    #[test]
    fn bubble_sort_1() {
        let mut list = [1, 3, 4, 2];
        bubble_sort(&mut list);
        assert_eq!(list, [1, 2, 3, 4]);
    }

    #[test]
    fn bubble_sort_2() {
        let mut list = [6, 3, 4, 2, 7, 5, 1];
        bubble_sort(&mut list);
        assert_eq!(list, [1, 2, 3, 4, 5, 6, 7]);
    }
}
