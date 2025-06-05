use color_eyre::eyre::Result;
use s2n_quic::{client::Connect, Client};
use std::{fs::File, io::{Read, Write}, net::SocketAddr, net::ToSocketAddrs, path::Path};
use md5;
use serde_json;
use crate::cli::pdu::PDU;

#[derive(Debug)]
struct ClientOptions {
    address: String,
    port: u16,
    cert: String,
}

#[tokio::main]
async fn execute_client(options: ClientOptions) -> Result<()> {
    // Resolve host and port
    let host_port = format!("{}:{}", options.address, options.port)
        .to_socket_addrs()?
        .next()
        .unwrap();

    let local_addr: SocketAddr = "0.0.0.0:0".parse()?;
    let client = Client::builder()
        .with_tls(Path::new(&options.cert))?
        .with_io(local_addr)?
        .start()?;

    println!("Connecting client...");
    let connect = Connect::new(host_port).with_server_name("localhost");
    let mut connection = client.connect(connect).await?;

    println!("Client connected...");
    connection.keep_alive(true)?;

    let stream = connection.open_bidirectional_stream().await?;
    let (mut receive_stream, mut send_stream) = stream.split();

    // Prompt for filename to upload
    let mut input_filename = String::new();
    println!("Enter the filename to upload:");
    std::io::stdin().read_line(&mut input_filename).unwrap();
    let filename = input_filename.trim().to_string();

    // Open and read the file
    let mut file = File::open(&filename).expect("Unable to open file");
    let mut file_content = Vec::new();
    file.read_to_end(&mut file_content).expect("Unable to read file");

    // Compute MD5 checksum
    let checksum = md5::compute(&file_content);
    let checksum_str = format!("{:x}", checksum);

    // Construct PDU
    let pdu = PDU::new(filename.clone(), checksum_str.clone());
    let pdu_bytes = serde_json::to_vec(&pdu).expect("Failed to serialize PDU");
    println!("PDU constructed: {:?}", pdu);

    // Send PDU to server
    send_stream.send(pdu_bytes.into()).await.expect("Stream closed unexpectedly");
    send_stream.send(b"\n".to_vec().into()).await.expect("Stream closed unexpectedly");
    println!("PDU sent to server");

    // Send file data
    send_stream.send(file_content.clone().into()).await.expect("Stream closed unexpectedly");
    println!("File content sent to server");

    // Close the send side
    send_stream.finish().expect("Failed to close send stream");
    println!("Send stream closed");

    // Collect incoming data
    let mut received_content = Vec::new();
    while let Ok(Some(chunk)) = receive_stream.receive().await {
        received_content.extend_from_slice(&chunk);
    }

    let received_filename = format!("received_{}", filename);
    let mut file = File::create(&received_filename).expect("Unable to create file");
    file.write_all(&received_content).expect("Unable to write file");
    println!("File downloaded as {}", received_filename);

    Ok(())
}

pub fn run_client(address: String, port: u16, cert: String) -> Result<()> {
    println!("Starting client...");
    println!("Connecting to {address} on port {port}...");

    let options = ClientOptions { address, port, cert };

    // Launch the async client routine
    execute_client(options)?;

    Ok(())
}
