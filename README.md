# Todo TUI 

A **terminal-based Todo application** built using **Rust** and **TUI**, designed to be fast, safe, and keyboard-driven. This project demonstrates real-world Rust concepts such as state management, terminal UI rendering, and persistent storage.


##  Features

*  View all todo tasks in a clean terminal UI
*  Add todos with custom descriptions
*  Mark todos as completed / not completed
*  Delete selected todos
*  Persistent storage using JSON (auto-saved)
*  Fully keyboard-driven interface
*  Memory-safe and performant Rust code


## Demo 

```
┌──────────────────────── Todo List ────────────────────────┐
│ >> [ ] Learn Rust TUI                                     │
│    [ ] Finish assignment                                  |
│    [ ] Push project to GitHub                             |
|                                                           |
|                                                           |
|                                                           |
|                                                           |
|                                                           |
│                                                           |
└───────────────────────────────────────────────────────────┘
┌────────────────────────────────────────────────────────────┐
│ Add Todo (Enter = save, Esc = cancel)                      │
└────────────────────────────────────────────────────────────┘
```


##  Key Bindings

| Key     | Action                 |
| ------- | ---------------------- |
| `a`     | Add a new todo         |
| `Enter` | Save todo (input mode) |
| `Esc`   | Cancel input           |
| `↑ / ↓` | Navigate todos         |
| `Space` | Toggle complete        |
| `d`     | Delete selected todo   |
| `q`     | Quit application       |



##  Project Structure

```text
todo-tui/
├── src/
│   ├── main.rs      # App entry point & event loop
│   ├── app.rs       # Application state & logic
│   ├── ui.rs        # Terminal UI rendering
│   └── storage.rs   # Load / save todos (JSON)
├── Cargo.toml
├── .gitignore
└── README.md
```



##  Getting Started

### 1️ Prerequisites

* Rust (stable)
* Cargo

Check installation:

```bash
rustc --version
cargo --version
```



### 2️ Clone the Repository

```bash
git clone https://github.com/ravindran-dev/todo-tui.git
cd todo-tui
```



### 3️ Run the Application

```bash
cargo run
```

The app will launch directly inside your terminal.


##  Data Persistence

* Todos are automatically saved to a local `todos.json` file
* The file is ignored in Git (`.gitignore`)
* On restart, todos are restored automatically



##  Tech Stack

* **Rust** – core language
* **tui** – terminal UI rendering
* **crossterm** – keyboard & terminal handling
* **serde / serde_json** – serialization
* **uuid** – unique todo identifiers



##  Concepts Demonstrated

* Rust ownership & borrowing
* State-driven UI design
* Event-based input handling
* Terminal cursor control
* Modular Rust project structure


## Additional features added

- Inline Todo Editing - Edit existing tasks directly within the terminal.

- Priority System with Auto-Sorting - Assign High, Medium, or Low priority to tasks, which automatically reorder based on importance.

- Search & Filter Mode - Quickly find tasks using a live, keyword-based search.

- Theme Toggle - Switch between a neon-themed UI and a minimal theme at runtime.

- Delete Confirmation Popup - Prevent accidental deletions with a confirmation dialog.

- Improved Help Popup - Clean, well-structured keybindings guide for better usability.


##  Contributing

Contributions, issues, and feature requests are welcome.

1. Fork the repo
2. Create a feature branch
3. Commit changes
4. Open a pull request

## Screenshot
<img width="1918" height="1028" alt="image" src="https://github.com/user-attachments/assets/b7b9ed93-889a-4f4c-bdfe-c764537da58e" />


##  License

This project is open-source and available under the **MIT License**.



## Acknowledgements

Built as a hands-on Rust learning project focusing on **systems programming**, **UI design**, and **clean architecture**.

If you like this project, consider giving it a ⭐ on GitHub!

##  Author - **Ravindran S** 


Developer • ML Enthusiast • Neovim Customizer • Linux Power User  

Hi! I'm **Ravindran S**, an engineering student passionate about:

-  Linux & System Engineering  
-  AIML (Artificial Intelligence & Machine Learning)  
-  Full-stack Web Development  
-  Hackathon-grade project development  





## 🔗 Connect With Me

You can reach me here:

###  **Socials**
<a href="www.linkedin.com/in/ravindran-s-982702327" target="_blank">
  <img src="https://img.shields.io/badge/LinkedIn-0A66C2?style=for-the-badge&logo=linkedin&logoColor=white">
</a>


<a href="https://github.com/ravindran-dev" target="_blank">
  <img src="https://img.shields.io/badge/GitHub-111111?style=for-the-badge&logo=github&logoColor=white">
</a>


###  **Contact**
<a href="mailto:ravindrans.dev@gmail.com" target="_blank">
  <img src="https://img.shields.io/badge/Gmail-D14836?style=for-the-badge&logo=gmail&logoColor=white">
</a>
