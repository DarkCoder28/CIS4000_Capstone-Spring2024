# Backend Website

This is the webapp that the client runs on as well as the server backend.

## Table of Contents

- [Features](#features)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Configuration](#configuration)
- [Usage](#usage)
- [HTTPS Support](#https-support)
- [Contributing](#contributing)
- [License](#license)

## Features

- Play the game.
- User registration and authentication.

## Getting Started

Follow these instructions to get the project up and running on your local machine.

### Prerequisites

- Rust (v1.72 or higher) (x86_64-unknown-linux-musl toolchain)
- Docker (if using Docker for building/running)
- musl compiler (Arch: `pacman -S musl`; Ubuntu/Debian: `sudo apt update && sudo apt -y install musl-tools`)

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/DarkCoder28/CIS4000_Capstone-Spring2024.git
   cd CIS4000_Capstone-Spring2024/server
   ```

2. Build the project using the provided `build.sh` script:

   ```bash
   ./build.sh
   ```

   Alternatively, you can build and generate a Docker image using the `buildWithDocker.sh` script:

   ```bash
   ./buildWithDocker.sh
   ```

### Configuration

Before running the application, you need to set some environment variables.
- The `MONGODB_URI` environment variable is used to connect to your MongoDB database.
- The `PK_ID` environment variable is used for passkey authentication and should contain your effective domain name.
- The `PK_ORIGIN` environment variable is used for passkey authentication and should contain your base URL, including protocol, domain name, and port.
- The `INSECURE` environment variable will allow the login function to work in a non-https environment. (Some passkey managers, ex. Bitwarden, will not work without HTTPS either way... maybe try [ngrok](https://ngrok.com/) for a test environment?)

Here is an example startup command:
```bash
MONGODB_URI=<your-mongodb-uri> PK_ID=<your-effective-domain-name> PK_ORIGIN=<your-base-url> ./recipe-book
```

#### Docker Addendum
- The environment variable in the docker image comes preset to `mongodb://mongodbserver:27017`, but can be overwritten using docker-compose or a `--env` tag in the docker run command.

## Usage

1. Start the application:

   If you built the Rust application directly:

   ```bash
   MONGODB_URI=<your-mongodb-uri> PK_ID=<your-effective-domain-name> PK_ORIGIN=<your-base-url> ./recipe-book
   ```

   If you generated a Docker image:

   ```bash
   docker load < recipe-book.tar
   docker run -d -p 3000:3000 -e MONGODB_URI=<your-mongodb-uri> -e PK_ID=<your-effective-domain-name> -e PK_ORIGIN=<your-base-url>  recipe-book:latest
   ```

2. Open your web browser and navigate to [http://localhost:3000](http://localhost:3000).

3. You can now start exploring and contributing to your Recipe Book!

## HTTPS Support

For production deployment and to ensure secure communication, it's recommended to use a reverse proxy like Apache or Nginx to enable HTTPS support. Here's a high-level overview of the steps to set up HTTPS:

1. Obtain an SSL/TLS certificate from a trusted certificate authority (CA). Ex: Let'sEncrypt

2. Install and configure your chosen reverse proxy server (e.g., Apache or Nginx).

3. Configure the reverse proxy to forward incoming HTTPS requests to the Rust application running on port 3000.

4. Update any shortcuts and make sure all assets are loaded securely (use `https://` instead of `http://`). It's a good idea to setup Rewrite rules to redirect HTTP traffic to HTTPS

For detailed instructions on setting up HTTPS with a reverse proxy, refer to the documentation of the specific server software you choose.
Also, make sure that you are proxying websocket connections (and upgrading them to wss (WebSocket Secure) if using https)

## Contributing

We welcome contributions from the community! If you'd like to contribute to this project, please make a pull request and we will see if any features can be added to the project.

## License

This project is licensed under the [MIT License](LICENSE). Feel free to use, modify, and distribute it as per the terms of the license.