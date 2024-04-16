# Ungoliant

<img align="left" src="img/logo.png" width="200" height="200" /> 

![](https://img.shields.io/crates/l/ungoliant?style=flat-square) 

🕷️ **Ungoliant is a high-performance pipeline that provides tools to build corpus generation pipelines from CommonCrawl.** 🕷️


## Installation

### Installing/Compiling the binary
* Via `git`: `cargo install --git https://github.com/kargaranamir/ungoliant`

Ungoliant needs numerous dependencies that should be compiled when installing. However `cmake / gcc` can be needed as the project uses [fasttext-rs](https://github.com/messense/fasttext-rs).

### KenLM feature

The KenLM feature is optional because it relies on unsafe code that can break if the supplied model files are not correct.

To enable it, install KenLM requirements:

```bash
apt install -y libboost-all-dev libeigen3-dev
```

and use `cargo install ungoliant --features kenlm` or `cargo b --features kenlm` if you're building from source.

### Getting a language identification file (for fastText):

By default, `ungoliant` expects the `lid.176.bin` model by meta. 
Use `wget https://huggingface.co/cis-lmu/glotlid/resolve/main/model.bin -O lid.176.bin` to get it.

However, you can use the model you want: just point to its path using `ungoliant download --lid-path <path to lid>`.

## Usage 

The usual way of generating corpora is:

1. Fetch the `wet.paths.gz` file from the last [CommonCrawl dump](https://commoncrawl.org/connect/blog/) and decompress it.
2. Download the files using the `download` command.
3. Generate the corpus using the `pipeline` command (it may take some time).
4. Head on to [oscar-tools](https://github.com/kargaranamir/oscar-tools) for the packaging steps

You can find more information on each command's `--help`.

```text
ungoliant 2
corpus generation tool.

USAGE:
    ungoliant <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    download    Download a CommonCrawl release
    help        Prints this message or the help of the given subcommand(s)
    pipeline    Run pipeline
    rebuild     Rebuild the corpus for a given language.
```

## Documentation

Ungoliant is not yet on docs.rs: use `cargo doc --bins --open` to open the documentation.

Head on to [OSCAR Documentation](https://oscar-project.github.io/documentation/) for more info about the project.

