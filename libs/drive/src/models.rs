use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize)]
struct Content {
  data: Data,
  status: i64,
  success: bool,
}

#[derive(Serialize, Deserialize)]
struct Data {
  t: i64,
  code_content: String,
  ck: String,
  result_code: i64,
}

#[derive(Serialize, Deserialize)]
struct RootInterface {
  content: Content,
  has_error: bool,
}