//! # Data Processing Module
//!
//! This module is responsible for processing raw event data into a more usable format.
//! It includes functionality to clean and format text, parse dates, and transform `Event` 
//! instances into `ProcessedEvent` instances with more structured and clean data.

use chrono::Local;
use crate::html_parser::Event;

/// Processes a vector of `Event` instances into `ProcessedEvent` instances.
///
/// This function takes raw event data and applies cleaning and formatting to the text and dates.
/// It ensures that the data is in a consistent and usable format.
///
/// # Arguments
///
/// * `events` - A vector of `Event` instances representing the raw event data.
///
/// # Returns
///
/// A vector of `ProcessedEvent` instances with cleaned and formatted data.
pub fn process_data(events: Vec<Event>) -> Vec<ProcessedEvent> {
    events.into_iter().map(|event| {
        ProcessedEvent {
            name: clean_text(&event.name),
            start_date: parse_date(&event.start_date, true),
            end_date: parse_date(&event.end_date, false),
            location: clean_text(&event.location),
            url: event.url,
        }
    }).collect()
}

/// Cleans and trims the given text.
///
/// # Arguments
///
/// * `text` - A string slice representing the text to be cleaned.
///
/// # Returns
///
/// A `String` with leading and trailing whitespace removed.
fn clean_text(text: &str) -> String {
    text.trim().to_string()
}

/// Parses a date string and returns a formatted date or a default value.
///
/// If the date string is empty and `is_start_date` is true, it returns today's date.
/// If the date string is empty and `is_start_date` is false, it returns "N/A".
///
/// # Arguments
///
/// * `date_str` - A string slice representing the date to be parsed.
/// * `is_start_date` - A boolean indicating whether the date is a start date.
///
/// # Returns
///
/// A `String` representing the parsed date or a default value.
fn parse_date(date_str: &str, is_start_date: bool) -> String {
    if date_str.trim().is_empty() {
        if is_start_date {
            // For start_date, return today's date if empty
            today_date()
        } else {
            // For end_date, return "N/A"
            "N/A".to_string()
        }
    } else {
        date_str.trim().to_string() // Return the date as-is
    }
}

/// Returns today's date in a formatted string.
///
/// # Returns
///
/// A `String` representing today's date in the format "%B %e".
fn today_date() -> String {
    let today = Local::now();
    today.format("%B%e").to_string() //%B is the full month name, %e is the day of the month
}

/// A struct representing a processed event with cleaned and formatted data.
#[derive(Debug, Clone, PartialEq)]
pub struct ProcessedEvent {
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
    fn test_process_data() {
        // Create mock raw events
        let raw_events = vec![
            Event {
                name: "Concert".to_string(),
                start_date: "January 1, 2023".to_string(),
                end_date: "".to_string(),
                location: "Park".to_string(),
                url: "http://example.com/concert".to_string(),
            },
            Event {
                name: "Festival".to_string(),
                start_date: "January 2, 2023".to_string(),
                end_date: "January 3, 2023           ".to_string(),
                location: "Beach".to_string(),
                url: "http://example.com/festival".to_string(),
            },
        ];

        // Call the process_data function
        let processed_events = process_data(raw_events);

        // Define expected processed events
        let expected_events = vec![
            ProcessedEvent {
                name: "Concert".to_string(),
                start_date: "January 1, 2023".to_string(),
                end_date: "N/A".to_string(),
                location: "Park".to_string(),
                url: "http://example.com/concert".to_string(),
            },
            ProcessedEvent {
                name: "Festival".to_string(),
                start_date: "January 2, 2023".to_string(),
                end_date: "January 3, 2023".to_string(),
                location: "Beach".to_string(),
                url: "http://example.com/festival".to_string(),
            },
        ];

        // Assertions
        assert_eq!(processed_events, expected_events);
    }
}


