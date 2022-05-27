pub mod models;
pub mod scan;

pub type Result<T> = anyhow::Result<T>;

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {}
}
