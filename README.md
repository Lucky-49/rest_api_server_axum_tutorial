All comments are in Russian.

To run testing on Windows, you need to run the Axum server in the IDE or terminal. Next, you need to launch the second terminal, go to the project directory and run the command:

cargo test --target-dir="target/test" -- --nocapture
