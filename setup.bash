# !/bin/bash

install_humble_ros2() {
	# Install Rust, e.g. as described in https://rustup.rs/
	# Install ROS 2 as described in https://docs.ros.org/en/humble/Installation.html
	# Assuming you installed the minimal version of ROS 2, you need these additional packages:
	sudo apt install -y git libclang-dev python3-pip python3-vcstool # libclang-dev is required by bindgen
	# Install these plugins for cargo and colcon:
	cargo install --debug cargo-ament-build # --debug is faster to install
	python3.10 -m pip install git+https://github.com/colcon/colcon-cargo.git
	python3.10 -m pip install git+https://github.com/colcon/colcon-ros-cargo.git

	mkdir src
	git clone https://github.com/ros2-rust/ros2_rust.git src/ros2_rust
	vcs import src <src/ros2_rust/ros2_rust_humble.repos
	. /opt/ros/humble/setup.sh
	colcon build
}

# create helper function
help() {
	echo "Usage: $0 [humble]"
	echo "humble: Install ROS 2 with Rust"
}

# use install humbe ros2 if humble argument is passed
if [ $1 == "humble" ]; then
	install_humble_ros2
else
	help
fi
