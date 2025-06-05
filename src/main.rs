use color_eyre::eyre::{eyre, Result};

mod cli;

pub(crate) mod protocol {
    pub mod echo;
}

fn main() -> Result<()> {
    color_eyre::install()?;

    // Parse command‐line arguments
    let matches = cli::parse_cli_options();

    // Extract listening port from matches
    let port = matches
        .get_one::<u16>("port")
        .ok_or_else(|| eyre!("failed to read port from CLI arguments"))?
        .to_owned();

    // Extract server address from matches
    let address = matches
        .get_one::<String>("address")
        .ok_or_else(|| eyre!("failed to read address from CLI arguments"))?
        .to_owned();

    // Extract certificate path from matches
    let cert = matches
        .get_one::<String>("cert")
        .ok_or_else(|| eyre!("failed to read certificate path from CLI arguments"))?
        .to_owned();

    // Extract private key path from matches
    let key = matches
        .get_one::<String>("key")
        .ok_or_else(|| eyre!("failed to read private key path from CLI arguments"))?
        .to_owned();

    // Dispatch to client or server mode
    match matches.subcommand() {
        Some(("client", _client_matches)) => {
            cli::client::run_client(address, port, cert)
        }
        Some(("server", _server_matches)) => {
            cli::server::run_server(address, port, cert, key)
        }
        Some((invalid, _)) => {
            unreachable!("Unexpected subcommand: {invalid}")
        }
        None => unreachable!("A subcommand (client or server) is required"),
    }
}
