use inovo_rs::ros_bridge::*;

use serde::{Deserialize, Serialize};
use tokio::io::AsyncReadExt;

use roslibrust::{rosbridge::ClientHandle, RosMessageType};

#[tokio::main]
async fn main() {
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

    // println!("{}", serde_json::to_string_pretty(&msg).unwrap());

    // return;

    println!("{}", SpeedStamped::ROS_TYPE_NAME);
    println!("{}", PoseStamped::ROS_TYPE_NAME);

    let client = roslibrust::rosbridge::ClientHandle::new("ws://192.168.1.127:9090")
        .await
        .unwrap();
    println!("ClientHandle connected");
    // {
    //     let sub = client
    //         .subscribe::<InovoMessage<Res>>(ClientHandle::TOPIC_TCP_SPEED)
    //         .await
    //         .unwrap();
    //     for i in 0..3 {
    //         println!("{:#?}", sub.next().await)
    //     }
    // }
    // {
    //     let sub = client
    //         .subscribe::<InovoMessage<Res>>(ClientHandle::TOPIC_TCP_POSE)
    //         .await
    //         .unwrap();
    //     for i in 0..3 {
    //         println!("{:#?}", sub.next().await)
    //     }
    // }

    // let j = client.arm_state().await;
    // for i in 0..3 {
    //     let js = j.next().await;
    //     println!("{:?}", js.payload)
    // }

    // return;

    // {
    //     let sub = client.subscribe::<Res>("/psu/estop/state").await.unwrap();

    //     for i in 0..3 {
    //         println!("{:#?}", sub.next().await);
    //     }
    // }

    // let res = client
    //     // .call_service::<Trigger>("/psu/estop/reset", Trigger {})
    //     .call_service::<Trigger>("/sequence/pause", Trigger {})
    //     .await
    //     .unwrap();
    // println!("{:?}", res);

    // return;

    {
        let sub = client.tcp_speed().await;
        for i in 0..3 {
            println!("{:#?}", sub.next().await)
        }
    }
    {
        let sub = client.tcp_pose().await;
        for i in 0..3 {
            println!("{:#?}", sub.next().await)
        }
    }
    // {
    //     let sub = client.joint_state().await;
    //     for i in 0..3 {
    //         println!("{:#?}", sub.next().await)
    //     }
    // }
    {
        let sub = client.power_state().await;
        for i in 0..3 {
            println!("{:#?}", sub.next().await)
        }
    }
    {
        let sub = client.robot_state().await;
        for i in 0..3 {
            println!("{:#?}", sub.next().await)
        }
    }
    {
        let sub = client.estop_state().await;
        for i in 0..3 {
            println!("{:#?}", sub.next().await)
        }
    }
    {
        let sub = client.safe_stop_state().await;
        for i in 0..3 {
            println!("{:#?}", sub.most_recent().await)
        }
    }
    {
        let sub = client.runtime_state().await;
        for i in 0..3 {
            println!("{:#?}", sub.most_recent().await)
        }
    }
    {
        let sub = client.arm_state().await;
        for i in 0..3 {
            println!("{:#?}", sub.most_recent().await)
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

    // println!("starting . . .");
    // let res = client.sequence_start().await;
    // println!("res : {:?}", res);
    // wait_line().await;
    println!("starting . . .");
    let res = client.sequence_function("do something").await;
    println!("res : {:?}", res);
    wait_line().await;

    println!("pausing . . .");
    let res = client.sequence_pause().await;
    println!("res : {:?}", res);
    wait_line().await;

    println!("stepping . . .");
    let res = client.sequence_step().await;
    println!("res : {:?}", res);
    wait_line().await;

    println!("debugging . . .");
    let res = client.sequence_debug().await;
    println!("res : {:?}", res);
    wait_line().await;

    println!("continuing . . .");
    let res = client.sequence_continue().await;
    println!("res : {:?}", res);
    wait_line().await;

    println!("stopping . . .");
    let res = client.sequence_stop().await;
    println!("res : {:?}", res);
    wait_line().await;

    // client.arm_disable().await;
    // wait_line().await;
    // client.power_off().await;
    // wait_line().await;

    return;

    {
        let client = client.clone();
        tokio::spawn(async move {
            let sub = client.jog().await;
            loop {
                let msg = sub.next().await;

                println!("{:#?}", msg);
            }
        });
    }

    let pubisher = client
        .advertise::<InovoMessage<CartesianJogDemand>>("/default_move_group/cartesian_jog")
        .await
        .unwrap();

    for _ in 0..1000 {
        // println!("a");
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
        // println!("b");
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
