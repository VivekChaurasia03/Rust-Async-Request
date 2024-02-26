use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    /* The .await keyword is used to wait for the asynchronous operation to complete.
    The result is assigned to the res variable, and the ? is used to propagate any errors that may occur. */
    let res = reqwest::get("http://httpbin.org/get").await?;

    /* It's often beneficial to fetch metadata like status and headers as soon as possible,
    even before the entire response body has been received. This allows you to start processing
    certain information without waiting for the complete response. */
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await?;
    println!("Body:\n{}", body);

    Ok(())
}
