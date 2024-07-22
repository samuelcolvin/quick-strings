use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

use quick_strings::{contains, ends_with, icontains, iends_with, istarts_with, starts_with};

macro_rules! run_case {
    ($function:ident, $haystack:literal, $needle:literal) => {
    black_box($function(black_box($haystack), black_box($needle)));
    }
}
macro_rules! run_cases {
    ($function:ident) => {{
        run_case!($function, "haystack", "hay");
        run_case!($function, "haystack", "haystack");
        run_case!($function, "haystack", "h");
        run_case!($function, "haystack", "k");
        run_case!($function, "haystack", "stack");
        run_case!($function, "haystack", "sta");
        run_case!($function, "hay£stack", "stack");
        run_case!($function, "hay£stack", "y£s");
        run_case!($function, "hay£stack", "£");
        run_case!($function, "hay£stack", "hay£");
        run_case!($function, "a", "a");
        run_case!($function, "xa", "a");
        run_case!($function, "short", "sh");
        run_case!($function, "short", "or");
        run_case!($function, "short", "rt");
        run_case!($function, "short", "hort");
        run_case!($function, "short", "short");
        run_case!($function, "something at start and something in the middle and something at the end", "some");
        run_case!($function, "something at start and something in the middle and something at the end", "start");
        run_case!($function, "something at start and something in the middle and something at the end", "in the middle");
        run_case!($function, "something at start and something in the middle and something at the end", "at the end");
        run_case!($function, "short string", "s");
        run_case!($function, "short string", "t");
        run_case!($function, "short string", "g");
        run_case!($function, "short string", "z");
        run_case!($function, "needle at the start", "needle");
        run_case!($function, "placed at the end", "end");
        run_case!($function, "non-empty", "");
        run_case!($function, "ABCDEFGHIJKLMNOPQRSTUVWXYZ", "XYZ");
        run_case!($function, "1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890", "67890");
        run_case!($function, "1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890x", "67890x");
        run_case!($function, "123456789012345678901234567890123456789012345678901234567890123456789012345678901234567x8901234567890", "7x8");
        run_case!($function, "special!@#$%^&*()_+", "!@#");
        run_case!($function, "here-is-a-mixed_string.with,different;separators", "with,different;");
        run_case!($function, "CaseSensitive", "Case");
        run_case!($function, "abababababababababababababababababababababababababababababababababababababababababab", "baba");
        run_case!($function, "utf8 test ñandú", "ñ");
        run_case!($function, "utf8 test 中文", "中文");
        run_case!($function, "symbols §!%&", "§!");
        run_case!($function, "emojis 😊😂👍", "😂👍");
        // not matching
        run_case!($function, "x", "s");
        run_case!($function, "haystack", "hy");
        run_case!($function, "haystack", "stackx");
        run_case!($function, "haystack", "x");
        run_case!($function, "haystack", "haystack haystack");
        run_case!($function, "short", "LONGERTHANHAYSTACK");
        run_case!($function, "Hello World", "hello");
        run_case!($function, "Hello World", "Hello");
        run_case!($function, "HELLO WORLD", "hello");
        run_case!($function, "HELLO WORLD", "world");
        run_case!($function, "A", "a");
        run_case!($function, "tiny", "tiny but longer");
        run_case!($function, "", "");
        run_case!($function, "non-empty", "");
        run_case!($function, "", "non-empty");
        run_case!($function, "CaseSensitive", "casesensitive");
        run_case!($function, "The quick brown fox jumps over the lazy dog", "QUICK");
        run_case!($function, "utf8 test ñandú", "😂👍");
        run_case!($function, "utf8 test 中文", "ñ");
        run_case!($function, "symbols §!%&", "中文");
        run_case!($function, "emojis 😊😂👍", "ñ");
    }};
}

fn str_dot_contains(haystack: &str, needle: &str) -> bool {
    haystack.contains(needle)
}

fn str_dot_starts_with(haystack: &str, needle: &str) -> bool {
    haystack.starts_with(needle)
}

fn str_dot_ends_with(haystack: &str, needle: &str) -> bool {
    haystack.ends_with(needle)
}

fn arrow_starts_with_ignore_ascii_case(haystack: &str, needle: &str) -> bool {
    let end = haystack.len().min(needle.len());
    haystack.is_char_boundary(end) && needle.eq_ignore_ascii_case(&haystack[..end])
}

fn arrow_ends_with_ignore_ascii_case(haystack: &str, needle: &str) -> bool {
    let start = haystack.len().saturating_sub(needle.len());
    haystack.is_char_boundary(start) && needle.eq_ignore_ascii_case(&haystack[start..])
}


fn cases(c: &mut Criterion) {
    // contains
    c.bench_function("str.contains", |b| {
        b.iter(|| run_cases!(str_dot_contains))
    });
    c.bench_function("quick_strings::contains", |b| {
        b.iter(|| run_cases!(contains))
    });
    c.bench_function("quick_strings::icontains", |b| {
        b.iter(|| run_cases!(icontains))
    });

    // starts with
    c.bench_function("str.starts_with", |b| {
        b.iter(|| run_cases!(str_dot_starts_with))
    });
    c.bench_function("quick_strings::starts_with", |b| {
        b.iter(|| run_cases!(starts_with))
    });
    c.bench_function("quick_strings::istarts_with", |b| {
        b.iter(|| run_cases!(istarts_with))
    });
    c.bench_function("arrow::starts_with_ignore_ascii_case", |b| {
        b.iter(|| run_cases!(arrow_starts_with_ignore_ascii_case))
    });

    // ends with
    c.bench_function("str.ends_with", |b| {
        b.iter(|| run_cases!(str_dot_ends_with))
    });
    c.bench_function("quick_strings::ends_with", |b| {
        b.iter(|| run_cases!(ends_with))
    });
    c.bench_function("quick_strings::iends_with", |b| {
        b.iter(|| run_cases!(iends_with))
    });
    c.bench_function("arrow::ends_with_ignore_ascii_case", |b| {
        b.iter(|| run_cases!(arrow_ends_with_ignore_ascii_case))
    });
}

criterion_group!(benches, cases);
criterion_main!(benches);
