use std::iter::zip;

pub fn contains(haystack: &str, needle: &str) -> bool {
    contains_kernel(haystack, needle, equals)
}

pub fn icontains(haystack: &str, needle: &str) -> bool {
    debug_assert_eq!(
        needle.to_ascii_lowercase(),
        needle,
        "needle must be lower case"
    );
    contains_kernel(haystack, needle, i_equals)
}

pub fn starts_with(haystack: &str, needle: &str) -> bool {
    if needle.len() > haystack.len() {
        false
    } else {
        zip(haystack.as_bytes().iter(), needle.as_bytes().iter()).all(equals)
    }
}

pub fn istarts_with(haystack: &str, needle: &str) -> bool {
    debug_assert_eq!(
        needle.to_ascii_lowercase(),
        needle,
        "needle must be lower case"
    );

    if needle.len() > haystack.len() {
        false
    } else {
        zip(haystack.as_bytes().iter(), needle.as_bytes().iter()).all(i_equals)
    }
}

pub fn ends_with(haystack: &str, needle: &str) -> bool {
    if needle.len() > haystack.len() {
        false
    } else {
        zip(haystack.as_bytes().iter().rev(), needle.as_bytes().iter().rev()).all(equals)
    }
}

pub fn iends_with(haystack: &str, needle: &str) -> bool {
    debug_assert!(needle.is_ascii(), "needle must be ascii");

    if needle.len() > haystack.len() {
        false
    } else {
        zip(haystack.as_bytes().iter().rev(), needle.as_bytes().iter().rev()).all(i_equals)
    }
}

fn i_equals((n, h): (&u8, &u8)) -> bool {
    n.to_ascii_lowercase() == h.to_ascii_lowercase()
}

fn equals((n, h): (&u8, &u8)) -> bool {
    n == h
}

fn contains_kernel(haystack: &str, needle: &str, kernel: impl Fn((&u8, &u8)) -> bool) -> bool {
    let Some((needle_first, needle_rest)) = needle.as_bytes().split_first() else {
        // needle is empty, contains always true, matches `str.contains()` behaviour
        return true;
    };
    let mut hay_iter = haystack.as_bytes().iter();
    if needle_rest.is_empty() {
        return hay_iter.any(|h| kernel((needle_first, h)));
    }

    let Some(stop_at) = haystack.len().checked_sub(needle.len()) else {
        // needle is longer than haystack
        return false;
    };
    let mut index: usize = 0;

    while let Some(hay_byte) = hay_iter.next() {
        if kernel((needle_first, hay_byte)) && rest_match(needle_rest, hay_iter.clone(), &kernel) {
            return true;
        } else {
            if index >= stop_at {
                break;
            }
            index += 1;
        }
    }
    false
}

fn rest_match<'a>(
    needle_rest: &[u8],
    hay_iter: impl Iterator<Item = &'a u8>,
    kernel: impl Fn((&u8, &u8)) -> bool,
) -> bool {
    needle_rest.iter().zip(hay_iter).all(kernel)
}
