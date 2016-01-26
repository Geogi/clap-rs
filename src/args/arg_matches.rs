use std::ffi::{OsString, OsStr};
use std::collections::HashMap;
use std::iter::Map;
use std::slice;
use std::borrow::Cow;

use vec_map;

use args::SubCommand;
use args::MatchedArg;
use utf8::INVALID_UTF8;

/// Used to get information about the arguments that where supplied to the program at runtime by
/// the user. To get a new instance of this struct you use `.get_matches()` of the `App` struct.
///
///
/// # Examples
///
/// ```no_run
/// # use clap::{App, Arg};
/// let matches = App::new("MyApp")
/// // adding of arguments and configuration goes here...
/// #                    .arg(Arg::with_name("config")
/// #                               .long("config")
/// #                               .required(true)
/// #                               .takes_value(true))
/// #                    .arg(Arg::with_name("debug")
/// #                                   .short("d")
/// #                                   .multiple(true))
///                     .get_matches();
/// // if you had an argument named "output" that takes a value
/// if let Some(o) = matches.value_of("output") {
///     println!("Value for output: {}", o);
/// }
///
/// // If you have a required argument you can call .unwrap() because the program will exit long
/// // before this point if the user didn't specify it at runtime.
/// println!("Config file: {}", matches.value_of("config").unwrap());
///
/// // You can check the presence of an argument
/// if matches.is_present("debug") {
///     // Another way to check if an argument was present, or if it occurred multiple times is to
///     // use occurrences_of() which returns 0 if an argument isn't found at runtime, or the
///     // number of times that it occurred, if it was. To allow an argument to appear more than
///     // once, you must use the .multiple(true) method, otherwise it will only return 1 or 0.
///     if matches.occurrences_of("debug") > 2 {
///         println!("Debug mode is REALLY on");
///     } else {
///         println!("Debug mode kind of on");
///     }
/// }
///
/// // You can get the sub-matches of a particular subcommand (in this case "test")
/// // If "test" had it's own "-l" flag you could check for it's presence accordingly
/// if let Some(ref matches) = matches.subcommand_matches("test") {
///     if matches.is_present("list") {
///         println!("Printing testing lists...");
///     } else {
///         println!("Not printing testing lists...");
///     }
/// }
#[derive(Debug, Clone)]
pub struct ArgMatches<'a> {
    #[doc(hidden)]
    pub args: HashMap<&'a str, MatchedArg>,
    #[doc(hidden)]
    pub subcommand: Option<Box<SubCommand<'a>>>,
    #[doc(hidden)]
    pub usage: Option<String>,
}

impl<'a> Default for ArgMatches<'a> {
    fn default() -> Self {
        ArgMatches {
            args: HashMap::new(),
            subcommand: None,
            usage: None,
        }
    }
}

impl<'a> ArgMatches<'a> {
    /// Creates a new instance of `ArgMatches`. This ins't called directly, but
    /// through the `.get_matches()` method of `App`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let matches = App::new("myprog").get_matches();
    /// ```
    #[doc(hidden)]
    pub fn new() -> Self { ArgMatches { ..Default::default() } }

    /// Gets the value of a specific option or positional argument (i.e. an argument that takes
    /// an additional value at runtime). If the option wasn't present at runtime
    /// it returns `None`.
    ///
    /// *NOTE:* If getting a value for an option or positional argument that allows multiples,
    /// prefer `values_of()` as `value_of()` will only return the _*first*_ value.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// let m = App::new("myapp")
    ///     .arg(Arg::with_name("output")
    ///         .takes_value(true))
    ///     .get_matches_from(vec!["myapp", "something"]);
    ///
    /// assert_eq!(m.value_of("output"), Some("something"));
    /// ```
    pub fn value_of<S: AsRef<str>>(&self, name: S) -> Option<&str> {
        if let Some(ref arg) = self.args.get(name.as_ref()) {
            if let Some(v) = arg.vals.values().nth(0) {
                return Some(v.to_str().expect(INVALID_UTF8));
            }
        }
        None
    }

    /// Gets the lossy value of a specific argument If the option wasn't present at runtime
    /// it returns `None`. A lossy value is one which contains invalid UTF-8 code points, those
    /// invalid points will be replaced with `\u{FFFD}`
    ///
    /// *NOTE:* If getting a value for an option or positional argument that allows multiples,
    /// prefer `lossy_values_of()` as `lossy_value_of()` will only return the _*first*_ value.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use clap::{App, Arg};
    /// use std::ffi::OsString;
    /// use std::os::unix::ffi::OsStrExt;
    ///
    /// let m = App::new("utf8")
    ///     .arg(Arg::from_usage("<arg> 'some arg'"))
    ///     .get_matches_from(vec![OsString::from("myprog"),
    ///                             // "Hi {0xe9}!"
    ///                             OsString::from_vec(vec![b'H', b'i', b' ', 0xe9, b'!'])]);
    /// assert_eq!(&*m.lossy_value_of("arg").unwrap(), "Hi \u{FFFD}!");
    /// ```
    pub fn lossy_value_of<S: AsRef<str>>(&'a self, name: S) -> Option<Cow<'a, str>> {
        if let Some(arg) = self.args.get(name.as_ref()) {
            if let Some(v) = arg.vals.values().nth(0) {
                return Some(v.to_string_lossy());
            }
        }
        None
    }

    /// Gets the OS version of a string value of a specific argument If the option wasn't present at
    /// runtime it returns `None`. An OS value on Unix-like systems is any series of bytes, regardless
    /// of whether or not they contain valid UTF-8 code points. Since `String`s in Rust are
    /// garunteed to be valid UTF-8, a valid filename as an argument value on Linux (for example) may
    /// contain invalid UTF-8 code points. This would cause a `panic!` or only the abiltiy to get a
    /// lossy version of the file name (i.e. one where the invalid code points were replaced with
    /// `\u{FFFD}`). This method returns an `OsString` which allows one to represent those strings
    /// which rightfully contain invalid UTF-8.
    ///
    /// *NOTE:* If getting a value for an option or positional argument that allows multiples,
    /// prefer `os_values_of()` as `os_value_of()` will only return the _*first*_ value.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use clap::{App, Arg};
    /// use std::ffi::OsString;
    /// use std::os::unix::ffi::OsStrExt;
    ///
    /// let m = App::new("utf8")
    ///     .arg(Arg::from_usage("<arg> 'some arg'"))
    ///     .get_matches_from(vec![OsString::from("myprog"),
    ///                             // "Hi {0xe9}!"
    ///                             OsString::from_vec(vec![b'H', b'i', b' ', 0xe9, b'!'])]);
    /// assert_eq!(&*m.os_value_of("arg").unwrap().as_bytes(), &[b'H', b'i', b' ', 0xe9, b'!']);
    /// ```
    pub fn os_value_of<S: AsRef<str>>(&self, name: S) -> Option<&OsStr> {
        self.args.get(name.as_ref()).map_or(None, |arg| arg.vals.values().nth(0).map(|v| v.as_os_str()))
    }

    /// Gets the values of a specific option or positional argument in a vector (i.e. an argument
    /// that takes multiple values at runtime). If the option wasn't present at runtime it
    /// returns `None`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myapp")
    /// #     .arg(Arg::with_name("output").takes_value(true)).get_matches();
    /// // If the program had option "-c" that took a value and was run
    /// // via "myapp -o some -o other -o file"
    /// // values_of() would return a [&str; 3] ("some", "other", "file")
    /// if let Some(os) = matches.values_of("output") {
    ///        for o in os {
    ///            println!("A value for output: {}", o);
    ///        }
    /// }
    /// ```
    pub fn values_of<S: AsRef<str>>(&'a self, name: S) -> Option<Values<'a>> {
        if let Some(ref arg) = self.args.get(name.as_ref()) {
            fn to_str_slice(o: &OsString) -> &str { o.to_str().expect(INVALID_UTF8) }
            let to_str_slice: fn(&OsString) -> &str = to_str_slice; // coerce to fn pointer
            return Some(Values { iter: arg.vals.values().map(to_str_slice) });
        }
        None
    }

    /// Gets the lossy values of a specific argument If the option wasn't present at runtime
    /// it returns `None`. A lossy value is one which contains invalid UTF-8 code points, those
    /// invalid points will be replaced with `\u{FFFD}`
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use clap::{App, Arg};
    /// use std::ffi::OsString;
    /// use std::os::unix::ffi::OsStrExt;
    ///
    /// let m = App::new("utf8")
    ///     .arg(Arg::from_usage("<arg> 'some arg'"))
    ///     .get_matches_from(vec![OsString::from("myprog"),
    ///                             // "Hi {0xe9}!"
    ///                             OsString::from_vec(vec![b'H', b'i', b' ', 0xe9, b'!'])]);
    /// let itr = m.lossy_values_of("arg").unwrap();
    /// assert_eq!(&*itr.next().unwrap(), "Hi");
    /// assert_eq!(&*itr.next().unwrap(), "\u{FFFD}!");
    /// assert_eq!(itr.next(), None);
    /// ```
    pub fn lossy_values_of<S: AsRef<str>>(&'a self, name: S) -> Option<Vec<String>> {
        if let Some(ref arg) = self.args.get(name.as_ref()) {
            return Some(arg.vals.values()
                           .map(|v| v.to_string_lossy().into_owned())
                           .collect());
        }
        None
    }

    /// Gets the OS version of a string value of a specific argument If the option wasn't present
    /// at runtime it returns `None`. An OS value on Unix-like systems is any series of bytes,
    /// regardless of whether or not they contain valid UTF-8 code points. Since `String`s in Rust
    /// are garunteed to be valid UTF-8, a valid filename as an argument value on Linux (for
    /// example) may contain invalid UTF-8 code points. This would cause a `panic!` or only the
    /// abiltiy to get a lossy version of the file name (i.e. one where the invalid code points
    /// were replaced with `\u{FFFD}`). This method returns an `OsString` which allows one to
    /// represent those strings which rightfully contain invalid UTF-8.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use clap::{App, Arg};
    /// use std::ffi::OsString;
    /// use std::os::unix::ffi::OsStrExt;
    ///
    /// let m = App::new("utf8")
    ///     .arg(Arg::from_usage("<arg> 'some arg'"))
    ///     .get_matches_from(vec![OsString::from("myprog"),
    ///                                 // "Hi"
    ///                                 OsString::from_vec(vec![b'H', b'i']),
    ///                                 // "{0xe9}!"
    ///                                 OsString::from_vec(vec![0xe9, b'!'])]);
    ///
    /// let itr = m.os_values_of("arg").unwrap();
    /// assert_eq!(itr.next(), Some(&*OsString::from("Hi")));
    /// assert_eq!(itr.next(), Some(&*OsString::from_vec(vec![0xe9, b'!'])));
    /// assert_eq!(itr.next(), None);
    /// ```
    pub fn os_values_of<S: AsRef<str>>(&'a self, name: S) -> Option<OsValues<'a>> {
        fn to_str_slice(o: &OsString) -> &OsStr { &*o }
        let to_str_slice: fn(&'a OsString) -> &'a OsStr = to_str_slice; // coerce to fn pointer
        if let Some(ref arg) = self.args.get(name.as_ref()) {
            return Some(OsValues { iter: arg.vals.values().map(to_str_slice) });
        }
        None
    }

    /// Returns if an argument was present at runtime.
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myapp")
    /// #     .arg(Arg::with_name("output").takes_value(true)).get_matches();
    /// if matches.is_present("output") {
    ///        println!("The output argument was used!");
    /// }
    /// ```
    pub fn is_present<S: AsRef<str>>(&self, name: S) -> bool {
        if let Some(ref sc) = self.subcommand {
            if sc.name == name.as_ref() {
                return true;
            }
        }
        self.args.contains_key(name.as_ref())
    }

    /// Returns the number of occurrences of an option, flag, or positional argument at runtime.
    /// If an argument isn't present it will return `0`. Can be used on arguments which *don't*
    /// allow multiple occurrences, but will obviously only return `0` or `1`.
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg};
    /// # let matches = App::new("myapp")
    /// #     .arg(Arg::with_name("output").takes_value(true)).get_matches();
    /// if matches.occurrences_of("debug") > 1 {
    ///     println!("Debug mode is REALLY on");
    /// } else {
    ///     println!("Debug mode kind of on");
    /// }
    /// ```
    pub fn occurrences_of<S: AsRef<str>>(&self, name: S) -> u8 {
        self.args.get(name.as_ref()).map_or(0, |a| a.occurs)
    }

    /// Returns the `ArgMatches` for a particular subcommand or None if the subcommand wasn't
    /// present at runtime.
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand};
    /// # let app_matches = App::new("myapp")
    /// #     .subcommand(SubCommand::with_name("test")).get_matches();
    /// if let Some(matches) = app_matches.subcommand_matches("test") {
    ///     // Use matches as normal
    /// }
    /// ```
    pub fn subcommand_matches<S: AsRef<str>>(&self, name: S) -> Option<&ArgMatches<'a>> {
        self.subcommand.as_ref().map(|s| if s.name == name.as_ref() { Some(&s.matches) } else { None } ).unwrap()
    }

    /// Returns the name of the subcommand used of the parent `App`, or `None` if one wasn't found
    ///
    /// *NOTE*: Only a single subcommand may be present per `App` at runtime, does *NOT* check for
    /// the name of sub-subcommand's names
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand};
    /// # let app_matches = App::new("myapp")
    /// #     .subcommand(SubCommand::with_name("test")).get_matches();
    /// match app_matches.subcommand_name() {
    ///     Some("test")   => {}, // test was used
    ///     Some("config") => {}, // config was used
    ///     _              => {}, // Either no subcommand or one not tested for...
    /// }
    /// ```
    pub fn subcommand_name(&self) -> Option<&str> {
        self.subcommand.as_ref().map(|sc| &sc.name[..])
    }

    /// Returns the name and `ArgMatches` of the subcommand used at runtime or ("", None) if one
    /// wasn't found.
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand};
    /// # let app_matches = App::new("myapp")
    /// #     .subcommand(SubCommand::with_name("test")).get_matches();
    /// match app_matches.subcommand() {
    ///     ("test", Some(matches))   => {}, // test was used
    ///     ("config", Some(matches)) => {}, // config was used
    ///     _                         => {}, // Either no subcommand or one not tested for...
    /// }
    /// ```
    pub fn subcommand(&self) -> (&str, Option<&ArgMatches<'a>>) {
        self.subcommand.as_ref().map_or(("",None), |sc| (&sc.name[..], Some(&sc.matches)))
    }

    /// Returns a string slice of the usage statement for the `App` (or `SubCommand`)
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use clap::{App, Arg, SubCommand};
    /// # let app_matches = App::new("myapp")
    /// #     .subcommand(SubCommand::with_name("test")).get_matches();
    /// println!("{}",app_matches.usage());
    /// ```
    pub fn usage(&self) -> &str {
        self.usage.as_ref().map_or("", |u| &u[..])
    }
}


// The following were taken and adapated from vec_map source
// repo: https://github.com/contain-rs/vec-map
// commit: be5e1fa3c26e351761b33010ddbdaf5f05dbcc33
// license: MIT - Copyright (c) 2015 The Rust Project Developers

#[derive(Clone)]
pub struct Values<'a> {
    iter: Map<vec_map::Values<'a, OsString>, fn(&'a OsString) -> &'a str>
}

impl<'a> Iterator for Values<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> { self.iter.next() }
    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}

impl<'a> DoubleEndedIterator for Values<'a> {
    fn next_back(&mut self) -> Option<&'a str> { self.iter.next_back() }
}

/// An iterator over the key-value pairs of a map.
#[derive(Clone)]
pub struct Iter<'a, V:'a> {
    front: usize,
    back: usize,
    iter: slice::Iter<'a, Option<V>>
}

impl<'a, V> Iterator for Iter<'a, V> {
    type Item = &'a V;

    #[inline]
    fn next(&mut self) -> Option<&'a V> {
        while self.front < self.back {
            if let Some(elem) = self.iter.next() {
                if let Some(x) = elem.as_ref() {
                    self.front += 1;
                    return Some(x);
                }
            }
            self.front += 1;
        }
        None
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.back - self.front))
    }
}

impl<'a, V> DoubleEndedIterator for Iter<'a, V> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a V> {
        while self.front < self.back {
            if let Some(elem) = self.iter.next_back() {
                if let Some(x) = elem.as_ref() {
                    self.back -= 1;
                    return Some(x);
                }
            }
            self.back -= 1;
        }
        None
    }
}

#[derive(Clone)]
pub struct OsValues<'a> {
    iter: Map<vec_map::Values<'a, OsString>, fn(&'a OsString) -> &'a OsStr>
}

impl<'a> Iterator for OsValues<'a> {
    type Item = &'a OsStr;

    fn next(&mut self) -> Option<&'a OsStr> { self.iter.next() }
    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}

impl<'a> DoubleEndedIterator for OsValues<'a> {
    fn next_back(&mut self) -> Option<&'a OsStr> { self.iter.next_back() }
}
