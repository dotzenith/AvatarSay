<h1 align="center"> ━━━━━━  ❖  ━━━━━━ </h1>

<!-- BADGES -->
<div align="center">
   <p></p>

   <img src="https://img.shields.io/github/stars/dotzenith/AvatarSay?color=F8BD96&labelColor=302D41&style=for-the-badge">

   <img src="https://img.shields.io/github/forks/dotzenith/AvatarSay?color=DDB6F2&labelColor=302D41&style=for-the-badge">

   <img src="https://img.shields.io/github/repo-size/dotzenith/AvatarSay?color=ABE9B3&labelColor=302D41&style=for-the-badge">

   <img src="https://img.shields.io/github/commit-activity/y/dotzenith/AvatarSay?color=96CDFB&labelColor=302D41&style=for-the-badge&label=COMMITS"/>
   <br>
</div>

<p/>

---

## ❖ AvatarSay

Beautiful quotes from Avatar: The Last Airbender, right in your terminal

  <img src="https://github.com/dotzenith/dotzenith/blob/main/assets/AvatarSay/quotes.gif" alt="quotes gif">

Note:

AvatarSay uses [viuer](https://github.com/atanunq/viuer) to display the images, but it does not use the [sixel](https://github.com/saitoha/libsixel) feature.

This means it only supports the [kitty](https://sw.kovidgoyal.net/kitty/graphics-protocol/) and [iTerm](https://iterm2.com/documentation-images.html) protocols.

AvatarSay was tested on the following terminal emulators:

- [Kitty](https://sw.kovidgoyal.net/kitty/)
- [WezTerm](https://wezfurlong.org/wezterm/index.html)
- [iTerm](https://iterm2.com/)

---

## ❖ Installation

#### Shell
```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/dotzenith/AvatarSay/releases/latest/download/avatarsay-installer.sh | sh
```

#### Brew
```sh
brew tap dotzenith/tap
brew install avatarsay
```

#### Powershell
```sh
irm https://github.com/dotzenith/AvatarSay/releases/latest/download/avatarsay-installer.ps1 | iex
```

#### Cargo
```sh
cargo install avatarsay
```

#### Binaries
Pre-Compiled binaries for linux, mac, and windows are available in [Releases](https://github.com/dotzenith/AvatarSay/releases)

#### Source
- First, install [rust](https://rustup.rs/)
```sh
git clone https://github.com/dotzenith/AvatarSay.git
cd AvatarSay
cargo build --release
./target/release/avatarsay
```

---

## ❖ Usage

```
Beautiful quotes from Avatar: The Last Airbender

Usage: avatarsay <COMMAND>

Commands:
  random     Get a random quote
  character  Get a quote from a specfic character
  nation     Get a quote from a character from a specfic nation
  bending    Get a quote from a character with specfic bending ability
  episode    Get a quote from a specfic episode
  book       Get a quote from a specfic book
  valid      Get all valid inputs for any given filter above
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

---

## ❖ What's New?

0.1.0 - Initial Release

---

<div align="center">

   <img src="https://img.shields.io/static/v1.svg?label=License&message=MIT&color=F5E0DC&labelColor=302D41&style=for-the-badge">

</div>
