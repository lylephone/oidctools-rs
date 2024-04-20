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
    let s = oidc(issuer);

    println!("{:?}", s)
}

fn oidc(issuer: &String) -> Result<(), Box<dyn Error>> {
    let provider_metadata =
        CoreProviderMetadata::discover(&IssuerUrl::new(issuer.to_owned())?, http_client)?;
    print!("{:?}", provider_metadata);

    Ok(())
}
