use crate::load_partial_parameters;

#[test]
fn test_load_partial_parameters() {
    let partial_parameters = load_partial_parameters();
    assert_eq!(3592860, partial_parameters.len());
}
