# BB84 – Non-Concurrent Simulation

This application simulates the exchange of symmetric keys using the **BB84** protocol according to the provided specifications (no concurrency; the attacker, if enabled, reads **before** the reader). It is written in **Rust**, with one file per object and a `main`. Extensive comments and verbose code.

## Requirements
- Rust (recommended toolchain: stable)
- Cargo
- Visual Studio Code (optional)

## Execution
```bash
cd bb84_simulation
cargo run
```

## Structure
- `src/config.rs` – global constants (e.g., `LUNG_MSG=8`, `ATTIVA_AVVERSARIO=false`).
- `src/public_channel.rs` – public channel: shared vectors and flags, and methods for publishing/reading.
- `src/quantum_channel.rs` – quantum channel: persistent tuple (polarization, value) and `Fotone_IN` flag.
- `src/writer.rs` – writer: initializes message, sends photons, publishes polarizations, builds raw and final key, attacker test.
- `src/reader.rs` – reader: measures photons, sends reading results, builds keys and sends test, final confirmation.
- `src/adversary.rs` – attacker (optional): reads photons before the reader.
- `src/main.rs` – sequential orchestration and printing of required tables.

## Installation
Clone the repository and navigate to the project folder:
```bash
git clone https://github.com/your-username/bb84-non-concurrent.git
cd bb84-non-concurrent
```
Install Rust and Cargo if not already installed:
```bash
rustup install stable
```

## Usage
Run the simulation:
```bash
cargo run
```
Adjust parameters in `src/config.rs` as needed (e.g., message length, attacker activation).

## Contributing
Contributions are welcome! Please fork the repository and submit a pull request with your improvements.

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
