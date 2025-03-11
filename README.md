
# Rust GenAI-Powered Conventional Commit Generator

<a href="https://crates.io/crates/mkcmt" rel="nofollow noopener noreferrer">
  <img src="https://img.shields.io/crates/v/mkcmt.svg" alt="Crates.io">
</a>

<!-- Version notice -->
<p style="font-style: italic; color: #ccc; margin-top: 0.5em;">
  You are reading documentation version <span id="doc-version" style="color: white;">0.2.0</span>.
  If this does not match the version displayed above, then you're not reading the latest documentation!
</p>

## Overview

This tool generates concise or lengthy, Conventional Commit-compatible commit messages using GPT-4o-mini (ChatGPT), based on your Git diff outputs. It intelligently handles both staged and unstaged changes, progressively refining commit suggestions based on user feedback, and can copy the final commit message directly to your clipboard.

## Features

- **Intelligent Commit Message Generation:** Leverages AI to analyze Git diffs and generate clear, descriptive commit messages.
- **Interactive Refinement:** If the initial suggestion isn't suitable, the tool progressively refines the prompt to generate better commit messages.
- **Clipboard Integration:** Offers the convenience of copying commit messages directly to the clipboard.
- **Detailed Logging:** Maintains logs of both generated commit messages and refined prompts for easy reference.

## Installation

### Prerequisites

- Rust and Cargo installed
- Access to the GPT-4o-mini model (requires an OpenAI API key)

### Setup

1. Clone the repository:

```sh
git clone https://github.com/davehorner/mkcmt.git
cd mkcmt
cargo install --path .
```


2. **Set API Key**

Set your OpenAI API key in the environment variable:

```bash
export OPENAI_API_KEY="your-api-key-here"
```


## Usage

run mkcmt in a git folder with some changes.  follow the prompts.


### Workflow:

- The tool checks for both staged (`git diff --cached`) and unstaged (`git diff`) changes.
- Prompts whether to combine staged and unstaged changes if both exist.
- Queries ChatGPT for a suitable commit message based on the provided diff.
- Presents the commit message and prompts for acceptance.
  - If accepted, optionally copies it to the clipboard.
  - If declined, GPT is used iteratively to refine the prompt until an acceptable commit message is produced.

## Output Logs

The tool maintains two log files for transparency and auditing purposes: 
   
- `cc` stands for conventional commit

- `output_cc_suggestions.txt`
  - Logs each generated commit message, clearly separated.

- `output_cc_prompts.txt`
  - Logs each refined prompt used for subsequent message generation.

The logs are continually appended with clear separators, facilitating easy review and continuous improvement.

## Requirements

- Rust
- OPENAI_API_KEY key for GPT-4o-mini accessible via your environment.
- git must be installed and accessible via command line.


## Example

```
$ cargo run
Both staged and unstaged changes detected.
Include both staged and unstaged changes in commit message? (y/n): y

Querying ChatGPT for commit message...

Suggested commit message:
feat: implement interactive AI-powered commit message generation tool

Accept this commit message? (y/n): n
Refining prompt for a better commit message...

Querying ChatGPT again with refined prompt...

Suggested commit message:
feat: add interactive refinement for better commit message accuracy based on user feedback

Accept this commit message? (y/n): y

Copy commit message to clipboard? (y/n): y
Commit message copied to clipboard.
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

<a href="https://crates.io/crates/mkcmt" rel="nofollow noopener noreferrer">
  <img src="https://img.shields.io/crates/v/mkcmt.svg" alt="Crates.io">
</a>

<!-- Version notice -->
<p style="font-style: italic; color: #ccc; margin-top: 0.5em;">
  You are reading documentation version <span id="doc-version" style="color: white;">0.2.0</span>.
  If this does not match the version displayed above, then you're not reading the latest documentation!
</p>
