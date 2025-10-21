use inovo_rs::prelude_non_blocking::*;

use tracing::info;

#[tokio::main]
async fn main() {
    rosbridge().await.unwrap();
}

async fn rosbridge() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    dotenvy::dotenv()?;

    let psu_host = std::env::var("DEFAULT_PSU_HOST")?;

    let client = rosbridge_connect(psu_host, std::time::Duration::from_secs(5)).await?;
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

    // until predicate
    topic::sequence::RuntimeState::until(&client, |rt_s| {
        rt_s.state == commander_msgs::RuntimeStatus::Idle
    })
    .await?;

    // Services
    //
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

    // direct call (less lsp support)
    let res = service::robot::Disable::call(&client, std_srvs::Trigger).await?;
    assert!(res.success, "arm disable failed due to : {}", res.message);
    // or
    let res = client
        .call_inovo::<service::psu::Disable>(std_srvs::Trigger)
        .await?;
    assert!(res.success, "psu poweroff failed due to : {}", res.message);

    Ok(())
}
