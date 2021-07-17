use std::io::Error;

use buildae::load_patterns;

#[test]
fn test_load_patterns() -> Result<(), Error> {
    let (include, exclude) = load_patterns("project", "tests/test.json");
    assert!(include.contains(&String::from("foo/*")));
    assert!(include.contains(&String::from("*.py")));
    assert!(exclude.contains(&String::from("foo/tests/*")));
    assert!(exclude.contains(&String::from("*.md")));
    assert!(exclude.contains(&String::from("*.rst")));
    Ok(())
}
