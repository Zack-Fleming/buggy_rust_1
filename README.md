# Bugged Rust Functions

10 functions in `src/lib.rs` have logic errors. The tests in `tests/tests.rs` will highlight how each function is wrong.

> [!TIP]
> [VS Code](https://code.visualstudio.com/) offers a great toolset for Rust development. Details [here](https://code.visualstudio.>com/docs/languages/rust).

### How to set up the assignment.

1. [Install Rust](https://www.rust-lang.org/tools/install). Windows users may additionally need to [install C++ build tools](https://learn.microsoft.com/en-us/windows/dev-environment/rust/setup).
2. Clone this repository to your computer. If needed, [instructions here](https://docs.github.com/en/repositories/creating-and-managing-repositories/cloning-a-repository).
3. Navigate into cloned repo and run `cargo test` in your command line or terminal. Test output will look something like this:

```shell
$ cargo test
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/buggy_rust_1-bb8581e2afd8396a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/tests.rs (target/debug/deps/tests-742fa85216031325)

running 11 tests
test tests::add_numbers_within_text_test ... FAILED
test tests::fizz_the_odds_test ... FAILED
test tests::get_second_two_elements_test ... FAILED
test tests::get_circle_area_test ... FAILED
test tests::negate_the_array_test ... FAILED
test tests::try_divide_test_two ... FAILED
test tests::string_length_excluding_whitespace ... FAILED
test tests::try_divide_test_one ... FAILED
test tests::point_quadrant_test ... FAILED
test tests::read_name_from_file_test ... FAILED
test tests::two_plus_two_is_four ... ok

failures:

---- tests::add_numbers_within_text_test stdout ----
thread 'tests::add_numbers_within_text_test' panicked at tests/tests.rs:60:9:
assertion `left == right` failed
  left: 15
 right: 2
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- tests::fizz_the_odds_test stdout ----
thread 'tests::fizz_the_odds_test' panicked at tests/tests.rs:41:9:
assertion `left == right` failed
  left: "fizz"
 right: "buzz"

---- tests::get_second_two_elements_test stdout ----
thread 'tests::get_second_two_elements_test' panicked at tests/tests.rs:23:9:
assertion `left == right` failed
  left: [11, 12]
 right: [11]

---- tests::get_circle_area_test stdout ----
thread 'tests::get_circle_area_test' panicked at tests/tests.rs:99:9:
assertion `left == right` failed
  left: 12.566370614359172
 right: 0.0

---- tests::negate_the_array_test stdout ----
thread 'tests::negate_the_array_test' panicked at tests/tests.rs:34:9:
assertion `left == right` failed
  left: [-10, -11, -12, -13]
 right: [9, 10, 11, 12]

---- tests::try_divide_test_two stdout ----
thread 'tests::try_divide_test_two' panicked at tests/tests.rs:75:9:
assertion `left == right` failed
  left: Err(DivisionByZero)
 right: Ok(0)

---- tests::string_length_excluding_whitespace stdout ----
thread 'tests::string_length_excluding_whitespace' panicked at tests/tests.rs:16:9:
assertion `left == right` failed
  left: 9
 right: 5

---- tests::try_divide_test_one stdout ----
thread 'tests::try_divide_test_one' panicked at tests/tests.rs:68:9:
assertion `left == right` failed
  left: None
 right: Some(0)

---- tests::point_quadrant_test stdout ----
thread 'tests::point_quadrant_test' panicked at tests/tests.rs:51:9:
assertion `left == right` failed
  left: One
 right: Two

---- tests::read_name_from_file_test stdout ----
thread 'tests::read_name_from_file_test' panicked at tests/tests.rs:86:25:
Error reading file: Os { code: 2, kind: NotFound, message: "No such file or directory" }


failures:
    tests::add_numbers_within_text_test
    tests::fizz_the_odds_test
    tests::get_circle_area_test
    tests::get_second_two_elements_test
    tests::negate_the_array_test
    tests::point_quadrant_test
    tests::read_name_from_file_test
    tests::string_length_excluding_whitespace
    tests::try_divide_test_one
    tests::try_divide_test_two

test result: FAILED. 1 passed; 10 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--test tests`
```

4. For each failure, fix the logic error in `src/lib.rs`. Don't change anything in `tests/tests.rs`! Save and re-run `cargo test` to confirm that you see <strong>ok</strong> instead of <strong>FAILED</strong>. When you've corrected all 10 errors, your `cargo test` output should look like this:

```shell
$ cargo test
   Compiling buggy_rust_1 v0.1.0 (/Users/tommahoney/Documents/GitHub/buggy_rust_1)
    Finished test [unoptimized + debuginfo] target(s) in 0.11s
     Running unittests src/lib.rs (target/debug/deps/buggy_rust_1-bb8581e2afd8396a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/tests.rs (target/debug/deps/tests-742fa85216031325)

running 11 tests
test tests::add_numbers_within_text_test ... ok
test tests::get_circle_area_test ... ok
test tests::fizz_the_odds_test ... ok
test tests::get_second_two_elements_test ... ok
test tests::negate_the_array_test ... ok
test tests::point_quadrant_test ... ok
test tests::string_length_excluding_whitespace ... ok
test tests::try_divide_test_one ... ok
test tests::read_name_from_file_test ... ok
test tests::try_divide_test_two ... ok
test tests::two_plus_two_is_four ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests buggy_rust_1

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```