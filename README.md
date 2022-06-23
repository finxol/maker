# Maker

Easily build all java files of a project.

Maker is build to work with JavaFX and JUnit for unit testing.
It is tested on Linux and Windows.

Initially made for [augustin-pasq/SAE-PNR](https://github.com/augustin-pasq/SAE-PNR)

## Usage

```bash
# Build and run the application
./maker run [--verbose, --file <file>]

# Only build the application
./maker build [--verbose]

# Generate the documentation
./maker doc

# Run JUnit tests
./maker test

# List all available commands
./maker help
```

## Download pre-built binaries

You can find prebuilt binaries for Linux and Windows in the [releases'](https://github.com/finxol/maker/releases) section on github.

## Build from source

To compile Maker, simply run
```bash
cargo build --release
```

The executable for your computer's architecture should now be in `target/release/maker`
