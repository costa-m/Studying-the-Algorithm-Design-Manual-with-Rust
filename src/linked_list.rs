use std::mem;

// This implementation of a linked list is the first implementation
// in the book "Learning Rust With Entirely Too Many Linked Lists".
// The book has better implementations. I am using this for
// pedagogical reasons.
#[derive(Debug, PartialEq, Clone)]
pub enum List<T: PartialEq> {
    Empty,
    Elem(T, Box<List<T>>),
}

// Search list for element.
// Return sublist headed by the element if found. Return the Empty list otherwise.
// Adapted from C code in: Skiena - Algorithm design manual
pub fn search_list<'a, T: std::cmp::PartialOrd>(list: &'a List<T>, item: &T) -> &'a List<T> {
    match list {
        List::Empty => list,
        List::Elem(head, tail) => {
            if head == item {
                list
            } else {
                search_list(tail, item)
            }
        }
    }
}

// Insert element at beginning of list,
// by doing so it, modifies the list.
// Adapted from C code in: Skiena - Algorithm design manual
pub fn insert<T: std::cmp::PartialEq>(list: &mut List<T>, item: T) {
    *list = List::Elem(item, Box::new(mem::replace(list, List::Empty)));
}
