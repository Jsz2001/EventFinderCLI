//! # HTML Parser Module
//!
//! This module provides functionality for parsing HTML content to extract event data.
//! It supports parsing both standard HTML structures and JSON data embedded within `<script>` tags.
//! The module defines structures for site-specific configuration (`SiteConfig`) and event data (`Event`),
//! and includes functions for parsing and URL resolution.

use scraper::{Html, Selector};
use serde_json::Value;
use url::Url;

/// Site-specific configuration for HTML parsing.
#[derive(Debug, Clone, PartialEq)]
pub struct SiteConfig {
    pub event_selector: String,
    pub name_selector: String,
    pub start_date_selector: String,
    pub end_date_selector: String,
    pub location_selector: String,
    pub url: String,
}

/// Parses HTML content to extract event data based on the provided site configuration.
///
/// # Arguments
///
/// * `html` - A string slice that holds the HTML content to be parsed.
/// * `config` - Site configuration specifying CSS selectors for different event components.
/// * `base_url` - The base URL of the site for resolving relative URLs.
///
/// # Returns
///
/// A vector of `Event` instances extracted from the HTML content.
pub fn parse_html(html: &str, config: &SiteConfig, base_url: &str) -> Vec<Event> {
    let mut events = Vec::new();

    // Parse the HTML document
    let document = Html::parse_document(html);

    // Create a Selector for the HTML elements that contain the event data
    // (Adjust the selector based on the actual HTML structure)
    if &config.event_selector == "script[type='application/ld+json']" {
        if let Some(script_content) = document.select(&Selector::parse(&config.event_selector).unwrap()).next() {
            let json_content = script_content.inner_html();
            // Inside the JSON parsing logic
            if let Ok(json) = serde_json::from_str::<Value>(&json_content) {
                // Assuming the JSON structure is an array of events or a single event object
                let events_iter: Box<dyn Iterator<Item = &Value>> = match json.as_array() {
                    Some(array) => Box::new(array.iter()), // Array of objects, create an iterator over the array
                    None => Box::new(std::iter::once(&json)), // Single object, create an iterator with one element
                };
            
                for event_json in events_iter {
                    let name = event_json["name"].as_str().unwrap_or_default().to_string();
                    let start_date = event_json["startDate"].as_str().unwrap_or_default().to_string();
                    let end_date = event_json["endDate"].as_str().unwrap_or_default().to_string();
                    let location = event_json["location"]["name"].as_str().unwrap_or_default().to_string();
                    let url = event_json["url"].as_str().unwrap_or_default().to_string();
            
                    // Create an Event object and add it to the events vector
                    let event = Event {
                        name,
                        start_date,
                        end_date,
                        location,
                        url,
                    };
                    events.push(event);
                }
            }

        }
    } else {
        let event_selector = Selector::parse(&config.event_selector).unwrap();
        // Iterate over each event element
        for event_element in document.select(&event_selector) {
            // Extract event details like name, date, location, etc.
            // (Adjust the selectors and extraction logic based on the actual HTML structure)
            let name = event_element.select(&Selector::parse(&config.name_selector).unwrap())
                                    .next()
                                    .map(|e| e.inner_html())
                                    .unwrap_or_default();
            let start_date = event_element.select(&Selector::parse(&config.start_date_selector).unwrap())
                                    .next()
                                    .map(|e| e.inner_html())
                                    .unwrap_or_default();
            let end_date = event_element.select(&Selector::parse(&config.end_date_selector).unwrap())
                                    .next()
                                    .map(|e| e.inner_html())
                                    .unwrap_or_default();
            let location = event_element.select(&Selector::parse(&config.location_selector).unwrap()) 
                                    .next()
                                    .map(|e| e.inner_html())
                                    .unwrap_or_default();
            let relative_url = event_element.select(&Selector::parse(&config.url).unwrap())
                                    .next()
                                    .and_then(|e| e.value().attr("href"))
                                    .unwrap_or_default()
                                    .to_string();

            // Create an absolute URL based on the base URL and the relative URL
            let url = resolve_url(base_url, &relative_url);

        // Create an Event object and add it to the events vector
        let event = Event {
            name,
            start_date,
            end_date,
            location,
            url,
        };
        events.push(event);
        }
   }
   events
}

/// Resolves a relative URL against a base URL.
///
/// # Arguments
///
/// * `base` - The base URL as a string slice.
/// * `relative` - The relative URL to be resolved against the base URL.
///
/// # Returns
///
/// A `String` representing the absolute URL.
fn resolve_url(base: &str, relative: &str) -> String {
    let base_url = Url::parse(base).expect("Failed to parse base URL");
    base_url.join(relative).unwrap().to_string()
}

/// Represents an event with its details extracted from HTML content.
#[derive(Debug, Clone, PartialEq)]
pub struct Event {
    pub name: String,
    pub start_date: String,
    pub end_date: String,
    pub location: String,
    pub url: String,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_html() {
        // Mock HTML content
        let html = r#"
            <div class="event">
                <h2 class="name">Event Name</h2>
                <span class="start-date">2023-01-01</span>
                <span class="end-date">2023-01-02</span>
                <span class="location">Event Location</span>
                <a class="url" href="http://example.com/event">Event Link</a>
            </div>
        "#;

        // Configure SiteConfig with selectors that match the mock HTML
        let config = SiteConfig {
            event_selector: ".event".to_string(),
            name_selector: ".name".to_string(),
            start_date_selector: ".start-date".to_string(),
            end_date_selector: ".end-date".to_string(),
            location_selector: ".location".to_string(),
            url: ".url".to_string(),
        };

        // Base URL for resolving relative URLs
        let base_url = "http://example.com";

        // Call the parse_html function
        let events = parse_html(html, &config, base_url);

        // Expected event
        let expected_event = Event {
            name: "Event Name".to_string(),
            start_date: "2023-01-01".to_string(),
            end_date: "2023-01-02".to_string(),
            location: "Event Location".to_string(),
            url: "http://example.com/event".to_string(),
        };

        // Assertions
        assert_eq!(events.len(), 1);
        assert_eq!(events[0], expected_event);
    }
}



