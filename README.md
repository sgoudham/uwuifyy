# uwuifyy

[![build](https://github.com/sgoudham/uwuifyy/actions/workflows/build.yml/badge.svg)](https://github.com/sgoudham/uwuifyy/actions/workflows/build.yml)
[![crate.io](https://img.shields.io/crates/v/uwuifyy)](https://crates.io/crates/uwuifyy)
[![downloads](https://img.shields.io/crates/d/uwuifyy)](https://crates.io/crates/uwuifyy)
[![license](https://img.shields.io/github/license/sgoudham/uwuifyy)](LICENSE)

> A robust, customizable, blazingly-fast, efficient and easy-to-use command line application to uwu'ify your text!
>
![](logo/uwuifyy-logo.png)
>
> Logo Credits: Jade Nelson

## Table Of Contents

- [uwuifyy](#uwuifyy)
    * [About](#about)
    * [Features](#features)
    * [Installation](#installation)
        * [Homebrew](#homebrew)
        * [Binaries](#binaries)
            + [Windows](#windows)
            + [Linux \ macOS](#Linux-\-macOS)
        * [Rust \ Cargo](#Rust-\-Cargo)
    * [Usage](#usage)
        + [Text Input to Text Output](#text-input-to-text-output)
        + [Text Input to File Output](#text-input-to-file-output)
        + [File Input to File Output](#file-input-to-file-output)
    * [Benchmarks](#benchmarks)
        + [The Complete Works of William Shakespeare](#the-complete-works-of-william-shakespeare)
        + [300k+ Tokyo 2020\21 Olympics Tweets](#300k+-tokyo-2020\21-olympics-tweets)
        + [3.5M Tiktok Google Play Reviews](#35m-tiktok-google-play-reviews)
        + [3.6M Amazon Reviews And Ratings](#36m-amazon-reviews-and-ratings)
    * [License](#license)
    * [Acknowledgement](#acknowledgement)

## About

Have you ever wanted to uwu'ify text and large files at blazing fast speeds? Well! You came to the right
place! `uwuifyy`
allows you to _uwu'ify_ text and files from within your own terminal in an _extremely_ fast manner!

## Features

- [x] _**BLAZINGLY FAST**_ (Check out -> [Benchmarks](#benchmarks))
- [x] Seeded by default to allow for reproducible results
- [x] Excludes URL's & Emails
- [x] UwU'ify files available locally on disk
- [x] Completely customisable modifiers

## Installation

### Homebrew

For `macOS` users, installation through [Homebrew](https://brew.sh/) is recommended.

```shell
$ brew tap sgoudham/tap
$ brew install uwuifyy
```

### Binaries

Binaries for **Windows**, **macOS** & **Linux** are available with every
single [release](https://github.com/sgoudham/uwuifyy/releases)

### Windows

1. Download either `uwuifyy-x86_64-pc-windows-msvc.zip` or `uwuifyy-x86_64-pc-windows-gnu.zip`


2. Extract into `\bin` folder at `C:\your\path\here\`

```
C:
|__your
    |__path
        |__here
            |__bin
                |__uwuifyy.exe
```

3. Set `uwuifyy.exe` in your path to access it globally

```shell
$ setx path "%path%;C:\your\path\here\bin"
```

4. Refresh command line and verify installation

```shell
$ uwuifyy --help
```

### Linux \ macOS

1. Download `uwuifyy-x86_64-unknown-linux-gnu.tar.gz` or `uwuifyy-x86_64-unknown-linux-musl.tar.gz`
   or `uwuifyy-x86_64-apple-darwin.tar.gz`


2. Extract into your local directory

```shell
# Linux
$ tar -xf uwuifyy-x86_64-unknown-linux-gnu.tar.gz
$ tar -xf uwuifyy-x86_64-unknown-linux-musl.tar.gz

# macOS
$ tar -xf uwuifyy-x86_64-apple-darwin.tar.gz
```

3. Move into `~/bin`

```shell
# Create ~/bin if it does not exist
$ mkdir -p ~/bin
$ mv uwuifyy ~/bin
```

4. Set permissions for executable

```shell
$ chmod 755 ~/bin/uwuifyy
```

5. Update `PATH` to use globally

```shell
# Linux
$ echo 'export PATH=~/bin:$PATH' >> ~/.bashrc 
$ source ~/.bashrc

# macOS
$ echo 'export PATH=~/bin:$PATH' >> ~/.bash_profile
$ source ~/.bash_profile
```

6. Verify installation

```shell
$ uwuifyy --version
uwuifyy 0.3.0
```

## Rust \ Cargo

Alternatively, if using Rust's package manager, `Cargo`, all that is needed is

```shell
$ cargo install uwuifyy
```

If you do not have `Cargo` available on your machine, you can download it as part of
Rust [here](https://www.rust-lang.org/tools/install)

## Usage

```commandline
$ uwuifyy --help

USAGE:
    uwuifyy.exe [OPTIONS] <--text <TEXT>|--infile <FILE>>

OPTIONS:
    -t, --text <TEXT>         The text to uwu'ify
    -i, --infile <FILE>       The file to uwu'ify
    -o, --outfile <FILE>      The file to output uwu'ified text
        --ascii-only          The uwu'ified text will only include ASCII faces
        --unicode-only        The uwu'ified text will only include UTF-8 faces
    -r, --random              The flag to enable randomized uwu'ified text
    -w, --words <VALUE>       The modifier to determine how many words to be uwu'ified [default: 1]
    -f, --faces <VALUE>       The modifier for uwu faces e.g hello -> (^-^*)/ hewwo [default: 0.05]
    -a, --actions <VALUE>     The modifier for actions e.g *shuffles over* [default: 0.125]
    -s, --stutters <VALUE>    The modifier for stutters e.g b-baka! [default: 0.225]
    -h, --help                Print help information
    -V, --version             Print version information
```

### Text Input to Text Output

![](examples/gifs/text-input-to-text-output.gif)

### Text Input to File Output

![](examples/gifs/text-input-to-file-output.gif)

### File Input to File Output

![](examples/gifs/text-file-to-file-output.gif)

## Benchmarks

- Benchmarks/Tests carried out on a `Dell XPS 15`
    - CPU: `11th Gen Intel(R) Core(TM) i7-11800H @ 2.30GHz`
    - SSD: `NVMe Micron 2300 1TB`

### The Complete Works of William Shakespeare

- [Dataset](https://www.kaggle.com/kewagbln/shakespeareonline)
- Size: `5.46 MiB`
- Time Taken: `< 1s`

![](examples/gifs/william-shakespeare.gif)

### 300k+ Tokyo 2020\21 Olympics Tweets

- [Dataset](https://www.kaggle.com/amritpal333/tokyo-olympics-2021-tweets)
- Size: `98.54 MiB`
- Time Taken: `1s`

![](examples/gifs/tokyo-2020-olympics-tweets.gif)

### 3.5M Tiktok Google Play Reviews

- [Dataset](https://www.kaggle.com/shivamb/35-million-tiktok-mobile-app-reviews)
- Size: `543.02 MiB`
- Time Taken: `4s`

![](examples/gifs/tiktok_app_reviews.gif)

### 3.6M Amazon Reviews And Ratings

- [Dataset](https://www.kaggle.com/bittlingmayer/amazonreviews?select=train.ft.txt.bz2)
- Size: `1.6 GiB`
- Time Taken: `21s`
- **DISCLAIMER:** The input is a 1.6GB file and the output is a 2.2GB file. They are not included in the repo.

![](examples/gifs/amazon-ratings-reviews.gif)

## License

[MIT License](LICENSE)

## Acknowledgement

This project is inspired from one of many existing normal to uwu converters:
[Uwuifier](https://github.com/Schotsl/Uwuifier-node)
