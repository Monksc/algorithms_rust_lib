# algorithms_rust_lib
A library written in rust that contains common algorithms.

# Cargo.toml

```
...
[dependencies]
algorithms = {  git = "https://github.com/Monksc/algorithms_rust_lib", rev = "c150912"}
```

# Examples

```
assert_eq!(seen_before_helper(123, 200, |index| -> bool { arr[index] < 129 }), 129-123);

assert_eq!(algorithms::seen_before(vec![5,8,10,22,99], 10), 2);
assert_eq!(algorithms::seen_before_or_equal(vec![5,8,10,22,99], 10), 3);

assert_eq!(algorithms::seen_before_address(vec![5,8,10,22,99], &10), 2);
assert_eq!(algorithms::seen_before_or_equal_address(vec![5,8,10,22,99], &10), 3);
```
