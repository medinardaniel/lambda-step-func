# Rust AWS Lambda Data Processing Pipeline

By Daniel Medina

Demo video: https://youtu.be/AsTqqmNIpIw

## Overview
This project utilizes Rust to develop AWS Lambda functions that are orchestrated with AWS Step Functions to create a robust data processing pipeline. The pipeline is designed to handle text manipulation tasks, such as removing punctuation, filtering stopwords, and determining the most frequent word in a text.

## Components

### Rust AWS Lambda Function
- **Purpose**: The Lambda functions are written in Rust, providing the benefit of Rust's performance and safety features. Each function handles a specific part of the text processing task:
  1. **Remove Punctuation**: Cleans the input text by stripping out all punctuation.
  2. **Remove Stopwords**: Filters out common stopwords to reduce the dataset to meaningful words only.
  3. **Find Most Frequent Word**: Analyzes the text to find and return the most frequently occurring word.

### AWS Step Functions
- **Purpose**: AWS Step Functions coordinate the multiple Lambda functions. This workflow management allows the data to be processed in a controlled and orderly sequence, ensuring that each step of the process is completed before the next begins.
- **Workflow**:
  1. **Start**: The text data is input.
  2. **Step 1**: The Remove Punctuation Lambda function is invoked.
  3. **Step 2**: The output from Step 1 is passed to the Remove Stopwords Lambda function.
  4. **Step 3**: The output from Step 2 is used by the Find Most Frequent Word Lambda function to determine the most frequent word.
  5. **End**: The result from Step 3 is returned.

## Deployment
Each Lambda function is deployed via a zip file containing a statically linked executable named `bootstrap`, which is the binary compiled from the Rust code. The zip files are uploaded to AWS Lambda through the AWS CLI or management console.

## Orchestration with Step Functions
- The Step Functions are defined in the AWS Management Console, where each state is configured to trigger the respective Lambda function.
- Transitions between states are managed by Step Functions based on the output of each Lambda function, passing the necessary data to the next function in the workflow.

## Usage
- **Input**: Start the Step Functions state machine with a string of text as input.
- **Output**: The state machine outputs the most frequent word from the processed text.

## Updating the Lambda Functions
To update any Lambda function:
1. Make changes to the Rust source code as needed.
2. Rebuild the binary with `cargo build --release --target x86_64-unknown-linux-musl`.
3. Rename the resulting binary to `bootstrap` and zip it.
4. Upload the new zip file to the respective Lambda function using the AWS CLI:
   ```bash
   aws lambda update-function-code --function-name <Function-Name> --zip-file fileb://<zip-file-name>.zip
