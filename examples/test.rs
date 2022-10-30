use dirs::cache_dir;

fn main() {
    let dir = cache_dir().unwrap();
    println!("{}", dir.to_str().unwrap())
}
