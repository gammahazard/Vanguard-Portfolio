use leptos::*;
use leptos::html::Input;
use leptos::html::Div;
use std::sync::atomic::{AtomicUsize, Ordering};
use wasm_bindgen::JsCast;

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

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

#[derive(Clone, PartialEq, Debug)]
enum OutputPart {
    Text(String),
    Bold(String),
    Section(String),
    Badge(String),
    CmdName(String),
    Link { text: String, url: String },
}

#[derive(Clone, PartialEq)]
struct TerminalLine {
    id: usize,
    prefix: String,
    parts: Vec<OutputPart>,
    is_command: bool,
    is_boot: bool,
}

impl TerminalLine {
    fn text(prefix: &str, content: &str, is_boot: bool) -> Self {
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            prefix: prefix.to_string(),
            parts: vec![OutputPart::Text(content.to_string())],
            is_command: false,
            is_boot,
        }
    }

    fn bold(prefix: &str, content: &str) -> Self {
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            prefix: prefix.to_string(),
            parts: vec![OutputPart::Bold(content.to_string())],
            is_command: false,
            is_boot: false,
        }
    }

    fn section(prefix: &str, content: &str) -> Self {
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            prefix: prefix.to_string(),
            parts: vec![OutputPart::Section(content.to_string())],
            is_command: false,
            is_boot: false,
        }
    }

    fn badges(prefix: &str, items: Vec<&str>) -> Self {
        let parts = items.into_iter()
            .map(|s| OutputPart::Badge(s.to_string()))
            .collect();
        
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            prefix: prefix.to_string(),
            parts,
            is_command: false,
            is_boot: false,
        }
    }

    fn help_entry(indent: &str, cmd: &str, desc: &str) -> Self {
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            prefix: indent.to_string(),
            parts: vec![
                OutputPart::CmdName(cmd.to_string()),
                OutputPart::Text(desc.to_string())
            ],
            is_command: false,
            is_boot: false,
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
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
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
        (100, TerminalLine::text("[    0.000]", " Initializing portfolio kernel...", true)),
        (350, TerminalLine::text("[    0.127]", " Loading modules: rust_core, wasm_runtime", true)),
        (600, TerminalLine::text("[    0.256]", " Mounting portfolio filesystem... OK", true)),
        (850, TerminalLine::text("[    0.384]", " Starting services... DONE", true)),
        (1100, TerminalLine::text("[    0.512]", " Authenticating... VERIFIED", true)),
        (1400, TerminalLine::text("", "", false)),
        (1700, TerminalLine::text("", "  CM MONGO", false)),
        (1800, TerminalLine::text("", "  Independent Systems Engineer | Edge Computing", false)),
        (2000, TerminalLine::text("", "", false)),
        (2300, TerminalLine::text("[OK]", " System Ready.", true)),
        (2400, TerminalLine::text("", "", false)),
        (2500, TerminalLine::section("", "  > AVAILABLE COMMANDS")),
        (2600, TerminalLine {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            prefix: "".to_string(),
            parts: vec![
                OutputPart::Bold("  projects".to_string()),
                OutputPart::Text("   View portfolio projects".to_string()),
            ],
            is_command: false,
            is_boot: false,
        }),
        (2700, TerminalLine {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            prefix: "".to_string(),
            parts: vec![
                OutputPart::Bold("  skills".to_string()),
                OutputPart::Text("     Technical stack".to_string()),
            ],
            is_command: false,
            is_boot: false,
        }),
        (2800, TerminalLine {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            prefix: "".to_string(),
            parts: vec![
                OutputPart::Bold("  about".to_string()),
                OutputPart::Text("      Profile and bio".to_string()),
            ],
            is_command: false,
            is_boot: false,
        }),
        (2900, TerminalLine {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            prefix: "".to_string(),
            parts: vec![
                OutputPart::Bold("  contact".to_string()),
                OutputPart::Text("    Get in touch".to_string()),
            ],
            is_command: false,
            is_boot: false,
        }),
        (3000, TerminalLine {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            prefix: "".to_string(),
            parts: vec![
                OutputPart::Bold("  clear".to_string()),
                OutputPart::Text("      Reset terminal".to_string()),
            ],
            is_command: false,
            is_boot: false,
        }),
        (3050, TerminalLine {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            prefix: "".to_string(),
            parts: vec![
                OutputPart::Bold("  help".to_string()),
                OutputPart::Text("       List all commands".to_string()),
            ],
            is_command: false,
            is_boot: false,
        }),
        (3100, TerminalLine::text("", "", false)),
        (3200, TerminalLine::text("", "", false)),
    ]
}





// Levenshtein distance for fuzzy matching
fn levenshtein(a: &str, b: &str) -> usize {
    let len_a = a.chars().count();
    let len_b = b.chars().count();
    if len_a == 0 { return len_b; }
    if len_b == 0 { return len_a; }

    let mut matrix = vec![vec![0; len_b + 1]; len_a + 1];

    for i in 0..=len_a { matrix[i][0] = i; }
    for j in 0..=len_b { matrix[0][j] = j; }

    for (i, char_a) in a.chars().enumerate() {
        for (j, char_b) in b.chars().enumerate() {
            let cost = if char_a == char_b { 0 } else { 1 };
            matrix[i + 1][j + 1] = std::cmp::min(
                std::cmp::min(matrix[i][j + 1] + 1, matrix[i + 1][j] + 1),
                matrix[i][j] + cost
            );
        }
    }
    matrix[len_a][len_b]
}

// portfolio projects - organized by technical domain
// showcases progression from full-stack to systems-level edge computing
fn get_projects_output() -> Vec<TerminalLine> {
    let divider = "  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -";
    vec![
        TerminalLine::text("", "", false),
        // flagship industrial edge projects - wasi 0.2 expertise
        TerminalLine::section("", "  // INDUSTRIAL EDGE"),
        TerminalLine::text("", "  ═══════════════════════════════════════════════════════════", false),
        TerminalLine::text("", "", false),
        TerminalLine::bold("", "  Guardian One Platform"),
        TerminalLine::text("", "  └─ Industrial edge security: WASM vs Docker comparison", false),
        TerminalLine::text("", "     2oo3 TMR • IEC 62443 • 0.05ms vs 3000ms+ • 6MB vs 135MB", false),
        TerminalLine::with_link("", "     ", "Live Demo", "https://guardian-one-web-demo.vercel.app", ""),
        TerminalLine::with_link("", " | ", "GitHub", "https://github.com/gammahazard/Guardian-one-web-demo", ""),
        TerminalLine::text("", "", false),
        TerminalLine::text("", divider, false),
        TerminalLine::text("", "", false),
        TerminalLine::bold("", "  Edge WASI Runtime"),
        TerminalLine::text("", "  └─ Secure IoT plugin host: Hot-swap Python on Raspberry Pi", false),
        TerminalLine::text("", "     Capability security • <10ms reload • Real GPIO hardware", false),
        TerminalLine::with_link("", "     ", "GitHub", "https://github.com/gammahazard/edge-wasi-runtime", ""),
        TerminalLine::text("", "", false),
        TerminalLine::text("", divider, false),
        TerminalLine::text("", "", false),
        TerminalLine::bold("", "  ICS Guardian"),
        TerminalLine::text("", "  └─ Capability-based security: WASI 0.2 data diode for ICS", false),
        TerminalLine::text("", "     14.7KB WASM vs 500MB+ Docker • 17 security tests", false),
        TerminalLine::with_link("", "     ", "Live Demo", "https://vanguard-ics-guardian.vercel.app", ""),
        TerminalLine::text("", "  ───────────────────────────────────────────────────────────", false),
        TerminalLine::text("", "", false),
        // distributed systems and protocol work
        TerminalLine::section("", "  // SYSTEMS ARCHITECTURE"),
        TerminalLine::text("", "  ═══════════════════════════════════════════════════════════", false),
        TerminalLine::text("", "", false),
        TerminalLine::bold("", "  Raft Consensus Cluster"),
        TerminalLine::text("", "  └─ Distributed consensus: Same binary runs in browser + Pi", false),
        TerminalLine::text("", "     120+ tests • PreVote protocol • Network partition handling", false),
        TerminalLine::with_link("", "     ", "Live Demo", "https://raft-consensus.vercel.app", ""),
        TerminalLine::with_link("", " | ", "GitHub", "https://github.com/gammahazard/Raft-Consensus", ""),
        TerminalLine::text("", "", false),
        TerminalLine::text("", divider, false),
        TerminalLine::text("", "", false),
        TerminalLine::bold("", "  Protocol Gateway Sandbox"),
        TerminalLine::text("", "  └─ Modbus → MQTT via WASM: Crash-contained translation", false),
        TerminalLine::text("", "     ~7ms recovery • IEC 62443 attack minimization • 2oo3 TMR", false),
        TerminalLine::with_link("", "     ", "Live Demo", "https://protocol-gateway-sandbox.vercel.app", ""),
        TerminalLine::with_link("", " | ", "GitHub", "https://github.com/gammahazard/protocol-gateway-sandbox", ""),
        TerminalLine::text("", "", false),
        TerminalLine::text("", divider, false),
        TerminalLine::text("", "", false),
        TerminalLine::bold("", "  Pacifica Engine"),
        TerminalLine::text("", "  └─ HFT Bot: Sub-ms execution, 5-factor signals", false),
        TerminalLine::with_link("", "     ", "GitHub", "https://github.com/gammahazard/auto-trade", ""),
        TerminalLine::text("", "", false),
        TerminalLine::text("", divider, false),
        TerminalLine::text("", "", false),
        TerminalLine::bold("", "  Ore Supervisor"),
        TerminalLine::text("", "  └─ macOS Daemon: Process health + auto-restart", false),
        TerminalLine::with_link("", "     ", "GitHub", "https://github.com/gammahazard/ore-app-mac", ""),
        TerminalLine::text("", "  ───────────────────────────────────────────────────────────", false),
        TerminalLine::text("", "", false),
        // production full-stack applications
        TerminalLine::section("", "  // FULL STACK"),
        TerminalLine::text("", "  ═══════════════════════════════════════════════════════════", false),
        TerminalLine::text("", "", false),
        TerminalLine::bold("", "  Kennel Platform"),
        TerminalLine::text("", "  └─ Flagship ERP: FIDO2 auth, RBAC, 61 endpoints", false),
        TerminalLine::with_link("", "     ", "GitHub", "https://github.com/gammahazard/Vanguard-kennel-frontend", ""),
        TerminalLine::with_link("", " | ", "Demo", "https://vanguard-frontend.vercel.app", ""),
        TerminalLine::text("", "", false),
        TerminalLine::text("", divider, false),
        TerminalLine::text("", "", false),
        TerminalLine::bold("", "  CyberVerse Exchange"),
        TerminalLine::text("", "  └─ Multi-chain bridge: SOL/ADA/ETH/ERGO wallets", false),
        TerminalLine::with_link("", "     ", "GitHub", "https://github.com/gammahazard/CyberVerse-exchange", ""),
        TerminalLine::text("", "  ───────────────────────────────────────────────────────────", false),
        TerminalLine::text("", "", false),
        // wasm technical demonstrations
        TerminalLine::section("", "  // WASM & TOOLS"),
        TerminalLine::text("", "  ═══════════════════════════════════════════════════════════", false),
        TerminalLine::text("", "", false),
        TerminalLine::bold("", "  PokeFrame"),
        TerminalLine::text("", "  └─ Rust GameBoy emulator → WASM, 60fps", false),
        TerminalLine::with_link("", "     ", "GitHub", "https://github.com/gammahazard/PokeFramePublic", ""),
        TerminalLine::text("", "", false),
        TerminalLine::text("", divider, false),
        TerminalLine::text("", "", false),
        TerminalLine::bold("", "  Fleet-SDK Builder"),
        TerminalLine::text("", "  └─ Ergo transaction builder: EIP-12 compliant", false),
        TerminalLine::with_link("", "     ", "GitHub", "https://github.com/gammahazard/Fleet-SDK-cc", ""),
        TerminalLine::with_link("", " | ", "Demo", "https://gammahazard.github.io/Fleet-SDK-cc/dist/index.html", ""),
        TerminalLine::text("", "", false),
    ]
}

// technical skills - organized by specialty area
// emphasizes wasi 0.2 and industrial edge expertise
fn get_skills_output() -> Vec<TerminalLine> {
    vec![
        TerminalLine::text("", "", false),
        TerminalLine::section("", "  // WASI & EDGE COMPUTING"),
        TerminalLine::badges("  ", vec!["WASI 0.2", "Component Model", "Wasmtime", "Leptos"]),
        TerminalLine::badges("  ", vec!["Hot Module Reloading", "Capability Security", "JCO"]),
        TerminalLine::text("", "", false),
        TerminalLine::section("", "  // SYSTEMS & PROTOCOLS"),
        TerminalLine::badges("  ", vec!["Rust", "C++", "C", "Distributed Systems"]),
        TerminalLine::badges("  ", vec!["Raft Consensus", "IEC 62443", "Modbus TCP", "MQTT"]),
        TerminalLine::text("", "", false),
        TerminalLine::section("", "  // EMBEDDED & HARDWARE"),
        TerminalLine::badges("  ", vec!["Raspberry Pi", "GPIO", "Embedded Linux"]),
        TerminalLine::badges("  ", vec!["2oo3 TMR", "Industrial Control", "DHT22 Sensors"]),
        TerminalLine::text("", "", false),
        TerminalLine::section("", "  // FULL STACK"),
        TerminalLine::badges("  ", vec!["TypeScript", "Next.js 14+", "React", "Node.js"]),
        TerminalLine::badges("  ", vec!["Tailwind CSS", "GraphQL", "gRPC"]),
        TerminalLine::text("", "", false),
        TerminalLine::section("", "  // INFRASTRUCTURE & DATA"),
        TerminalLine::badges("  ", vec!["Docker", "AWS", "PostgreSQL", "MongoDB"]),
        TerminalLine::badges("  ", vec!["Redis", "Supabase", "CI/CD"]),
        TerminalLine::text("", "", false),
        TerminalLine::section("", "  // SECURITY & WEB3"),
        TerminalLine::badges("  ", vec!["FIDO2/WebAuthn", "OAuth 2.0", "JWT"]),
        TerminalLine::badges("  ", vec!["Zero Trust", "Cloud Identity", "WebSockets"]),
        TerminalLine::text("", "", false),
    ]
}

// contact info
fn get_contact_output() -> Vec<TerminalLine> {
    vec![
        TerminalLine::text("", "", false),
        TerminalLine::with_link("", "  Email   ", "cm.mongo.web3@gmail.com", "mailto:cm.mongo.web3@gmail.com", ""),
        TerminalLine::with_link("", "  GitHub  ", "gammahazard", "https://github.com/gammahazard", ""),
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
    
    // Track uptime
    let start_time = js_sys::Date::now();

    // Auto-scroll logic
    create_effect(move |_| {
        let _ = history.get();
        if let Some(el) = terminal_body_ref.get() {
            set_timeout(move || {
                let window_width = window().inner_width().ok().and_then(|w| w.as_f64()).unwrap_or(1024.0);
                if window_width < 768.0 {
                    // On mobile, try to scroll the last user command to the top
                    let commands = document().get_elements_by_class_name("user-command");
                    if commands.length() > 0 {
                        let last_cmd = commands.item(commands.length() - 1).unwrap();
                        last_cmd.scroll_into_view_with_bool(true); // true = align to top
                    } else {
                        let _ = el.set_scroll_top(el.scroll_height());
                    }
                } else {
                    // On desktop, standard console behavior (scroll to bottom)
                    let _ = el.set_scroll_top(el.scroll_height());
                }
            }, std::time::Duration::from_millis(10));
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
                id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
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
                TerminalLine::section("", "  > AVAILABLE COMMANDS"),
                TerminalLine::text("", "", false),
                TerminalLine {
                    id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
                    prefix: "".to_string(),
                    parts: vec![
                        OutputPart::Bold("  projects".to_string()),
                        OutputPart::Text("     List all portfolio projects".to_string()),
                    ],
                    is_command: false,
                    is_boot: false,
                },
                TerminalLine {
                    id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
                    prefix: "".to_string(),
                    parts: vec![
                        OutputPart::Bold("  skills".to_string()),
                        OutputPart::Text("       View technical stack and expertise".to_string()),
                    ],
                    is_command: false,
                    is_boot: false,
                },
                TerminalLine {
                    id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
                    prefix: "".to_string(),
                    parts: vec![
                        OutputPart::Bold("  about".to_string()),
                        OutputPart::Text("        Professional profile and bio".to_string()),
                    ],
                    is_command: false,
                    is_boot: false,
                },
                TerminalLine {
                    id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
                    prefix: "".to_string(),
                    parts: vec![
                        OutputPart::Bold("  contact".to_string()),
                        OutputPart::Text("      Get in touch".to_string()),
                    ],
                    is_command: false,
                    is_boot: false,
                },
                TerminalLine {
                    id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
                    prefix: "".to_string(),
                    parts: vec![
                        OutputPart::Bold("  clear".to_string()),
                        OutputPart::Text("        Reset terminal".to_string()),
                    ],
                    is_command: false,
                    is_boot: false,
                },
                TerminalLine::text("", "", false),
                TerminalLine::section("", "  > EASTER EGGS"),
                TerminalLine::text("", "  Try: neofetch, whoami, ls, sudo hire me, ping, date", false),
                TerminalLine::text("", "", false),
            ],
            "projects" | "ls projects" | "repos" => get_projects_output(),
            "skills" | "stack" | "tech" => get_skills_output(),
            "contact" | "email" => get_contact_output(),
            "about" => vec![
                TerminalLine::text("", "", false),
                TerminalLine::bold("", "  CM Mongo"),
                TerminalLine::text("", "  Systems Engineer | Edge Computing Specialist", false),
                TerminalLine::text("", "", false),
                TerminalLine::text("", "  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━", false),
                TerminalLine::text("", "", false),
                TerminalLine::section("", "  > ENGINEERING FOCUS"),
                TerminalLine::text("", "    Building crash-contained edge infrastructure with Rust & WASI 0.2.", false),
                TerminalLine::text("", "    Specializing in capability-based security for industrial control systems.", false),
                TerminalLine::text("", "", false),
                TerminalLine::section("", "  > FLAGSHIP WORK"),
                TerminalLine::bold("", "    Guardian One"),
                TerminalLine::text("", "    └─ Industrial edge platform (100,000x performance vs Python)", false),
                TerminalLine::with_link("", "       ", "[Demo]", "https://guardian-one-web-demo.vercel.app", ""),
                TerminalLine::with_link("", " ", "[GitHub]", "https://github.com/gammahazard/Guardian-one-web-demo", ""),
                TerminalLine::text("", "", false),
                TerminalLine::bold("", "    Edge WASI Runtime"),
                TerminalLine::text("", "    └─ Hot-swappable IoT plugins on Raspberry Pi bare metal", false),
                TerminalLine::with_link("", "       ", "[GitHub]", "https://github.com/gammahazard/edge-wasi-runtime", ""),
                TerminalLine::text("", "", false),
                TerminalLine::bold("", "    Raft Consensus"),
                TerminalLine::text("", "    └─ Distributed systems running in browser + hardware", false),
                TerminalLine::with_link("", "       ", "[Demo]", "https://raft-consensus.vercel.app", ""),
                TerminalLine::with_link("", " ", "[GitHub]", "https://github.com/gammahazard/Raft-Consensus", ""),
                TerminalLine::text("", "", false),
                TerminalLine::section("", "  > CORE EXPERTISE"),
                TerminalLine::text("", "    • WASI 0.2 Component Model", false),
                TerminalLine::text("", "    • IEC 62443 Industrial Security", false),
                TerminalLine::text("", "    • Distributed Consensus (Raft)", false),
                TerminalLine::text("", "    • Hot Module Reloading", false),
                TerminalLine::text("", "    • Zero-Liability Architecture", false),
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
            "neofetch" => {
                let now = js_sys::Date::now();
                let uptime_ms = now - start_time;
                let minutes = (uptime_ms / 60000.0).floor();
                let seconds = ((uptime_ms % 60000.0) / 1000.0).floor();
                
                vec![
                    TerminalLine::text("", "", false),
                    TerminalLine::text("", "        /\\         cm_mongo@vanguard", false),
                    TerminalLine::text("", "       /  \\        os: vanguardos 2.0", false),
                    TerminalLine::text("", "      / 🦀 \\       kernel: rust + wasm", false),
                    TerminalLine::text("", "     /______\\      shell: leptos", false),
                    TerminalLine::text("", &format!("                   uptime: {}m {}s", minutes, seconds), false),
                    TerminalLine::text("", "", false),
                ]
            },
            "whoami" => vec![TerminalLine::text("", "  vanguard", false)],
            "pwd" => vec![TerminalLine::text("", "  /home/vanguard/portfolio", false)],
            "uptime" => vec![TerminalLine::text("", "  up 1337 days, building systems", false)],
            "sudo hire me" | "hire" => vec![
                TerminalLine::text("", "", false),
                TerminalLine::text("", "  [sudo] password for visitor: **********", false),
                TerminalLine::text("", "  authenticating...", false),
                TerminalLine::text("", "  ACCESS GRANTED.", false),
                TerminalLine::text("", "", false),
                TerminalLine::text("", "  initiating onboarding_sequence.sh...", false),
                TerminalLine::text("", "  > Priority connection established.", false),
                TerminalLine::text("", "  > Ready to architect your next system.", false),
                TerminalLine::text("", "", false),
                TerminalLine::with_link("", "  [ EXECUTE OFFER ] ", "Send Email", "mailto:cm.mongo.web3@gmail.com?subject=Priority%20Job%20Offer", ""),
                TerminalLine::text("", "", false),
            ],
            "sudo" => vec![TerminalLine::text("", "  try: sudo hire me", false)],
            "exit" | "quit" => vec![TerminalLine::text("", "  no escape. type 'help' :)", false)],
            "rm" | "rm -rf" | "rm -rf /" => vec![TerminalLine::text("", "  🛑 nice try.", false)],
            "rust" | "🦀" => vec![TerminalLine::text("", "  🦀 btw i use rust", false)],
            "ping" => {
                let set_history = set_history.clone();
                spawn_local(async move {
                    let start = js_sys::Date::now();
                    // Fetch current page just to measure round trip to origin/cache
                    let _ = gloo_net::http::Request::get("/").send().await;
                    let end = js_sys::Date::now();
                    let duration = end - start;
                    
                    set_history.update(|h| {
                        h.push(TerminalLine::text("", &format!("  pong. {:.0}ms", duration), false));
                    });
                });
                vec![TerminalLine::text("", "  pinging...", false)]
            },
            "date" => vec![TerminalLine::text("", "  2026. building the future.", false)],
            "cat readme" | "cat readme.md" => vec![
                TerminalLine::text("", "", false),
                TerminalLine::text("", "  # vanguard secure solutions", false),
                TerminalLine::text("", "  > engineering complete systems", false),
                TerminalLine::text("", "", false),
            ],
            "" => vec![],
            _ => {
                let valid_commands = vec![
                    "about", "clear", "contact", "help", "neofetch", 
                    "projects", "skills", "whoami", "ls", "sudo", "date", "ping"
                ];
                
                let mut closest_cmd = "";
                let mut min_dist = usize::MAX;

                for valid in valid_commands {
                    let dist = levenshtein(&cmd_lower, valid);
                    if dist < min_dist {
                        min_dist = dist;
                        closest_cmd = valid;
                    }
                }

                if min_dist <= 2 {
                    vec![
                        TerminalLine::text("", &format!("  command not found: {}", cmd), false),
                        TerminalLine::text("", &format!("  did you mean '{}'? (dist: {})", closest_cmd, min_dist), false),
                        TerminalLine::text("", "  type 'help' for commands", false),
                    ]
                } else {
                    vec![
                        TerminalLine::text("", &format!("  command not found: {}", cmd), false),
                        TerminalLine::text("", "  type 'help' for commands", false),
                    ]
                }
            },
        };

        for line in responses {
            set_history.update(|h| h.push(line));
        }
        set_current_input.set(String::new());
    };

    let handle_submit = move || {
        let cmd = current_input.get();
        if !cmd.is_empty() {
            process_command(cmd);
            
            // On mobile, blur the input to dismiss keyboard
            // Use timeout to ensure UI updates don't steal focus back
            set_timeout(move || {
                let window_width = window().inner_width().ok().and_then(|w| w.as_f64()).unwrap_or(1024.0);
                if window_width < 768.0 {
                    let input_el = window().document().unwrap().get_element_by_id("cmd-input");
                    if let Some(el) = input_el {
                        if let Some(html_el) = el.dyn_ref::<web_sys::HtmlElement>() {
                            let _ = html_el.blur();
                        }
                    }
                }
            }, std::time::Duration::from_millis(10));
        }
    };

    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Enter" {
            handle_submit();
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
                            key=|line| line.id
                            children=move |line| {
                                view! {
                                    <div class="terminal-line" class:boot-line=line.is_boot class:user-command=line.is_command>
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
                                                OutputPart::Bold(text) => view! {
                                                    <span class="output bold">
                                                        {text.clone()}
                                                    </span>
                                                }.into_view(),
                                                OutputPart::Section(text) => view! {
                                                    <span class="output section-title">
                                                        {text.clone()}
                                                    </span>
                                                }.into_view(),
                                                OutputPart::Badge(text) => view! {
                                                    <span class="skill-badge">
                                                        {text.clone()}
                                                    </span>
                                                }.into_view(),
                                                OutputPart::CmdName(text) => view! {
                                                    <span class="cmd-name">
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
                                id="cmd-input"
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
                            <button class="send-btn" on:click=move |_| handle_submit()>
                                "➜"
                            </button>
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
