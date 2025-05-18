use reqwest::blocking::Client;
use serde::Serialize;
use sha2::{Digest, Sha256};

pub struct KeenClient {
    client: Client,
    ip_addr: String,
}

impl KeenClient {
    pub fn new(ip_addr: String) -> Result<Self, reqwest::Error> {
        let client = Client::builder()
            .cookie_store(true) // Ensure cookies are enabled
            .user_agent("curl") // Match Python's UA
            .build()?;

        Ok(KeenClient { client, ip_addr })
    }

    pub fn get(&self, endpoint: &str) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let url = format!("http://{}/{}", self.ip_addr, endpoint);
        println!("GET {}", url);
        self.client.get(&url).send()
    }

    pub fn post<T: Serialize>(
        &self,
        endpoint: &str,
        data: &T,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let url = format!("http://{}/{}", self.ip_addr, endpoint);
        println!("POST {}", url);
        let body = serde_json::to_string(data).unwrap();
        println!("Request body: {}", body);
        self.client.post(&url).json(data).send()
    }

    pub fn auth(&self, login: &str, password: &str) -> Result<bool, reqwest::Error> {
        let response = self.get("auth")?;
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

            #[derive(Serialize)]
            struct AuthPayload<'a> {
                login: &'a str,
                password: &'a str,
            }

            let auth_data = AuthPayload {
                login,
                password: &sha_hex,
            };

            let post_response = self.post("auth", &auth_data)?;
            println!("Auth POST response status: {}", post_response.status());
            // save cookie
            Ok(post_response.status() == 200)
        } else {
            Ok(response.status() == 200)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings;
    use anyhow::ensure;
    use anyhow::Error;

    #[test]
    fn test_auth() -> Result<(), Error> {
        let client = KeenClient::new(settings::ROUTER_IP.to_string());
        let login = settings::ROUTER_USER;
        let passw = settings::ROUTER_PASS;

        let result = client?.auth(login, &passw).unwrap();
        ensure!(result == true);
        Ok(())
    }

    #[test]
    fn test_get() -> Result<(), Error> {
        let client = KeenClient::new(settings::ROUTER_IP.to_string())?;
        let login = settings::ROUTER_USER;
        let passw = settings::ROUTER_PASS;

        let result = client.auth(login, &passw)?;
        ensure!(result == true);
        let response = client.get("rci/show/interface/WifiMaster0")?;
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
        let passw = settings::ROUTER_PASS;

        let result = client.auth(login, &passw)?;
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
        )?;
        let s = response.status();
        println!("Response status: {}", s);
        println!("Response text: {}", response.text()?);
        ensure!(s == 200);
        Ok(())
    }
}
