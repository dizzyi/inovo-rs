use inovo_rs::ros_bridge::{
    commander_msgs::*, geometry_msgs::*, service::Service, topic::Topic, *,
};

use tokio::io::AsyncReadExt;

use roslibrust::rosbridge::ClientHandleOptions;

use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let client = roslibrust::rosbridge::ClientHandle::new_with_options(
        ClientHandleOptions::new("ws://192.168.1.122:9090")
            .timeout(std::time::Duration::from_secs(5)),
    )
    .await
    .unwrap();
    info!("ClientHandle connected");

    if true {
        let io = topic::beckhoff_io::IO::subscribe(&client).await.unwrap();

        loop {
            println!("{:?}", io.next().await);
        }
    }

    if false {
        let client = client.clone();

        let log_task = async move {
            let log_sub = topic::RosOut::subscribe(&client).await.unwrap();
            loop {
                let msg = log_sub.next().await;
                let origin = format!("{}:{}:{}", msg.file, msg.line, msg.function);
                println!(
                    "{:>6?}|{:<30}|{:>10}|{}",
                    msg.level, origin, msg.name, msg.msg
                )
            }
        };

        tokio::spawn(log_task);
    }
    if true {
        let client = client.clone();

        let task = async move {
            let sub = topic::psu::Status::subscribe(&client).await.unwrap();
            loop {
                let msg = sub.next().await;
                println!("{:?}", msg);
            }
        };

        tokio::spawn(task);
    }
    if true {
        let client = client.clone();
        let task = async move {
            let sub = topic::robot::RobotStatus::subscribe(&client).await.unwrap();
            loop {
                let msg = sub.next().await;
                println!("{:?}", msg);
            }
        };
        tokio::spawn(task);
    }
    if true {
        let client = client.clone();
        let task = async move {
            let sub = topic::robot::RobotState::subscribe(&client).await.unwrap();
            loop {
                let msg = sub.next().await;
                println!("{:?}", msg);
            }
        };
        tokio::spawn(task);
    }

    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    let res = service::psu::Enable::call(&client, Default::default()).await;
    println!("{res:#?}");

    tokio::time::sleep(std::time::Duration::from_secs(20)).await;

    let res = service::robot::Enable::call(&client, std_srvs::Trigger).await;
    println!("{res:#?}");
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    let res = service::robot::Disable::call(&client, std_srvs::Trigger).await;
    println!("{res:#?}");
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    let res = service::psu::Disable::call(&client, Default::default()).await;
    println!("{res:#?}");

    // service::robot::Reset::call(&client, std_srvs::Trigger);

    return;

    if false {
        let sub = topic::robot::RobotState::subscribe(&client).await.unwrap();

        println!("subscribed");

        for i in 0..3 {
            println!("{:#?}", sub.next().await);
            println!(
                "{}",
                serde_json::to_string_pretty(&sub.most_recent().await).unwrap()
            )
        }
    }

    // {
    //     let publisher = client
    //         .advertise::<std_msgs::ColorRGBA>(topic::robot::WristColor::NAME)
    //         .await
    //         .unwrap();
    //     let mut i: f32 = 0.0;
    //     loop {
    //         let new_color = std_msgs::ColorRGBA {
    //             r: 0.5 + 0.5 * i.sin(),
    //             g: 0.5 + 0.5 * (i + 2.0 * std::f32::consts::PI / 3.0).sin(),
    //             b: 0.5 + 0.5 * (i + 2.0 * std::f32::consts::PI / 3.0 * 2.0).sin(),
    //             a: 1.0,
    //         };
    //         println!("new_color: {:?}", new_color);
    //         for i in 0..20 {
    //             publisher.publish(&new_color).await.unwrap();
    //             tokio::time::sleep(std::time::Duration::from_millis(1)).await;
    //         }
    //         i += 0.1;
    //     }
    // }

    if true {
        let sub = topic::default_move_group::MoveResult::subscribe(&client)
            .await
            .unwrap();
        let sub2 = topic::default_move_group::MoveStatus::subscribe(&client)
            .await
            .unwrap();
        let sub3 = topic::default_move_group::MoveGroupInfo::subscribe(&client)
            .await
            .unwrap();
        let sub4 = topic::default_move_group::MoveFeedback::subscribe(&client)
            .await
            .unwrap();

        let publish_name = topic::default_move_group::MoveGoal::NAME;
        let publish = client
            .advertise::<commander_msgs::MotionActionGoal>(publish_name)
            .await
            .unwrap();

        let goal = MotionActionGoal {
            header: Default::default(),
            goal_id: actionlib_msgs::GoalID {
                stamp: Default::default(),
                id: format!(
                    "this-{}",
                    std::time::SystemTime::now()
                        .duration_since(std::time::SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_secs()
                ),
            },
            goal: MotionGoal {
                motion_sequence: [
                    MotionSequencePoint {
                        pose: Pose {
                            position: Point {
                                x: 0.0,
                                y: -0.01,
                                z: 0.0,
                            },
                            ..Default::default()
                        },
                        relative: true,
                        use_joint_space_target: false,
                        cartesian: true,
                        max_velocity: LinAng {
                            linear: 1.0,
                            angular: 3.14,
                        },
                        max_joint_acceleration: 1.0,
                        max_joint_velocity: 1.0,
                        frame_id: "".to_string(),
                        tcp_id: "tcp 1".to_string(),
                        use_nearest_joint_space_target: true,
                        joint_names: vec![],
                        joint_angles: vec![],
                        unwind_joint_target: true,
                        blend: LinAng {
                            linear: 0.01,
                            angular: 0.01,
                        },
                    },
                    MotionSequencePoint {
                        pose: Pose {
                            position: Point {
                                x: 0.0,
                                y: 0.01,
                                z: 0.0,
                            },
                            ..Default::default()
                        },
                        relative: true,
                        use_joint_space_target: false,
                        cartesian: true,
                        max_velocity: LinAng {
                            linear: 1.0,
                            angular: 3.14,
                        },
                        max_joint_acceleration: 1.0,
                        max_joint_velocity: 1.0,
                        frame_id: "".to_string(),
                        tcp_id: "tcp 1".to_string(),
                        use_nearest_joint_space_target: true,
                        joint_names: vec![],
                        joint_angles: vec![],
                        unwind_joint_target: true,
                        blend: LinAng {
                            linear: 0.01,
                            angular: 0.01,
                        },
                    },
                ]
                .to_vec(),
                ignore_scaling: false,
                end_effector: "tcp 1".to_string(),
            },
        };

        println!("{}", serde_json::to_string_pretty(&goal).unwrap());

        publish.publish(&goal).await.unwrap();

        let res_cb = |res: MotionActionResult| {
            if res.status.goal_id.id.eq(&"this") {
                println!("{:#?}", res);
            }
        };
        let status_cb = |res: actionlib_msgs::GoalStatusArray| {
            if res
                .status_list
                .iter()
                .any(|s| s.goal_id.id.contains("this"))
            {
                println!("{:#?}", res);
            }
        };
        let fb_cb = |res: MotionActionFeedback| {
            if res.status.goal_id.id.contains("this") {
                println!("{:#?}", res);
            }
        };

        loop {
            tokio::select! {
                msg = sub.next() => {
                    res_cb(msg)
                },
                msg = sub2.next() => {
                    status_cb(msg)
                },
                msg = sub3.next() => {
                    println!("{:?}", msg);
                }
                msg = sub4.next() => {
                    fb_cb(msg)
                }
            }

            // let msg = sub.next().await;
            // println!("{:#?}", msg);
        }
    }

    for i in 0..10 {
        println!("{}", "=".repeat(50));
    }

    if false {
        let sub = topic::default_move_group::MoveGoal::subscribe(&client)
            .await
            .unwrap();

        let sub2 = topic::robot::joint_trajectory_controller::Goal::subscribe(&client)
            .await
            .unwrap();
        let sub3 = topic::robot::scaled_joint_trajectory_controller::Goal::subscribe(&client)
            .await
            .unwrap();

        loop {
            tokio::select! {
                msg = sub.next() => {
                    println!("move goal => {:#?}", msg);
                }
                msg = sub2.next() => {
                    println!("jtc => {:#?}", msg);
                }
                msg = sub3.next() => {
                    println!("scaled => {:#?}", msg);
                }
            }
            // let msg = sub.next().await;

            // println!("{:?}", msg);
        }
    }

    return;

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
                linear: Vector3 {
                    z: 0.01,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .await
        .unwrap();
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    }
    for i in 0..1000 {
        jog.publish(&CartesianJogDemand {
            twist: Twist {
                linear: Vector3 {
                    z: -0.01,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
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
        .advertise::<CartesianJogDemand>("/default_move_group/cartesian_jog")
        .await
        .unwrap();

    for _ in 0..1000 {
        // info!("a");
        pubisher
            .publish(&CartesianJogDemand {
                twist: Twist {
                    linear: Vector3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.01,
                    },
                    angular: Vector3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                        // w: 0.0,
                    },
                },
                ..Default::default()
            })
            .await
            .unwrap();
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    for _ in 0..1000 {
        // info!("b");
        pubisher
            .publish(&CartesianJogDemand {
                twist: Twist {
                    linear: Vector3 {
                        x: 0.0,
                        y: 0.0,
                        z: -0.01,
                    },
                    angular: Vector3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                        // w: 0.0,
                    },
                },
                ..Default::default()
            })
            .await
            .unwrap();
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
}
