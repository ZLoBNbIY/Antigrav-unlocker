//! DNS-over-TLS (RFC 7858) transport for user-configured resolvers.

use std::io::{Read, Write};
use std::net::ToSocketAddrs;
use std::sync::{Arc, OnceLock};
use std::time::{Duration, Instant};

use rustls::pki_types::ServerName;
use rustls::{ClientConfig, ClientConnection, RootCertStore};

const DEFAULT_PORT: u16 = 853;
const MAX_DNS_MESSAGE: usize = u16::MAX as usize;

fn tls_config() -> &'static Arc<ClientConfig> {
    static CONFIG: OnceLock<Arc<ClientConfig>> = OnceLock::new();
    CONFIG.get_or_init(|| {
        let roots = RootCertStore {
            roots: webpki_roots::TLS_SERVER_ROOTS.to_vec(),
        };
        Arc::new(
            ClientConfig::builder()
                .with_root_certificates(roots)
                .with_no_client_auth(),
        )
    })
}

/// Sends one DNS wire-format message to `tls://host[:port]` and returns its
/// length-prefixed answer. Connections leave through the same selected network
/// interface as UDP resolvers and the built-in DoH provider.
pub fn query(
    endpoint: &str,
    wire: &[u8],
    timeout: Duration,
    if_index: u32,
) -> Result<Vec<u8>, String> {
    if wire.is_empty() || wire.len() > MAX_DNS_MESSAGE {
        return Err("неверный размер DNS-запроса".into());
    }
    let url = url::Url::parse(endpoint).map_err(|_| "неверный адрес DoT-сервера")?;
    if !matches!(url.scheme(), "tls" | "dot") {
        return Err("ожидался адрес tls:// или dot://".into());
    }
    let host = url
        .host_str()
        .ok_or("в адресе DoT не указано имя сервера")?;
    let port = url.port().unwrap_or(DEFAULT_PORT);
    let addresses: Vec<_> = (host, port)
        .to_socket_addrs()
        .map_err(|e| format!("не удалось найти DoT-сервер: {e}"))?
        .collect();
    if addresses.is_empty() {
        return Err("DoT-сервер не имеет IP-адресов".into());
    }

    let server_name =
        ServerName::try_from(host.to_string()).map_err(|_| "неверное имя DoT-сервера")?;
    let deadline = Instant::now() + timeout;
    let mut last_error = "DoT-сервер не ответил".to_string();
    for address in addresses {
        let left = deadline.saturating_duration_since(Instant::now());
        if left.is_zero() {
            break;
        }
        let socket = match crate::net::connect(address, left, if_index) {
            Ok(socket) => socket,
            Err(error) => {
                last_error = error.to_string();
                continue;
            }
        };
        socket.set_read_timeout(Some(left)).ok();
        socket.set_write_timeout(Some(left)).ok();
        let connection = match ClientConnection::new(tls_config().clone(), server_name.clone()) {
            Ok(connection) => connection,
            Err(error) => {
                last_error = error.to_string();
                continue;
            }
        };
        let mut stream = rustls::StreamOwned::new(connection, socket);
        let length = (wire.len() as u16).to_be_bytes();
        let attempt = (|| -> Result<Vec<u8>, String> {
            stream
                .write_all(&length)
                .and_then(|_| stream.write_all(wire))
                .and_then(|_| stream.flush())
                .map_err(|e| format!("ошибка отправки DoT-запроса: {e}"))?;
            let mut size = [0u8; 2];
            stream
                .read_exact(&mut size)
                .map_err(|e| format!("ошибка чтения DoT-ответа: {e}"))?;
            let size = u16::from_be_bytes(size) as usize;
            if size == 0 {
                return Err("DoT-сервер прислал пустой ответ".into());
            }
            let mut answer = vec![0; size];
            stream
                .read_exact(&mut answer)
                .map_err(|e| format!("неполный DoT-ответ: {e}"))?;
            Ok(answer)
        })();
        match attempt {
            Ok(answer) => {
                crate::net::note_reached();
                return Ok(answer);
            }
            Err(error) => last_error = error,
        }
    }
    Err(last_error)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_dot_urls_and_uses_the_standard_port() {
        let url = url::Url::parse("tls://dns.example").unwrap();
        assert_eq!(url.host_str(), Some("dns.example"));
        assert_eq!(url.port().unwrap_or(DEFAULT_PORT), 853);
        let explicit = url::Url::parse("dot://dns.example:8853").unwrap();
        assert_eq!(explicit.port(), Some(8853));
    }
}
