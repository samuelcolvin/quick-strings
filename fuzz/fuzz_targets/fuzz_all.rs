#![no_main]

use libfuzzer_sys::fuzz_target;

fn check(haystack: &str, needle: &str) {
    // haystack.contains(needle);
    assert_eq!(haystack.contains(needle), quick_strings::contains(haystack, needle));
    assert_eq!(haystack.starts_with(needle), quick_strings::starts_with(haystack, needle));
    assert_eq!(haystack.ends_with(needle), quick_strings::ends_with(haystack, needle));

    if needle.is_ascii() {
        let hay_lower = haystack.to_lowercase();
        let nee_lower = needle.to_lowercase();
        assert_eq!(hay_lower.as_str().contains(&nee_lower), quick_strings::icontains(haystack, needle));
        assert_eq!(hay_lower.as_str().starts_with(&nee_lower), quick_strings::istarts_with(haystack, needle));
        assert_eq!(hay_lower.as_str().ends_with(&nee_lower), quick_strings::iends_with(haystack, needle));
    }
}


fuzz_target!(|data: String| {
    for split in 0..data.len() {
        if let Some((haystack, needle)) = data.split_at_checked(split) {
            check(&haystack, &needle);
        }
    }
});
