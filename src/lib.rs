// A function that grabs the content from a wikipedia page using wikipedia-rs crate
// Reference: https://github.com/noahgift/rust-mlops-template/blob/main/hfdemo
extern crate wikipedia;
use rust_bert::pipelines::question_answering::{QaInput, QuestionAnsweringModel};
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

pub fn question_answering(question: &str, page: &str) -> String {
    let content = get_wiki_content(page);
    let context = content.chars().collect::<String>().replace('\n', " "); // remove newlines with spaces
    let qa_model = QuestionAnsweringModel::new(Default::default()).unwrap();
    let input_qa = QaInput {
        question: question.to_string(),
        context,
    };
    let output = qa_model.predict(&[input_qa], 1, 32);
    output[0].iter().map(|x| x.answer.to_string()).collect()
}
