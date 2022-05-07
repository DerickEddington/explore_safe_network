pub use sn_client::utils::test_utils::read_network_conn_info;
use sn_client::{
    Client,
    ClientConfig,
};


pub async fn connect_to_testnet() -> Result<Client, Box<dyn std::error::Error>>
{
    let (genesis_key, bootstrap_nodes) = read_network_conn_info()?;
    let config = ClientConfig::new(None, None, genesis_key, None, None, None, None).await;
    Ok(Client::new(config, bootstrap_nodes, None).await?)
}


#[cfg(test)]
mod tests
{
    #[test]
    fn read_network_conn_info()
    {
        let _a = super::read_network_conn_info().unwrap();
    }

    #[tokio::test]
    async fn connect_to_testnet()
    {
        let _a = super::connect_to_testnet().await.unwrap();
    }
}
