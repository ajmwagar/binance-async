use crate::futures::model::*;
use crate::client::*;
use crate::errors::*;
use serde_json::from_str;

#[derive(Clone)]
pub struct FuturesGeneral {
    pub client: Client,
}

impl FuturesGeneral {
    // Test connectivity
    pub async fn ping(&self) -> Result<String> {
        self.client.get("/fapi/v1/ping", "").await?;
        Ok("pong".into())
    }

    // Check server time
    pub async fn get_server_time(&self) -> Result<ServerTime> {
        let data: String = self.client.get("/fapi/v1/time", "").await?;
        let server_time: ServerTime = from_str(data.as_str())?;

        Ok(server_time)
    }

    // Obtain exchange information
    // - Current exchange trading rules and symbol information
    pub async fn exchange_info(&self) -> Result<ExchangeInformation> {
        let data: String = self.client.get("/fapi/v1/exchangeInfo", "").await?;
        let info: ExchangeInformation = from_str(data.as_str())?;

        Ok(info)
    }

    // Get Symbol information
    pub async fn get_symbol_info<S>(&self, symbol: S) -> Result<Symbol>
    where
        S: Into<String>,
    {
        let upper_symbol = symbol.into().to_uppercase();
        match self.exchange_info().await {
            Ok(info) => {
                for item in info.symbols {
                    if item.symbol == upper_symbol {
                        return Ok(item);
                    }
                }
                bail!("Symbol not found")
            }
            Err(e) => Err(e),
        }
    }
}
