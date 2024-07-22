use quick_strings;

#[test]
fn test_contains() {
    assert!(quick_strings::contains("haystack", "hay"));
    assert!(quick_strings::contains("haystack", "haystack"));
    assert!(quick_strings::contains("haystack", "h"));
    assert!(quick_strings::contains("haystack", "k"));
    assert!(quick_strings::contains("haystack", "stack"));
    assert!(quick_strings::contains("haystack", "sta"));
    assert!(quick_strings::contains("hay£stack", "stack"));
    assert!(quick_strings::contains("hay£stack", "y£s"));
    assert!(quick_strings::contains("hay£stack", "£"));
    assert!(quick_strings::contains("a", "a"));
    // not matching
    assert!(!quick_strings::contains("haystack", "hy"));
    assert!(!quick_strings::contains("haystack", "stackx"));
    assert!(!quick_strings::contains("haystack", "x"));
    assert!(!quick_strings::contains("haystack", "haystack haystack"));
}

#[test]
fn test_icontains() {
    assert!(quick_strings::icontains("haystack", "hay"));
    assert!(quick_strings::icontains("haystack", "stack"));
    assert!(quick_strings::icontains("haystack", "sta"));

    assert!(quick_strings::icontains("HAYSTACK", "hay"));
    assert!(quick_strings::icontains("HAYSTACK", "stack"));
    assert!(quick_strings::icontains("HAYSTACK", "sta"));

    assert!(quick_strings::icontains("HaYsTaCk", "hay"));
    assert!(quick_strings::icontains("HaYsTaCk", "stack"));
    assert!(quick_strings::icontains("HaYsTaCk", "sta"));

    assert!(quick_strings::icontains("A", "a"));
    assert!(!quick_strings::icontains("]", "}"));
}

#[test]
fn test_starts_with() {
    assert!(quick_strings::starts_with("haystack", "hay"));
    assert!(quick_strings::starts_with("h£aystack", "h£ay"));
    assert!(quick_strings::starts_with("haystack", "haystack"));
    assert!(quick_strings::starts_with("haystack", "ha"));
    assert!(quick_strings::starts_with("haystack", "h"));
    assert!(quick_strings::starts_with("haystack", ""));

    assert!(!quick_strings::starts_with("haystack", "stack"));
    assert!(!quick_strings::starts_with("haystack", "haystacks"));
    assert!(!quick_strings::starts_with("haystack", "HAY"));
    assert!(!quick_strings::starts_with("haystack", "h£ay"));
    assert!(!quick_strings::starts_with("h£aystack", "hay"));
}

#[test]
fn test_ends_with() {
    assert!(quick_strings::ends_with("haystack", "stack"));
    assert!(quick_strings::ends_with("hayst£ack", "st£ack"));
    assert!(quick_strings::ends_with("haystack", "haystack"));
    assert!(quick_strings::ends_with("haystack", "ck"));
    assert!(quick_strings::ends_with("haystack", "k"));
    assert!(quick_strings::ends_with("haystack", ""));

    assert!(!quick_strings::ends_with("haystack", "hay"));
    assert!(!quick_strings::ends_with("haystack", "STACK"));
    assert!(!quick_strings::ends_with("haystack", "haystacks"));
    assert!(!quick_strings::ends_with("haystack", "xhaystack"));
    assert!(!quick_strings::ends_with("haystack", "st£ack"));
    assert!(!quick_strings::ends_with("hayst£ack", "stack"));
}

#[test]
fn test_istarts_with() {
    assert!(quick_strings::istarts_with("haystack", "hay"));
    assert!(quick_strings::istarts_with("HAYSTACK", "hay"));
    assert!(quick_strings::istarts_with("HaYsTaCk", "hay"));
    assert!(quick_strings::istarts_with("HaYsTaCk", "haystack"));
    assert!(quick_strings::istarts_with("HaYsTaCk", ""));

    assert!(!quick_strings::istarts_with("haystack", "stack"));
    assert!(!quick_strings::istarts_with("haystack", "haystacks"));
    assert!(!quick_strings::istarts_with("haystack", "h.ay"));
    assert!(!quick_strings::istarts_with("h£aystack", "hay"));
}

#[test]
fn test_iends_with() {
    assert!(quick_strings::iends_with("haystack", "stack"));
    assert!(quick_strings::iends_with("HAYSTACK", "stack"));
    assert!(quick_strings::iends_with("HAYsTaCk", "stack"));
    assert!(quick_strings::iends_with("haystack", "haystack"));
    assert!(quick_strings::iends_with("HAYSTACK", "haystack"));
    assert!(quick_strings::iends_with("haystack", "ck"));
    assert!(quick_strings::iends_with("haystacK", "ck"));
    assert!(quick_strings::iends_with("haystack", ""));

    assert!(!quick_strings::iends_with("haystack", "hay"));
    assert!(!quick_strings::iends_with("HAYSTACK", "stac"));
    assert!(!quick_strings::iends_with("haystack", "haystacks"));
    assert!(!quick_strings::iends_with("haystac£k", "stack"));
    assert!(!quick_strings::iends_with("haystack", "xhaystack"));
}
