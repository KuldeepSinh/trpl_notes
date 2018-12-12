//any test under the tests directory are integration tests.
//As one of the unit tests is failing, cargo test will not run following tests.
//To run following test independently, use command = cargo test --test integration_test
use adder;

#[test]
fn it_adds_two() {
    assert_eq!(adder::add_us(2, 3), 4);
}
