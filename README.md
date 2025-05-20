Monitoring Keyboard Plug/Unplug Events in Rust on Windows.

To detect when a keyboard is plugged or unplugged in Windows and display notifications in a Rust terminal, 
you'll need to use Windows API functions through Rust's FFI (Foreign Function Interface).
Here's how to implement this:

[dependencies]
winapi = { version = "0.3", features = ["winuser", "dbt", "libloaderapi"] }
widestring = "0.4"
