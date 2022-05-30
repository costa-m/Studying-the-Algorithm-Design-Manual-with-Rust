mod linked_list;

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

// Selection sort
// adapted from C code in: Skiena - Algorithm design manual
fn selection_sort<T: std::cmp::PartialOrd>(list: &mut [T]) {
    let length = list.len();
    for i in 0..length {
        let minimum_element_index = i;
        for j in i + 1..length {
            if list[j] < list[minimum_element_index] {
                list.swap(j, minimum_element_index);
            }
        }
        // at this point list[i] <= list[l], for i < l
    }
}

#[cfg(test)]
mod tests {
    use super::linked_list as ll;
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

    // TODO: unify tests
    // bubble sort
    #[test]
    fn selection_sort_empty_list() {
        let mut list: [i32; 0] = [];
        selection_sort(&mut list);
        assert_eq!(list, []);
    }

    #[test]
    fn selection_sort_singleton_list() {
        let mut list = [1];
        selection_sort(&mut list);
        assert_eq!(list, [1]);
    }

    #[test]
    fn selection_sort_ordered() {
        let mut list = [1, 2, 3, 4];
        selection_sort(&mut list);
        assert_eq!(list, [1, 2, 3, 4]);
    }

    #[test]
    fn selection_sort_reversed() {
        let mut list = [4, 3, 2, 1];
        selection_sort(&mut list);
        assert_eq!(list, [1, 2, 3, 4]);
    }

    #[test]
    fn selection_sort_1() {
        let mut list = [1, 3, 4, 2];
        selection_sort(&mut list);
        assert_eq!(list, [1, 2, 3, 4]);
    }

    #[test]
    fn selection_sort_2() {
        let mut list = [6, 3, 4, 2, 7, 5, 1];
        selection_sort(&mut list);
        assert_eq!(list, [1, 2, 3, 4, 5, 6, 7]);
    }

    // inserting elements
    #[test]
    fn insert_element_on_empty_list() {
        let list = ll::List::Elem(5, Box::new(ll::List::Empty));
        let mut list2 = ll::List::Empty;
        ll::insert(&mut list2, 5);
        assert_eq!(list, list2);
    }

    #[test]
    fn insert_multiple_elements_on_empty_list() {
        let list = ll::List::Elem(5, Box::new(ll::List::Elem(7, Box::new(ll::List::Empty))));
        let mut list2 = ll::List::Empty;
        ll::insert(&mut list2, 7);
        ll::insert(&mut list2, 5);
        assert_eq!(list, list2);
    }

    #[test]
    fn insert_element_on_nonempty_list() {
        let list = ll::List::Elem(5, Box::new(ll::List::Elem(7, Box::new(ll::List::Empty))));
        let mut list2 = ll::List::Elem(7, Box::new(ll::List::Empty));
        ll::insert(&mut list2, 5);
        assert_eq!(list, list2);
    }

    // searching elements
    #[test]
    fn search_element_on_empty_list() {
        let list = ll::List::Empty;
        assert_eq!(ll::search_list(&list, &5), &ll::List::<i32>::Empty);
    }

    #[test]
    fn search_element_on_list_not_containing_it() {
        let mut list = ll::List::Empty;
        ll::insert(&mut list, 7);
        ll::insert(&mut list, 5);
        assert_eq!(ll::search_list(&list, &8), &ll::List::<i32>::Empty);
    }

    #[test]
    fn search_element_on_list_containing_it_in_beginning() {
        let mut list = ll::List::Empty;
        ll::insert(&mut list, 9);
        ll::insert(&mut list, 7);
        ll::insert(&mut list, 5);
        assert_eq!(ll::search_list(&list, &5), &list);
    }

    #[test]
    fn search_element_on_list_containing_it_in_end() {
        let mut list = ll::List::Empty;
        ll::insert(&mut list, 9);
        let tail = list.clone();
        ll::insert(&mut list, 7);
        ll::insert(&mut list, 5);
        assert_eq!(ll::search_list(&list, &9), &tail);
    }

    #[test]
    fn search_element_on_list_containing_it_in_middle() {
        let mut list = ll::List::Empty;
        ll::insert(&mut list, 9);
        ll::insert(&mut list, 7);
        let tail = list.clone();
        ll::insert(&mut list, 5);
        assert_eq!(ll::search_list(&list, &7), &tail);
    }
}
