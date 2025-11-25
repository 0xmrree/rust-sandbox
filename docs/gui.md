# Rust UI Libraries & Frameworks Guide

## Overview
Rust's UI ecosystem is growing rapidly. Here's a breakdown of the main options organized by use case.

---

## Desktop GUI Frameworks

### 1. **egui** ⭐ (Recommended for Beginners)
**Best for:** Tools, editors, debug UIs, immediate mode GUIs

```rust
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click me").clicked() {
                println!("Button clicked!");
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}
```

**Pros:**
- Very easy to learn (immediate mode)
- Great docs and examples
- Cross-platform (Windows, macOS, Linux, Web via WASM)
- Fast iteration, no need to manage state carefully
- Active development

**Cons:**
- Not native look and feel
- Limited styling compared to web frameworks
- Immediate mode may feel unusual coming from retained mode (React, WPF)

**Similar to:** Dear ImGui (C++)

---

### 2. **Tauri** ⭐ (Recommended for Modern Desktop Apps)
**Best for:** Cross-platform desktop apps with web frontend

Tauri lets you build desktop apps using web technologies (HTML/CSS/JS) for the frontend and Rust for the backend.

**Architecture:**
```
Frontend (HTML/CSS/JS/React/Vue/Svelte)
           ↕ IPC
Backend (Rust) - handles system access, business logic
```

**Example project structure:**
```rust
// src-tauri/src/main.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

```javascript
// Frontend (React/Vue/Svelte/Vanilla JS)
import { invoke } from '@tauri-apps/api/tauri'

// Call Rust backend
const greeting = await invoke('greet', { name: 'World' })
console.log(greeting)
```

**Pros:**
- Use familiar web technologies for UI
- Much smaller bundle than Electron (~3MB vs ~80MB)
- Native system access from Rust
- Excellent documentation
- Very active community

**Cons:**
- Requires knowledge of both Rust and web tech
- Not pure Rust (if that matters to you)

**Similar to:** Electron, but lighter and more secure

---

### 3. **Slint** (formerly SixtyFPS)
**Best for:** Embedded systems, resource-constrained devices, modern desktop apps

```rust
slint::slint! {
    export component MainWindow inherits Window {
        in-out property<string> greeting: "Hello World";
        
        VerticalBox {
            Text {
                text: greeting;
                font-size: 24px;
            }
            
            HorizontalBox {
                LineEdit {
                    text <=> greeting;
                }
                Button {
                    text: "Reset";
                    clicked => { greeting = "Hello World"; }
                }
            }
        }
    }
}

fn main() {
    MainWindow::new().unwrap().run().unwrap();
}
```

**Pros:**
- Declarative UI language (similar to QML)
- Hardware accelerated
- Very low resource usage
- Good for embedded systems
- Hot reload during development

**Cons:**
- Smaller ecosystem than egui/Tauri
- Custom markup language to learn
- Less styling flexibility than web-based solutions

**Similar to:** Qt/QML

---

### 4. **Iced**
**Best for:** Elm-inspired, type-safe, reactive UIs

```rust
use iced::{Element, Sandbox, Settings, TextInput, button, Button, Column, Text};

pub fn main() -> iced::Result {
    Counter::run(Settings::default())
}

struct Counter {
    value: i32,
}

#[derive(Debug, Clone)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self { value: 0 }
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => self.value += 1,
            Message::DecrementPressed => self.value -= 1,
        }
    }

    fn view(&self) -> Element<Message> {
        Column::new()
            .push(Button::new(Text::new("Increment")).on_press(Message::IncrementPressed))
            .push(Text::new(self.value.to_string()).size(50))
            .push(Button::new(Text::new("Decrement")).on_press(Message::DecrementPressed))
            .into()
    }
}
```

**Pros:**
- Type-safe, predictable state management (Elm architecture)
- Cross-platform
- Responsive and reactive
- Good for complex state management

**Cons:**
- Still evolving (API changes)
- Smaller community than egui/Tauri
- Learning curve for Elm architecture

**Similar to:** Elm, Flutter's widget system

---

### 5. **GTK-rs** (GTK 4 bindings)
**Best for:** Linux-native apps, GNOME ecosystem

```rust
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

fn main() {
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Hello, World!")
            .build();

        let button = Button::with_label("Click me!");
        button.connect_clicked(|_| {
            println!("Button clicked!");
        });

        window.set_child(Some(&button));
        window.present();
    });

    app.run();
}
```

**Pros:**
- Native Linux/GNOME integration
- Mature, stable toolkit
- Good documentation

**Cons:**
- GTK dependency can be large
- Windows/macOS support exists but less polished
- C-style API can feel awkward in Rust

**Similar to:** Native GTK (C/Python)

---

### 6. **Dioxus** ⭐
**Best for:** React developers, cross-platform apps (Desktop, Web, Mobile)

```rust
use dioxus::prelude::*;

fn main() {
    dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    cx.render(rsx! {
        div {
            h1 { "High-Five counter: {count}" }
            button { 
                onclick: move |_| count += 1,
                "Up high!" 
            }
            button { 
                onclick: move |_| count -= 1,
                "Down low!" 
            }
        }
    })
}
```

**Pros:**
- React-like API (hooks, components, JSX-like syntax)
- Write once, run everywhere (Desktop, Web, Mobile, SSR)
- Great for React developers transitioning to Rust
- Hot reload support
- Active development

**Cons:**
- Still relatively young
- Mobile support is experimental
- Smaller ecosystem than React

**Similar to:** React, React Native

---

## Web Frameworks (WebAssembly)

### 1. **Leptos** ⭐
**Best for:** Full-stack web apps, server-side rendering

```rust
use leptos::*;

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    view! { cx,
        <button on:click=move |_| set_count.update(|n| *n += 1)>
            "Click me: " {count}
        </button>
    }
}

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}
```

**Pros:**
- Fine-grained reactivity (like SolidJS)
- Server-side rendering
- Very fast
- Small bundle sizes

**Cons:**
- Young ecosystem
- Fewer learning resources
- Still evolving

---

### 2. **Yew**
**Best for:** Single-page web applications

```rust
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
```

**Pros:**
- Component-based (React-like)
- Mature for Rust web frameworks
- Good documentation
- Multi-threaded web apps possible

**Cons:**
- Bundle sizes can be large
- Performance overhead vs vanilla JS
- Learning curve

---

## Game UI / Immediate Mode

### **egui** (mentioned above)
Great for game debug tools, level editors, etc.

### **imgui-rs**
Rust bindings for Dear ImGui - the industry standard for game tools.

```rust
use imgui::*;

fn main() {
    let mut value = 0.0f32;
    
    ui.window("Hello world")
        .size([300.0, 100.0], Condition::FirstUseEver)
        .build(|| {
            ui.text("Hello world!");
            ui.slider("Slider", 0.0, 100.0, &mut value);
            if ui.button("Click me") {
                println!("Button clicked!");
            }
        });
}
```

---

## Terminal UI (TUI)

### **Ratatui** ⭐
**Best for:** CLI tools, terminal-based interfaces

```rust
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io;

fn main() -> io::Result<()> {
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    
    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default()
            .title("My TUI App")
            .borders(Borders::ALL);
        let paragraph = Paragraph::new("Hello, Ratatui!")
            .block(block);
        f.render_widget(paragraph, size);
    })?;
    
    Ok(())
}
```

**Pros:**
- Rich terminal UIs
- Cross-platform
- Good for CLI tools and dashboards
- Active development

**Examples:** htop-like tools, database CLIs, monitoring dashboards

---

## Quick Comparison Table

| Framework | Platform | Style | Difficulty | Use Case |
|-----------|----------|-------|------------|----------|
| **egui** | Desktop, Web | Immediate | ⭐ Easy | Tools, editors, prototypes |
| **Tauri** | Desktop | Web-based | ⭐⭐ Medium | Modern desktop apps |
| **Dioxus** | Multi | Retained | ⭐⭐ Medium | Cross-platform, React devs |
| **Slint** | Desktop, Embedded | Declarative | ⭐⭐ Medium | Embedded, low-resource |
| **Iced** | Desktop, Web | Elm-style | ⭐⭐⭐ Hard | Complex state management |
| **GTK-rs** | Desktop (Linux) | Retained | ⭐⭐⭐ Hard | GNOME apps |
| **Leptos** | Web | Reactive | ⭐⭐⭐ Hard | Full-stack web apps |
| **Yew** | Web | Component | ⭐⭐ Medium | SPAs in Rust |
| **Ratatui** | Terminal | Immediate | ⭐⭐ Medium | CLI tools |

---

## Recommendations by Experience Level

### Just Learning Rust?
**Start with:** egui
- Easiest to learn
- Quick feedback loop
- Focus on Rust, not UI complexity

### Coming from Web Development?
**Start with:** Tauri or Dioxus
- Leverage existing web skills
- Tauri if you want to keep using React/Vue/Svelte
- Dioxus if you want pure Rust

### Building Production Desktop Apps?
**Choose:** Tauri (most mature) or Slint (if embedded/low-resource)

### Building Web Apps?
**Choose:** Leptos (modern, fast) or Yew (more mature)

### Building CLI Tools?
**Choose:** Ratatui

---

## Getting Started Example: egui

Create a new project:
```bash
cargo new my-gui-app
cd my-gui-app
```

Add to `Cargo.toml`:
```toml
[dependencies]
eframe = "0.28"
egui = "0.28"
```

Replace `src/main.rs` with the egui example from above, then:
```bash
cargo run
```

You'll have a working GUI app in seconds!

---

## Real-World Examples

### Built with egui:
- **Rerun**: ML visualization tool
- Many game dev tools and editors

### Built with Tauri:
- **GitButler**: Git client
- **Clash Verge**: Proxy tool
- **Nota**: Note-taking app

### Built with GTK:
- **GNOME apps**: Many Linux desktop apps

### Built with Dioxus:
- **Freya**: Native desktop apps
- Various cross-platform tools

---

## Final Thoughts

**The Rust UI ecosystem is growing fast but still maturing.** Here's my advice:

1. **For learning:** Start with egui - it's the easiest
2. **For production desktop apps:** Use Tauri - most mature, great DX
3. **For web apps:** Leptos is exciting but Yew is more mature
4. **For embedded/IoT:** Slint is your best bet
5. **For CLI tools:** Ratatui is excellent

Don't expect the polish of .NET WPF or the maturity of React ecosystem yet, but Rust UI is definitely viable for real applications!

---

## Resources

- **Are We GUI Yet?** - https://areweguiyet.com/
- **egui demo:** https://www.egui.rs/#demo
- **Tauri docs:** https://tauri.app/
- **Dioxus website:** https://dioxuslabs.com/

Let me know if you want to dive deeper into any specific framework!