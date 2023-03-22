// Reference: https://github.com/noahgift/rust-mlops-template/blob/main/hfdemo
extern crate wikipedia;
use rust_bert::pipelines::keywords_extraction::{KeywordExtractionConfig, KeywordExtractionModel};
use rust_bert::pipelines::summarization::SummarizationModel;

pub fn get_wiki_content(page: &str) -> String {
    let wiki = wikipedia::Wikipedia::<wikipedia::http::default::Client>::default();
    let page = wiki.page_from_title((page).to_owned());
    page.get_content().unwrap()
}

//build a function that summarizes the content from a wikipedia page
pub fn summarize_content(content: &str) -> String {
    let model = SummarizationModel::new(Default::default()).unwrap();
    //clean up input to max 2000 characters
    let input = content
        .chars()
        .take(2000)
        .collect::<String>()
        // remove newlines with spaces
        .replace('\n', " ");
    //convert to a vector of strings
    let input = vec![input];
    //summarize the content
    let output = model.summarize(&input);
    //return the first element of the vector
    output[0].clone()
}

pub fn keyword_extraction(page: &str) -> Vec<String> {
    let content = get_wiki_content(page);
    let context = content.chars().collect::<String>().replace('\n', " "); // remove newlines with spaces
    let config = KeywordExtractionConfig {
        num_keywords: 10,
        ..Default::default()
    };
    let keyword_extraction_model = KeywordExtractionModel::new(config).unwrap();
    let output = keyword_extraction_model.predict(&[context]);
    // Convert the result to a vector of String
    let keywords = match output {
        Ok(keywords_vec) => keywords_vec
            .into_iter()
            .flatten()
            .map(|keyword| keyword.text)
            .collect(),
        Err(err) => {
            eprintln!("Error: {err:?}");
            Vec::new()
        }
    };
    keywords
}
