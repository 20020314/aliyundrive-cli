use conf::rw::RW;
use conf::{Config, Context};

fn main() {
    let read_config = Context::read().unwrap();
    println!("{:?}", read_config);

    let p1 = conf::AuthorizationToken::new(String::from("a1"), String::from("a2"));
    let p2 = conf::AuthorizationToken::new(String::from("a3"), String::from("a4"));
    let config = Config::new(Some(p1), Some(p2));
    crate::Context::write(config).unwrap();
    let read_config = Context::read().unwrap();
    println!("{:?}", read_config);

    let t1 = Context::read_token(false).unwrap();
    println!("{:?}", t1);
    let t2 = Context::read_token(true).unwrap();
    println!("{:?}", t2);

    let p3 = conf::AuthorizationToken::new(String::from("a5"), String::from("a6"));
    let p4 = conf::AuthorizationToken::new(String::from("a7"), String::from("a8"));
    Context::write_token(false, p3).unwrap();
    Context::write_token(true, p4).unwrap();

    let read_config = crate::Context::read().unwrap();
    println!("{:?}", read_config);
}
