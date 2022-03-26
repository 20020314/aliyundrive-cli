use anyhow::{Error, Result};

fn main() {
    let re = get_cluster_info();
    match re {
        Ok(something) => println!("{:#?}", something),
        Err(e) => println!("{:#?}", e),
    }
}

fn get_cluster_info() -> Result<String, Error> {
    let config = std::fs::read_to_string("cluster.json")?;
    let map = serde_json::from_str(&config)?;
    Ok(map)
}
