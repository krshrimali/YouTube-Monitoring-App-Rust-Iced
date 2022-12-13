## YouTube Monitoring App (using Rust)

![Workflow Status](https://github.com/krshrimali/YouTube-Monitoring-App-Rust-Iced/actions/workflows/build-yt-monitor.yml/badge.svg)

## Description

This app is built on the top of [iced library](https://github.com/iced-rs/iced). If you're curious what this is about, check out the [YT monitoring App stream series](https://www.youtube.com/playlist?list=PLfjzHJeA53gS-RyxHcpNdf85Q4tR_ZJ6_) on my [YouTube channel](https://youtube.com/c/kushashwaraviShrimali).

I have written a blog: [I started building an app using Rust and here is how it went…](https://krshrimali.github.io/posts/2022/12/i-started-building-an-app-using-rust-and-here-is-how-it-went.../) on my experience developing this app so far.

**What will this app do, when ready?**

1. Allow users to choose their favorite 12 creators.
2. Monitor the following for chosen creators:
    * Their live status.
    * Watching of the stream.
    * Live count of subscribers.
3. The app view will be dynamic based on their live status (including appearance)

**Why is this needed?**

* Not always you are notified by YouTube (on time) when a streamer goes live.
* Developing an app with dynamic views, notifications, JSON parsing, is a good learning experience for me.
* Iced is a rapidly growing GUI library written in Rust, and I wanted to explore it.

## Instructions

Assuming you have `cargo` installed and rust setup, following Instructions should work:

```bash
cargo build
cargo run --release
```

For development, please ensure that all the tests pass when and if you create a PR using: `cargo test`. If you are on Linux, you might have to install some extra dependencies before doing `cargo run --release`:

```bash
sudo apt update
sudo apt install build-essential
sudo apt install cmake, pkg-config
sudo apt install fontconfig libfontconfig-dev
```

## Demo

![image](https://user-images.githubusercontent.com/19997320/206906945-a6df5f53-8db7-4680-9e19-5f7cccea7736.png)

**NOTE:** Another version of this project is present [here](https://github.com/krshrimali/youtuber-monitoring-app).
