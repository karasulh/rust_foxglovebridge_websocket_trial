use futures_util::{future, pin_mut, stream::{SplitSink, SplitStream}, SinkExt, StreamExt};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, WebSocketStream};
use serde::{Serialize, Deserialize};
use serde_json::{json,Value};


// Define a Foxglove Subscribe Request
#[derive(Serialize)]
struct SubscribeRequest {
    #[serde(rename = "op")]
    operation: String,
    id: String,
    topic: String,
}

// Define a Foxglove Message structure for publishing data
#[derive(Serialize)]
struct FoxglovePublish {
    #[serde(rename = "op")]
    operation: String,
    id: String,
    topic: String,
    msg: serde_json::Value,
}

#[derive(Deserialize, Debug)]
struct FoxgloveResponse {
    topic: String,
    data: String,
}

async fn websocket_client() {
    // Connect to the Foxglove Bridge WebSocket server
    let url = "ws://localhost:8765";
    let protocol =  ["foxglove.websocket.v1"];
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    let (mut write, mut read) = ws_stream.split();

    // Subscribe to the ROS2 topic "topic"
    let subscribe_msg = SubscribeRequest {
        operation: "subscribe".to_string(),
        id: "1".to_string(),
        topic: "topic".to_string(),
    };
    let subscribe_json = serde_json::to_string(&subscribe_msg).unwrap();
    write.send(Message::Text(subscribe_json)).await.unwrap();

    // Read incoming messages from the WebSocket
    tokio::spawn(async move {
        while let Some(message) = read.next().await {
            if let Ok(msg) = message {
                if let Message::Text(text) = msg {
                    let response: FoxgloveResponse = serde_json::from_str(&text).unwrap();
                    println!("Received data from topic {}: {:?}", response.topic, response.data);
                }
            }
        }
    });

    // Publish an Int32 message to ROS2 topic "topic"
    let int_msg = json!({
        "data": 42  // Example data, Int32 ROS message structure
    });

    let publish_msg = FoxglovePublish {
        operation: "publish".to_string(),
        id: "2".to_string(),
        topic: "topic".to_string(),
        msg: int_msg,
    };

    let publish_json = serde_json::to_string(&publish_msg).unwrap();
    write.send(Message::Text(publish_json)).await.unwrap();
}

#[tokio::main]
async fn main() {
    websocket_client().await;
}




// #[tokio::main]
// async fn main() {
//     let url : &str = "ws://localhost:8765";
//     let (mut tx_ch, mut rx_ch) = futures_channel::mpsc::unbounded();

//     let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
//     println!("WebSocket handshake has been successfully completed");

//     let (mut sender_ws, mut reader_ws) = ws_stream.split();


//     let subscribe_msg = json!(
//         {
//             "op": "subscribe",
//             "topic": "/topic",
//             "type": "std_msgs/Int32",
//         }
//     );
//     let subscribe_msg = Message::Text(subscribe_msg.to_string());
//     println!("Sending subscription: {:?}", &subscribe_msg);
//     sender_ws.send(subscribe_msg);

//     // Spawn a task to process incoming messages
//     tokio::spawn(async move {
//         while let Some(Ok(msg)) = reader_ws.next().await {
//             if let Message::Text(text_msg) = msg {
//                 // Send message to the channel for parallel processing
//                 if tx_ch.unbounded_send(text_msg).is_err() {
//                     println!("Receiver dropped");
//                     return;
//                 }
//             }
//         }
//     });

//     // Spawn tasks to process messages concurrently from the channel
//     while let Some(msg) = rx_ch.next().await {
//         // Spawn a new task for each message to process them in parallel
//         tokio::spawn(handle_message(msg));
//     }

//     //Handle incoming messages in a seperate task
//     //let read_handle = tokio::spawn(handle_incoming_messages(rx_ch));
//     //let _ = tokio::try_join!(read_handle);


//     //pin_mut!(rxch_to_ws, ws_to_txch);
//     //future::select(rxch_to_ws, ws_to_txch).await;
// }




// async fn handle_incoming_messages(mut rx_ch:futures_channel::mpsc::UnboundedReceiver<Message>){
    
//     while let Some(message) = rx_ch.next().await{
//         if let Ok(msg) = message.into_text(){
//             tokio::spawn(handle_message(msg));
//         }
       
//     }
// }

// async fn handle_message(msg:String){
//     println!("Handling message: {:?}",msg);
//     //let v  = serde_json::from_str(&msg).unwrap();
// }

// async fn send_msg(write: &mut SplitSink<WebSocketStream<impl AsyncWrite + AsyncRead + Unpin>, Message>,msg: &str){
//     println!("Sending a message {}",msg);
//     let msg = Message::Text(msg.into());
//     write.send(msg).await.expect("Failed to send msg");
// }


//#####TOKIO TUNGSTENITE WEBSOCKET EXAMPLE


// use futures_util::{stream::{SplitSink, SplitStream}, SinkExt, StreamExt};
// use tokio::io::{self, AsyncBufReadExt, AsyncRead, AsyncWrite};
// use tokio_tungstenite::{connect_async, tungstenite::Message, WebSocketStream};


// async fn read_and_send_messages(mut write: SplitSink<WebSocketStream<impl AsyncRead + AsyncWrite + Unpin>, Message>) {
//     let mut reader = io::BufReader::new(io::stdin()).lines();
//     while let Some(line) = reader.next_line().await.expect("Failed to read line") {
//         if !line.trim().is_empty() {
//             write.send(Message::Text(line)).await.expect("Failed to send message");
//         }
//     }
// }

// async fn handle_incoming_messages(mut read: SplitStream<WebSocketStream<impl AsyncWrite + AsyncRead + Unpin>>){
    
//     while let Some(message) = read.next().await{
//         match message {
//             Ok(msg) => println!("Received message: {}",msg),
//             Err(e) => eprintln!("Failed to read message, Error receiving message {}",e)
//         }
//     }
// }

// async fn send_msg(write: &mut SplitSink<WebSocketStream<impl AsyncWrite + AsyncRead + Unpin>, Message>,msg: &str){
//     println!("Sending a message {}",msg);
//     let msg = Message::Text(msg.into());
//     write.send(msg).await.expect("Failed to send msg");
// }

// #[tokio::main]
// async fn main(){

//     let url : &str = "wss://echo.websocket.events";
//     println!("Connecting to {}",url);

//     let(ws_stream,_) = connect_async(url).await.expect("failed to conenect");
//     println!("Connected to URL");

//     let (mut write, mut read) = ws_stream.split();
//     let msg = "aloha echo server".into();

//     if let Some(message) = read.next().await{
//         let message = message.expect("Failed to read message");
//         println!("Received message {}",message);
//     }

//     send_msg(&mut write, msg).await;

//     if let Some(message) = read.next().await{
//         let message = message.expect("Failed to read message");
//         println!("Received message {}",message);
//     }

//     //Handle incoming messages in a seperate task
//     let read_handle = tokio::spawn(handle_incoming_messages(read));

//     // Read from command line and send messages
//     let write_handle = tokio::spawn(read_and_send_messages(write));

//     let _ = tokio::try_join!(read_handle,write_handle);
// }



//#####ROSLIBRUST ROSBRIDGE EXAMPLE



// use log::*;
// use roslibrust::ClientHandle;
// use std::time::Duration;
// use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

// roslibrust_codegen_macro::find_and_generate_ros_messages!("/Users/mbkara/Documents/_3_Projects/_15_ROS2/ros2_common_interfaces/std_msgs",
//                                                           "/Users/mbkara/Documents/_3_Projects/_15_ROS2/humble_docker/ros2_ws/src/trial_interfaces");

// /// To run this example a rosbridge websocket server should be running at the deafult port (8765).
// #[tokio::main]
// async fn main() -> Result<(), anyhow::Error> {
//     simple_logger::SimpleLogger::new()
//         .with_level(log::LevelFilter::Debug)
//         .without_timestamps() // required for running wsl2
//         .init()
//         .unwrap();

//     let client = ClientHandle::new("ws://localhost:9090").await?;
//     let publisher = client.advertise::<std_msgs::Int32>("topic2").await?;
//     let publisher_number = client.advertise::<trial_interfaces::Numbers>("topic4").await?;

//     let subscriber = client.subscribe::<std_msgs::Int32>("topic").await?;
//     info!("Successfully subscribed to topic: topic");

//     let subscriber_number  = client.subscribe::<trial_interfaces::Numbers>("topic3").await?;
//     info!("Successfully subscribed to topic: topic3");

//     let mut i:i32 = 0;

//     tokio::spawn(async move{
//         loop{
//             let msg = subscriber.next().await;
//             info!("Got msg: {:?}", msg);

//             let msg_number = subscriber_number.next().await;
//             info!("Got msg Number: {:?}", msg_number);
        
//             //handle_message(msg).await;
//             tokio::spawn(handle_message_number(msg_number));
//         }
//     });

//     loop {

//         i+=1;
//         let msg = std_msgs::Int32{
//             data: i
//         };
//         //info!("About to publish");
//         let result = publisher.publish(msg).await;
//         match result {
//             Ok(()) => {
//                 info!("Published msg!");
//             }
//             Err(e) => {
//                 error!("Failed to publish msg: {e}");
//             }
//         }

//         let msg_number = trial_interfaces::Numbers{
//             number1: i,
//             number2: i as f32*1.2,
//             number3: std_msgs::Int16{
//                 data: i as i16*2
//             }
//         };
//         //info!("About to publish number");
//         let result = publisher_number.publish(msg_number).await;
//         match result {
//             Ok(()) => {
//                 info!("Published msg number!");
//             }
//             Err(e) => {
//                 error!("Failed to publish msg number: {e}");
//             }
//         }

//         tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
//     }

// }

// // subscription message handler
// async fn handle_message_number(msg: trial_interfaces::Numbers) {
//     // Simulate processing time or some heavy computation
//     tokio::time::sleep(Duration::from_millis(50)).await;
//     info!("Processed message: {:?}", msg);
// }