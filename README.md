# Async API Calls CLI Project

## Overview

This small CLI project showcases the usage of asynchronous API calls in Rust, employing popular crates like "Tokio" and "Reqwest." The project utilizes asynchronous programming to make HTTP requests, demonstrating the benefits of concurrent operations.

## Features

-   **Asynchronous Operation:** The project leverages asynchronous programming principles to perform API calls concurrently, enhancing efficiency and responsiveness.

-   **Tokio and Reqwest Integration:** The use of the "Tokio" and "Reqwest" crates demonstrates how these libraries can be seamlessly integrated to achieve asynchronous tasks in Rust.

## Code Details

The main functionality is encapsulated within the `main` function, marked with the `#[tokio::main]` attribute, enabling asynchronous execution. The program performs the following steps:

1. **HTTP Request:** Asynchronously sends an HTTP GET request to "http://httpbin.org/get."

2. **Metadata Retrieval:** Fetches and prints the HTTP status code and headers of the response immediately upon their availability.

3. **Body Retrieval:** Asynchronously reads and prints the response body, ensuring that the program doesn't proceed until the body is fully received.

## How to Run

To execute the project, follow these steps:

1. Clone the repository to your local machine.

    ```bash
    git clone https://github.com/VivekChaurasia03/Rust-Async-Request.git
    ```

2. Navigate to the project directory.

    ```bash
    cd Rust-Async-Request
    ```

3. Run the project using Cargo.
    ```bash
    cargo run
    ```

## Dependencies

-   **Tokio:** A runtime for writing reliable asynchronous applications in Rust.

-   **Reqwest:** An easy-to-use HTTP client for Rust, providing support for asynchronous requests.

## Contributing

Feel free to contribute by opening issues, suggesting enhancements, or submitting pull requests. Your feedback and involvement are highly appreciated!
