# cisnlp/Ungoliant

![](https://img.shields.io/crates/l/ungoliant?style=flat-square) 

🕷️ **Ungoliant is a high-performance pipeline that provides tools to build corpus generation pipelines from CommonCrawl.** 🕷️

This pipeline was originally used to process the OSCAR dataset. It uses the fasttext lid.176.bin model to generate labels for 176 languages. We forked the code here so that it can function with GlotLID, which is also a fasttext model but can label text for more than 2000 languages.

The outcome of this new dataset is the GlotCC dataset, available at: https://github.com/cisnlp/GlotCC

## Installation

### Installing/Compiling the binary
* Via `git`: `cargo install --git https://github.com/cisnlp/ungoliant`

Ungoliant needs numerous dependencies that should be compiled when installing. However `cmake / gcc` can be needed as the project uses [fasttext-rs](https://github.com/messense/fasttext-rs).


### Getting a language identification file (for fastText):

By default, `ungoliant` expects the `lid.176.bin` model name. 
Use `wget https://huggingface.co/cis-lmu/glotlid/resolve/main/model.bin -O glotlid.bin` to get GlotLID as `glotlid.bin`.

However, you can use the model you want: just point to its path using `ungoliant download --lid-path <path to lid>`. 


## Usage 

The usual way of generating corpora is:

1. First create this structure of folders with `mkdir`:

```
res
├── annotation             
│   └── ...
├── blocklist             
│   └── ...
├── corpus             
│   └── ...
├── filter             
│   └── ...
└── shards             
    └── ...
```

1. Fetch the `wet.paths.gz` file from the last [CommonCrawl dump](https://commoncrawl.org/get-started). <br />
1.1 Decompress it using `gzip -d wet.paths.gz`. <br />
1.2 Download the files using the `download` command: `ungoliant download wet.paths res/shards`. <br />
   
2. Download website categorizations using  `wget https://github.com/olbat/ut1-blacklists/archive/refs/heads/master.zip`. <br />
2.1 Decompress it using `unzip master.zip`. <br />
2.2 Move the blacklists to the `res/blocklist` using  `mv ut1-blacklists-master/blacklists/* res/blocklist`. <br />
2.3 Decompress the adult block using `gzip -d res/blocklist/adult/domains.gz`. <br />
2.4 Remove the blacklists-master using `rm -r ut1-blacklists-master`. <br />

3. Generate the corpus using the `pipeline` command (it may take some time): `ungoliant pipeline ./res/shards/ ./res/corpus --lid-path glotlid.bin --blocklist-path ./res/blocklist/`. <br />

4. Head on to [glotcc-filters](https://github.com/cisnlp/GlotCC/tree/main/filters/) for the additional filter steps.

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
