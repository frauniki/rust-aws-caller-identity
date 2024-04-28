use aws_config;
use aws_sdk_sts as sts;

#[tokio::main]
async fn main() -> Result<(), sts::Error> {
    let config = aws_config::load_from_env().await;
    let client = sts::Client::new(&config);

    let resp = client.get_caller_identity().send().await.unwrap();
    println!("Caller identity arn: {}", resp.arn.unwrap());

    Ok(())
}
