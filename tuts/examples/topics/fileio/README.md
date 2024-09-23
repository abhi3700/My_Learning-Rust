# File I/O operations

## Coding

### Path

- In production code, use `PathBuf` insted of `Path` as the former is owned & the latter is borrowed.

### Create

### Read

### Delete

- `fs::remove_dir_all()`: Removes a directory and all its contents.
