# Seneca
Seneca is a mail client built on top of the [notmuch](https://notmuchmail.org/) email indexing tool. It is written in Rust using the [Druid](https://github.com/linebender/druid) GUI toolkit.

## Installation
Building seneca will require the notmuch libraries for your platform. See [here](https://notmuchmail.org/#index7h2) for the right package for your particular environment. You will also need whatever the Druid requirements are for your platform. See the notes [here](https://github.com/linebender/druid#platform-notes) for details.

Once all of that is in place, clone this repo and execute `cargo install --path .` to build and install seneca to your cargo `bin` directory. Alternately, execute `cargo build --release` and copy the resulting executable from `target/release/seneca` to wherever in your path you desire.

## Configuration
Seneca expects a config file in `$XDG_CONFIG_HOME/seneca/` on Linux and within `Library/Application Support/seneca/` on Mac called `config.toml`. There is an example config file in the `assets/` directory of this repo.

## Contributing
This project follows the [Mozilla Community Participation Guidelines](https://www.mozilla.org/en-US/about/governance/policies/participation/). If you see violations of those guidelines occurring in this project in any way, please inform ross@rbs.io.