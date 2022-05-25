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

#[cfg(test)]
mod tests {
    use super::*;

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
}
