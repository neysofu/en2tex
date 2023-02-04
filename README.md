# `en2tex`

A CLI tool that translates formulas, equations from English to LaTeX.

[![demo](https://asciinema.org/a/ii79xAvU4kIklFvaHYiL5o4J0.svg?rows=15)](https://asciinema.org/a/ii79xAvU4kIklFvaHYiL5o4J0?autoplay=1&rows=15)

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
$ en2tex --help
Translated English into LaTeX formulas, using OPENAI APIs

Usage: en2tex [OPTIONS] [PROMPT]...

Arguments:
  [PROMPT]...  English description of the desired LaTeX output

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
