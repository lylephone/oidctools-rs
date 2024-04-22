use std::error::Error;

use config::Config;
use std::collections::HashMap;

use openidconnect::core::{
    CoreAuthenticationFlow, CoreClient, CoreProviderMetadata, CoreResponseType, CoreUserInfoClaims,
};
use openidconnect::reqwest::http_client;
use openidconnect::{
    AccessTokenHash, AuthenticationFlow, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    IssuerUrl, Nonce, PkceCodeChallenge, RedirectUrl, Scope,
};

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
    let metadata = get_metadata(issuer).unwrap();

    // println!("{:?}", s.unwrap());

    let client = get_client(
        metadata,
        config.get("client_id").unwrap(),
        config.get("client_secret").unwrap(),
        config.get("redirect_url").unwrap()
    )
    .unwrap();

    println!("{:?}", client);
    // Generate a PKCE challenge.
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the full authorization URL.
    let (auth_url, csrf_token, nonce) = client
        .authorize_url(
            CoreAuthenticationFlow::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        )
        // Set the desired scopes.
        .add_scope(Scope::new("*".to_string()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_challenge)
        .url();
    println!("auth url:{}",auth_url);
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

fn get_client(
    metadata: CoreProviderMetadata,
    client_id: &String,
    client_secret: &String,
    redirect_url: &String
) -> Result<CoreClient, Box<dyn Error>> {
    let client = CoreClient::from_provider_metadata(
        metadata,
        ClientId::new(client_id.to_owned()),
        Some(ClientSecret::new(client_secret.to_owned())),
    )
    // Set the URL the user will be redirected to after the authorization process.
    .set_redirect_uri(RedirectUrl::new(redirect_url.to_owned())?);
    Ok(client)
}
