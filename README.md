# quick strings

---

**NOTE: See [BurntSushi/memchr#156](https://github.com/BurntSushi/memchr/pull/156) for details on the shortcomings of this crate.**

---

Some string comparison functions in Rust that a faster than the standard library.

| Function                      | Performance(ns) | Relative Performance |
|-------------------------------|-----------------|----------------------|
| `str.contains`                | 964.96          | 1.00x (reference)    |
| `quick_strings::contains`     | 344.27          | 2.80x faster         |
| `quick_strings::icontains`    | 475.31          |                      |
| `str.starts_with`             | 161.38          | 1.00x (reference)    |
| `quick_strings::starts_with`  | 47.30           | 3.41x faster         |
| `quick_strings::istarts_with` | 72.61           |                      |
| `str.ends_with`               | 165.90          | 1.00x (reference)    |
| `quick_strings::ends_with`    | 72.64           | 2.28x faster         |
| `quick_strings::iends_with`   | 116.24          |                      |

* All `quick_strings` functions have the signature `fn(haystack: &str, needle: &str) -> bool`.
* all `i*` functions (case-insensitive) require the `needle` to be ascii only.
