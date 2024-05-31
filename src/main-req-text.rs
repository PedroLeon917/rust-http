use reqwest;

#[tokio::main]
async fn main() {
    let res = reqwest::get("http://localhost:8000/api/name/max").await;
    match res {
        Err(e) => {
            println!("An error happened: {:?}", e);
        },
        Ok(resp) => {
            println!("Good response: {:?}", resp);
            let v : String = resp.text().await.unwrap();
            println!("Response Value {}", v);
        },
    }
}
