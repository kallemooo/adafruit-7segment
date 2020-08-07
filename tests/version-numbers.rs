use version_sync::{assert_html_root_url_updated, assert_markdown_deps_updated};

#[test]
fn test_readme_deps() {
    assert_markdown_deps_updated!("README.md");
}

#[test]
fn test_html_root_url() {
    assert_html_root_url_updated!("src/lib.rs");
}

#[test]
fn test_readme_mentions_version() {
    version_sync::assert_contains_regex!(
        "README.md",
        r"^### Version {version} — .* \d\d?.., 20\d\d$"
    );
}
