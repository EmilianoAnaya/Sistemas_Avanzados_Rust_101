@echo off
echo "Program in Execution, please don't close this window"
echo "If you want to end this, close the window."

d:
cd Codes/Rust/Sistemas_Avanzados_Rust_101/system_monitor

cargo build
cargo run
pause