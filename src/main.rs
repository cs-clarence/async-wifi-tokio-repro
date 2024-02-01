use std::future::Future;

use esp_idf_svc::{
    eventloop::{EspEventLoop, EspSystemEventLoop},
    hal::peripherals::Peripherals,
    nvs::EspDefaultNvsPartition,
    timer::EspTimerService,
    wifi::{AsyncWifi, EspWifi},
};

/// Just to check if it's not just the [`tokio::spawn`] that's causing the compile error
fn send_future<F: Future<Output = ()> + Send + 'static>(
    f: F,
) -> impl Future<Output = ()> + Send + 'static {
    f
}

async fn async_main() -> anyhow::Result<()> {
    let periphs = Peripherals::take()?;
    let sysloop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;
    let event_loop = EspEventLoop::take()?;
    let timer_service = EspTimerService::new()?;

    let wifi = EspWifi::new(periphs.modem, sysloop, Some(nvs))?;

    let mut wifi = AsyncWifi::wrap(wifi, event_loop, timer_service)?;

    let fut = async move {
        wifi.connect().await.expect("Failed to start wifi");
    };

    tokio::task::spawn(fut);

    send_future(fut);

    Ok(())
}

fn main() -> anyhow::Result<()> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(async_main())?;

    Ok(())
}
