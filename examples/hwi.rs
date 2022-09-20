use async_hwi::HWI;

#[cfg(feature = "specter")]
use async_hwi::specter::{Specter, SpecterSimulator};

#[tokio::main]
pub async fn main() {
    let list = list_hardware_wallets().await;
    eprintln!(
        "{} device{} connected",
        list.len(),
        if list.len() > 1 { "s" } else { "" }
    );

    for hw in list {
        eprintln!("{}", hw.device_type())
    }
}

pub async fn list_hardware_wallets() -> Vec<Box<dyn HWI + Send>> {
    let mut hws = Vec::new();

    #[cfg(feature = "specter")]
    if let Ok(device) = SpecterSimulator::try_connect().await {
        hws.push(device.into());
    }

    #[cfg(feature = "specter")]
    if let Ok(device) = Specter::try_connect_serial() {
        hws.push(device.into());
    }

    hws
}
