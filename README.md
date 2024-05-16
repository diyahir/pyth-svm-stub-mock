# Pyth-SVM-Stub-Mock

Pyth-Stub is a Solana program that allows you to write and read mock price data to a Solana account using the Pyth Network's price feed format.

## Getting Started

These instructions will help you set up the project on your local machine for development and testing purposes.

### Prerequisites

Ensure you have the following software installed:

- [Node.js](https://nodejs.org/) (v14 or later)
- [Yarn](https://yarnpkg.com/)
- [Rust](https://www.rust-lang.org/tools/install)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor](https://project-serum.github.io/anchor/getting-started/installation.html)

### Installation

1. Clone the repository:

   ```sh
   git clone https://github.com/diyahir/pyth-svm-stub-mock.git
   cd pyth-stub
   ```

2. Install Node.js dependencies:

   ```sh
   yarn install
   ```

### Building the Program

Compile the Solana program to ensure everything is set up correctly.

```sh
anchor build
```

### Deploying the Program

Deploy the Solana program to the local network or to the desired Solana cluster (devnet, testnet, or mainnet).

```sh
anchor deploy
```

### Running Tests

Run the tests to ensure the program works as expected.

```sh
anchor test
```

## Usage

Here is a brief overview of the main commands used in this project:

- `anchor build`: Compiles the Solana program.
- `anchor deploy`: Deploys the compiled program to the specified Solana cluster.
- `anchor test`: Runs the test suite to verify the functionality of the program.
