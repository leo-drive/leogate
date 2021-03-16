use std::sync::Arc;

use futures::lock::Mutex;
use futures_util::stream::{Stream, StreamExt};

use async_graphql::*;

use ros_manager::RosManager;
use ros_msgs::*;

use crate::TopicInput;

#[derive(Clone, Default)]
pub struct TopicEchoSubscription;

#[Subscription]
impl TopicEchoSubscription {
    async fn topic_echo(
        &self,
        ctx: &Context<'_>,
        input: TopicInput,
    ) -> impl Stream<Item = Option<ros_msgs::RosMessage>> {
        ctx.data_unchecked::<Arc<Mutex<RosManager>>>()
            .lock()
            .await
            .subscribe(input.topic, input.msg_type)
            .await
            .map(|payload| {
                let json_str = payload.json.as_str();
                let msg: Option<ros_msgs::RosMessageType> = match serde_json::from_str(json_str) {
                    Ok(payload) => {
                        let ret: ros_msgs::RosMessageType = payload;
                        Some(ret)
                    }
                    Err(e) => {
                        println!("Error in payload => {}", e);
                        None
                    }
                };

                let msg_type = &msg.as_ref().unwrap().msg_type;
                match msg_type.as_str() {
                    "std_msgs/Int8" => match serde_json::from_str::<std_msgs::Int8>(json_str) {
                        Ok(d) => Some(d.into()),
                        Err(e) => {
                            println!("Error in payload body std_msgs/Int8 => {}", e);
                            None
                        }
                    },
                    "std_msgs/Int16" => match serde_json::from_str::<std_msgs::Int16>(json_str) {
                        Ok(d) => Some(d.into()),
                        Err(e) => {
                            println!("Error in payload body std_msgs/Int16 => {}", e);
                            None
                        }
                    },
                    "std_msgs/Int32" => match serde_json::from_str::<std_msgs::Int32>(json_str) {
                        Ok(d) => Some(d.into()),
                        Err(e) => {
                            println!("Error in payload body std_msgs/Int32 => {}", e);
                            None
                        }
                    },
                    "std_msgs/Int64" => match serde_json::from_str::<std_msgs::Int64>(json_str) {
                        Ok(d) => Some(d.into()),
                        Err(e) => {
                            println!("Error in payload body std_msgs/Int64 => {}", e);
                            None
                        }
                    },
                    _ => None,
                }
            })
    }
}
