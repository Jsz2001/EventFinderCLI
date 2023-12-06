//! # Web Request Module
//!
//! This module provides functionality for making web requests to fetch HTML content.
//! It utilizes the `reqwest` crate for making HTTP requests and is designed to perform
//! synchronous (blocking) web requests to retrieve data from specified URLs.

use reqwest;

/// Fetches HTML content from a specified URL using a synchronous (blocking) HTTP GET request.
///
/// This function is designed to retrieve the raw HTML content of a web page for further processing
/// or parsing. It uses the `reqwest` crate's blocking client to perform the HTTP request.
///
/// # Arguments
///
/// * `url` - A string slice representing the URL from which to fetch the HTML content.
///
/// # Returns
///
/// A `Result` containing the HTML content as a `String` if successful, or a `reqwest::Error` if the request fails.
pub fn fetch_url(url: &str) -> Result<String, reqwest::Error> {
    // Make a blocking GET request to the URL
    let response = reqwest::blocking::get(url)?;

    // Extract the text (HTML) from the response
    let body = response.text()?;

    // Return the HTML content
    Ok(body)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, server_url};

    #[test]
    fn test_fetch_url() {
        let _m = mock("GET", "/test")
            .with_status(200)
            .with_body("mocked response")
            .create();

        let url = &format!("{}/test", server_url());
        let response = fetch_url(url).unwrap();

        assert_eq!(response, "mocked response");
    }
}

