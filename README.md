# V_name_generator

## What is this?

V_name_generator is a program that generates short names starting with V.

Implements a "Las Vegas" algorithm under the hood and gives you a valid creation.

It can be used from the cli, as library for other rust projects or to send a ntfy notification.

## Why use it?

Have you ever been in trouble trying to think of a short, v-starting name?

V_name_generator gives you all the procedurally generated names that you want without thinking for a second.

You will only need to pick the best one.

## Installation

You can install V_name_generator using pre-built binaries or compile it from source.

### Automated Install (Recommended)

#### Linux & macOS

Run the following in your terminal to download and install the latest release

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/000Volk000/v_name_generator/releases/latest/download/v_name_generator-installer.sh | sh
```

#### Windows

Run this in PowerShell

```PowerShell
irm https://github.com/000Volk000/v_name_generator/releases/latest/download/v_name_generator-installer.ps1 | iex
```

### Package Managers

#### Homebrew

```bash
brew install 000Volk000/v_name_generator/v_name_generator
```

### From Source

If you have [Rust](https://rust-lang.org/) installed, you can get it directly from [crates.io](https://crates.io/):

```bash
cargo install v_name_generator
```

Or build it from the repository:

```bash
git clone https://github.com/000Volk000/v_name_generator.git
cd v_name_generator
cargo install --path .
```

## Usage

There are 3 main ways of using it:

### CLI Generation

For this you will only need to type this onto your terminal.

```bash
v_name_generator
```

And you will get a name in the same terminal.

### NTFY notification

You shall execute it with the "ntfy" argument, having set the enviroment variable NTFY_POST_URL to the desired ntfy URL;

#### Linux & macOS

```bash
NTFY_POST_URL="https://ntfy.sh/*********" v_name_generator ntfy
```

#### Windows

```PowerShell
& { $env:NTFY_POST_URL = "https://ntfy.sh/*********"; v_name_generator ntfy }
```

### Library for other rust projects

In the desired project you shall execute.

```bash
cargo add v_name_generator
```

Then you can add the following dependency to your code and call it how you want.

```rust
use v_name_generator::generate_valid_name;

fn main() {
  let generated_name = generate_valid_name();
}
```

## License

Created under the MIT License. See [LICENSE](https://github.com/000Volk000/v_name_generator/blob/main/LICENSE) for more information.

Created by [Darío Martínez Kostyuk](https://linktree.volkhost.es/) - 2026
