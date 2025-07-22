use inovo_rs::ros_bridge::*;

use serde::{Deserialize, Serialize};
use tokio::io::AsyncReadExt;

use roslibrust::{rosbridge::ClientHandle, RosMessageType};

use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // let msg = InovoMessage {
    //     header: InovoHeader {
    //         frame_id: "s".to_string(),
    //         seq: 12,
    //         stamp: Stamp {
    //             nsecs: 10,
    //             secs: 10,
    //         },
    //     },
    //     tcp_id: "".to_string(),
    //     payload: PoseStamped::default(),
    // };

    // info!("{}", serde_json::to_string_pretty(&msg).unwrap());

    // return;

    info!("{}", SpeedStamped::ROS_TYPE_NAME);
    info!("{}", PoseStamped::ROS_TYPE_NAME);

    let client = roslibrust::rosbridge::ClientHandle::new("ws://192.168.1.127:9090")
        .await
        .unwrap();
    info!("ClientHandle connected");
    // {
    //     let sub = client
    //         .subscribe::<InovoMessage<Res>>(ClientHandle::TOPIC_TCP_SPEED)
    //         .await
    //         .unwrap();
    //     for i in 0..3 {
    //         info!("{:#?}", sub.next().await)
    //     }
    // }
    // {
    //     let sub = client
    //         .subscribe::<InovoMessage<Res>>(ClientHandle::TOPIC_TCP_POSE)
    //         .await
    //         .unwrap();
    //     for i in 0..3 {
    //         info!("{:#?}", sub.next().await)
    //     }
    // }

    // let j = client.arm_state().await;
    // for i in 0..3 {
    //     let js = j.next().await;
    //     info!("{:?}", js.payload)
    // }

    // return;

    // {
    //     let sub = client.subscribe::<Res>("/psu/estop/state").await.unwrap();

    //     for i in 0..3 {
    //         info!("{:#?}", sub.next().await);
    //     }
    // }

    // let res = client
    //     // .call_service::<Trigger>("/psu/estop/reset", Trigger {})
    //     .call_service::<Trigger>("/sequence/pause", Trigger {})
    //     .await
    //     .unwrap();
    // info!("{:?}", res);

    // return;

    {
        let sub = client.tcp_speed().await.unwrap();
        for i in 0..3 {
            info!("{:#?}", sub.next().await)
        }
    }
    {
        let sub = client.tcp_pose().await.unwrap();
        for i in 0..3 {
            info!("{:#?}", sub.next().await)
        }
    }
    // {
    //     let sub = client.joint_state().await;
    //     for i in 0..3 {
    //         info!("{:#?}", sub.next().await)
    //     }
    // }
    {
        let sub = client.power_state().await.unwrap();
        for i in 0..3 {
            info!("{:#?}", sub.next().await)
        }
    }
    {
        let sub = client.robot_state().await.unwrap();
        for i in 0..3 {
            info!("{:#?}", sub.next().await)
        }
    }
    {
        let sub = client.estop_state().await.unwrap();
        for i in 0..3 {
            info!("{:#?}", sub.next().await)
        }
    }
    {
        let sub = client.safe_stop_state().await.unwrap();
        for i in 0..3 {
            info!("{:#?}", sub.most_recent().await)
        }
    }
    {
        let sub = client.runtime_state().await.unwrap();
        for i in 0..3 {
            info!("{:#?}", sub.most_recent().await)
        }
    }
    {
        let sub = client.arm_state().await.unwrap();
        for i in 0..3 {
            info!("{:#?}", sub.most_recent().await)
        }
    }

    return;

    let wait_line = || async move {
        let mut buf = [0_u8];
        loop {
            tokio::io::stdin().read_exact(&mut buf).await.unwrap();
            if buf[0] == b'\n' {
                break;
            }
        }
    };

    let jog = client.jog_pub().await;

    for i in 0..1000 {
        jog.publish(&CartesianJogDemand {
            twist: Twist {
                linear: Vec3 {
                    z: 0.01,
                    ..Default::default()
                },
                ..Default::default()
            },
        })
        .await
        .unwrap();
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    }
    for i in 0..1000 {
        jog.publish(&CartesianJogDemand {
            twist: Twist {
                linear: Vec3 {
                    z: -0.01,
                    ..Default::default()
                },
                ..Default::default()
            },
        })
        .await
        .unwrap();
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    }

    return;

    // client.safe_stop_reset().await;
    // wait_line().await;
    // client.estop_reset().await;
    // wait_line().await;
    // client.power_on().await;
    // wait_line().await;
    // client.arm_enable().await;
    // wait_line().await;

    // info!("starting . . .");
    // let res = client.sequence_start().await;
    // info!("res : {:?}", res);
    // wait_line().await;
    info!("starting . . .");
    let res = client.sequence_function("do something").await;
    info!("res : {:?}", res);
    wait_line().await;

    info!("pausing . . .");
    let res = client.sequence_pause().await;
    info!("res : {:?}", res);
    wait_line().await;

    info!("stepping . . .");
    let res = client.sequence_step().await;
    info!("res : {:?}", res);
    wait_line().await;

    info!("debugging . . .");
    let res = client.sequence_debug().await;
    info!("res : {:?}", res);
    wait_line().await;

    info!("continuing . . .");
    let res = client.sequence_continue().await;
    info!("res : {:?}", res);
    wait_line().await;

    info!("stopping . . .");
    let res = client.sequence_stop().await;
    info!("res : {:?}", res);
    wait_line().await;

    // client.arm_disable().await;
    // wait_line().await;
    // client.power_off().await;
    // wait_line().await;

    return;

    {
        let client = client.clone();
        tokio::spawn(async move {
            let sub = client.jog().await.unwrap();
            loop {
                let msg = sub.next().await;

                info!("{:#?}", msg);
            }
        });
    }

    let pubisher = client
        .advertise::<InovoMessage<CartesianJogDemand>>("/default_move_group/cartesian_jog")
        .await
        .unwrap();

    for _ in 0..1000 {
        // info!("a");
        pubisher
            .publish(&InovoMessage {
                header: InovoHeader {
                    frame_id: "".to_string(),
                    seq: 0,
                    stamp: Stamp { nsecs: 0, secs: 0 },
                },
                tcp_id: "".to_string(),
                payload: CartesianJogDemand {
                    twist: Twist {
                        linear: Vec3 {
                            x: 0.0,
                            y: 0.0,
                            z: 0.01,
                        },
                        angular: Vec3 {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                            // w: 0.0,
                        },
                    },
                },
            })
            .await
            .unwrap();
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    for _ in 0..1000 {
        // info!("b");
        pubisher
            .publish(&InovoMessage {
                header: InovoHeader {
                    frame_id: "".to_string(),
                    seq: 0,
                    stamp: Stamp { nsecs: 0, secs: 0 },
                },
                tcp_id: "".to_string(),
                payload: CartesianJogDemand {
                    twist: Twist {
                        linear: Vec3 {
                            x: 0.0,
                            y: 0.0,
                            z: -0.01,
                        },
                        angular: Vec3 {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                            // w: 0.0,
                        },
                    },
                },
            })
            .await
            .unwrap();
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
}
