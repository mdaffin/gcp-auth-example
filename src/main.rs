extern crate hyper;
extern crate hyper_rustls;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate yup_oauth2 as oauth2;

use std::env;
use std::path::PathBuf;
use std::fs::File;

#[derive(Serialize, Deserialize, Debug)]
struct ApplicationDefaultCredentials {
    #[serde(rename = "type")] key_type: String,
    client_id: String,
    client_secret: String,
    refresh_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
enum ServiceAccountKey {
    ServiceAccount(oauth2::ServiceAccountKey),
    AuthorizedUser(ApplicationDefaultCredentials),
}

fn get_explicit_environ_credentials() -> Option<(ServiceAccountKey, Option<String>)> {
    use ServiceAccountKey::*;
    if let Ok(google_application_credentials) = env::var("GOOGLE_APPLICATION_CREDENTIALS") {
        if let Ok(account_key) =
            oauth2::service_account_key_from_file(&google_application_credentials.into())
        {
            let project_id = account_key.project_id.clone();
            Some((ServiceAccount(account_key), project_id))
        } else {
            None
        }
    } else {
        None
    }
}

fn get_application_default_credentials_path() -> PathBuf {
    if let Ok(config_dir) = env::var("CLOUDSDK_CONFIG") {
        config_dir.into()
    } else {
        //TODO support windows and remove expect
        env::home_dir()
            .expect("missing home dir")
            .join(".config/gcloud")
    }.join("application_default_credentials.json")
}

fn get_gcloud_sdk_credentials() -> Option<(ServiceAccountKey, Option<String>)> {
    use ServiceAccountKey::*;
    if let Ok(file) = File::open(get_application_default_credentials_path()) {
        // Read the JSON contents of the file as an instance of `User`.
        if let Ok(u) = serde_json::from_reader(file) {
            // TODO load project_id
            Some((AuthorizedUser(u), None))
        } else {
            None
        }
    } else {
        None
    }
}

fn get_gae_credentials() -> Option<(ServiceAccountKey, Option<String>)> {
    // TODO google app engine
    None
}

fn get_gce_credentials() -> Option<(ServiceAccountKey, Option<String>)> {
    // TODO google compute engine
    None
}


fn main() {
    if let Some((credentials, project_id)) = get_explicit_environ_credentials() {
        println!(
            "Project ID: {:?}\nEnviron Credentials: {:#?}",
            project_id,
            credentials
        );
    } else {
        println!("Environ credentials not found");
    }

    if let Some((credentials, project_id)) = get_gcloud_sdk_credentials() {
        println!(
            "Project ID: {:?}\nSDK Credentials: {:#?}",
            project_id,
            credentials
        );
    } else {
        println!("GCloud SDK credentials not found");
    }

    if let Some((credentials, project_id)) = get_gae_credentials() {
        println!(
            "Project ID: {:?}\nGAE Credentials: {:#?}",
            project_id,
            credentials
        );
    } else {
        println!("GCloud GAE credentials not found");
    }

    if let Some((credentials, project_id)) = get_gce_credentials() {
        println!(
            "Project ID: {:?}\nGCE Credentials: {:#?}",
            project_id,
            credentials
        );
    } else {
        println!("GCloud GCE credentials not found");
    }
}
