use crate::{author::entities::Authors, errors::MitCommitMessageLintsError};
use std::convert::TryFrom;

impl TryFrom<&str> for Authors {
    type Error = MitCommitMessageLintsError;

    fn try_from(yaml: &str) -> Result<Self, Self::Error> {
        serde_yaml::from_str(yaml)
            .map_err(MitCommitMessageLintsError::from)
            .map(Authors::new)
    }
}

impl TryFrom<Authors> for String {
    type Error = MitCommitMessageLintsError;

    fn try_from(value: Authors) -> Result<Self, Self::Error> {
        serde_yaml::to_string(&value.authors).map_err(MitCommitMessageLintsError::from)
    }
}

#[cfg(test)]
mod tests_able_to_load_config_from_yaml {
    use std::collections::BTreeMap;

    use pretty_assertions::assert_eq;

    use crate::author::entities::{Author, Authors};
    use indoc::indoc;
    use std::convert::TryFrom;

    #[test]
    fn must_be_valid_yaml() {
        let actual = Authors::try_from("Hello I am invalid yaml : : :");
        assert_eq!(true, actual.is_err())
    }

    #[test]
    fn it_can_parse_a_standard_yaml_file() {
        let actual = Authors::try_from(indoc!(
            "
            ---
            bt:
                name: Billie Thompson
                email: billie@example.com
            "
        ));

        let mut input: BTreeMap<String, Author> = BTreeMap::new();
        input.insert(
            "bt".into(),
            Author::new("Billie Thompson", "billie@example.com", None),
        );
        let expected = Ok(Authors::new(input));

        assert_eq!(expected, actual);
    }

    #[test]
    fn yaml_files_can_contain_signing_keys() {
        let actual = Authors::try_from(indoc!(
            "
            ---
            bt:
                name: Billie Thompson
                email: billie@example.com
                signingkey: 0A46826A
            "
        ));

        let mut expected_authors: BTreeMap<String, Author> = BTreeMap::new();
        expected_authors.insert(
            "bt".into(),
            Author::new("Billie Thompson", "billie@example.com", Some("0A46826A")),
        );
        let expected = Ok(Authors::new(expected_authors));

        assert_eq!(expected, actual);
    }
}

#[cfg(test)]
mod tests_able_to_convert_authors_to_yaml {
    use std::collections::BTreeMap;

    use pretty_assertions::assert_eq;

    use crate::author::entities::{Author, Authors};
    use indoc::indoc;
    use std::convert::TryInto;

    #[test]
    fn it_converts_to_standard_yaml() {
        let mut map: BTreeMap<String, Author> = BTreeMap::new();
        map.insert(
            "bt".into(),
            Author::new("Billie Thompson", "billie@example.com", None),
        );
        let actual: String = Authors::new(map).try_into().unwrap();
        let expected = indoc!(
            "
            ---
            bt:
              name: Billie Thompson
              email: billie@example.com"
        )
        .to_string();

        assert_eq!(expected, actual);
    }

    #[test]
    fn it_includes_the_signing_key_if_set() {
        let mut map: BTreeMap<String, Author> = BTreeMap::new();
        map.insert(
            "bt".into(),
            Author::new("Billie Thompson", "billie@example.com", Some("0A46826A")),
        );
        let actual: String = Authors::new(map).try_into().unwrap();
        let expected = indoc!(
            "
            ---
            bt:
              name: Billie Thompson
              email: billie@example.com
              signingkey: 0A46826A"
        )
        .to_string();

        assert_eq!(expected, actual);
    }
}