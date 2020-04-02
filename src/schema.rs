table! {
    summoners (puuid) {
        puuid -> Text,
        account_id -> Nullable<Text>,
        profile_icon_id -> Nullable<Int4>,
        revision_date -> Nullable<Int8>,
        name -> Nullable<Text>,
        summoner_id -> Nullable<Text>,
        summoner_level -> Nullable<Int8>,
        region -> Nullable<Varchar>,
    }
}
