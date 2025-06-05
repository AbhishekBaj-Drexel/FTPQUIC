# QUIC File Transfer Protocol

This repository provides a simple file transfer system over QUIC, allowing both text and binary files (for example, images) to be sent and received. All communication is secured with TLS certificates, and transferred files are verified via checksums to ensure integrity.

## Features

- Send and receive text or binary files (e.g., images) using QUIC.
- Encrypted communication with TLS certificates.
- Automatic checksum verification to confirm file integrity.
- Separate client and server modes.
- Supports uploading files to the server and downloading them back to the client.

## Requirements

- **Rust**: Install Rust on your machine.
- **OpenSSL**: Required if you want to generate custom keys and certificates.

### Generating Custom Certificates

A set of keys and certificates is already provided in the `certs` directory. If you prefer to create your own certificates, follow these steps:

1. Ensure **OpenSSL** is installed on your machine.
2. Run the certificate generation script:
    ```sh
    # Run the certificate generation script
    ./certgen.sh
    ```
3. Edit `certificate.conf` if you need to adjust certificate details.

## Usage

1. (Optional) Compile the project:
    ```sh
    cargo build
    ```
   Note: Running the `cargo run` commands below will also build the code if necessary.

2. Launch the server:
    ```sh
    cargo run -- server --cert ./certs/server.crt --key ./certs/server.key
    ```

3. Start the client:
    ```sh
    cargo run --bin quicrs -- client --address 127.0.0.1 --port 54321 --cert ./certs/ca.cert
    ```
   
   Make sure you execute these commands from the project’s root directory (where `Cargo.toml` resides).  
   When the client starts, it will prompt you to enter the name of a file located in the root folder.

## Workflow Overview

1. **Client Connects**  
   The client begins by connecting to the server at the specified address and port. A secure QUIC connection is established using the provided TLS certificate.

2. **Select File**  
   After connecting, the client prompts the user to type in the filename (which must exist in the project root).  

3. **Build PDU**  
   The client constructs a Protocol Data Unit (PDU) containing the file’s name and its MD5 checksum.

4. **Upload File**  
   The client sends the PDU followed by the file’s contents. The server receives the PDU, verifies the checksum, and writes the file to disk.

5. **Echo Back**  
   Once the server successfully saves the file, it reads it back from disk and sends it back to the client. The client then writes the received bytes to `received_<filename>` in the root directory.


## Extra Credits

- Uploaded source code on GitHub
- Used asynchronous tasks to handle multiple clients by creating a new task for each accepted connection.
- Included a learning summary.
- Used systems programming language (Rust)
