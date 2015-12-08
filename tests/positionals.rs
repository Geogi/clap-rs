extern crate clap;

use clap::{App, Arg, ClapErrorType};

#[test]
fn positional() {
    let m = App::new("positional")
        .args(vec![
            Arg::from_usage("-f, --flag 'some flag'"),
            Arg::with_name("positional")
                .index(1)
            ])
        .get_matches_from(vec!["", "-f", "test"]);
    assert!(m.is_present("positional"));
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("positional").unwrap(), "test");

    let m = App::new("positional")
        .args(vec![
            Arg::from_usage("-f, --flag 'some flag'"),
            Arg::with_name("positional")
                .index(1)
            ])
        .get_matches_from(vec!["", "test", "--flag"]);
    assert!(m.is_present("positional"));
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("positional").unwrap(), "test");
}

#[test]
fn positional_multiple() {
    let m = App::new("positional_multiple")
        .args(vec![
            Arg::from_usage("-f, --flag 'some flag'"),
            Arg::with_name("positional")
                .index(1)
                .multiple(true)
            ])
        .get_matches_from(vec!["", "-f", "test1", "test2", "test3"]);
    assert!(m.is_present("positional"));
    assert!(m.is_present("flag"));
    assert_eq!(m.values_of("positional").unwrap(), vec!["test1", "test2", "test3"]);

    let m = App::new("positional_multiple")
        .args(vec![
            Arg::from_usage("-f, --flag 'some flag'"),
            Arg::with_name("positional")
                .index(1)
                .multiple(true)
            ])
        .get_matches_from(vec!["", "test1", "test2", "test3", "--flag"]);
    assert!(m.is_present("positional"));
    assert!(m.is_present("flag"));
    assert_eq!(m.values_of("positional").unwrap(), vec!["test1", "test2", "test3"]);
}

#[test]
fn positional_multiple_2() {
    let result = App::new("positional_multiple")
        .args(vec![
            Arg::from_usage("-f, --flag 'some flag'"),
            Arg::with_name("positional")
                .index(1)
            ])
        .get_matches_from_safe(vec!["", "-f", "test1", "test2", "test3"]);
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.error_type, ClapErrorType::UnexpectedArgument);
}

#[test]
fn positional_possible_values() {
    let m = App::new("positional_possible_values")
        .args(vec![
            Arg::from_usage("-f, --flag 'some flag'"),
            Arg::with_name("positional")
                .index(1)
                .possible_value("test123")
            ])
        .get_matches_from(vec!["", "-f", "test123"]);
    assert!(m.is_present("positional"));
    assert!(m.is_present("flag"));
    assert_eq!(m.values_of("positional").unwrap(), vec!["test123"]);
}

#[test]
fn create_positional() {
    let _ = App::new("test")
                .arg(Arg::with_name("test")
                            .index(1)
                            .help("testing testing"))
                .get_matches();
}

#[test]
fn create_positional_usage() {
    let a = Arg::from_usage("[pos] 'some help info'");
    assert_eq!(a.name, "pos");
    assert_eq!(a.help.unwrap(), "some help info");
    assert!(!a.multiple);
    assert!(!a.required);
    assert!(a.val_names.is_none());
    assert!(a.num_vals.is_none());

    let b = Arg::from_usage("<pos> 'some help info'");
    assert_eq!(b.name, "pos");
    assert_eq!(b.help.unwrap(), "some help info");
    assert!(!b.multiple);
    assert!(b.required);
    assert!(b.val_names.is_none());
    assert!(b.num_vals.is_none());

    let c = Arg::from_usage("[pos]... 'some help info'");
    assert_eq!(c.name, "pos");
    assert_eq!(c.help.unwrap(), "some help info");
    assert!(c.multiple);
    assert!(!c.required);
    assert!(c.val_names.is_none());
    assert!(c.num_vals.is_none());

    let d = Arg::from_usage("<pos>... 'some help info'");
    assert_eq!(d.name, "pos");
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(d.multiple);
    assert!(d.required);
    assert!(d.val_names.is_none());
    assert!(d.num_vals.is_none());

    let b = Arg::from_usage("<pos>");
    assert_eq!(b.name, "pos");
    assert!(!b.multiple);
    assert!(b.required);
    assert!(b.val_names.is_none());
    assert!(b.num_vals.is_none());

    let c = Arg::from_usage("[pos]...");
    assert_eq!(c.name, "pos");
    assert!(c.multiple);
    assert!(!c.required);
    assert!(c.val_names.is_none());
    assert!(c.num_vals.is_none());
}