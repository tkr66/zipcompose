## Installation

```sh
cargo install zipcompose
```

## Configuration

Here is an example of a zip-compose.yaml file:

```yaml
archives:
  test:
    filename: test.zip
    entries:
      - dest_dir: .
        files:
          - ./src/main.rs
          - src: ./src/main.rs
            dest: renamed_main.rs
```
