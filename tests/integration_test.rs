use std::io::Error;

use buildae::load_patterns;

#[test]
fn test_load_patterns() -> Result<(), Error> {
    let (include, exclude) = load_patterns("project", "tests/data/test.json");
    assert!(include.contains(&String::from("foo/*")));
    assert!(include.contains(&String::from("*.py")));
    assert!(exclude.contains(&String::from("foo/tests/*")));
    assert!(exclude.contains(&String::from("*.md")));
    assert!(exclude.contains(&String::from("*.rst")));
    Ok(())
}

#[test]
fn test_load_patterns_with_no_pattern() -> Result<(), Error> {
    let (include, exclude) = load_patterns("project", "tests/data/no_pattern.json");
    assert!(include.contains(&String::from("*")));
    assert!(exclude.contains(&String::from("*")));
    Ok(())
}
