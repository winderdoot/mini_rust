# Data types

- integer overflow:
  - panics in debug
  - wraps in release but is considered an error

Use methods:
- ```wrapped_*``` to safely use wrapping of ints
- ```checked_*``` to return None in case overflow happens
- ```overflowing_*``` to return a bool indicating whether there is overflow
- ```saturating_*``` to perform the operation and return a max value if overflowed

