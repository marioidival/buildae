use std::io::Error;

use buildae::{has_diff, load_patterns};

#[test]
fn test_load_patterns() -> Result<(), Error> {
    let (include, exclude) = load_patterns("tests/data/test.json");
    assert!(include.contains(&String::from("foo/*")));
    assert!(include.contains(&String::from("*.py")));
    assert!(exclude.contains(&String::from("foo/tests/*")));
    assert!(exclude.contains(&String::from("*.md")));
    assert!(exclude.contains(&String::from("*.rst")));
    Ok(())
}

#[test]
fn test_load_patterns_with_no_pattern() -> Result<(), Error> {
    let (include, exclude) = load_patterns("tests/data/no_pattern.json");
    assert!(include.contains(&String::from("*")));
    assert!(exclude.contains(&String::from("*")));
    Ok(())
}

#[test]
fn test_has_diff() -> Result<(), Error> {
    let include = vec![String::from("foo/*")];
    let exclude = vec![String::from("*.md"), String::from("tests/*")];

    assert!(has_diff(vec!["foo/main.rs"], &include, &exclude));
    assert!(has_diff(vec!["foo/tests/test_main.rs"], &include, &exclude));
    assert!(!has_diff(vec!["foo/README.md"], &include, &exclude));
    assert!(!has_diff(vec!["tests/test_lib.rs"], &include, &exclude));
    Ok(())
}
