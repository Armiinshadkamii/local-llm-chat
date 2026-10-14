# 🦀 Local LLM Chat

A lightning-fast, lightweight, and native desktop application for chatting with local Large Language Models (LLMs). 

Built entirely in **Rust** using `egui` for the UI and `llama-cpp-2` for inference, this project offers a high-performance alternative to heavy, Electron-based or Python-based local LLM interfaces.

![Demo Screenshot](screenshot.png)  
*(📸 Tip: Replace `screenshot.png` with an actual screenshot or GIF of your app running!)*

---

## ✨ Why Rust?

Most local LLM chat UIs are built with Python (Gradio/Streamlit) or web technologies (Electron/Tauri), which can introduce significant memory overhead and slower startup times. 

This project leverages Rust to provide:
- ⚡ **Instant Startup**: Native compilation means the app opens in milliseconds.
- 🧠 **Low Memory Footprint**: Minimal overhead leaves more RAM available for the actual LLM.
- 🖥️ **True Native UI**: `egui` provides immediate-mode rendering that is incredibly responsive, even while the model is generating tokens.
- 🔒 **Privacy First**: 100% offline. Your prompts and model weights never leave your machine.

---

## 🚀 Features

- **Simple Model Loading**: Point to any local `.gguf` file and load it directly into memory.
- **GPU Acceleration**: Configurable GPU layer offloading (defaults to 35 layers for optimal performance).
- **Chat History**: Clean, scrollable interface that keeps track of your conversation.
- **Custom Prompt Templates**: Currently optimized for **Qwen 2.5 Instruct** models, but easily adaptable to other architectures (Llama 3, Mistral, etc.).
- **Error Handling**: Graceful UI feedback if a model path is invalid or generation fails.

---

## 🛠️ Prerequisites

Because this project uses `llama-cpp-2` (Rust bindings for `llama.cpp`), you will need a C++ compiler installed on your system to build the native dependencies.

- **Windows**: Install [Build Tools for Visual Studio](https://visualstudio.microsoft.com/visual-cpp-build-tools/) (make sure "C++ build tools" is checked).
- **macOS**: Install Xcode Command Line Tools (`xcode-select --install`).
- **Linux**: Install `build-essential` (`sudo apt install build-essential`).
- **Rust**: Ensure you have the latest Rust toolchain installed ([rustup.rs](https://rustup.rs/)).

---

## 📦 Installation & Running

1. **Clone the repository**:
   ```bash
   git clone https://github.com/Armiinshadkamii/local-llm-chat.git
   cd local-llm-chat
   ```

2. **Download a Model**: 
   Download a `.gguf` model (e.g., [Qwen2.5-7B-Instruct-GGUF](https://huggingface.co/Qwen/Qwen2.5-7B-Instruct-GGUF)) and note its local path.

3. **Update the Default Path (Optional)**:
   Open `src/main.rs` and update the `model_path` in the `Default` implementation to point to your downloaded `.gguf` file.

4. **Build and Run**:
   ```bash
   cargo run --release
   ```
   *(Note: Always use `--release` for LLM inference to get maximum performance and enable GPU optimizations).*

---

## ⚙️ Configuration

You can easily tweak the inference behavior by modifying `src/model.rs`:

- **GPU Layers**: Change `.with_n_gpu_layers(35)` to match your VRAM capacity. Set to `0` for CPU-only inference.
- **Context Window**: Adjust `.with_n_ctx(Some(NonZeroU32::new(2048)))` to increase or decrease the memory allocated for the conversation history.
- **Prompt Template**: The chat loop in `main.rs` currently uses the `<|im_start|>` format for Qwen. If you switch to a Llama 3 model, update the prompt formatting string accordingly.

---

## 🧰 Tech Stack

- **Language**: Rust
- **UI Framework**: `eframe` / `egui`
- **LLM Inference**: `llama-cpp-2` (Bindings for `llama.cpp`)
- **Model Format**: GGUF (Quantized)

---

## 🤝 Contributing & Freelance Work

This project is open source and contributions are welcome! If you find a bug or want to add a feature (like file-picker dialogs, multi-model support, or different prompt templates), feel free to open an Issue or Pull Request.

> 💼 **Need a custom local AI tool?**  
> I specialize in building high-performance, low-overhead Rust applications for local AI and desktop tooling. If you need a custom feature built or a similar tool tailored to your workflow, feel free to reach out via [Your Email / Twitter / Telegram].

---

## 📄 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.