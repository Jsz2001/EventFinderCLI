[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-24ddc0f5d75046c5622901739e7c5dd533143b0c8e959d652212380cedb1ea36.svg)](https://classroom.github.com/a/RQfdh2iK)
# EventFinder

## Description

EventFinder is a comprehensive web scraping CLI application developed in Rust, designed to help people discover local events happening in Nashville. In today's fast-paced world, finding interesting local activities such as concerts, art exhibitions, workshops, and community gatherings can be a time-consuming task, often requiring individuals to visit multiple websites and sift through a countless information. This application addresses this challenge by aggregating data from various event-listing websites into a single, cohesive platform. By doing so, it not only saves time but also provides a broader range of options for users seeking to engage with their local community or explore new interests.

The core of EventFinder lies in its scraping engine, capable of handling different website layouts and extracting key information. This engine is the heart of the application, ensuring that data from multiple sources is consistently up-to-date and reliable. The application stands out in its ability to normalize disparate data formats. Whether you're looking for a local music festival, an educational workshop, or a community art show, EventFinder makes finding these events as simple and efficient as possible.

## Installation

To install Event Finder, follow these steps:

1. Clone the Repository: <br>
git clone https://github.com/your-username/event-finder.git <br>
cd event-finder

2. Build the Project: <br>
Inside the project directory, run: <br>
cargo build --release

3. Run the Application: <br>
After building, you can start the application using: <br>
./target/release/event_finder <br>
This command runs the Event Finder application.

## How to use

After starting the application, follow the on-screen prompts to choose an event category and view events. You can select from music, unique, general, or all events. To exit the application, choose the 'Quit' option.