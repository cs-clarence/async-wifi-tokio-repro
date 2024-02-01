use esp_idf_svc::{
    eventloop::{EspEventLoop, EspSystemEventLoop},
    hal::peripherals::Peripherals,
    nvs::EspDefaultNvsPartition,
    timer::EspTimerService,
    wifi::{AsyncWifi, EspWifi},
};

async fn async_main() -> anyhow::Result<()> {
    let periphs = Peripherals::take().expect("Failed to take peripherals");
    let sysloop = EspSystemEventLoop::take().expect("Failed to take system event loop");
    let nvs = EspDefaultNvsPartition::take().expect("Failed to take NVS partition");
    let event_loop = EspEventLoop::take().expect("Failed to take event loop");
    let timer_service = EspTimerService::new()?;

    let wifi = EspWifi::new(periphs.modem, sysloop, Some(nvs))?;

    let mut wifi = AsyncWifi::wrap(wifi, event_loop, timer_service)?;

    tokio::task::spawn(async move {
        wifi.start().await.expect("Failed to start wifi");
    });

    Ok(())
}

fn main() -> anyhow::Result<()> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(async_main())?;

    Ok(())
}
