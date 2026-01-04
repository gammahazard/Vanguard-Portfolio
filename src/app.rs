use leptos::*;
use leptos::html::Input;
use leptos::html::Div;

// main app component - keeps it simple
#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="container">
            <Terminal />
            <Footer />
        </div>
    }
}

// output can be text or a clickable link
#[derive(Clone, PartialEq, Debug)]
enum OutputPart {
    Text(String),
    Link { text: String, url: String },
}

#[derive(Clone, PartialEq)]
struct TerminalLine {
    prefix: String,
    parts: Vec<OutputPart>,
    is_command: bool,
    is_boot: bool,
}

impl TerminalLine {
    fn text(prefix: &str, content: &str, is_boot: bool) -> Self {
        Self {
            prefix: prefix.to_string(),
            parts: vec![OutputPart::Text(content.to_string())],
            is_command: false,
            is_boot,
        }
    }
    
    fn with_link(prefix: &str, text_before: &str, link_text: &str, url: &str, text_after: &str) -> Self {
        let mut parts = vec![];
        if !text_before.is_empty() {
            parts.push(OutputPart::Text(text_before.to_string()));
        }
        parts.push(OutputPart::Link { text: link_text.to_string(), url: url.to_string() });
        if !text_after.is_empty() {
            parts.push(OutputPart::Text(text_after.to_string()));
        }
        Self {
            prefix: prefix.to_string(),
            parts,
            is_command: false,
            is_boot: false,
        }
    }
}

// boot sequence - simulates linux startup
fn get_boot_sequence() -> Vec<(u64, TerminalLine)> {
    vec![
        (100, TerminalLine::text("[    0.000]", " initializing vanguard kernel...", true)),
        (350, TerminalLine::text("[    0.127]", " loading modules: rust_core, wasm_runtime", true)),
        (600, TerminalLine::text("[    0.256]", " mounting portfolio filesystem... ok", true)),
        (850, TerminalLine::text("[    0.384]", " starting services... done", true)),
        (1100, TerminalLine::text("[    0.512]", " authenticating... verified", true)),
        (1400, TerminalLine::text("", "", false)),
        (1700, TerminalLine::text("", "  vanguard secure solutions", false)),
        (1800, TerminalLine::text("", "  full stack systems architecture", false)),
        (2000, TerminalLine::text("", "", false)),
        (2300, TerminalLine::text("[ok]", " system ready. type 'help' for commands.", true)),
        (2500, TerminalLine::text("", "", false)),
    ]
}

// projects - all repos organized by category
fn get_projects_output() -> Vec<TerminalLine> {
    vec![
        TerminalLine::text("", "", false),
        TerminalLine::text("", "  // full stack", false),
        TerminalLine::text("", "", false),
        TerminalLine::text("", "  kennel platform", false),
        TerminalLine::text("", "  └─ flagship erp: fido2 auth, rbac, 61 endpoints", false),
        TerminalLine::with_link("", "     ", "github", "https://github.com/gammahazard/Vanguard-kennel-frontend", ""),
        TerminalLine::with_link("", " | ", "demo", "https://vanguard-frontend.vercel.app", ""),
        TerminalLine::text("", "", false),
        TerminalLine::text("", "  cyberverse exchange", false),
        TerminalLine::text("", "  └─ multi-chain bridge: sol/ada/eth/ergo wallets", false),
        TerminalLine::with_link("", "     ", "github", "https://github.com/gammahazard/CyberVerse-exchange", ""),
        TerminalLine::text("", "", false),
        TerminalLine::text("", "  // systems", false),
        TerminalLine::text("", "", false),
        TerminalLine::text("", "  pacifica engine", false),
        TerminalLine::text("", "  └─ hft bot: sub-ms execution, 5-factor signals", false),
        TerminalLine::with_link("", "     ", "github", "https://github.com/gammahazard/auto-trade", ""),
        TerminalLine::text("", "", false),
        TerminalLine::text("", "  ore supervisor", false),
        TerminalLine::text("", "  └─ macos daemon: process health + auto-restart", false),
        TerminalLine::with_link("", "     ", "github", "https://github.com/gammahazard/ore-app-mac", ""),
        TerminalLine::text("", "", false),
        TerminalLine::text("", "  // wasm", false),
        TerminalLine::text("", "", false),
        TerminalLine::text("", "  pokeframe", false),
        TerminalLine::text("", "  └─ rust gameboy emulator → wasm, 60fps", false),
        TerminalLine::with_link("", "     ", "github", "https://github.com/gammahazard/PokeFramePublic", ""),
        TerminalLine::text("", "", false),
        TerminalLine::text("", "  fleet-sdk builder", false),
        TerminalLine::text("", "  └─ ergo tx builder: eip-12 compliant", false),
        TerminalLine::with_link("", "     ", "github", "https://github.com/gammahazard/Fleet-SDK-cc", ""),
        TerminalLine::with_link("", " | ", "demo", "https://gammahazard.github.io/Fleet-SDK-cc/dist/index.html", ""),
        TerminalLine::text("", "", false),
        TerminalLine::text("", "  emscripten demos", false),
        TerminalLine::text("", "  └─ c++ → wasm including doom engine", false),
        TerminalLine::with_link("", "     ", "github", "https://github.com/gammahazard/Emscripten-portfolio", ""),
        TerminalLine::text("", "", false),
    ]
}

// tech stack - condensed
fn get_skills_output() -> Vec<TerminalLine> {
    vec![
        TerminalLine::text("", "", false),
        TerminalLine::text("", "  // core", false),
        TerminalLine::text("", "  rust | c++ | wasm | emscripten", false),
        TerminalLine::text("", "", false),
        TerminalLine::text("", "  // web", false),
        TerminalLine::text("", "  next.js | react | typescript | node.js", false),
        TerminalLine::text("", "", false),
        TerminalLine::text("", "  // data", false),
        TerminalLine::text("", "  postgresql | mongodb | redis | sqlite", false),
        TerminalLine::text("", "", false),
        TerminalLine::text("", "  // infra", false),
        TerminalLine::text("", "  docker | aws | vercel | github actions", false),
        TerminalLine::text("", "", false),
        TerminalLine::text("", "  // security", false),
        TerminalLine::text("", "  fido2/webauthn | jwt | rbac | zero-trust", false),
        TerminalLine::text("", "", false),
    ]
}

// contact info
fn get_contact_output() -> Vec<TerminalLine> {
    vec![
        TerminalLine::text("", "", false),
        TerminalLine::with_link("", "  email   ", "cm.mongo.web3@gmail.com", "mailto:cm.mongo.web3@gmail.com", ""),
        TerminalLine::with_link("", "  github  ", "gammahazard", "https://github.com/gammahazard", ""),
        TerminalLine::with_link("", "  live    ", "vanguard-frontend.vercel.app", "https://vanguard-frontend.vercel.app", ""),
        TerminalLine::text("", "", false),
    ]
}

#[component]
fn Terminal() -> impl IntoView {
    let (history, set_history) = create_signal::<Vec<TerminalLine>>(vec![]);
    let (current_input, set_current_input) = create_signal(String::new());
    let (show_input, set_show_input) = create_signal(false);
    let input_ref = create_node_ref::<Input>();
    let terminal_body_ref = create_node_ref::<Div>();

    // auto-scroll to bottom when history changes
    create_effect(move |_| {
        let _ = history.get();
        if let Some(el) = terminal_body_ref.get() {
            // small delay to let dom update
            set_timeout(
                move || {
                    let _ = el.set_scroll_top(el.scroll_height());
                },
                std::time::Duration::from_millis(10),
            );
        }
    });

    // boot animation
    create_effect(move |_| {
        let boot_lines = get_boot_sequence();
        for (delay, line) in boot_lines {
            let line_clone = line.clone();
            set_timeout(
                move || {
                    set_history.update(|h| h.push(line_clone.clone()));
                },
                std::time::Duration::from_millis(delay),
            );
        }
        set_timeout(
            move || {
                set_show_input.set(true);
            },
            std::time::Duration::from_millis(2700),
        );
    });

    let focus_input = move |_| {
        if let Some(input) = input_ref.get() {
            let _ = input.focus();
        }
    };

    let process_command = move |cmd: String| {
        let cmd_lower = cmd.to_lowercase().trim().to_string();
        
        // add command to history
        set_history.update(|h| {
            h.push(TerminalLine { 
                prefix: "λ".to_string(), 
                parts: vec![OutputPart::Text(format!(" {}", cmd))],
                is_command: true,
                is_boot: false,
            });
        });

        // process
        let responses: Vec<TerminalLine> = match cmd_lower.as_str() {
            "help" | "h" | "?" => vec![
                TerminalLine::text("", "", false),
                TerminalLine::text("", "  commands:", false),
                TerminalLine::text("", "  projects   list all repos", false),
                TerminalLine::text("", "  skills     tech stack", false),
                TerminalLine::text("", "  about      who i am", false),
                TerminalLine::text("", "  contact    get in touch", false),
                TerminalLine::text("", "  clear      reset terminal", false),
                TerminalLine::text("", "", false),
                TerminalLine::text("", "  easter eggs: neofetch, ls, sudo hire me", false),
                TerminalLine::text("", "", false),
            ],
            "projects" | "ls projects" | "repos" => get_projects_output(),
            "skills" | "stack" | "tech" => get_skills_output(),
            "contact" | "email" => get_contact_output(),
            "about" | "whoami" => vec![
                TerminalLine::text("", "", false),
                TerminalLine::text("", "  cm mongo", false),
                TerminalLine::text("", "  principal systems architect", false),
                TerminalLine::text("", "", false),
                TerminalLine::text("", "  i don't just build frontends.", false),
                TerminalLine::text("", "  i engineer complete systems.", false),
                TerminalLine::text("", "", false),
                TerminalLine::text("", "  focus:", false),
                TerminalLine::text("", "  • zero-liability architecture", false),
                TerminalLine::text("", "  • high-reliability automation", false),
                TerminalLine::text("", "  • rust/wasm → web bridging", false),
                TerminalLine::text("", "", false),
            ],
            "clear" | "cls" => {
                set_history.set(vec![
                    TerminalLine::text("[ok]", " cleared.", true),
                ]);
                set_current_input.set(String::new());
                return;
            },
            // easter eggs
            "ls" | "ls -la" => vec![
                TerminalLine::text("", "", false),
                TerminalLine::text("", "  drwxr-xr-x  kennel-platform/", false),
                TerminalLine::text("", "  drwxr-xr-x  auto-trade/", false),
                TerminalLine::text("", "  drwxr-xr-x  CyberVerse-exchange/", false),
                TerminalLine::text("", "  drwxr-xr-x  Fleet-SDK-cc/", false),
                TerminalLine::text("", "  drwxr-xr-x  PokeFramePublic/", false),
                TerminalLine::text("", "  -rw-r--r--  .env [redacted]", false),
                TerminalLine::text("", "", false),
            ],
            "neofetch" => vec![
                TerminalLine::text("", "", false),
                TerminalLine::text("", "        /\\         cm_mongo@vanguard", false),
                TerminalLine::text("", "       /  \\        os: vanguardos 2.0", false),
                TerminalLine::text("", "      / 🦀 \\       kernel: rust + wasm", false),
                TerminalLine::text("", "     /______\\      shell: leptos", false),
                TerminalLine::text("", "                   uptime: 1337 days", false),
                TerminalLine::text("", "", false),
            ],
            "pwd" => vec![TerminalLine::text("", "  /home/vanguard/portfolio", false)],
            "uptime" => vec![TerminalLine::text("", "  up 1337 days, building systems", false)],
            "sudo hire me" | "hire" => vec![
                TerminalLine::text("", "", false),
                TerminalLine::text("", "  🎉 access granted", false),
                TerminalLine::text("", "", false),
                TerminalLine::with_link("", "  → ", "cm.mongo.web3@gmail.com", "mailto:cm.mongo.web3@gmail.com", ""),
                TerminalLine::with_link("", "  → ", "github.com/gammahazard", "https://github.com/gammahazard", ""),
                TerminalLine::text("", "", false),
            ],
            "sudo" => vec![TerminalLine::text("", "  try: sudo hire me", false)],
            "exit" | "quit" => vec![TerminalLine::text("", "  no escape. type 'help' :)", false)],
            "rm -rf /" => vec![TerminalLine::text("", "  🛑 nice try.", false)],
            "rust" | "🦀" => vec![TerminalLine::text("", "  🦀 btw i use rust", false)],
            "ping" => vec![TerminalLine::text("", "  pong. 0.42ms", false)],
            "date" => vec![TerminalLine::text("", "  2026. building the future.", false)],
            "cat readme" | "cat readme.md" => vec![
                TerminalLine::text("", "", false),
                TerminalLine::text("", "  # vanguard secure solutions", false),
                TerminalLine::text("", "  > engineering complete systems", false),
                TerminalLine::text("", "", false),
            ],
            "" => vec![],
            _ => vec![
                TerminalLine::text("", &format!("  command not found: {}", cmd), false),
                TerminalLine::text("", "  type 'help' for commands", false),
            ],
        };

        for line in responses {
            set_history.update(|h| h.push(line));
        }
        set_current_input.set(String::new());
    };

    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Enter" {
            let cmd = current_input.get();
            process_command(cmd);
        }
    };

    view! {
        <section class="hero">
            <div class="terminal-window" on:click=focus_input>
                <div class="terminal-header">
                    <div class="terminal-buttons">
                        <span class="btn-close"></span>
                        <span class="btn-min"></span>
                        <span class="btn-max"></span>
                    </div>
                    <span class="terminal-title">"vanguard@portfolio:~"</span>
                </div>
                <div class="terminal-body" node_ref=terminal_body_ref>
                    <div class="terminal-history">
                        <For
                            each=move || history.get()
                            key=|line| format!("{:?}", line.parts)
                            children=move |line| {
                                view! {
                                    <div class="terminal-line" class:boot-line=line.is_boot>
                                        {if !line.prefix.is_empty() {
                                            Some(view! { 
                                                <span class=if line.is_command { "prompt" } else if line.is_boot { "boot-prefix" } else { "" }>
                                                    {line.prefix.clone()}
                                                </span> 
                                            })
                                        } else {
                                            None
                                        }}
                                        {line.parts.iter().map(|part| {
                                            match part {
                                                OutputPart::Text(text) => view! {
                                                    <span class=if line.is_command { "command" } else { "output" }>
                                                        {text.clone()}
                                                    </span>
                                                }.into_view(),
                                                OutputPart::Link { text, url } => view! {
                                                    <a href={url.clone()} target="_blank" class="terminal-link">
                                                        {text.clone()}
                                                    </a>
                                                }.into_view(),
                                            }
                                        }).collect_view()}
                                    </div>
                                }
                            }
                        />
                    </div>
                    <Show when=move || show_input.get()>
                        <div class="terminal-input-line">
                            <span class="prompt">"λ "</span>
                            <input
                                type="text"
                                class="terminal-input"
                                placeholder="type a command..."
                                node_ref=input_ref
                                on:keydown=on_keydown
                                on:input=move |ev| {
                                    set_current_input.set(event_target_value(&ev));
                                }
                                prop:value=move || current_input.get()
                                autofocus
                            />
                            <span class="cursor-blink">"_"</span>
                        </div>
                    </Show>
                </div>
            </div>
        </section>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="footer">
            <div class="footer-content">
                <div class="footer-meta">
                    <p>"built with rust → wasm by cm mongo"</p>
                </div>
            </div>
        </footer>
    }
}
