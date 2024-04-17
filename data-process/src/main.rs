
use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
use std::collections::HashSet;
use serde::{Deserialize, Serialize};

/// This is a made-up example. Requests come into the runtime as unicode
/// strings in json format, which can map to any structure that implements `serde::Deserialize`
/// The runtime pays no attention to the contents of the request payload.
#[derive(Deserialize)]
struct Request {
    text: String,
}

/// This is a made-up example of what a response structure may look like.
/// There is no restriction on what it can be. The runtime requires responses
/// to be serialized into json. The runtime pays no attention
/// to the contents of the response payload.
#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // Extract some useful info from the request
    let text = event.payload.text;

    // Remove all stopwords
    let stopwords: HashSet<&str> = [
        "a", "about", "above", "after", "again", "against", "all", "am", "an", "and", "any", 
        "are", "as", "at", "be", "because", "been", "before", "being", "below", "between", 
        "both", "but", "by", "could", "did", "do", "does", "doing", "down", "during", "each", 
        "few", "for", "from", "further", "had", "has", "have", "having", "he", "he'd", "he'll", 
        "he's", "her", "here", "here's", "hers", "herself", "him", "himself", "his", "how", 
        "how's", "i", "i'd", "i'll", "i'm", "i've", "if", "in", "into", "is", "it", "it's", 
        "its", "itself", "let's", "me", "more", "most", "my", "myself", "nor", "of", "on", 
        "once", "only", "or", "other", "ought", "our", "ours", "ourselves", "out", "over", 
        "own", "same", "she", "she'd", "she'll", "she's", "should", "so", "some", "such", 
        "than", "that", "that's", "the", "their", "theirs", "them", "themselves", "then", 
        "there", "there's", "these", "they", "they'd", "they'll", "they're", "they've", 
        "this", "those", "through", "to", "too", "under", "until", "up", "very", "was", "we", 
        "we'd", "we'll", "we're", "we've", "were", "what", "what's", "when", "when's", "where", 
        "where's", "which", "while", "who", "who's", "whom", "why", "why's", "with", "would", 
        "you", "you'd", "you'll", "you're", "you've", "your", "yours", "yourself", "yourselves"
    ].iter().cloned().collect();

    let text = text.split_whitespace()
        .filter(|word| !stopwords.contains(word))
        .collect::<Vec<&str>>()
        .join(" ");

    // Prepare the response
    let resp = Response {
        req_id: event.context.request_id,
        msg: format!("{}", text),
    };

    // Return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
