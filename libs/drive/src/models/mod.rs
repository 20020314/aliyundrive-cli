pub mod gen;
pub mod query;
pub mod suc;

pub trait Ok {
    fn ok(&self) -> bool;
}
