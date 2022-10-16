use std::{net::IpAddr, str::FromStr};

const DEFAULT_PORT_NUMBER: u16 = 7489;

fn main() {
    let matches = clap::Command::new("buqic")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("Buqi CLI")
        .arg(
            clap::Arg::new("client")
                .long("client")
                .short('c')
                .takes_value(true)
                .help("Client mode, connect to specified IP"),
        )
        .arg(
            clap::Arg::new("server")
                .long("server")
                .short('s')
                .takes_value(true)
                .help("Server mode, bind to specified IP"),
        )
        .arg(
            clap::Arg::new("port")
                .long("port")
                .short('p')
                .takes_value(true)
                .help("TCP Port number to bind or connect"),
        )
        .get_matches();
    let port = matches
        .value_of("port")
        .map(|p| p.parse::<u16>().unwrap())
        .unwrap_or(DEFAULT_PORT_NUMBER);

    if let Some(ip) = matches.value_of("server") {
        run_server(IpAddr::from_str(ip).unwrap(), port);
    } else if let Some(ip) = matches.value_of("client") {
        run_client(IpAddr::from_str(ip).unwrap(), port);
    } else {
        panic!("Need to define --server or --client");
    }
}

fn run_server(ip: IpAddr, port: u16) {
    let net_fd = BuqicIpcTcp::new().unwrap();
}

fn run_client(ip: IpAddr, port: u16) {
    todo!()
}
