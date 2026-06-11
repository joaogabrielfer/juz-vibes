# Toolchain Overview

The toolchain area covers the pieces that are neither pure syntax nor ordinary standard
library surface.

Current toolchain concepts:

- `pyx` as the CLI entry point
- `pvm` as the SLUR bytecode runtime
- plugins as activated compilation surfaces
- build scripts and initializers
- compile-time module and package processing

Use this area for workflow and lifecycle questions such as:

- how a package is built
- how a plugin is loaded
- where compiler helpers live
- how bytecode is launched

See:

- [[build-system-and-cli]]
- [[plugins]]
- [[compiler-pipeline]]
- [[runtime-pvm]]
