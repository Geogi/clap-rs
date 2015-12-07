extern crate clap;

use clap::{App, Arg, ArgGroup, ClapErrorType};

#[test]
fn required_group_missing_arg() {
    let result = App::new("group")
        .args_from_usage("-f, --flag 'some flag'
                          -c, --color 'some other flag'")
        .arg_group(ArgGroup::with_name("req")
            .add_all(&["flag", "color"])
            .required(true))
        .get_matches_from_safe(vec![""]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.error_type, ClapErrorType::MissingRequiredArgument);
}

#[test]
fn group_single_value() {
    let m = App::new("group")
        .args_from_usage("-f, --flag 'some flag'
                          -c, --color [color] 'some option'")
        .arg_group(ArgGroup::with_name("grp")
            .add_all(&["flag", "color"]))
        .get_matches_from(vec!["", "-c", "blue"]);
    assert!(m.is_present("grp"));
    assert_eq!(m.value_of("grp").unwrap(), "blue");
}

#[test]
fn group_single_flag() {
    let m = App::new("group")
        .args_from_usage("-f, --flag 'some flag'
                          -c, --color [color] 'some option'")
        .arg_group(ArgGroup::with_name("grp")
            .add_all(&["flag", "color"]))
        .get_matches_from(vec!["", "-f"]);
    assert!(m.is_present("grp"));
    assert!(m.value_of("grp").is_none());
}

#[test]
fn group_empty() {
    let m = App::new("group")
        .args_from_usage("-f, --flag 'some flag'
                          -c, --color [color] 'some option'")
        .arg_group(ArgGroup::with_name("grp")
            .add_all(&["flag", "color"]))
        .get_matches_from(vec![""]);
    assert!(!m.is_present("grp"));
    assert!(m.value_of("grp").is_none());
}

#[test]
fn group_reqired_flags_empty() {
    let result = App::new("group")
        .args_from_usage("-f, --flag 'some flag'
                          -c, --color 'some option'")
        .arg_group(ArgGroup::with_name("grp")
            .required(true)
            .add_all(&["flag", "color"]))
        .get_matches_from_safe(vec![""]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.error_type, ClapErrorType::MissingRequiredArgument);
}

#[test]
fn group_multi_value_single_arg() {
    let m = App::new("group")
        .args_from_usage("-f, --flag 'some flag'
                          -c, --color [color]... 'some option'")
        .arg_group(ArgGroup::with_name("grp")
            .add_all(&["flag", "color"]))
        .get_matches_from(vec!["", "-c", "blue", "red", "green"]);
    assert!(m.is_present("grp"));
    assert_eq!(m.values_of("grp").unwrap(), &["blue", "red", "green"]);
}

#[test]
#[should_panic]
fn empty_group() {
    let _ = App::new("empty_group")
        .arg(Arg::from_usage("-f, --flag 'some flag'"))
        .arg_group(ArgGroup::with_name("vers")
            .required(true))
        .get_matches();
}

#[test]
#[should_panic]
fn empty_group_2() {
    let _ = App::new("empty_group")
        .arg(Arg::from_usage("-f, --flag 'some flag'"))
        .arg_group(ArgGroup::with_name("vers")
            .required(true)
            .add_all(&["ver", "major"]))
        .get_matches();
}

#[test]
#[should_panic]
fn errous_group() {
    let _ = App::new("errous_group")
        .arg(Arg::from_usage("-f, --flag 'some flag'"))
        .arg_group(ArgGroup::with_name("vers")
            .add("vers")
            .required(true))
        .get_matches();
}