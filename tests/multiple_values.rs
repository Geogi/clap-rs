extern crate clap;

use clap::{App, Arg, ClapErrorType};

#[test]
fn multiple_values_of_option_long() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("option")
            .long("option")
            .help("multiple options")
            .takes_value(true)
            .multiple(true))
        .get_matches_from_safe(vec![
            "",
            "--option", "val1",
            "--option", "val2",
            "--option", "val3",
        ]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 3);
    assert_eq!(m.values_of("option"), Some(vec!["val1", "val2", "val3"]));
}

#[test]
fn multiple_values_of_option_short() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("option")
            .short("o")
            .help("multiple options")
            .takes_value(true)
            .multiple(true))
        .get_matches_from_safe(vec![
            "",
            "-o", "val1",
            "-o", "val2",
            "-o", "val3",
        ]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 3);
    assert_eq!(m.values_of("option"), Some(vec!["val1", "val2", "val3"]));
}

#[test]
fn multiple_values_of_option_mixed() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("option")
            .long("option")
            .short("o")
            .help("multiple options")
            .takes_value(true)
            .multiple(true))
        .get_matches_from_safe(vec![
            "",
            "-o", "val1",
            "--option", "val2",
            "--option", "val3",
            "-o", "val4",
        ]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 4);
    assert_eq!(m.values_of("option"), Some(vec!["val1", "val2", "val3", "val4"]));
}

#[test]
fn multiple_values_of_option_exact_exact() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("option")
            .short("o")
            .help("multiple options")
            .takes_value(true)
            .multiple(true)
            .number_of_values(3))
        .get_matches_from_safe(vec![
            "",
            "-o", "val1",
            "-o", "val2",
            "-o", "val3",
        ]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 3);
    assert_eq!(m.values_of("option"), Some(vec!["val1", "val2", "val3"]));
}

#[test]
fn multiple_values_of_option_exact_less() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("option")
            .short("o")
            .help("multiple options")
            .takes_value(true)
            .multiple(true)
            .number_of_values(3))
        .get_matches_from_safe(vec![
            "",
            "-o", "val1",
            "-o", "val2",
        ]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().error_type, ClapErrorType::WrongNumValues);
}

#[test]
fn multiple_values_of_option_exact_more() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("option")
            .short("o")
            .help("multiple options")
            .takes_value(true)
            .multiple(true)
            .number_of_values(3))
        .get_matches_from_safe(vec![
            "",
            "-o", "val1",
            "-o", "val2",
            "-o", "val3",
            "-o", "val4",
        ]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().error_type, ClapErrorType::WrongNumValues);
}

#[test]
fn multiple_values_of_option_min_exact() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("option")
            .short("o")
            .help("multiple options")
            .takes_value(true)
            .min_values(3))
        .get_matches_from_safe(vec![
            "",
            "-o", "val1",
            "-o", "val2",
            "-o", "val3",
        ]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 3);
    assert_eq!(m.values_of("option"), Some(vec!["val1", "val2", "val3"]));
}

#[test]
fn multiple_values_of_option_min_less() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("option")
            .short("o")
            .help("multiple options")
            .takes_value(true)
            .min_values(3))
        .get_matches_from_safe(vec![
            "",
            "-o", "val1",
            "-o", "val2",
        ]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().error_type, ClapErrorType::TooFewValues);
}

#[test]
fn multiple_values_of_option_min_more() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("option")
            .short("o")
            .help("multiple options")
            .takes_value(true)
            .min_values(3))
        .get_matches_from_safe(vec![
            "",
            "-o", "val1",
            "-o", "val2",
            "-o", "val3",
            "-o", "val4",
        ]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 4);
    assert_eq!(m.values_of("option"), Some(vec!["val1", "val2", "val3", "val4"]));
}

#[test]
fn multiple_values_of_option_max_exact() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("option")
            .short("o")
            .help("multiple options")
            .takes_value(true)
            .max_values(3))
        .get_matches_from_safe(vec![
            "",
            "-o", "val1",
            "-o", "val2",
            "-o", "val3",
        ]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 3);
    assert_eq!(m.values_of("option"), Some(vec!["val1", "val2", "val3"]));
}

#[test]
fn multiple_values_of_option_max_less() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("option")
            .short("o")
            .help("multiple options")
            .takes_value(true)
            .max_values(3))
        .get_matches_from_safe(vec![
            "",
            "-o", "val1",
            "-o", "val2",
        ]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 2);
    assert_eq!(m.values_of("option"), Some(vec!["val1", "val2"]));
}

#[test]
fn multiple_values_of_option_max_more() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("option")
            .short("o")
            .help("multiple options")
            .takes_value(true)
            .max_values(3))
        .get_matches_from_safe(vec![
            "",
            "-o", "val1",
            "-o", "val2",
            "-o", "val3",
            "-o", "val4",
        ]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().error_type, ClapErrorType::TooManyValues);
}

#[test]
fn multiple_values_of_positional() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("pos")
            .help("multiple positionals")
            .index(1)
            .multiple(true))
        .get_matches_from_safe(vec!["", "val1", "val2", "val3"]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("pos"));
    assert_eq!(m.occurrences_of("pos"), 3);
    assert_eq!(m.values_of("pos"), Some(vec!["val1", "val2", "val3"]));
}

#[test]
fn multiple_values_of_positional_exact_exact() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("pos")
            .help("multiple positionals")
            .index(1)
            .multiple(true)
            .number_of_values(3))
        .get_matches_from_safe(vec!["", "val1", "val2", "val3"]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("pos"));
    assert_eq!(m.occurrences_of("pos"), 3);
    assert_eq!(m.values_of("pos"), Some(vec!["val1", "val2", "val3"]));
}

#[test]
fn multiple_values_of_positional_exact_less() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("pos")
            .help("multiple positionals")
            .index(1)
            .multiple(true)
            .number_of_values(3))
        .get_matches_from_safe(vec!["", "val1", "val2"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().error_type, ClapErrorType::WrongNumValues);
}

#[test]
fn multiple_values_of_positional_exact_more() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("pos")
            .help("multiple positionals")
            .index(1)
            .multiple(true)
            .number_of_values(3))
        .get_matches_from_safe(vec!["", "val1", "val2", "val3", "val4"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().error_type, ClapErrorType::WrongNumValues);
}

#[test]
fn multiple_values_of_positional_min_exact() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("pos")
            .help("multiple positionals")
            .index(1)
            .min_values(3))
        .get_matches_from_safe(vec!["", "val1", "val2", "val3"]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("pos"));
    assert_eq!(m.occurrences_of("pos"), 3);
    assert_eq!(m.values_of("pos"), Some(vec!["val1", "val2", "val3"]));
}

#[test]
fn multiple_values_of_positional_min_less() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("pos")
            .help("multiple positionals")
            .index(1)
            .min_values(3))
        .get_matches_from_safe(vec!["", "val1", "val2"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().error_type, ClapErrorType::TooFewValues);
}

#[test]
fn multiple_values_of_positional_min_more() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("pos")
            .help("multiple positionals")
            .index(1)
            .min_values(3))
        .get_matches_from_safe(vec!["", "val1", "val2", "val3", "val4"]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("pos"));
    assert_eq!(m.occurrences_of("pos"), 4);
    assert_eq!(m.values_of("pos"), Some(vec!["val1", "val2", "val3", "val4"]));
}

#[test]
fn multiple_values_of_positional_max_exact() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("pos")
            .help("multiple positionals")
            .index(1)
            .max_values(3))
        .get_matches_from_safe(vec!["", "val1", "val2", "val3"]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("pos"));
    assert_eq!(m.occurrences_of("pos"), 3);
    assert_eq!(m.values_of("pos"), Some(vec!["val1", "val2", "val3"]));
}

#[test]
fn multiple_values_of_positional_max_less() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("pos")
            .help("multiple positionals")
            .index(1)
            .max_values(3))
        .get_matches_from_safe(vec!["", "val1", "val2"]);

    assert!(m.is_ok());
    let m = m.unwrap();

    assert!(m.is_present("pos"));
    assert_eq!(m.occurrences_of("pos"), 2);
    assert_eq!(m.values_of("pos"), Some(vec!["val1", "val2"]));
}

#[test]
fn multiple_values_of_positional_max_more() {
    let m = App::new("multiple_values")
        .arg(Arg::with_name("pos")
            .help("multiple positionals")
            .index(1)
            .max_values(3))
        .get_matches_from_safe(vec!["", "val1", "val2", "val3", "val4"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().error_type, ClapErrorType::TooManyValues);
}
