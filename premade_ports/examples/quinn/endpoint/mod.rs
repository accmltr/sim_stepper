#![cfg(any(feature = "rustls-aws-lc-rs", feature = "rustls-ring"))]
//! Commonly used code in most examples.
use quinn::{
    ClientConfig, Endpoint, ServerConfig,
    crypto::rustls::{QuicClientConfig, QuicServerConfig},
};
use rustls::pki_types::{CertificateDer, PrivatePkcs8KeyDer};

use std::{error::Error, net::SocketAddr, sync::Arc};

/// Constructs a QUIC endpoint configured for use a client only.
///
/// ## Args
///
/// - server_certs: list of trusted certificates.
#[allow(unused)]
pub fn make_client_endpoint(
    bind_addr: SocketAddr,
    server_certs: &[&[u8]],
) -> Result<Endpoint, Box<dyn Error + Send + Sync + 'static>> {
    let mut certs = rustls::RootCertStore::empty();
    for cert in server_certs {
        certs.add(CertificateDer::from(*cert))?;
    }

    let mut client_crypto = rustls::ClientConfig::builder()
        .with_root_certificates(Arc::new(certs.clone()))
        .with_no_client_auth();

    client_crypto.alpn_protocols = ALPN_QUIC_HTTP.iter().map(|&x| x.into()).collect();

    // let client_config = ClientConfig::with_root_certificates(Arc::new(certs.clone()))?;
    let client_config = ClientConfig::new(Arc::new(QuicClientConfig::try_from(client_crypto)?));

    let mut endpoint = Endpoint::client(bind_addr)?;
    endpoint.set_default_client_config(client_config);
    Ok(endpoint)
}

/// Constructs a QUIC endpoint configured to listen for incoming connections on a certain address
/// and port.
///
/// ## Returns
///
/// - a stream of incoming QUIC connections
/// - server certificate serialized into DER format
#[allow(unused)]
pub fn make_server_endpoint(
    bind_addr: SocketAddr,
) -> Result<(Endpoint, CertificateDer<'static>), Box<dyn Error + Send + Sync + 'static>> {
    let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
    let cert_der = CertificateDer::from(cert.cert);
    let priv_key = PrivatePkcs8KeyDer::from(cert.signing_key.serialize_der());
    let priv_key = rustls::pki_types::PrivateKeyDer::Pkcs8(priv_key);

    let mut server_crypto = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(vec![cert_der.clone()], priv_key)?;

    server_crypto.alpn_protocols = ALPN_QUIC_HTTP.iter().map(|&x| x.into()).collect();

    let mut server_config =
        ServerConfig::with_crypto(Arc::new(QuicServerConfig::try_from(server_crypto)?));
    let transport_config = Arc::get_mut(&mut server_config.transport).unwrap();
    transport_config.max_concurrent_uni_streams(0_u8.into());

    let endpoint = Endpoint::server(server_config, bind_addr)?;
    Ok((endpoint, cert_der))
}

#[allow(unused)]
pub const ALPN_QUIC_HTTP: &[&[u8]] = &[b"hq-29"];
