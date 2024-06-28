# Experiment ground for ros2 rust

The goal is to start build essential component for a fully operated robot, using ros2 and rust.
Main components are :

- Perception 
- Manipulation
- Navigation
- Process manager
- Dialog

## Setup workspace

Make sure to have ros2 humble installed as well as rustup. Eventually a docker would be provided.

```bash
git submodule update --init --recursive
```

## TODO

- [ ]  Build a framework to do vision based model inference
  - [ ] Build a simple perception node doing object detection with yolov10
- [ ] Implement a Gaussian splatting SLAM
- [ ] Provide a docker for development with neovim.
  - [ ] Do an explanary video for the workflow
- [ ] Describe scenarios to test robot functionalities
- [ ] Provide a template for ros2 rust nodes
- [ ] Make it [open-rmf](https://github.com/open-rmf) complient
