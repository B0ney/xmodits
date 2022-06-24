pub fn load_to_array(array: &mut [char], data: &[u8]) {
    assert!(array.len() <= data.len());

    for i in 0..array.len() {
        array[i] = data[i] as char;
    }
}