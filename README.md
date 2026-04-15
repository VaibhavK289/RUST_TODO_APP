# Rust Todo List

A minimalistic, standard-library-only Todo List Application written in Rust.

## Features
- Add tasks
- List tasks
- Complete tasks
- Remove tasks
- Zero external dependencies
- Persistent local storage (`.todo.txt`)

## Usage

Navigate to the `todo-cli` directory:
```bash
cd todo-cli
```

### Add a Task
```bash
cargo run -- add "Buy groceries"
```

### List Tasks
```bash
cargo run -- list
```

### Mark as Done
```bash
cargo run -- done 1
```

### Remove a Task
```bash
cargo run -- remove 1
```
