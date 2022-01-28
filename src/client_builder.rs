use anyhow::Result;
use num_cpus;
use reqwest;
use reqwest::Client;

/// A `ClientBuilder` can be used to create a `Client` with custom configuration.
#[derive(Clone, Debug)]
pub struct ClientBuilder {
    pub reqwest_client: Option<reqwest::Client>,
    max_requests_per_second: usize,
    max_threads_cpu: usize,
}

impl Default for ClientBuilder {
    /// Creates a `ClientBuilder` with the following default settings:
    /// * `max_requests_per_second` = `10`
    /// * `max_threads_cpu` = number of logical cores on your machine
    /// * `max_threads_io` = `100`
    /// * `reqwest_client` = default `reqwest::Client` plus `gzip` set to `false` and `timeout` set to `None`
    fn default() -> ClientBuilder {
        ClientBuilder {
            max_requests_per_second: 10,
            max_threads_cpu: num_cpus::get(),
            reqwest_client: None,
        }
    }
}

impl ClientBuilder {
    /// Set the maximum number of requests per second.
    pub fn set_max_requests_per_second(mut self, max_requests_per_second: usize) -> ClientBuilder {
        self.max_requests_per_second = max_requests_per_second;
        self
    }

    /// Set the maximum number of cpu threads (those used for PDF conversion).
    pub fn set_max_threads_cpu(mut self, max_threads_cpu: usize) -> ClientBuilder {
        self.max_threads_cpu = max_threads_cpu;
        self
    }

    /// Provide your own customized `reqwest::Client`.
    pub fn set_reqwest_client(mut self, reqwest_client: reqwest::Client) -> ClientBuilder {
        self.reqwest_client = Some(reqwest_client);
        self
    }

    /// Returns a `Client` that uses this `ClientBuilder` configuration.
    pub fn build(self) -> Result<Client> {
        let reqwest_client = match self.reqwest_client {
            Some(reqwest_client) => reqwest_client,
            None => reqwest::ClientBuilder::new().build()?,
        };
        Ok(reqwest_client.clone())
    }
}
