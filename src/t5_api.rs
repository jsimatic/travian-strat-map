use reqwest::blocking::Client;

pub fn get_raw_map_data(priv_key: &str) -> reqwest::Result<String> {
    let params = [("action", "getMapData"), ("privateApiKey", priv_key)];
    let client = Client::new();
    let res = client
        .get("https://com3.kingdoms.com/api/external.php")
        .query(&params)
        .send()?
        .text()?;
    Ok(res)
}
