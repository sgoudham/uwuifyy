# uwuifyy

<p align="center">
    <img src="https://img.shields.io/crates/v/uwuifyy"/>
    <img src="https://img.shields.io/github/license/sgoudham/uwuifyy"/>
    <img src="https://img.shields.io/badge/project%20type-personal-blueviolet"/>
    <img src="https://img.shields.io/github/last-commit/sgoudham/uwuifyy"/>
    <img src="https://img.shields.io/github/issues/sgoudham/uwuifyy?label=issues"/>
</p>

> A robust, customizable, efficient and easy-to-use command line application to uwu'ify your text! 
> 
![](logo/UwUifyy.png)
>
> Logo Credits: Jade Nelson

## Table Of Contents

- [uwuifyy](#uwuifyy)
    * [About](#about)
    * [Features](#features)
        + [Modifiers](#modifiers)
    * [Installation](#installation)
    * [Quick Start](#quick-start)
        + [Text Input to Text Output](#text-input-to-text-output)
        + [Text Input to File Output](#text-input-to-file-output)
        + [File Input to File Output](#file-input-to-file-output)
    * [Demos](#Demos)
        + [The Complete Works of William Shakespeare](#the-complete-works-of-william-shakespeare)
        + [300k+ Tokyo 2020/21 Olympics Tweets](#300k+-tokyo-2020/21-olympics-tweets)
        + [3.5M Tiktok Google Play Reviews](#35m-tiktok-google-play-reviews)
        + [3.6M Amazon Reviews And Ratings](#36m-amazon-reviews-and-ratings)
    * [License](#license)
    * [Acknowledgement](#acknowledgement)

## About

Have you ever wanted to uwu'ify text and large files at blazing fast speeds? Well! You came to the right
place! `uwuifyy`
allows you to _uwu'ify_ text and files from within your own terminal in an _extremely_ fast manner!

## Features

- [x] Seeded by default to allow for reproducible results
- [x] UwU'ify files available locally on disk
- [x] Completely customisable modifiers
- [x] Excludes URL's & Emails

### Modifiers

- **words:** The modifier to determine how many words to be uwu'ified
- **faces:** The modifier for uwu faces e.g hello -> hewwo
- **actions:** The modifier for actions e.g *shuffles over*
- **stutters:** The modifier for stutters e.g b-baka!
- **random:** Flag to enable/disable random uwu'ifying

## Installation

Through the use of Cargo:
```commandline
cargo install uwuifyy
```

**Binaries will be available soon™**

## Quick Start

### Text Input to Text Output

Usage:

```commandline
C:\Your\Path\Here> uwuifyy --text "According to all known laws of aviation, there is no way a bee should be able to fly."
> Accowding to aww knyown waws of aviation, thewe xDD is nyo way :3 a bee shouwd be abwe to *screams* fwy.
```

### Text Input to File Output

Usage:

```commandline
C:\Your\Path\Here> uwuifyy --text "According to all known laws of aviation, there is no way a bee should be able to fly." --outfile your/path/here/outfile.txt
  [00:00:00] [############################################################] 104B/104B (0s) UwU'ifying Complete!
```

Outfile:

```text
your/path/here/outfile.txt
-------------------------------------
Accowding to aww knyown waws of aviation, thewe xDD is nyo way :3 a bee shouwd be abwe to *screams* fwy.
```

### File Input to File Output

Infile:

```text
your/path/here/infile.txt
-------------------------------------
According to all known laws of aviation, there is no way a bee should be able to fly.
```

Usage:

```commandline
PS D:\Programming\Personal\uwuifyy> uwuifyy --infile your/path/here/infile.txt --outfile your/path/here/outfile.txt                                                                           
  [00:00:00] [############################################################] 85B/85B (0s) UwU'ifying Complete! 
```

Outfile:

```text
your/path/here/outfile.txt
-------------------------------------
Accowding to aww knyown waws of aviation, thewe xDD is nyo way :3 a bee shouwd be abwe to *screams* fwy.
```

## Demos

- Examples carried out on a `Dell XPS 15`
    - CPU: `11th Gen Intel(R) Core(TM) i7-11800H @ 2.30GHz`
    - SSD: `NVMe Micron 2300 1TB`

### The Complete Works of William Shakespeare

- [Dataset](https://www.kaggle.com/kewagbln/shakespeareonline)
- Size: `5.46 MiB`
- Time Taken: `1s`

![](examples/gifs/william-shakespeare.gif)

### 300k+ Tokyo 2020/21 Olympics Tweets

- [Dataset](https://www.kaggle.com/amritpal333/tokyo-olympics-2021-tweets)
- Size: `98.54 MiB`
- Time Taken: `8s`

![](examples/gifs/tokyo-2020-olympics-tweets.gif)

### 3.5M Tiktok Google Play Reviews

- [Dataset](https://www.kaggle.com/shivamb/35-million-tiktok-mobile-app-reviews)
- Size: `543.02 MiB`
- TIme Taken: `38s`

![](examples/gifs/tiktok_app_reviews.gif)

### 3.6M Amazon Reviews And Ratings

- [Dataset](https://www.kaggle.com/bittlingmayer/amazonreviews?select=train.ft.txt.bz2)
- Size: `1.6 GiB`
- Time Taken: `2:16m`
- **DISCLAIMER:** The input is a 1.6GB file and the output is a 2GB file. They are not included in the repo.

![](examples/gifs/amazon-ratings-reviews.gif)

## License

[MIT License](LICENSE)

## Acknowledgement

This project is inspired from one of many existing normal to uwu converters:
[Uwuifier](https://github.com/Schotsl/Uwuifier-node)