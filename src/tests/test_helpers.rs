use std::env;

pub fn fixture_path(fixture: &str) -> String {
    format!(
        "{}/src/tests/fixtures/{}",
        env::var("CARGO_MANIFEST_DIR").unwrap(),
        &fixture
    )
}
