# 🚀 Simple Rust To-Do List

A minimalist, high-performance, and bulletproof Command Line Interface (CLI) To-Do List application built from scratch using **Rust**. This project is modularly structured, robustly engineered against crash-prone user inputs, and optimized for cross-platform terminal management.

---

## ✨ Features

* **⚡ Robust Input Handling:** Custom string and integer sanitization logic that loops automatically until valid data is provided—completely memory-safe and zero crashes!
* **🗑️ Intuitive Task Management:** Seamlessly add, view, and delete tasks dynamically using dynamic vectors (`Vec<String>`).
* **🔀 Smart Indexing Converter:** Converts human-friendly 1-based indexing (`1, 2, 3`) into programmer-friendly 0-based indexing underneath safely.
* **🧼 Clean Console Experience:** Autodetects operating systems (`cfg!(target_os)`) to execute platform-specific commands (`cls` for Windows, `clear` for Unix/Linux/Mac) before rendering.
* **🛑 Graceful Interruption:** Handles EOF (`Ok(0)`) or stream breaks elegantly, ensuring proper terminal exit.

---

## 📂 Project Structure

```text
src/
├── main.rs              # App initializer and central state control loop
├── to_do_list.rs        # Core backend struct (ToDoList) and business logic methods
├── input_handler.rs     # Bulletproof user input parsers (String & i32)
└── helper.rs            # Cross-platform utility commands (Screen Clear)
