# `en2tex`

A CLI utility to generate LaTeX mathematical expressions, powered by OpenAI APIs.

![demo](resources/demo.gif)

## Installation

```
cargo install en2tex
```

You may need to close and reopen your terminal after installation.

## Usage

`en2tex` uses [GPT-3](https://beta.openai.com/). To use it, you'll need to grab an API key from [your dashboard](https://beta.openai.com/), and save it to `OPENAI_API_KEY` environment variable as follows (I suggest saving it in your shell's profile for persistance between sessions):

```bash
export OPENAI_API_KEY='sk-XXXXXXXX'
```

Once you have configured your environment, run `en2tex` followed by whatever mathematical expression that you want.

To get a full overview of all available options, run `en2tex --help`

```
A CLI utility to generate LaTeX mathematical expressions, powered by OpenAI APIs

Usage: en2tex [OPTIONS] [PROMPT]...

Arguments:
  [PROMPT]...  Description of the desired LaTeX output. You can use English words, abbreviations, or any notation resembling LaTeX commands and AsciiMath

Options:
  -m, --model <MODEL>  Which OpenAI model to use [default: text-davinci-003]
  -c, --copy           Copy the generated LaTeX without asking for confirmation first
  -h, --help           Print help information
  -V, --version        Print version information
```

## Acknowledgements 

This repository is a fork of https://github.com/m1guelpf/plz-cli, which inspired `en2tex`. The original code was simple enough for me to seamlessly adapt it to LaTeX. Thank you to the original authors!

## License

This project is open-sourced under the MIT license. See [the License file](LICENSE) for more information.
