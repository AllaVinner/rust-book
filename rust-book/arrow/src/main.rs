use arrow::*;
use arrow::array::Array;

fn main() {
    let array = array::Int32Array::from(vec![Some(1), None, Some(3)]);
    assert_eq!(array.len(), 3);
    assert_eq!(array.value(0), 1);
    assert_eq!(array.is_null(1), true);
    
    let collected: Vec<_> = array.iter().collect();
    assert_eq!(collected, vec![Some(1), None, Some(3)]);
    assert_eq!(array.values(), &[1, 0, 3])
}
