# Launchpad Pro Userspace Keyboard Driver
This application allows you to map MIDI Devices to Keyboard Inputs making them effective computer keyboards.

### Configuration
```toml
[device]
# Input and output ports. Will prompt for entry if left blank
#input = "Launchpad Pro MK3 LPProMK3 MIDI"
#output = "Launchpad Pro MK3 LPProMK3 MIDI"

[mapping]

# Refer to https://docs.rs/rdev/latest/rdev/enum.Key.html for a list of possible keyboard values
A3 = "KeyW"
E3 = "KeyA"
F3 = "KeyS"
FS3 = "KeyD"
C6 = "KeyE"
D6 = "KeyF"
E5 = "Escape"
```

### TODOs
- Mapper Tool
- Real Gamepad Emulation on Linux
- Extended Mapping Support for more than Keyboards