pub mod login;
pub mod models;

pub type Result<T> = anyhow::Result<T>;

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {}
}
