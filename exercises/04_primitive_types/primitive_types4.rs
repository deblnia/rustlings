// primitive_types4.rs
//
// Get a slice out of Array a where the ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand
// for a hint.

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..4];
    // so rust indices are inclusive
    // from the book: 
        // where starting_index is the first position in the slice and ending_index is one more than the last position in the slice. 

    assert_eq!([2, 3, 4], nice_slice)
}
