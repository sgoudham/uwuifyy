# uwuifyy

[![build](https://github.com/sgoudham/uwuifyy/actions/workflows/build.yml/badge.svg)](https://github.com/sgoudham/uwuifyy/actions/workflows/build.yml)
[![crate.io](https://img.shields.io/crates/v/uwuifyy)](https://crates.io/crates/uwuifyy)
[![downloads](https://img.shields.io/crates/d/uwuifyy)](https://crates.io/crates/uwuifyy)
[![license](https://img.shields.io/github/license/sgoudham/uwuifyy)](LICENSE)

> *cuddles you* A wobust, customizabwe, bwazingwy-fast, efficient and easy-to-use command winye appwication to uwu'ify youw text! 
>
![](logo/uwuifyy-logo.png)
>
> Wogo Cwedits: Jade Nyewson 

## Tabwe O-Of Contents 

- [uwuifyy](#uwuifyy)
    * [About](#about)
    * [Featuwes](#featuwes)
    * [Instawwation](#instawwation)
        * [Homebwew](#homebwew)
        * [Binyawies](#binyawies)
            + [Windows](#windows)
            + [Winyux \ macOS](#winyux-\-macos)
        * [Wust \ Cawgo](#wust-\-cawgo)
    * [Usage](#usage)
        + [Text I-Input to Text Output ](#text-i-input-to-text-output)
        + [Text I-Input to F-Fiwe Output ](#text-i-input-to-f-fiwe-output)
        + [Fiwe I-Input to F-Fiwe Output](#fiwe-i-input-to-f-fiwe-output) 
    * [Benchmawks](#benchmawks) 
        + [The Compwete Wowks of Wiwwiam Shakespeawe](#the-compwete-wowks-of-wiwwiam-shakespeawe)
        + [300k+ Tokyo 2020\21 O-Owympics Tweets](#300k-tokyo-202021-o-owympics-tweets) 
        + [3.5M Tiktok Googwe Pway Weviews](#35m-tiktok-googwe-pway-weviews) 
        + [3.6M Amazon Weviews And Watings](#36m-amazon-weviews-and-watings)
    * [Wicense](#w-wicense) 
    * [Acknyowwedgement](#acknyowwedgement) 

## About

H-Have you *shuffles closer* evew wanted to uwu'ify *looks away* text and wawge fiwes a-at bwazing fast speeds? Weww! You came to *teleports behind you* the wight pwace! `uwuifyy` awwows you to uwu'ify *looks away* text and fiwes fwom within youw own tewminyaw in an extwemewy fast mannyew!

## Featuwes 

- [x] _**BWAZINGWY FAST**_ (Check out -> [Benchmawks](#Benchmawks))
- [x] Seeded by d-defauwt to awwow (╬ Ò﹏Ó) fow wepwoducibwe w-wesuwts 
- [x] Excwudes UWW's *screams* & Emaiws 
- [x] *teleports behind you* UwU'ify fiwes avaiwabwe w-wocawwy on disk 
- [x] Compwetewy customisabwe modifiews 

## Instawwation 

### Homebwew 

Fow *cuddles you* `macOs` usews, instawwation thwough [Homebwew](https://brew.sh/) is wecommended. 

```shell
$ brew tap sgoudham/tap
$ brew install uwuifyy
```

### Binyawies 

Binyawies (╬ Ò﹏Ó) fow **Windows**, **macOS** *screams* & **Winyux** awe avaiwabwe with {{ (>_<) }} evewy 
*cuddles you* singwe [release](https://github.com/sgoudham/uwuifyy/releases) 

### Windows

1. Downwoad eithew `uwuifyy-x86_64-pc-windows-msvc.zip` ow `uwuifyy-x86_64-pc-windows-gnu.zip` 


2. Extwact into `\bin` f-fowdew a-at *looks at you*`C:\your\path\here\`

```
C:
|__your
    |__path
        |__here
            |__bin
                |__uwuifyy.exe
```

3. Set `uwuifyy.exe` in youw ヽ(・∀・)ﾉ path to *looks away* access it gwobawwy

```shell
$ setx path "%path%;C:\your\path\here\bin"
```

4. Wefwesh command winye and vewify instawwation 

```shell
$ uwuifyy --help
```

### Winyux \ macOS

1. Downwoad `uwuifyy-x86_64-unknown-linux-gnu.tar.gz` ow `uwuifyy-x86_64-unknown-linux-musl.tar.gz` 
ow `uwuifyy-x86_64-apple-darwin.tar.gz` 



2. Extwact into youw *twerks* wocaw *leans over* diwectowy 

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

4. Set pewmissions (╬ Ò﹏Ó) fow executabwe 

```shell
$ chmod 755 ~/bin/uwuifyy
```

5. Update `PATH` to use gwobawwy 

```shell
# Linux
$ echo 'export PATH=~/bin:$PATH' >> ~/.bashrc 
$ source ~/.bashrc

# macOS
$ echo 'export PATH=~/bin:$PATH' >> ~/.bash_profile
$ source ~/.bash_profile
```

6. Vewify instawwation 

```shell
$ uwuifyy --version
uwuifyy 0.3.0
```

## Wust \ Cawgo

Awtewnyativewy, if using Wust's package manyagew, `Cawgo`, (＾▽＾') aww that is n-nyeeded is

```shell
$ cargo install uwuifyy
```

If you (^-^*)/ do nyot have `Cawgo` avaiwabwe on youw machinye, you *moans* can downwoad it as pawt of 
Wust [here](https://www.rust-lang.org/tools/install) 

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

### Text I-Input to Text Output 

![](examples/gifs/text-input-to-text-output.gif)

### Text I-Input to F-Fiwe Output 

![](examples/gifs/text-input-to-file-output.gif)

### Fiwe I-Input to F-Fiwe Output

![](examples/gifs/text-file-to-file-output.gif)

## Benchmawks

- Benchmawks/Tests cawwied out on a `Deww XPS 15`
    - CPU: `11th Gen Intew(W) Cowe(TM) i7-11800H @ 2.30GHz` 
    - SSD: `NVMe Micwon 2300 1TB`

### The Compwete Wowks of Wiwwiam Shakespeawe 

- [Dataset](https://www.kaggle.com/kewagbln/shakespeareonline)
- Size: `5.46 MiB`
- Time Taken: `< 1s`

![](examples/gifs/william-shakespeare.gif)

### 300k+ Tokyo 2020\21 O-Owympics Tweets

- [Dataset](https://www.kaggle.com/amritpal333/tokyo-olympics-2021-tweets)
- Size: `98.54 MiB`
- Time Taken: `1s`

![](examples/gifs/tokyo-2020-olympics-tweets.gif)

### 3.5M Tiktok Googwe Pway Weviews

- [Dataset](https://www.kaggle.com/shivamb/35-million-tiktok-mobile-app-reviews)
- Size: `543.02 MiB`
- TIme Taken: `4s`

![](examples/gifs/tiktok_app_reviews.gif)

### 3.6M Amazon Weviews And Watings

- [Dataset](https://www.kaggle.com/bittlingmayer/amazonreviews?select=train.ft.txt.bz2)
- Size: `1.6 GiB`
- Time Taken: `21s`
- **DISCWAIMEW:** The input is a xDD 1.6GB fiwe and *teleports behind you* the output is a 2.2GB fiwe. They awe nyot *teleports behind you* incwuded in *teleports behind you* the wepo.

![](examples/gifs/amazon-ratings-reviews.gif)

## W-Wicense 

[MIT Wicense](LICENSE)

## Acknyowwedgement

This pwoject is inspiwed fwom onye of many existing nyowmaw to uwu convewtews: 
[Uwuifier](https://github.com/Schotsl/Uwuifier-node) 
