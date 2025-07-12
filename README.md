# Path Offset

A simple Rust library for offsetting paths, leveraging other powerful libraries for robust geometric calculations.

## Features

- **Path Offsetting**: Easily offset complex paths.
- **Multiple Backends**: Choose between `flo_curves` and `cavalier_contours` for the offsetting algorithm.
- **Path Utilities**: Includes utilities for path manipulation, such as finding the outer shell of a complex path.
- **SVG Path Support**: Parse SVG path data and convert paths back to SVG path strings.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
path-offset = "0.1.1"
```

### Offsetting a Path

```rust
use path_offset::offset::Offset;
use path_offset::path::Path;
use std::str::FromStr;

let path = Path::from_str("M10,10 L20,10 L20,20 L10,20 Z").unwrap();

// Use one of the available offsetters
let offsetter = path_offset::offset::cavalier_contours::CavalierContours::new(1.0);
let offset_path = offsetter.offset_path(&path).unwrap();

println!("Offset path: {}", offset_path);
```

### Finding the Outer Shell

```rust
use path_offset::path::Path;
use std::str::FromStr;

let path = Path::from_str("M0,0 L10,0 L10,10 L0,10 Z M2,2 L8,2 L8,8 L2,8 Z").unwrap();
let outer_shell = path.find_outer_shell().unwrap();

println!("Outer shell: {}", outer_shell);
```

## Backends

This library uses the following backends for path offsetting:

- **`flo_curves`**: A robust library for path and curve manipulation.
- **`cavalier_contours`**: A fast and reliable library for path offsetting.

You can choose the backend that best suits your needs.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

This project is licensed under the Apache-2.0 License.
