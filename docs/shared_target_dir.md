---
title: Shared Target Directory for Rust Projects
---

This document guides through how to manage target folders in Rust projects more efficiently by using a **shared target directory**. This approach helps reduce disk space consumption and avoid repetitive libraries across multiple projects.

### Setting up a Shared Target Directory

To set up a shared target directory for all your Rust projects, follow these steps:

1. Open or create the global Cargo configuration file located at `~/.cargo/config`.
2. Add the following lines to the file:

   ```
   [build]
   target-dir = "/path/to/shared/target/directory"
   ```

   Replace `/path/to/shared/target/directory` with the desired path for your shared target directory.

### Creating a New Project

When creating a new Rust project, you don't need to mention anything in the local `Cargo.toml` file regarding the shared target directory. The global Cargo configuration file (`~/.cargo/config`) takes care of setting the shared target directory for all your projects.

### Limitations

Using a shared target directory works well for most cases, but there are some limitations:

- `cargo clean` will delete the entire target directory, preventing you from cleaning a specific project.

### Alternative Approach

Another approach to manage target folders is to periodically delete them using a Rust app or script that recursively searches for and removes 'target' directories. This can help you reclaim disk space, but keep in mind that you'll need to rebuild your projects after deleting the target folders.

### Conclusion

Using a shared target directory is a practical solution for managing target folders in Rust projects. However, be aware of the limitations and consider alternative approaches if needed.
