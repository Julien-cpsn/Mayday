MAYDAY 🗣️✉️
===

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![GitHub Release](https://img.shields.io/github/v/release/julien-cpsn/mayday?link=https%3A%2F%2Fgithub.com%2FJulien-cpsn%2FmaydayC%2Freleases%2Flatest)
[![Crates.io](https://repology.org/badge/version-for-repo/crates_io/mayday.svg)](https://crates.io/crates/mayday)

A multi-messaging-sevice aggregator into an all-in-one application (android's app beeper-like).
The goal is to extend the supported messaging services by writing and sharing your drivers!

Please fully read [How to use](#how-to-use)

Currently supported:
- [x] loopback
- [x] IRC
- [x] ChatGPT
- [ ] Telegram
- [ ] Messenger
- [ ] Matrix
- [ ] ...add yours! see [contribute](#contribute)

![demo](./demo.gif)

## How to use

The app is splat in two:
- a TUI client which displays messages
- a worker which receive messages

### Install

```shell
cargo install mayday
```

### Compile

```shell
cargo build
```

```shell
cargo build --release
```

### Run

You can replace `mayday` with `cargo run --`

TUI client

```shell
mayday
```

worker

```shell
mayday worker
```

### Using messaging services

After the first usage, mayday will create a config repository which stores messaging service drivers:
- Linux: `/home/$USER/.config/mayday`
- Windows: `C:\Users\$USER\AppData\Roaming\Julien-cpsn\mayday`
- macOS: `Users/$USER/Library/Application Support/com.Julien-cpsn.mayday`

Add your desired messing service configuration file in this folder. Each service template is available in [`example_resources`](./example_resources).

E.g. for IRC
```toml
uuid = "60de91cf-41fc-48ab-893e-b50ff514a706"
discussion_name = "IRC freenode"

[driver]
type = "irc"
channel = "#rust"
server = "chat.freenode.net"
#port = 6697
nickname = "mayday user"
alt_nicks = ["[mayday-user]"]
```

### Cleaning conversations

After the first usage, mayday will create a cache repository which stores messages history:
- Linux: `/home/$USER/.config/mayday`
- Windows: `C:\Users\$USER\AppData\Roaming\Julien-cpsn\mayday`
- macOS: `Users/$USER/Library/Application Support/com.Julien-cpsn.mayday`

Erasing a file will reset the history.

## Contribute

You can develop a new messaging service driver as follows.

1. Copy `loopback.rs` from `src/drivers` to `src/drivers/<my_awesome_driver>.rs`
2. Add your driver to the `MessagingDriverConfigs` enum in `src/drivers/mod.rs`
3. Fill the copied driver with your code. Keep in mind that:
    - `active_poll_received_messages` is called by the client (so when you are using the app)
    - `passive_poll_received_messages` is called by the background worker
4. Update this README
5. Add a configuration example in [`example_resources`](./example_resources)
5. Create a pull request!

## Star history

<a href="https://www.star-history.com/#julien-cpsn/mayday&Date">
 <picture>
   <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=julien-cpsn/mayday&type=Date&theme=dark" />
   <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=julien-cpsn/mayday&type=Date" />
   <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=julien-cpsn/mayday&type=Date" />
 </picture>
</a>

## License

The MIT license for this project can be seen [here](./LICENSE)