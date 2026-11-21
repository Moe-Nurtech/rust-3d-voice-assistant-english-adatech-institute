# English Robot Voice Assistant (Rust, native desktop)

Press SPACE, speak, press SPACE again. The robot transcribes you (Whisper), asks your
Ollama model `ada-en`, speaks the answer with `en-GB-RyanNeural`, and plays its
talking animations while the voice plays. Drag with the mouse to rotate, wheel to zoom.

## One-time setup

1. **Install Rust**:
   - **macOS / Linux**: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
   - **Windows**: Download and run `rustup-init.exe` from https://rustup.rs/ (and install Visual Studio C++ Build Tools if prompted).

2. **Install Ollama, pull `qwen2.5:1.5b`, and create `ada-en`**:
   - Install Ollama from https://ollama.com/ (or via your package manager).
   - Pull the base model:
     ```bash
     ollama pull qwen2.5:1.5b
     ```
   - Create the `ada-en` model from the `Modelfile` inside the `ollama` folder:
     ```bash
     cd ollama
     ollama create ada-en -f ./Modelfile
     cd ..
     ```

3. **Expose Ollama** (if another machine/container/service needs access, or to bind beyond localhost):
   - Set `OLLAMA_HOST=0.0.0.0:11434` (and `OLLAMA_ORIGINS=*` if browser/cross-origin clients connect) before starting or restarting the Ollama service:
     - **Windows (PowerShell, user environment variable)**:
       ```powershell
       [Environment]::SetEnvironmentVariable("OLLAMA_HOST", "0.0.0.0:11434", "User")
       ```
       *(Restart the Ollama desktop app or service after setting environment variables.)*
     - **Linux (`systemd` override)**:
       ```bash
       sudo systemctl edit ollama.service
       ```
       Add under `[Service]`:
       ```ini
       [Service]
       Environment="OLLAMA_HOST=0.0.0.0:11434"
       ```
       Then reload and restart:
       ```bash
       sudo systemctl daemon-reload && sudo systemctl restart ollama
       ```
     - **macOS (`launchctl`)**:
       ```bash
       launchctl setenv OLLAMA_HOST "0.0.0.0:11434"
       ```
       *(Restart Ollama after running this.)*


   
4. **Build tools for Whisper** (`whisper-rs` compiles `whisper.cpp`):
   - **CMake**: https://cmake.org/download/ (tick "Add CMake to system PATH")
   - **LLVM / libclang** (needed by `bindgen`):
     - **Windows**: `winget install LLVM.LLVM`, then set `LIBCLANG_PATH` to `C:\Program Files\LLVM\bin`.
     - **Windows** also needs Visual Studio C++ Build Tools (Rust already asked for them).
     - **Linux**:
       ```bash
       sudo apt install build-essential cmake libclang-dev libasound2-dev libudev-dev pkg-config libwayland-dev libxkbcommon-dev
       ```
5. **Whisper model**:
   - Download `ggml-base.en.bin` from https://huggingface.co/ggerganov/whisper.cpp/tree/main and put it in `models/`.
6. **Robot**:
   - Copy `robot.glb` into `assets/`.

## Run

```bash
cargo run --release