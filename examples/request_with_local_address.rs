use rquest::{redirect::Policy, Impersonate};
use std::net::IpAddr;

#[tokio::main]
async fn main() -> Result<(), rquest::Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    // Build a client to impersonate Safari18
    let client = rquest::Client::builder()
        .impersonate(Impersonate::Safari18)
        .build()?;

    let resp = client
        .get("http://www.baidu.com")
        .redirect(Policy::default())
        .local_address(IpAddr::from([192, 168, 1, 226]))
        .send()
        .await?;

    println!("{}", resp.text().await?);

    Ok(())
}
