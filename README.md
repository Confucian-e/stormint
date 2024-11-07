# Stormint

A blazing fast script that supports multi-account minting of FreeMint tokens similar to [$Daram](https://x.com/daram010).

## Features

- Multi-account generation
- Automated distribution of gas (like ether)
- Batch accounts token minting

## Installation

Clone the repository:

```bash
git clone https://github.com/Confucian-e/stormint.git
```

Navigate to the project directory:

```bash
cd stormint
```

Install the required dependencies:

### Rust

Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed. Then, run:

```bash
cargo build --release
```

### Contracts

Ensure you have [Foundry](https://getfoundry.sh/) installed. Then, run:

```bash
git submodule update --init --recursive

cd contracts/
forge soldeer install
forge build
```

### Testing

To run the tests, use the following command:

```bash
cargo test
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.
