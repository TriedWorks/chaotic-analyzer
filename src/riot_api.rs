use super::api_urls;
use super::config;
use super::models::{Summoner, SummonerPromo, SummonerRanked};

use reqwest::Client;

pub async fn summoner_by_name(name: &str, region: &str, client: &Client) -> Summoner {
    let region_link = get_region_link(&region);
    let summoner_url = api_urls::SUMMONER_URL_BY_NAME
        .replace("in1", region_link)
        .replace("in2", name)
        .replace("in3", &config::get_riot_api_key());

    let summoner_data: &str = &client
        .get(&summoner_url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    Summoner::from_json(summoner_data, &region)
}

pub async fn summoner_ranked_by_id(
    summoner_id: &str,
    region: &str,
    client: &Client,
) -> SummonerRanked {
    let region_link = get_region_link(&region);
    let summoner_ranked_url = api_urls::SUMMONER_RANK_URL_BY_SUMMONER_ID
        .replace("in1", region_link)
        .replace("in2", summoner_id)
        .replace("in3", &config::get_riot_api_key());

    let summoner_ranked_data: &str = &client
        .get(&summoner_ranked_url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("{:?}", &summoner_ranked_data);
    SummonerRanked::from_json(summoner_ranked_data)
}

pub async fn get_summoner_id_from_summoner(summoner: &Summoner) -> &str {
    summoner.summoner_id.as_str()
}

#[allow(dead_code)]
async fn get_summoner_id_by_name(name: &str, region: &str, client: &Client) -> String {
    let region_link = get_region_link(&region);
    let summoner_url = api_urls::SUMMONER_URL_BY_NAME
        .replace("in1", region_link)
        .replace("in2", name)
        .replace("in3", &config::get_riot_api_key());

    let summoner_data: &str = &client
        .get(&summoner_url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    Summoner::from_json(summoner_data, &region).summoner_id
}

fn get_region_link(region: &str) -> &str {
    match region {
        "BR" => api_urls::BASE_URL_BR,
        "EUNE" => api_urls::BASE_URL_EUNE,
        "EUW" => api_urls::BASE_URL_EUW,
        "JP" => api_urls::BASE_URL_JP,
        "KR" => api_urls::BASE_URL_KR,
        "LAN" => api_urls::BASE_URL_LAN,
        "LAS" => api_urls::BASE_URL_LAS,
        "NA" => api_urls::BASE_URL_NA,
        "OCE" => api_urls::BASE_URL_OCE,
        "TR" => api_urls::BASE_URL_TR,
        "RU" => api_urls::BASE_URL_RU,
        &_ => api_urls::BASE_URL_EUW,
    }
}
