
Educational Content Quiz Generator
==================================

This project is designed to extract educational content from a given URL, generate quiz questions and descriptive questions using the `ask_gemini` API, and save the generated content into a Markdown file. The project uses Rust and several crates, including `reqwest`, `scraper`, `serde`, and `ask_gemini`.

Features
--------

*   Extracts content from a specified URL.
*   Generates quiz questions and descriptive questions based on the extracted content.
*   Saves the generated content into a Markdown file.

Dependencies
------------

*   `ask_gemini`: Provides access to the Gemini API for generating questions.
*   `reqwest`: For making HTTP requests.
*   `scraper`: For parsing HTML content.
*   `serde`: For serializing and deserializing data.

Installation
------------

1.  **Clone the Repository**
    
    ```bash
    git clone https://github.com/thekartikanil/yourrepository.git
    cd yourrepository
    ```
    
2.  **Install Rust**
    
    If you haven't already installed Rust, you can do so by following the instructions at [rust-lang.org](https://www.rust-lang.org/learn/get-started).
    
3.  **Add Dependencies**
    
    Ensure you have the following dependencies in your `Cargo.toml` file:
    
    ```toml
    
    [dependencies]
    ask_gemini = "0.1.4"
    reqwest = {version = "0.12.7", features = ["json"]}
    scraper = "0.20.0"
    serde = {version = "1.0.209", features = ["derive"]}
    serde-wasm-bindgen = "0.6.5"
    serde_json = "1.0.127"
    tokio ={version="1.39.3",features = ["full", "macros"]}
    ```
    

Configuration
-------------

1.  **API Key**
    
    Replace `"YOUR_API_KEY"` in the `ask_gemini` function with your actual Gemini API key.
    
2.  **URL**
    
    Update the `url` variable in the `main` function with the URL of the content you want to process.
    

Usage
-----

1.  **Build the Project**
    
    ```bash
    cargo build
    ```
    
2.  **Run the Project**
    
    ```bash
    cargo run
    ```
    
    The program will extract content from the specified URL, generate quiz and descriptive questions, and save the results in a Markdown file located in the `pdfs/` directory.
    

Code Explanation
----------------

*   **`extract_content(url: &str) -> Result<String, reqwest::Error>`**
    
    Fetches and parses HTML content from the specified URL, extracting text from elements with the class `.content`.
    
*   **`ask_gemini(content: &str)`**
    
    Sends a request to the Gemini API with the provided content and a prompt for generating quiz questions.
    
*   **`save_responce(response: Vec<String>) -> std::io::Result<()>`**
    
    Saves the response from Gemini into a Markdown file named after the first line of the response content.
    
*   **`main()`**
    
    Entry point of the application. It extracts content from the URL, generates quiz questions, and prints the extracted content.
    

Contributing
------------

Feel free to fork the repository and submit pull requests. Contributions and suggestions are always welcome!

Thanks for your interest in contributing to this project!

Contact
-------

For questions or feedback, please contact [Me](mailto:thekartikanil@gmail.com).

* * *

Replace placeholder values like `"YOUR_API_KEY"` and `https://github.com/yourusername/yourrepository.git` with actual values related to your project.