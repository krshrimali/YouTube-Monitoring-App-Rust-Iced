extern crate google_youtube3 as youtube3;

use youtube3::hyper::client::HttpConnector;
use youtube3::hyper_rustls::HttpsConnector;
use youtube3::Error;
use youtube3::{hyper, hyper_rustls, oauth2, YouTube};

async fn authenticate() -> YouTube<HttpsConnector<HttpConnector>> {
    let google_credentials = std::env::var("GOOGLE_APPLICATION_CREDENTIALS")
        .expect("GOOGLE_APPLICATION_CREDENTIALS env variable not found...");
    let secret = yup_oauth2::read_application_secret(google_credentials)
        .await
        .expect("Couldn't read the credentials file. Is it valid?");

    let auth = oauth2::InstalledFlowAuthenticator::builder(
        secret,
        oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    )
    .build()
    .await
    .unwrap();
    YouTube::new(
        hyper::Client::builder().build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_or_http()
                .enable_http1()
                .enable_http2()
                .build(),
        ),
        auth,
    )
}

async fn get_results(hub: YouTube<HttpsConnector<HttpConnector>>) {
    let result = hub
        .channels()
        .list(&vec![
            "contentDetails".into(),
            "statistics".into(),
            "id".into(),
        ])
        .add_id("UCArZ7MT8VZjBlTZ4__Z05Ig")
        .doit()
        .await;
    println!("Result: {:?}", result);
    match result {
        Err(e) => match e {
            Error::HttpError(_)
            | Error::Io(_)
            | Error::Cancelled
            | Error::UploadSizeLimitExceeded(_, _)
            | Error::Failure(_)
            | Error::BadRequest(_)
            | Error::MissingAPIKey
            | Error::MissingToken(_)
            | Error::FieldClash(_)
            | Error::JsonDecodeError(_, _) => println!("Error string: {:?}", e),
        },
        Ok(res) => println!("Result: {:?}", res.1),
    };
}

#[tokio::main]
async fn main() -> google_youtube3::Result<()> {
    let hub = authenticate().await;
    get_results(hub).await;
    Ok(())
}
