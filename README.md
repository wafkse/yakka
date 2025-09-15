# Yakka

Yakka is the backing code and firmware for my final school project.

# WARNING

This project was done in just 2 weeks, so the code is incomplete and may have bugs.

# Architecture

There are three primary directories:

- `host`: Host-specific crates, such as the remote control interface.
- `shared`: Platform-agnostic crates or crates which do not directly depend on low-level firmware details
- `firmware`: Code destined for use in the drone only. 

# Features

- Type-checked units of measure (time, distance, power, etc.), and conversion between them.
- Bit manipulation libraries.
- Fixed-point arithmetic.
- Device drivers for the **BMI160** and **LSM6DS3** inertial measurement units.
- Generic motor and powerplant control.
- PID and generic closed/open control cycle interfaces.
- WiFi AP and TCP server stack for in-flight communication with the drone.
- Log forwarding over the network and USB.
- Peripheral distribution managed by custom macro.
- Control vector mixing and power output management.
- Generic ESC drivers.

# License

Copyright (C) 2025 W. Frakchi

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

See [the full license agreement](LICENSE.md) for further information.