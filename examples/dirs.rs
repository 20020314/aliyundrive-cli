fn main() {
    let dir = dirs::cache_dir()?;
    println!("{:?}", dir);
    let buf = dir.join("aliyundrive-cli");
}
