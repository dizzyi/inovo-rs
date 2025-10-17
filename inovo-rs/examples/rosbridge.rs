use inovo_rs::prelude_non_blocking::*;

use roslibrust::rosbridge::ClientHandleOptions;

use tracing::info;

#[tokio::main]
async fn main() {
    rosbridge().await.unwrap();
}

async fn rosbridge() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    dotenv::dotenv()?;

    let psu_host = std::env::var("DEFAULT_PSU_HOST")?;

    let client = roslibrust::rosbridge::ClientHandle::new_with_options(
        ClientHandleOptions::new(psu_host.to_ws_url()).timeout(std::time::Duration::from_secs(5)),
    )
    .await?;
    info!("ClientHandle connected");

    // Topics
    // using client's short hand
    {
        let sub = client.psu_status().await?;
        for _ in 0..3 {
            println!("{:?}", sub.next().await);
        }
    }
    // via topics
    {
        let sub = topic::psu::EStopState::subscribe(&client).await?;
        for _ in 0..3 {
            println!("{:?}", sub.next().await);
        }
    }
    // or
    {
        let sub = client
            .subscribe_inovo::<topic::psu::SafeStopState>()
            .await?;
        for _ in 0..3 {
            println!("{:?}", sub.next().await);
        }
    }

    // Services
    // using client's short hand
    let res = client.arm_enable().await?;
    assert!(res.success, "arm enable failed due to : {}", res.message);

    // via services
    let service_client = service::psu::EStopReset::service(&client).await?;
    let res = service_client.call(std_srvs::Trigger).await?;
    assert!(res.success, "reset estop failed due to : {}", res.message);

    // or
    let service_client = client
        .service_inovo::<service::psu::SafeStopReset>()
        .await?;
    let res = service_client.call(std_srvs::Trigger).await?;
    assert!(
        res.success,
        "reset safestop failed due to : {}",
        res.message
    );

    // direct call
    let res = service::robot::Disable::call(&client, std_srvs::Trigger).await?;
    assert!(res.success, "arm disable failed due to : {}", res.message);
    // or
    let res = client
        .call_inovo::<service::psu::Disable>(std_srvs::Trigger)
        .await?;
    assert!(res.success, "psu poweroff failed due to : {}", res.message);

    return Ok(());
}
