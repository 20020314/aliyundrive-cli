#![allow(dead_code)]
use std::fmt::Debug;

pub mod auth;
pub mod gen;
pub mod query;
pub mod suc;

pub const CODE_KEY: &str = "code";
pub const LOGIN_TYPE: &str = "normal";
pub const CK_KEY: &str = "ck";
pub const T_KEY: &str = "t";

pub trait CkForm: Debug {
    fn map_form(&self) -> std::collections::HashMap<String, String>;
}

pub trait AuthorizationToken {
    fn access_token(&self) -> Option<String>;

    fn refresh_token(&self) -> Option<String>;
}

pub trait Ok {
    fn ok(&self) -> bool;
}
