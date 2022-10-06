// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand for a hint.


#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..4]; // starting_index..ending_index where ending_index = 1 + last position in the slice
    // internally: length of the slice = ending_index - starting_index

    assert_eq!([2, 3, 4], nice_slice)
}
