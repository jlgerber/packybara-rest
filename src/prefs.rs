//! This module provides the implementation of the pbgui preferences. pbgui preferences are
//! written in yaml.
pub use preferences::{traits::*, DDContext, DDPathProvider, DDPreferenceFinder, PreferenceName};
use serde::Deserialize;

/* Example document
---
database:
    host:
    user:
    password:
    dbname:
    port:
*/

#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Prod,
    Test,
}

/// Struct which models the pbgui preference. It implements serde::Deserialize so as
/// to be deserializable.
#[derive(Debug, PartialEq, Deserialize)]
pub struct PackybaraRestPrefs {
    pub prod: PackybaraRestDbPrefs,
    pub test: PackybaraRestDbPrefs,
}
use std::fmt;

/// ConnectParams provide connection parameters for the ClientProxy via
/// ClientProxy::new.
#[derive(PartialEq, Eq, Debug)]
pub struct ConnectParams<'a> {
    pub host: &'a str,
    pub user: &'a str,
    pub password: &'a str,
    pub dbname: &'a str,
    pub port: u64,
}

impl<'a> ConnectParams<'a> {
    /// New up a ConnectParams instance.
    ///
    /// # Arguments
    /// * `host` - The name or address of the host
    /// * `user` - The user name
    /// * `password` - The user's password
    /// * `dbname` - The database name
    /// * `port` - The port on which the database is listening
    pub fn new(
        host: &'a str,
        user: &'a str,
        password: &'a str,
        dbname: &'a str,
        port: u64,
    ) -> Self {
        Self {
            host,
            user,
            password,
            dbname,
            port,
        }
    }
}

impl<'a> std::fmt::Display for ConnectParams<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "host={} user={} dbname={} password={} port={}",
            self.host, self.user, self.dbname, self.password, self.port
        )
    }
}
impl<'a> Default for ConnectParams<'a> {
    fn default() -> ConnectParams<'a> {
        ConnectParams::new("127.0.0.1", "postgres", "example", "packrat", 5432)
    }
}


/// Models the database section of the PackybaraRestDbPrefs
#[derive(Debug, PartialEq, Deserialize)]
pub struct PackybaraRestDbPrefs {
    pub host: String,
    pub user: String,
    pub password: String,
    pub dbname: String,
    pub port: u64,
}

impl std::default::Default for PackybaraRestDbPrefs {
    fn default() -> Self {
        let cp = ConnectParams::default();
        Self {
            host: cp.host.to_string(),
            user: cp.user.to_string(),
            password: cp.password.to_string(),
            dbname: cp.dbname.to_string(),
            port: cp.port,
        }
    }
}

impl std::default::Default for PackybaraRestPrefs {
    fn default() -> Self {
        Self {
            prod: PackybaraRestDbPrefs::default(),
            test: PackybaraRestDbPrefs::default()
        }
    }
}
// This gives us two functions -- load and load_file
impl Preference for PackybaraRestPrefs {
    type PreferenceStruct = PackybaraRestPrefs;
    type PreferenceFinder = DDPreferenceFinder<DDPathProvider>;
}

impl PackybaraRestPrefs {
    /// Construct a ConectParams instance from a config. Note that the
    /// lifetime of the ConnectParams is intrinsicly tied to that of
    /// the prefs, as ConnectParams is non-owning.
    pub fn as_connectparams(&self, mode: Mode) -> ConnectParams {
        match mode {
            Mode::Prod => ConnectParams::new(
                            self.prod.host.as_str(),
                            self.prod.user.as_str(),
                            self.prod.password.as_str(),
                            self.prod.dbname.as_str(),
                            self.prod.port,
                        ),
            Mode::Test => ConnectParams::new(
                            self.prod.host.as_str(),
                            self.prod.user.as_str(),
                            self.prod.password.as_str(),
                            self.prod.dbname.as_str(),
                            self.prod.port,
                        )

        }
    }
}
