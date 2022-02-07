use error_chain::error_chain;
use std::io::Read;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    /*
    let mut result = reqwest::blocking::get("http://httpbin.org/get")?;
    let mut body = String::new();
    result.read_to_string(&mut body)?;

    println!("Status: {}", result.status());
    println!("Headers:\n{:#?}", result.headers());
    println!("body = {:?}", body);
    */

    // we have to use the ClientBuilder in order to get a Client that accepts self-signed/invalid certs
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    // this is where we make our actual API request
    let response = client
        .get("https://192.168.50.12/rest/asset")
        .header("X-ApiKey", "accessKey=1234;secretKey=1234")
        .header("accept", "*/*")
        .header("content-type", "application/json")
        .send() // send the request
        .await
        .unwrap()
        .text() // get the response
        .await
        .unwrap();

    // this prints out the raw response
    println!("{:}", response);

    // let's deserialize the response
    let v: serde_json::Value = serde_json::from_str(&response).unwrap();
    //println!("{:#?}", v); // pretty-print the whole JSON structure
    // pretty-print a specific part of the JSON response
    println!("The first asset is {:#?}", v["response"]["usable"][0]);

    Ok(())
}
