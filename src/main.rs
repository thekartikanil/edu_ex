use ask_gemini::Gemini;
use reqwest;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
struct GeminiResponse {
    questions: Vec<Question>,
    learning_paths: Vec<LearningPath>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Question {
    question: String,
    choices: Vec<String>,
    answer: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct LearningPath {
    topic: String,
    url: String,
}

async fn extract_content(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?.text().await?;
    let document = Html::parse_document(&response);
    let selector = Selector::parse(".content").unwrap();

    let mut content = String::new();
    for element in document.select(&selector) {
        let text = element.text().collect::<Vec<_>>().join(" ");
        let trimmed_text = text
            .lines()
            .filter(|line| !line.trim().is_empty())
            .collect::<Vec<_>>()
            .join("\n");
        content.push_str(&trimmed_text);
        content.push('\n');
    }

    Ok(content.trim().to_string())
}
const PROMPT: &str = "
You are provided with educational content aimed at teaching a particular topic. Your task is to generate a set of quiz questions and descriptive questions to help learners understand the content better. The quiz should be divided into two parts:
1. **Explaination**
   - Provide an explanation of the content.
   - Explain the topics present in the content with definitions.
   - Provide examples of how the content can be used in practice.
   - Ensure that the explanation is relevant to the content and varies in difficulty to accommodate different levels of understanding.
   - Give explanation such that the learner get core insights of the content.
2. **Multiple-Choice Questions**:
   - Create 10-15 all possible easy level multiple-choice questions based on the content.
   - Create 10-15 medium and hard level multiple-choice questions based on the content.
   - Each question should have 4 options, labeled (A), (B), (C), and (D).
   - Clearly indicate the correct answer for each question.

3. **Descriptive Questions**:
   - Generate 10+ descriptive questions that require the learner to provide detailed, written responses.
   - These questions should be designed to encourage critical thinking and a deeper understanding of the topic.
   - Ensure that the questions are relevant to the content and vary in difficulty to accommodate different levels of understanding.
   - Provide examples of how these questions can be used in practice.
   - if content is related to coding make some coding practice question based on the content topics.

Please follow this format for each quiz:

### Multiple-Choice Questions
1. **Question 1**: [Insert question here]
   - (A) [Option A]
   - (B) [Option B]
   - (C) [Option C]
   - (D) [Option D]
   - **Correct Answer**: [Answer]
[...Continue for remaining multiple-choice questions...]

### Descriptive Questions 
1. **Question 1**: [Insert descriptive question here]
2. **Question 2**: [Insert descriptive question here]
3. **Question 3**: [Insert descriptive question here]

Please ensure that the questions are directly related to the provided content and vary in difficulty to accommodate different levels of understanding. The multiple-choice questions should be straightforward, while the descriptive questions should challenge the learner to think critically and explore the topic in depth.
";

async fn ask_gemini(content: &str) {
    let gemini = Gemini::new(Some("YOUR_API_KEY"), None);
    let prompt = format!("{} {}", content, PROMPT);

    match gemini.ask(&prompt).await {
        Ok(response) => {
            if let Err(e) = save_responce(response) {
                eprintln!("Failed to save the response: {}", e);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn save_responce(response: Vec<String>) -> std::io::Result<()> {
    // let filename = "quiz_and_descriptive_questions.txt";
    let first_line = response[0].lines().next().unwrap_or("output").trim();
    let filename = format!("pdfs/{}.md", first_line);
    let mut file = File::create(filename)?;
    file.write_all(response.join("\n").as_bytes())?;
    println!("Response saved");
    Ok(())
}

#[tokio::main]
async fn main() {
    let url = "www.gfg.com"; // Replace with the desired URL

    match extract_content(url).await {
        Ok(content) => {
            ask_gemini(&content).await;
            let first_line = content.lines().next().unwrap_or("output").trim();
            println!("Extracted Content: {}", first_line);
        }
        Err(e) => eprintln!("Error extracting content: {}", e),
    }
}