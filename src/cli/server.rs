use color_eyre::eyre::Result;
use s2n_quic::Server;
use std::{fs::File, io::{Read, Write}, net::ToSocketAddrs, path::Path};
use md5;
use serde_json;
use crate::cli::pdu::PDU;

#[derive(Debug)]
struct ServerOptions {
    address: String,
    port: u16,
    cert: String,
    key: String,
}

#[tokio::main]
async fn execute_server(options: ServerOptions) -> Result<()> {
    // Resolve bind address and port
    let host_port = format!("{}:{}", options.address, options.port)
        .to_socket_addrs()?
        .next()
        .unwrap();

    let mut server = Server::builder()
        .with_tls((Path::new(&options.cert), Path::new(&options.key)))?
        .with_io(host_port)?
        .start()?;
    println!("{:#?} Server is running...", options);

    while let Some(mut connection) = server.accept().await {
        tokio::spawn(async move {
            while let Ok(Some(mut stream)) = connection.accept_bidirectional_stream().await {
                tokio::spawn(async move {
                    let mut header_bytes = Vec::new();
                    let mut file_data = Vec::new();
                    let mut separator_found = false;

                    // Read until separator is found
                    while let Ok(Some(chunk)) = stream.receive().await {
                        if !separator_found {
                            header_bytes.extend_from_slice(&chunk);
                            if let Some(pos) = header_bytes.iter().position(|&b| b == b'\n') {
                                separator_found = true;
                                file_data.extend_from_slice(&header_bytes[pos + 1..]);
                                header_bytes.truncate(pos);
                                break;
                            }
                        } else {
                            file_data.extend_from_slice(&chunk);
                        }
                    }

                    if !separator_found {
                        println!("No separator detected in incoming stream");
                        return;
                    }

                    let pdu: PDU = serde_json::from_slice(&header_bytes)
                        .expect("Failed to parse PDU");
                    println!("Parsed PDU: {:?}", pdu);

                    // Read remaining file bytes
                    while let Ok(Some(chunk)) = stream.receive().await {
                        file_data.extend_from_slice(&chunk);
                    }

                    // Compute and compare checksum
                    let calculated = md5::compute(&file_data);
                    let calculated_str = format!("{:x}", calculated);
                    if pdu.checksum == calculated_str {
                        println!("Checksum matches.");
                    } else {
                        println!("Checksum mismatch.");
                    }

                    // Write received file to disk
                    let mut out_file = File::create(&pdu.filename)
                        .expect("Failed to create output file");
                    out_file.write_all(&file_data)
                        .expect("Failed to write file");
                    println!("Saved file: {}", pdu.filename);

                    // Read file back for echo
                    let mut echo_file = File::open(&pdu.filename)
                        .expect("Failed to reopen file for sending");
                    let mut echo_data = Vec::new();
                    echo_file.read_to_end(&mut echo_data)
                        .expect("Failed to read file for sending");
                    stream.send(echo_data.into())
                        .await
                        .expect("Stream closed unexpectedly");
                    println!("Echoed file back: {}", pdu.filename);

                    stream.finish().expect("Failed to close stream");
                });
            }
        });
    }

    Ok(())
}

pub fn run_server(address: String, port: u16, cert: String, key: String) -> Result<()> {
    println!("Starting server...");
    println!("Listening on {address} at port {port}...");

    let options = ServerOptions { address, port, cert, key };
    execute_server(options)?;

    Ok(())
}
