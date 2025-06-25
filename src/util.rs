pub(crate) fn strip_nocase_prefix<'a>(s: &'a str, prefix: &str) -> Option<&'a str> {
    let (pre, post) = s.split_at_checked(prefix.len())?;
    pre.eq_ignore_ascii_case(prefix).then_some(post)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("foobar", "foo", Some("bar"))]
    #[case("FOOBAR", "foo", Some("BAR"))]
    #[case("fööBAR", "foo", None)]
    #[case("barfoo", "foo", None)]
    fn test_strip_nocase_prefix(#[case] s: &str, #[case] prefix: &str, #[case] r: Option<&str>) {
        assert_eq!(strip_nocase_prefix(s, prefix), r);
    }
}
