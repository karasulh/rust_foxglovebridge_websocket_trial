# Rust client foxglovebridge ROS2 Usage 

TODO
NOT WORKING YET

This repo contains the basic usage of foxglovebridge. I added this because its repo seems a bit complicated for the first viewers. 
Repo implements Rust client of foxglovebridge to speak with ROS/ROS2 domains via JSON messages.

Topic (ROS2 Domain) <=> JSON (in Rust with Rust handler)

To use foxglovebridge client in Rust; 

- ###firstly give ROS2 messages addresses to roslibrust_codegen to copy this topic type msgs into Rust data structures
- subscribe to and publish the suitable topics
- run foxglovebridge server in the side of ROS2 as in foxglovebridge_start_command.txt
- run ROS2 any node like "pubsubint" in my other repo: "VSCode_ROS2_DevContainer_FullTemplate": ros2_ws/src/pubsubint and trial_interfaces msg : ros2 run pubsubint pubsubint_node
- run Rust foxglovebridge side: cargo run