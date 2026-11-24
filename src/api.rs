use serde::Deserialize;


#[derive(Deserialize)]
pub struct Geko {
        pub bitcoin: Coins,
}

#[derive(Deserialize)]
pub struct Coins {
        pub usd: f64,
        pub eur: f64,
}


pub async fn geko_api() -> Result<Geko, reqwest::Error> {
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=eur,usd";
        let client = reqwest::Client::builder()
            .user_agent("crypto_viewer/0.1")
            .build()?;


    let response = client
        .get(url)
        .send()
        .await?
        .json::<Geko>()
        .await?;
    Ok(response)

}

pub async fn mempool_api() -> Result<String, reqwest::Error> {
        let url = "https://mempool.space/api/blocks/tip/height";
        let client = reqwest::Client::builder()
            .user_agent("crypto_viewer/0.1")
            .build()?;

    let response= client
        .get(url)
        .send()
        .await?
        .text()
        .await?;
    Ok(response)

}