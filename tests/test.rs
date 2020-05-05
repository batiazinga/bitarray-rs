use bitarray::Array;

#[test]
#[should_panic]
fn get_row_out_of_bounds() {
    let a = Array::new(12, 20);
    let _discard = a.get(12, 10);
}

#[test]
#[should_panic]
fn get_column_out_of_bounds() {
    let a = Array::new(12, 20);
    let _discard = a.get(10, 20);
}
