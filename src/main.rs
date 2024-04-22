use std::error::Error;

use config::Config;
use std::collections::HashMap;

use openidconnect::reqwest::http_client;
use openidconnect::{core::CoreProviderMetadata, IssuerUrl};

fn main() {
    let settings = Config::builder()
        // Add in `./Settings.toml`
        .add_source(config::File::with_name("settings"))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    let config = settings
        .try_deserialize::<HashMap<String, String>>()
        .unwrap();
    // Print out our settings (as a HashMap)
    println!("{:?}", config);

    let issuer = config.get("issuer").unwrap();
    println!("Issuer is {}", issuer);
    let s = get_metadata(issuer);

    println!("{:?}", s.unwrap())
}

fn get_metadata(url: &String) -> Result<CoreProviderMetadata, Box<dyn Error>> {

    let issuer_url = IssuerUrl::new(url.to_owned())?;
    // let issuer_url = match IssuerUrl::new(url.to_owned()) {
    //     Ok(value) => value,
    //     Err(e) => return e,
    // };
    //Err(e)=> return Err(e),

    let provider_metadata = CoreProviderMetadata::discover(&issuer_url, http_client)?;
    // let provider_metadata = match CoreProviderMetadata::discover(&issuer_url, http_client) {
    //     Ok(value) => value,
    //     Err(e) => return e,
    // };
    Ok(provider_metadata)
}
