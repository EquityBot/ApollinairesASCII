use std::collections::BTreeSet;

const RULE: &str = "============================================================";

/// `name version` of every locked package that has a source, so the root crate is excluded.
fn locked_packages(lock: &str) -> BTreeSet<String> {
    let mut packages = BTreeSet::new();
    for block in lock.split("[[package]]").skip(1) {
        let field = |key: &str| {
            block.lines().find_map(|line| {
                line.strip_prefix(key)?
                    .strip_prefix(" = \"")?
                    .strip_suffix('"')
            })
        };
        let name = field("name").expect("locked package without a name");
        let version = field("version").expect("locked package without a version");
        if field("source").is_some() {
            packages.insert(format!("{name} {version}"));
        }
    }
    packages
}

/// Headings of package sections (a rule, the heading, then `Licence:`), with any duplicates.
/// Sections for code or fonts copied from outside Cargo give `Source:` under the heading instead.
fn notice_sections(notices: &str) -> (BTreeSet<String>, Vec<String>) {
    let mut sections = BTreeSet::new();
    let mut duplicates = Vec::new();
    let lines: Vec<_> = notices.lines().collect();
    for window in lines.windows(3) {
        if let [RULE, heading, licence] = window
            && licence.starts_with("Licence: ")
            && !sections.insert(heading.to_string())
        {
            duplicates.push(heading.to_string());
        }
    }
    (sections, duplicates)
}

// Kim Wexler would want every licence on file before anything ships.
#[test]
fn notice_bundle_has_one_section_per_locked_package() {
    let root = env!("CARGO_MANIFEST_DIR");
    let lock = std::fs::read_to_string(format!("{root}/Cargo.lock")).unwrap();
    let notices = std::fs::read_to_string(format!("{root}/web/THIRD_PARTY_NOTICES.txt")).unwrap();
    let locked = locked_packages(&lock);
    let (sections, duplicates) = notice_sections(&notices);
    let missing: Vec<_> = locked.difference(&sections).collect();
    let stale: Vec<_> = sections.difference(&locked).collect();
    assert!(
        missing.is_empty() && stale.is_empty() && duplicates.is_empty(),
        "web/THIRD_PARTY_NOTICES.txt is out of date with Cargo.lock; see THIRD_PARTY_NOTICES.md\n\
         missing sections: {missing:?}\nsections for unlocked packages: {stale:?}\n\
         duplicate sections: {duplicates:?}"
    );
}
