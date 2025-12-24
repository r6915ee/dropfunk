[egui]: https://github.com/emilk/egui
[Funkin']: https://funkin.me/
[Friday Night Funkin']: https://funkin.me/
[Rust]: https://rust-lang.org/

# Dropfunk

> _A transparent engine manager for [Funkin']_

[Friday Night Funkin'], as an open-source game, has a very extensive modding
community. Mods are typically provided through _engines_ that provide varying
mod support.

This comes at a cost, however. Engines almost never act the same as other
engines or even their own forks. That, and they are never packaged into a
package management system due to there being none. Most engines also place
modpacks into specialized directories _under that version's directory_, so
modpacks aren't in one place and may use up extra disk space if they're copied.
Developers can't test modpacks with multiple versions, and players can't use
specific versions easily.

_Dropfunk_ aims to remedy this issue. It silently wraps engine versions into
specialized directories and creates a symbolic link to a universal mods
directory specific to that engine. From there, it is capable of launching
different engine versions with the same mods directory, reducing the amount of
disk usage overall. It also allows engines and modpacks to provide custom
metadata so that they can get prettily displayed in the GUI.

## Features

- [Rust] and [egui]-based for performance and safety
- Manage multiple engine versions simultaneously
- Modpacks are shared across engine versions
- Extensive metadata support
  - Unified metadata using JSON for both engines and modpacks
  - Multiple optional values that can be used for UIs
- Main backend is separate from the frontend, allowing third-party tools

todo
