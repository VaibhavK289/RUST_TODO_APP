# Storage Architecture 
The application persists data utilizing pure Rust standard libraries (`std::fs`, `std::io`) to read/write into a plain `.todo.txt` file inside the local working directory. Each line represents an individual task formatted string.

# Components
- **CLI Arg Parser**: Utilizes `std::env::args` to retrieve and switch between valid commands (`add`, `list`, `done`, `remove`). No third-party parser crate is utilized.
- **I/O Engine**: File creation, appending, and updating utilize standard `std::fs::OpenOptions` arrays, regenerating the list state seamlessly based on user triggers.
