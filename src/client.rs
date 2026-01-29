use reqwest::{Client, ClientBuilder, RequestBuilder};
use serde::{de::DeserializeOwned, Deserialize};

use crate::{
    component::Component, incident::Incident, metainfo::Metainfo, status::Status, summary::Summary,
};

pub struct StatusClient {
    client: Client,
    base_url: String,
}

impl StatusClient {
    pub fn new(url: impl AsRef<str>) -> crate::Result<Self> {
        let base_url = format!("{}/api/v2", url.as_ref());
        let client = ClientBuilder::new().build()?;

        Ok(Self { base_url, client })
    }

    #[inline(always)]
    fn request(&self, method: reqwest::Method, endpoint: &str) -> RequestBuilder {
        self.client
            .request(method, format!("{}{endpoint}", self.base_url))
    }

    #[inline(always)]
    fn get(&self, endpoint: &str) -> RequestBuilder {
        self.request(reqwest::Method::GET, endpoint)
    }

    #[inline(always)]
    async fn get_json<T: DeserializeOwned>(&self, endpoint: &str) -> crate::Result<T> {
        self.get(endpoint)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
            .map_err(|e| crate::error::Error::Reqwest(e))
    }

    pub async fn get_summary(&self) -> crate::Result<Summary> {
        self.get_json("/summary.json").await
    }

    pub async fn get_status(&self) -> crate::Result<Status> {
        #[derive(Deserialize)]
        struct APIResponse {
            #[allow(unused)]
            page: Metainfo,
            status: Status,
        }

        let response: APIResponse = self.get_json("/status.json").await?;
        Ok(response.status)
    }

    pub async fn get_components(&self) -> crate::Result<Vec<Component>> {
        #[derive(Deserialize)]
        struct APIResponse {
            #[allow(unused)]
            page: Metainfo,
            components: Vec<Component>,
        }

        let response: APIResponse = self.get_json("/components.json").await?;
        Ok(response.components)
    }

    pub async fn get_incidents(&self) -> crate::Result<Vec<Incident>> {
        #[derive(Deserialize)]
        struct APIResponse {
            #[allow(unused)]
            page: Metainfo,
            incidents: Vec<Incident>,
        }

        let response: APIResponse = self.get_json("/incidents.json").await?;
        Ok(response.incidents)
    }

    pub async fn get_incident(&self, id: &str) -> crate::Result<Incident> {
        #[derive(Deserialize)]
        struct APIResponse {
            #[allow(unused)]
            page: Metainfo,
            incident: Incident,
        }

        let response: APIResponse = self.get_json(&format!("/incidents/{id}.json")).await?;
        Ok(response.incident)
    }
}
