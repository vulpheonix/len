
<div align="center">
<a href="https://github.com/vulpheonix/len/issues">
   	<img src="https://img.shields.io/github/issues/vulpheonix/len"/> 
</a>

<a href="https://github.com/vulpheonix/len/forks">
   	<img src="https://img.shields.io/github/forks/vulpheonix/len"/> 
</a>

<a href="https://github.com/vulpheonix/len/blob/main/LICENSE">
   	<img src="https://img.shields.io/github/license/vulpheonix/len"/> 
</a>

<a href="https://github.com/vulpheonix/len/graphs/contributors">
   	<img src="https://img.shields.io/github/contributors/vulpheonix/len"/> 
</a>


<a href="https://github.com/vulpheonix/len/releases">
   	<img src="https://img.shields.io/github/downloads/vulpheonix/len/total?style=social"/> 
</a>

<a href="https://asciinema.org/a/510956" target="_blank"><img src="https://asciinema.org/a/510956.svg" /></a>
</div>

#### len is a linux command line program to provide file weights right into your terminal

# ![](https://img.icons8.com/external-flaticons-lineal-color-flat-icons/32/000000/external-product-circular-economy-flaticons-lineal-color-flat-icons-4.png) Usage
Run `len -?` or `len --help`, to get a brief idea of what it can do.
```shell
Get file weights right into your Terminal with len
Usage:  len (same as len .)
        len path-to-dir
        len path-to-file
        len wild-card
        len search-text
        len path-to-dir wild-card
        len path-to-dir search-text
brief description
# Using wild-card *
len "*.mp3" //Remember, to wrap your query in double quotes while working with wild card option. Only asterisk(*) is supported at the moment.
# Using search-text
len the //Lists All items in the current directory which have "the" in their names.
For updates, keep a track on https://github.com/vulpheonix/len

```

# ![](https://img.icons8.com/color/32/000000/command.png) Available Command line flags
`len --version` prints version info of current len installation

`len --help` prints a brief description of usage (mention above).

# ![](https://img.icons8.com/fluency/32/000000/get-started-app.png) Installation
The recommended way to install len, is to,
- clone this repo
- and run `install.sh` **script**

OR, you can even grab the latest binary from the [release section](https://github.com/vulpheonix/len/releases)

# ![](https://img.icons8.com/external-flaticons-flat-flat-icons/32/000000/external-build-computer-programming-flaticons-flat-flat-icons.png) Build from source

len only depends on crossterm
```toml
[package]
name = "len"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
crossterm = "0.24.0"

```

## ![](https://img.icons8.com/color/32/000000/idea.png) Have an idea?
- Open up an issue
- Or fork the project and let that pull request rock! 😎


## ![](https://img.icons8.com/emoji/32/000000/smiling-face-with-sunglasses.png) [Happy Hacking!!](https://github.com/omegaui)
