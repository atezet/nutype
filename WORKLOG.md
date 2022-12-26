## Roadmap

### TODO:
* Support decimals libraries:
  * https://crates.io/crates/rust_decimal
* Refactor parsers
* Finalize syntax!
* Run UI tests only against stable
* Write a good detailed README
  * Quickstart
  * Philosophy
  * For devs
* Add LICENSE
* Add all Meta info to Cargo.toml and github repo
* Maybe: rename nutype_macros -> nutype_macro ?
* Create a logo?
  * Use this font: https://www.fontspace.com/stoner-font-f81576

### TODO Refactor:
* Introduce newtypes for type_name, error_type_name, etc.

### TODO UI:
* UI: Impl  "did you mean" hints:
  UI: * E.g. unknown validation rule `min`. Did you mean `min_len`?
* UI: Generate documentation automatically.
* UI: Intercept derive of DerefMut, AsMut, BorrowMut and print an explaining error message
* UI: On derive handle the following dependencies:
  * Copy requires Clone
  * Ord requires PartialOrd
  * Eq requires PartialEq

### Later
* Support Arbitrary
* Setup CI
* String sanitizers:
  * capitalize
  * truncate
  * Remove extra spaces
* Extra validations for floats:
  * `is_number` / `is_finite` (aka not NaN, and not `Inifinity`)
  * This should allow to derive Eq and Ord
* Consider extending errors to keep the invalid value?
* Regex
  * See https://github.com/CryptArchy/regex_generate to impl support with arbitrary

### Maybe
* Add #[repr(transparent)]
* Add #[allow(non_snake_case)] for module names
* Support time libraries (e.g. chrono, time)


### Done
* Custom sanitizers for strings
* Custom validators for strings
* Custom sanitizers for numbers
* Custom validators for numbers
* Setup compiletests
* Use `new`, instead of `from` and `try_from`
* Respect visibility
* Respect documentation
* Implement std::error::Error for errors
* Support derive for String
* Support derive of From and TryFrom for String types
* Add UI tests to detect conflicts:
  * derive(TryFrom) without validations
  * derive(From) with validations
* Support derive Hash for String
* Impl FromStr for String types
* Support derive of Borrow<str> and Borrow<String> for String types
* Refactor numbers and split into groups: integer and float.
* Support derive for integers
* Support derive for floats
* Support derive of Into trait for String
* Support derive of Into trait for integers
* Support derive of Into trait for floats
* Refactor: extract common generator functions
* Cleanup tests: split number tests into integer and float
* Use absolute path to `Result` in the generated code
* Rename inner generated `validate` and `sanitize` methods into something more unique. Otherwise it may conflict because of `use super::*;`
* Rename default inner modules into something less scary
* Impl FromStr for floats
* Impl Display for errors on integers and floats. + add tests
* Improve Display for parse error of float: src/common/gen/parse_error.rs
* Impl FromStr for integer
* Derive Display
* Rename nutype_derive to nutype_macros
* Rename nutype_test_suite to `test_suite`
* Remove sanitizer `clamp(a, b)` from integer
* Remove sanitizer `clamp(a, b)` from float
* Address TODO, todo!(), unimplemented!(), etc.
* Reduce duplications: the types share mainly same structure
* UI: Hide private module docs: Use #[doc(hidden)] on the module!
* UI: Validate and show helpful error on attempt to make inner field public. E.g. `Value(pub i32)`
* Find a way to bypass serde1 feature flag from `nutype` to `nutype_macros`.
* Support serde: impl Serialize
* Impl Serialize tests for: integer, float, string
* Impl Clone tests for: integer, float, string
* Impl Copy tests for: integer, float
* Support serde: Deserialize
* Use `Infallible` type



## Similar crates

* bounded_integer
* semval
* refinement


## Why to use nutype

## Why not to use nutype?
* Meta programming can hit you sometimes by surprise and it could be too much magic.
* IDEs might not be smart enough to handle some refactoring of a code generated with proc macro.