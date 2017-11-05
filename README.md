# Astral (alpha)
Astronomy toolkit for finding the position of the Sun, Moon, planets and other celestial bodies at a place and time. 

** Accuracy is currently low, though should provide alitude/azimuth values within +-3 degrees from other sources **

## Commands
Most results are relative to a time and location on earth. 

Run `astral <command> --help` for command details

`astral moon` Information on Earth's moon.

`astral sun` Information on Earth's sun, relative to a time and location on earth.

`astral planet mars` Information on planets of the local solar system

`astral star polaris` Information on a star

`astral geocode "Oranjemund,Namibia"` Get Latitude/Longitude Coordinates for location

## Installation

#### Ubuntu
`apt-get install astral`
#### Arch
`apt-get install astral`

#### Mac
`brew install astral`

#### From source
Prerequisites: Rust environment with Cargo package manager([Rustup](https://www.rustup.rs) is recommended)

```
cargo install astral-cli; // installs binary from crates.io
```

or

```
git clone git@github.com:manguluka/astral-cli.git \
cd astral-cli \
cargo install; // installs binary from current directory
```



