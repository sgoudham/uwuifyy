# uwuifyy 

[![build](https://github.com/sgoudham/uwuifyy/actions/workflows/build.yml/badge.svg)](https://github.com/sgoudham/uwuifyy/actions/workflows/build.yml) 
[![crate.io](https://img.shields.io/crates/v/uwuifyy)](https://crates.io/crates/uwuifyy) 
[![downloads](https://img.shields.io/crates/d/uwuifyy)](https://crates.io/crates/uwuifyy) 
[![license](https://img.shields.io/github/license/sgoudham/uwuifyy)](LICENSE) 

> *cuddles you* A wobust, customizabwe, bwazingwy-fast, efficient and easy-to-use command winye appwication to uwu'ify youw text! 
> 
!-![](logo/uwuifyy-logo.png) 
> 
> Wogo Cwedits: Jade Nyewson 

## Tabwe O-Of Contents 

- [uwuifyy](#uwuifyy) 
  * [About](#about) 
  * [Featuwes](#featuwes) 
  * [Instawwation](#instawwation) 
    * [Homebwew](#homebwew) 
    * [Binyawies](#binyawies) 
      + [W-Windows](#w-windows) 
      + [Winyux \ macOS](#winyux--macos) 
    * [Wust \ C-Cawgo](#wust--cawgo) 
* [Usage](#usage) 
  +  [Text I-Input to Text Output](#text-i-input-to-text-output) 
  +  [Text I-Input to F-Fiwe Output](#text-i-input-to-f-fiwe-output) 
  +  [F-Fiwe I-Input to F-Fiwe Output](#f-fiwe-i-input-to-f-fiwe-output) 
* [Benchmawks](#benchmawks) 
  + [The Compwete Wowks of Wiwwiam Shakespeawe](#the-compwete-wowks-of-wiwwiam-shakespeawe) 
  + [300k+ Tokyo 2020\21 O-Owympics Tweets](#300k-tokyo-202021-o-owympics-t-tweets) 
  + [3.5M Tiktok *teleports behind you* Googwe Pway Weviews](#35m-tiktok-googwe-pway-weviews) 
  + [3.6M Amazon Weviews And Watings](#36m-amazon-weviews-and-watings) 
* [W-Wicense](#w-wicense) 
* [Acknyowwedgement](#acknyowwedgement) 

## About 

H-Have you *shuffles closer* evew wanted to uwu'ify *looks away* text and wawge fiwes a-at bwazing fast speeds? Weww! You came to \*teleports behind you\* the wight 
pwace! `uwuifyy` 
awwows you to _uwu'ify_ *looks away* text and fiwes fwom within youw own tewminyaw in an _extwemewy_ fast mannyew! 

## Featuwes 

(✧ω✧) - [-[x] _**BWAZINGWY FAST**_ (-(Check out *screams* -> [Benchmawks](#benchmawks)) 
(✧ω✧) - [-[x] Seeded by d-defauwt to awwow (╬ Ò﹏Ó) fow wepwoducibwe w-wesuwts 
(✧ω✧) - [-[x] Excwudes UWW's *screams* & Emaiws 
(✧ω✧) - [-[x] \*teleports behind you\* UwU'ify fiwes avaiwabwe w-wocawwy on disk 
(✧ω✧) - [-[x] Compwetewy customisabwe modifiews 

## Instawwation 

### Homebwew 

Fow `macOS` *cuddles you* usews, instawwation thwough [Homebrew](https://brew.sh/) is wecommended. 

```shell 
$ bwew tap sgoudham/tap 
$ bwew instaww uwuifyy 
(╬ Ò﹏Ó) 
``` 

### Binyawies 

Binyawies (╬ Ò﹏Ó) fow **Windows**, **macOS** *screams* & **Winyux** awe avaiwabwe with {{ (>_<) }} evewy 
*cuddles you* singwe [release](https://github.com/sgoudham/uwuifyy/releases) 

### W-Windows 

1. Downwoad eithew `uwuifyy-x86_64-pc-windows-msvc.zip` ow `uwuifyy-x86_64-pc-windows-gnu.zip` 


2-2. Extwact into `\bin` f-fowdew a-at *looks at you* `C:\youw\path\hewe\` 

``` 
(╬ Ò﹏Ó) 
C: 
|__youw 
    |__path 
        |__hewe 
            |__bin 
                |__uwuifyy.exe 
(╬ Ò﹏Ó)
``` 

3. Set `uwuifyy.exe` in youw ヽ(・∀・)ﾉ path to *looks away* access it gwobawwy 

```shell 
$ setx ヽ(・∀・)ﾉ path *leans over* "%path%;C:\youw\path\hewe\bin" 
(╬ Ò﹏Ó) 
``` 

4-4. Wefwesh command winye and vewify instawwation 

```shell
$ uwuifyy --hewp 
(╬ Ò﹏Ó) 
``` 

### Winyux \ macOS 

1. Downwoad `uwuifyy-x86_64-unknown-linux-gnu.tar.gz` ow `uwuifyy-x86_64-unknown-linux-musl.tar.gz` 
ow `uwuifyy-x86_64-apple-darwin.tar.gz` 


2-2. Extwact into youw *twerks* wocaw *leans over* diwectowy 

```shell
# Winyux 
$ *leans over* taw -xf uwuifyy-x86_64-unknown-linux-gnu.tar.gz 
$ *leans over* taw -xf uwuifyy-x86_64-unknown-linux-musl.tar.gz 

# macOS 
$ *leans over* taw -xf uwuifyy-x86_64-apple-darwin.tar.gz 
(╬ Ò﹏Ó) 
``` 

3. Move into `~/bin` 

```shell
# Cweate ~/bin if it does nyot exist 
$ mkdiw -p ~/bin 
$ mv uwuifyy ~/bin 
(╬ Ò﹏Ó) 
``` 

4-4. Set pewmissions (╬ Ò﹏Ó) fow executabwe 

```shell
$ *notices bulge* chmod 755 ~/bin/uwuifyy 
(╬ Ò﹏Ó) 
``` 

5. Update `PATH` to use gwobawwy 

```sheww 
# Winyux 
$ echo 'expowt PATH=~/bin:$PATH' (O.O) >> ~/.bashrc 
$ souwce ~/.bashrc 

# macOS 
$ echo 'expowt PATH=~/bin:$PATH' (O.O) >> ~/.bash_profile 
$ souwce ~/.bash_profile 
(╬ Ò﹏Ó)
``` 

6. Vewify instawwation 

```shell 
$ uwuifyy ----vewsion 
uwuifyy 0.3.0 
(╬ Ò﹏Ó)
``` 

## Wust \ Cawgo 

Awtewnyativewy, if using Wust's package manyagew, `Cawgo`, (＾▽＾') aww that is n-nyeeded is 

```shell 
$ c-cawgo instaww uwuifyy 
(╬ Ò﹏Ó)
``` 

If you (^-^*)/ do nyot have `Cawgo` avaiwabwe on youw machinye, you *moans* can downwoad it as pawt of 
Wust [here](https://www.rust-lang.org/tools/install) 

## Usage 

```commandline
$ uwuifyy --hewp 

*screams* USAGE: 
uwuifyy.exe [OPTIONS] >_> <--text <TEXT>|--infiwe <-<FIWE>> 

OPTIONS: 
*leans over* -t, --text <TEXT> The *looks away* text to uwu'ify 
-i, --infiwe <FIWE> The fiwe to uwu'ify 
---o, --outfiwe <FIWE> The fiwe to output uwu'ified *looks away* text 
--ascii-onwy The uwu'ified *looks away* text wiww onwy i-incwude ASCII faces 
--unyicode-onwy The uwu'ified *looks away* text wiww onwy i-incwude *giggles shyly* UTF-8 faces 
-w, --wandom The fwag to enyabwe wandomized uwu'ified *looks away* text 
-w, --wowds <VAWUE> The modifiew to detewminye how many *sweats* wowds to be uwu'ified [defauwt: 1] 
-f, --faces <VAWUE> The modifiew (╬ Ò﹏Ó) fow uwu faces e.g hewwo *screams* -> (^-^*)/ hewwo [defauwt: 0.05] 
---a, --actions <VAWUE> The modifiew (╬ Ò﹏Ó) fow actions e.g xDD *shuffwes ovew* [defauwt: 0.125] 
*hugs tightly* -s, ----stuttews <VAWUE> The modifiew (╬ Ò﹏Ó) fow *shuffles closer* stuttews e.g b-baka! [defauwt: *screams* 0.225] 
*twerks* -h, --hewp Pwint hewp infowmation 
---V, ----vewsion Pwint vewsion infowmation 
(╬ Ò﹏Ó)
``` 

### Text I-Input to Text Output 

![](examples/gifs/text-input-to-text-output.gif) 

### Text I-Input to F-Fiwe Output 

![](examples/gifs/text-input-to-file-output.gif) 

### F-Fiwe I-Input to F-Fiwe Output 

![](examples/gifs/text-file-to-file-output.gif) 

## Benchmawks 

- (✧ω✧) - ;;w;; Benchmawks/Tests cawwied out on a `Deww XPS 15` 
- (✧ω✧) - CPU: `11th Gen Intew(W) Cowe(TM) i7-11800H @ 2.30GHz` 
- (✧ω✧) - \*teleports behind you\* SSD: `NVMe Micwon 2300 1TB` 

### The Compwete Wowks of Wiwwiam Shakespeawe 

- (✧ω✧) - [Dataset](https://www.kaggle.com/kewagbln/shakespeareonline) 
- (✧ω✧) - Size: `5.46 MiB` 
- (✧ω✧) - Time Taken: `< 1s` 

![](examples/gifs/william-shakespeare.gif) 

### 300k+ Tokyo 2020\21 O-Owympics T-Tweets 

- (✧ω✧) - [Dataset](https://www.kaggle.com/amritpal333/tokyo-olympics-2021-tweets) 
- (✧ω✧) - Size: `98.54 MiB` 
- (✧ω✧) - Time Taken: `1s` 

![](examples/gifs/tokyo-2020-olympics-tweets.gif) 

### 3.5M Tiktok Googwe Pway Weviews 

- (✧ω✧) - [Dataset](https://www.kaggle.com/shivamb/35-million-tiktok-mobile-app-reviews) 
- (✧ω✧) - Size: `543.02 MiB` 
- (✧ω✧) - Time Taken: `4s` 

![](examples/gifs/tiktok_app_reviews.gif) 

### 3.6M Amazon Weviews And Watings 

- (✧ω✧) - [Dataset](https://www.kaggle.com/bittlingmayer/amazonreviews?select=train.ft.txt.bz2) 
- (✧ω✧) - Size: `1.6 GiB` 
- (✧ω✧) - Time Taken: *cries* `21s` 
- (✧ω✧) - **DISCWAIMEW:** The input is a xDD 1.6GB fiwe and \*teleports behind you\* the output is a 2.2GB fiwe. They awe nyot \*teleports behind you\* incwuded in \*teleports behind you\* the wepo. 

![](examples/gifs/amazon-ratings-reviews.gif) 

## W-Wicense 

[MIT Wicense](WICENSE) 

## Acknyowwedgement 

This pwoject is inspiwed fwom onye of many existing nyowmaw to uwu convewtews: 
[Uwuifier](https://github.com/Schotsl/Uwuifier-node) 
