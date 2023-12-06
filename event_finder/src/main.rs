//! # Event Finder CLI Application
//!
//! This application is a command-line interface (CLI) for finding events.
//! It allows users to fetch and view event information based on different categories
//! such as music, unique, general, and all. The application fetches event data from
//! specified URLs, processes it, and displays it in a user-friendly format.
//!
//! The program demonstrates the use of external crates like `reqwest` for web requests,
//! `chrono` for date and time handling, and custom modules for HTML parsing and data processing.

mod web_requests;
mod html_parser;
mod data_processing;

use chrono::{Local, Datelike, Timelike};
use std::io::{self, Write};

/// The entry point of the Event Finder CLI application.
///
/// This function handles user input to select different event types to view,
/// calls appropriate functions to fetch and display events, and manages the application flow.
fn main() {
    // Welcome message
    println!("Welcome to the Event Finder!\n");

    // Print today's date and time
    let now = Local::now();
    println!("Today's date is {}-{}-{}", now.year(), now.month(), now.day());
    println!("Current time is {}:{}:{}\n", now.hour(), now.minute(), now.second());


    loop {
        // Ask the user to choose an event type
        println!("Please choose an event type:");
        println!("1: Music");
        println!("2: Unique");
        println!("3: General");
        println!("4: All");
        println!("5: Quit");

        // Read user input
        let mut input = String::new();
        io::stdout().flush().unwrap(); // Flush to make sure the prompt is printed before reading input
        io::stdin().read_line(&mut input).unwrap();

        // Process user input
        match input.trim() {
            "1" | "Music" | "music" => {
                println!("Fetching music events...");
                fetch_music_events("https://www.songkick.com/metro-areas/11104-us-nashville/tonight");
            },
            "2" | "Unique" | "unique" => {
                println!("Fetching unique events...");
                fetch_unique_events("https://en.perto.com/us/nashville-10005/events-today/");
            },
            "3" | "General" | "general" => {
                println!("Fetching general events...");
                fetch_general_events("https://www.nashville.com/calendar-of-events/");
            },
            "4" | "All" | "all" => {
                println!("Fetching all events...");
                fetch_music_events("https://www.songkick.com/metro-areas/11104-us-nashville/tonight");
                fetch_unique_events("https://en.perto.com/us/nashville-10005/events-today/");
                fetch_general_events("https://www.nashville.com/calendar-of-events/");
            },
            "5" | "quit" | "Quit" => {
                println!("Exiting the Event Finder.");
                break;
            },
            _ => {
                println!("Invalid input. Please enter a number (1-4) or event type.");
                // The loop will continue
            }
        }

        // Ask if the user wants to continue or quit
        if !should_continue() {
            println!("Thank you for using the Event Finder!");
            break;
        }
    }   
}


/// Fetches and displays general events from a specified URL.
///
/// # Arguments
///
/// * `url` - The URL from which to fetch general events.
fn fetch_general_events(url: &str) {

    let gen_url = url;

    let html_content = match web_requests::fetch_url(gen_url) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error fetching URL: {}", e);
            return;
        }
    };

    let gen_config = html_parser::SiteConfig {
        event_selector: String::from(".tribe-events-calendar-list__event"),
        name_selector: String::from(".tribe-events-calendar-list__event-title-link"),
        start_date_selector: String::from(".tribe-event-date-start"),
        end_date_selector: String::from(".tribe-event-date-end"),
        location_selector: String::from(".tribe-events-calendar-list__event-venue-title"),
        url: String::from(".tribe-events-calendar-list__event-title-link"),
    };

    let events = html_parser::parse_html(&html_content, &gen_config, "https://www.nashville.com");

    // Process the raw events to get processed events
    let processed_events = data_processing::process_data(events);

    for event in processed_events {
        println!("Name: {}\nStart Date: {}\nEnd Date: {}\nLocation: {}\nURL: {}", 
        event.name, event.start_date, event.end_date, event.location, event.url);
        println!(""); // Add a blank line between events
    }
}

/// Fetches and displays music events from a specified URL.
///
/// # Arguments
///
/// * `url` - The URL from which to fetch music events.
fn fetch_music_events(url: &str) {
    let songkick_url = url;

    let song_html_content = match web_requests::fetch_url(songkick_url) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error fetching URL: {}", e);
            return;
        }
    };

    let song_config = html_parser::SiteConfig {
        event_selector: String::from(".event-listings-element"),
        name_selector: String::from(".artists > a > span > strong"),
        start_date_selector: String::from(".time"),
        end_date_selector: String::from(".time"),
        location_selector: String::from(".location > span > a"),
        url: String::from(".artists > .event-link"),
    };

    let events = html_parser::parse_html(&song_html_content, &song_config, "https://www.songkick.com");

    // Process the raw events to get processed events
    let processed_events = data_processing::process_data(events);

    for event in processed_events {
        println!("Name: {}\nStart Date: {}\nEnd Date: {}\nLocation: {}\nURL: {}", 
        event.name, event.start_date, event.end_date, event.location, event.url);
        println!(""); // Add a blank line between events
    }
}

/// Fetches and displays unique events from a specified URL.
///
/// # Arguments
///
/// * `url` - The URL from which to fetch unique events.
fn fetch_unique_events(url: &str) {

    let unique_url = url;

    let html_content = match web_requests::fetch_url(unique_url) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error fetching URL: {}", e);
            return;
        }
    };

    let unique_config = html_parser::SiteConfig {
        event_selector: String::from(".pt_col"),
        name_selector: String::from(".infos > a > strong"),
        start_date_selector: String::from(".infos > ul > li > span"),
        end_date_selector: String::from(".time"),
        location_selector: String::from(".infos > ul > .pt_list-item.event-location > span"),
        url: String::from("a"),
    };

    let events = html_parser::parse_html(&html_content, &unique_config, "https://en.perto.com");

    // Process the raw events to get processed events
    let processed_events = data_processing::process_data(events);

    for event in processed_events {
        println!("Name: {}\nStart Date: {}\nEnd Date: {}\nLocation: {}\nURL: {}", 
        event.name, event.start_date, event.end_date, event.location, event.url);
        println!(""); // Add a blank line between events
    }
}

/// Prompts the user to choose whether to continue using the application.
///
/// # Returns
///
/// A boolean value indicating whether the user wants to continue (`true`) or exit (`false`).
fn should_continue() -> bool {
    loop {
        println!("\nWould you like to choose another option? (yes/no)");
        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().to_lowercase().as_str() {
            "yes" | "y" => return true,
            "no" | "n" => return false,
            _ => println!("Invalid input. Please enter 'yes' or 'no'."),
        }
    }
}