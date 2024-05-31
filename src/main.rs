use serde_derive::{Serialize, Deserialize};
use reqwest::Client;

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: i32,
}

#[tokio::main]
async fn main() {
    let p = Person{
        name: "Max".to_string(),
        age: 54,
    };
    let client = Client::new();
    let res = client.post("http://localhost:8000/api/person")
        .json(&p)
        .send().await;
    match res {
        Err(e) => {
            println!("An error happened: {:?}", e);
        },
        Ok(resp) => {
            println!("Good response: {:?}", resp);
            let q : Person = resp.json().await.unwrap();
            println!("{:?}", q);
        },
    }
}
