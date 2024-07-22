# quick strings

Some string comparison functions in Rust that a faster than the standard library.

| Function                      | Performance(ns) | Relative Performance |
|-------------------------------|-----------------|----------------------|
| `str.contains`                | 936.91          | 1.00x (reference)    |
| `quick_strings::contains`     | 385.53          | 2.43x faster         |
| `quick_strings::icontains`    | 547.25          |                      |
| `str.starts_with`             | 153.02          | 1.00x (reference)    |
| `quick_strings::starts_with`  | 89.95           | 1.70x                |
| `quick_strings::istarts_with` | 114.94          |                      |
| `str.ends_with`               | 159.36          | 1.00x (reference)    |
| `quick_strings::ends_with`    | 108.57          | ???                  |
| `quick_strings::iends_with`   | 163.86          | 1.47x                |
