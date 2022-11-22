use anyhow::Result;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

pub struct Address {
    pub host: String,
    pub port: u16,
}

impl Address {
    pub fn new(host: String, port: u16) -> Self {
        Self { host, port }
    }
}

pub struct Proxy {
    client: Address,
    server: Address,
}

impl Proxy {
    pub fn new(client: Address, server: Address) -> Self {
        Self { client, server }
    }

    pub async fn start(&self) -> Result<()> {
        let client =
            TcpListener::bind(format!("{}:{}", self.client.host, self.client.port)).await?;

        while let Ok((in_bound, _)) = client.accept().await {
            let out_bound =
                TcpStream::connect(format!("{}:{}", self.server.host, self.server.port)).await?;
            tokio::spawn(async move {
                if let Err(e) = forwarding(in_bound, out_bound).await {
                    tracing::error!("failed to proxy connection: {}", e);
                }
            });
        }

        Ok(())
    }
}

// 把客户端接收到的数据转发给服务端
async fn forwarding(mut in_bound: TcpStream, mut out_bound: TcpStream) -> Result<()> {
    let (mut ri, mut wi) = in_bound.split();
    let (mut ro, mut wo) = out_bound.split();

    let client_to_server = async { tokio::io::copy(&mut ri, &mut wo).await };
    let server_to_client = async { tokio::io::copy(&mut ro, &mut wi).await };

    tokio::try_join!(client_to_server, server_to_client)?;

    Ok(())
}

// test
mod tests {

    #[tokio::test]
    async fn test_listen() {
        use super::{Address, Proxy};
        let client = Address::new("localhost".to_string(), 8080);
        let server = Address::new("localhost".to_string(), 8181);
        let listener = Proxy::new(client, server);
        listener.start().await.unwrap();
    }
}
