use reqwest::blocking::Client;
use serde::Serialize;
use sha2::{Digest, Sha256};

pub struct KeenClient {
    ip_addr: String,
}

#[derive(Serialize)]
struct AuthPayload {
    login: Box<str>,
    password: Box<str>,
}

impl KeenClient {
    pub fn new(ip_addr: String) -> Result<Self, reqwest::Error> {
        Ok(KeenClient { ip_addr })
    }

    pub fn get(
        &self,
        endpoint: &str,
        client: &Client,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let url = format!("http://{}/{}", self.ip_addr, endpoint);
        println!("GET {}", url);
        client.get(&url).send()
    }

    pub fn post<T: Serialize>(
        &self,
        endpoint: &str,
        data: &T,
        client: &Client,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let url = format!("http://{}/{}", self.ip_addr, endpoint);
        println!("POST {}", url);
        let body = serde_json::to_string(data).unwrap();
        println!("Request body: {}", body);
        client.post(&url).json(data).send()
    }

    pub fn auth(
        &self,
        login: &str,
        password: &str,
        client: &Client,
    ) -> Result<bool, reqwest::Error> {
        let response = self.get("auth", client)?;
        println!("Initial auth response status: {}", response.status());

        if response.status() == 401 {
            let headers = response.headers();
            println!("Response headers: {:?}", headers);

            let realm = headers
                .get("X-NDM-Realm")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("");
            let challenge = headers
                .get("X-NDM-Challenge")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("");

            println!("Realm: '{}', Challenge: '{}'", realm, challenge);

            // Compute MD5 hash
            let md5_input = format!("{}:{}:{}", login, realm, password);
            let md5_digest = md5::compute(&md5_input);
            let md5_hex = format!("{:x}", md5_digest);
            println!("MD5 hash: {}", md5_hex);

            // Compute SHA256 hash
            let mut sha = Sha256::new();
            sha.update(format!("{}{}", challenge, md5_hex));
            let sha_hex = format!("{:x}", sha.finalize());
            println!("SHA256 hash: {}", sha_hex);

            let auth_data = AuthPayload {
                login: login.into(),
                password: sha_hex.into(),
            };

            let post_response = self.post("auth", &auth_data, client)?;
            println!("Auth POST response status: {}", post_response.status());
            Ok(post_response.status() == 200)
        } else {
            Ok(response.status() == 200)
        }
    }
}

pub fn get_interface(client: &KeenClient, rest_client: &reqwest::blocking::Client) -> String {
    match client.get("rci/show/interface/PPPoE0", rest_client) {
        Ok(res) => res
            .text()
            .unwrap_or_else(|_| "Failed to get response text".to_string()),
        Err(e) => e.to_string(),
    }
}

pub fn reroll_interface(
    client: &KeenClient,
    rest_client: &reqwest::blocking::Client,
    dry_run: bool,
) -> String {
    let req = match dry_run {
        true => &serde_json::json!([
            {
                "parse": "interface PPPoE0 up"
            },
        ]),
        false => &serde_json::json!([
            {
                "parse": "interface PPPoE0 down"
            },
            {
                "parse": "interface PPPoE0 up"
            },
        ]),
    };
    match client.post("rci/", req, rest_client) {
        Ok(res) => res
            .text()
            .unwrap_or_else(|_| "Failed to get response text".to_string()),
        Err(e) => e.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings;
    use anyhow::ensure;
    use anyhow::Error;
    use std::env;

    fn get_rest_client() -> Client {
        Client::builder()
            .cookie_store(true)
            .user_agent("curl")
            .build()
            .expect("Failed to create HTTP client for Router")
    }

    fn get_router_pass() -> String {
        env::var("ROUTER_PASS").expect("ROUTER_PASS environment variable not set")
    }

    #[test]
    fn test_auth() -> Result<(), Error> {
        let client = KeenClient::new(settings::ROUTER_IP.to_string());
        let login = settings::ROUTER_USER;
        let passw = get_router_pass();

        let result = client?.auth(login, &passw, &get_rest_client())?;
        ensure!(result == true);
        Ok(())
    }

    #[test]
    fn test_get() -> Result<(), Error> {
        let client = KeenClient::new(settings::ROUTER_IP.to_string())?;
        let login = settings::ROUTER_USER;
        let passw = get_router_pass();
        let rest_client = &get_rest_client();
        let result = client.auth(login, &passw, rest_client)?;
        ensure!(result == true);
        let response = client.get("rci/show/interface/WifiMaster0", rest_client)?;
        let s = response.status();
        println!("Response status: {}", s);
        println!("Response text: {}", response.text()?);
        ensure!(s == 200);
        Ok(())
    }

    #[test]
    fn test_post() -> Result<(), Error> {
        let client = KeenClient::new(settings::ROUTER_IP.to_string())?;
        let login = settings::ROUTER_USER;
        let passw = get_router_pass();
        let rest_client = &get_rest_client();
        let result = client.auth(login, &passw, rest_client)?;
        ensure!(result == true);
        let response = client.post(
            "rci/",
            &serde_json::json!([
            // {
            //     "parse": "interface PPPoE0 down"
            // },
                {
                    "parse": "interface PPPoE0 up"
                }
            ]),
            rest_client,
        )?;
        let s = response.status();
        println!("Response status: {}", s);
        println!("Response text: {}", response.text()?);
        ensure!(s == 200);
        Ok(())
    }
}
