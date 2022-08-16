use anyhow::Context;
use drive::conf::AppConf;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let result = list_all("root").await?;
    // println!("{:#?}", result);
    // Ok(())
    let credentials = AppConf::read().await?;
    println!("{:?}", credentials);
    let x = credentials.read_refresh_token();
    println!("{}", x.unwrap_or_default());
    Ok(())
}
//
// pub async fn list_all(parent_file_id: &str) -> anyhow::Result<Vec<drive::model::CloudFile>> {
//     let mut files = Vec::new();
//     let mut marker = None;
//     loop {
//         let res = list(parent_file_id, marker.as_deref()).await?;
//         files.extend(res.items.into_iter().map(|f| f.into()));
//         if res.next_marker.is_empty() {
//             break;
//         }
//         marker = Some(res.next_marker);
//     }
//     Ok(files)
// }
//
// pub async fn list(
//     parent_file_id: &str,
//     marker: Option<&str>,
// ) -> anyhow::Result<drive::model::ListFileResponse> {
//     let drive_id = "78405871";
//     let req = drive::model::ListFileRequest {
//         drive_id,
//         parent_file_id,
//         limit: 200,
//         all: false,
//         image_thumbnail_process: "image/resize,w_400/format,jpeg",
//         image_url_process: "image/resize,w_1920/format,jpeg",
//         video_thumbnail_process: "video/snapshot,t_0,f_jpg,ar_auto,w_300",
//         fields: "*",
//         order_by: "updated_at",
//         order_direction: "DESC",
//         marker,
//     };
//     request(
//         String::from("https://api.aliyundrive.com/v2/file/list"),
//         &req,
//     )
//     .await
//     .and_then(|res| res.context("expect response"))
// }
//
// async fn request<T, U>(url: String, req: &T) -> anyhow::Result<Option<U>>
// where
//     T: Serialize + ?Sized,
//     U: DeserializeOwned,
// {
//     let client = reqwest::Client::new();
//     let url = reqwest::Url::parse(&url)?;
//     let res = client
//         .post(url.clone())
//         .bearer_auth("eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiI4NDNkZjM1MzU3OWQ0NjcwYWI4MTg0NjAwNWZlZDA3MiIsImN1c3RvbUpzb24iOiJ7XCJjbGllbnRJZFwiOlwiMjVkelgzdmJZcWt0Vnh5WFwiLFwiZG9tYWluSWRcIjpcImJqMjlcIixcInNjb3BlXCI6W1wiRFJJVkUuQUxMXCIsXCJTSEFSRS5BTExcIixcIkZJTEUuQUxMXCIsXCJVU0VSLkFMTFwiLFwiVklFVy5BTExcIixcIlNUT1JBR0UuQUxMXCIsXCJTVE9SQUdFRklMRS5MSVNUXCIsXCJCQVRDSFwiLFwiT0FVVEguQUxMXCIsXCJJTUFHRS5BTExcIixcIklOVklURS5BTExcIixcIkFDQ09VTlQuQUxMXCIsXCJTWU5DTUFQUElORy5MSVNUXCJdLFwicm9sZVwiOlwidXNlclwiLFwicmVmXCI6XCJodHRwczovL3d3dy5hbGl5dW5kcml2ZS5jb20vXCIsXCJkZXZpY2VfaWRcIjpcIjRhODFmODlhMzI5NzQzNjFhMWJmOWQwNDlmNmNiOTMyXCJ9IiwiZXhwIjoxNjU1ODMxODA3LCJpYXQiOjE2NTU4MjQ1NDd9.ZGBVQxLEkmkatbJ6gaW1_ss6zNHdKwLyEnD3BzvrblmS0TihJQV_A2jnZoT5aU30AmExl5sCaclQ5Hw0wyC7F6AnnEksgTaf5o-4UypNUeXMOoXClMY-3rmEp65NihPrMlptQ_7jEYSfR6ajio-jFAv88qvmUepWVZfbRIJLxY8")
//         .json(&req)
//         .send()
//         .await?
//         .error_for_status();
//     match res {
//         Ok(res) => {
//             if res.status() == StatusCode::NO_CONTENT {
//                 return Ok(None);
//             }
//             let res = res.json::<U>().await?;
//             Ok(Some(res))
//         }
//         Err(err) => {
//             match err.status() {
//                 Some(
//                     status_code
//                     @
//                     // 4xx
//                     (StatusCode::UNAUTHORIZED
//                     | StatusCode::REQUEST_TIMEOUT
//                     | StatusCode::TOO_MANY_REQUESTS
//                     // 5xx
//                     | StatusCode::INTERNAL_SERVER_ERROR
//                     | StatusCode::BAD_GATEWAY
//                     | StatusCode::SERVICE_UNAVAILABLE
//                     | StatusCode::GATEWAY_TIMEOUT),
//                 ) => {
//                     // if status_code == StatusCode::UNAUTHORIZED {
//                     //     // refresh token and retry
//                     //     let token_res = do_refresh_token_with_retry(None).await?;
//                     //     access_token = token_res.access_token;
//                     // } else {
//                     //     // wait for a while and retry
//                     //     std::time::sleep(std::time::Duration::from_secs(1)).await;
//                     // }
//                     let res = client
//                         .post(url)
//                         .bearer_auth("eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiI4NDNkZjM1MzU3OWQ0NjcwYWI4MTg0NjAwNWZlZDA3MiIsImN1c3RvbUpzb24iOiJ7XCJjbGllbnRJZFwiOlwiMjVkelgzdmJZcWt0Vnh5WFwiLFwiZG9tYWluSWRcIjpcImJqMjlcIixcInNjb3BlXCI6W1wiRFJJVkUuQUxMXCIsXCJTSEFSRS5BTExcIixcIkZJTEUuQUxMXCIsXCJVU0VSLkFMTFwiLFwiVklFVy5BTExcIixcIlNUT1JBR0UuQUxMXCIsXCJTVE9SQUdFRklMRS5MSVNUXCIsXCJCQVRDSFwiLFwiT0FVVEguQUxMXCIsXCJJTUFHRS5BTExcIixcIklOVklURS5BTExcIixcIkFDQ09VTlQuQUxMXCIsXCJTWU5DTUFQUElORy5MSVNUXCJdLFwicm9sZVwiOlwidXNlclwiLFwicmVmXCI6XCJodHRwczovL3d3dy5hbGl5dW5kcml2ZS5jb20vXCIsXCJkZXZpY2VfaWRcIjpcIjRhODFmODlhMzI5NzQzNjFhMWJmOWQwNDlmNmNiOTMyXCJ9IiwiZXhwIjoxNjU1ODMxODA3LCJpYXQiOjE2NTU4MjQ1NDd9.ZGBVQxLEkmkatbJ6gaW1_ss6zNHdKwLyEnD3BzvrblmS0TihJQV_A2jnZoT5aU30AmExl5sCaclQ5Hw0wyC7F6AnnEksgTaf5o-4UypNUeXMOoXClMY-3rmEp65NihPrMlptQ_7jEYSfR6ajio-jFAv88qvmUepWVZfbRIJLxY8")
//                         .json(&req)
//                         .send()
//                         .await?
//                         .error_for_status()?;
//                     if res.status() == StatusCode::NO_CONTENT {
//                         return Ok(None);
//                     }
//                     let res = res.json::<U>().await?;
//                     Ok(Some(res))
//                 }
//                 _ => Err(err.into()),
//             }
//         }
//     }
// }

// async fn do_refresh_token(
//     refresh_token: &str,
//     client_type: ClientType,
// ) -> anyhow::Result<RefreshTokenResponse> {
//     let mut data = HashMap::new();
//     data.insert("refresh_token", refresh_token);
//     data.insert("grant_type", "refresh_token");
//     // if let Some(app_id) = self.config.app_id.as_ref() {
//     //     data.insert("app_id", app_id);
//     // }
//     // let refresh_token_url = if self.config.app_id.is_none() {
//     //     client_type.refresh_token_url()
//     // } else {
//     //     &self.config.refresh_token_url
//     // };
//     let res = reqwest::Client::new()
//         .post("https://websv.aliyundrive.com/token/refresh")
//         .json(&data)
//         .send()
//         .await?;
//     match res.error_for_status_ref() {
//         Ok(_) => {
//             let res = res.json::<RefreshTokenResponse>().await?;
//             Ok(res)
//         }
//         Err(err) => {
//             let msg = res.text().await?;
//             let context = format!("{}: {}", err, msg);
//             Err(err).context(context)
//         }
//     }
// }
