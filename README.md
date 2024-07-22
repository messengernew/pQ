# Package Updater

A command-line application for updating AUR packages and system packages on Arch Linux using `pacman` and `aura`. This tool checks for package updates, manages the upgrade process.

## Features

- **Update AUR Packages:** Check for and update packages from the AUR.
- **Update System Packages:** Check for and update system packages using `pacman`.
- **Batch Processing:** Send requests to AUR API in batches to avoid `414 Request-URI Too Large` errors.

## Installation

1. **Clone the Repository:**

   ```bash
   mkdir pQ && cd pQ && \
   curl https://raw.githubusercontent.com/messengernew/pQ/master/PKGBUILD -o PKGBUILD
   ```

2. **Build the Application:**

   Ensure you have Rust installed. Build the project using Cargo:

   ```bash
   makepkg -sic
   ```

3. **Run the Application:**

   You can run the application directly from the build directory:

   ```bash
   pq
   ```

## Configuration

The application reads configuration from `/etc/pacman.conf` to determine system repositories. Ensure this file is correctly configured for your system.

## Usage

To update AUR and system packages, simply execute the application:

```bash
pq
```

The application will:

1. Check for updates in the AUR.
2. Check for updates in system repositories.
3. Update packages.

## Contributing

Feel free to contribute by opening issues or submitting pull requests. Ensure your changes are tested and adhere to the project's coding standards.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.