Drop-in alternative WebSocket connector for [buttplug](https://buttplug.io) servers and clients, without the dependency on `tokio/net`.

This became useful to me while writing [SmashPlug](https://github.com/ashkitten/smashplug), due to `socket2` and `mio` not supporting the Skyline toolchain for Switch mods. Perhaps it will be useful to someone else.
