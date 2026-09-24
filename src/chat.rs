// src/chat.rs
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use inquire::Text;
use rig::agent::Agent;
use rig::completion::{Chat, CompletionModel, Message};
use std::env;

/// Foolproof padding helper: 
/// It counts the characters in the `plain` strings to calculate exact spacing, 
/// but prints the `colored` strings to the terminal.
fn print_complex_row(
    left_plain: &str,
    left_colored: &str,
    right_plain: &str,
    right_colored: &str,
) {
    let border = |s: &str| s.truecolor(15, 98, 254);
    let inner_border = |s: &str| s.bright_black();
    
    // Left column is strictly 42 chars wide, Right is strictly 63 chars wide (Total width = 108 chars)
    let l_spaces = 42_usize.saturating_sub(left_plain.chars().count());
    let r_spaces = 63_usize.saturating_sub(right_plain.chars().count());

    println!(
        "{}{}{}{}{}{}{}",
        border("│"),
        left_colored,
        " ".repeat(l_spaces),
        inner_border("│"),
        right_colored,
        " ".repeat(r_spaces),
        border("│")
    );
}

/// Renders a highly attractive, cyberpunk-themed terminal UI banner with flawless border alignment
pub fn print_banner(provider: &str, model: &str) {
    let current_dir = env::current_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "~".to_string());

    // Truncate directory path cleanly to fit inside the fixed-width UI
    let max_path = 24;
    let dir_display = if current_dir.chars().count() > max_path {
        let skip = current_dir.chars().count() - (max_path - 3);
        format!("...{}", current_dir.chars().skip(skip).collect::<String>())
    } else {
        current_dir.clone()
    };

    let border = |s: &str| s.truecolor(15, 98, 254);

    // Top border: Exactly 108 characters total (1 + 19 text + 87 dashes + 1)
    println!("{}", border(&format!("╭── MANO CLI v0.1.0 {}╮", "─".repeat(87))));

    // --- ROW 0 (Spacing) ---
    print_complex_row("", "", "", "");

    // --- ROW 1 (Bee Head + Folded Wings) ---
    let l1_p = "        \\_(⊙.⊙)_/";
    let l1_c = format!("        {}{}{}", "\\_".cyan(), format!("({}{}{})", "⊙".green().bold(), ".".normal(), "⊙".green().bold()), "_/".cyan());
    let r1_p = "  [ SYSTEM STATUS ]";
    let r1_c = r1_p.truecolor(15, 98, 254).to_string();
    print_complex_row(l1_p, &l1_c, r1_p, &r1_c);

    // --- ROW 2 (Bee Body Stripe 1 + Wing Span) ---
    let l2_p = "       / |=====| \\";
    let l2_c = format!("       {} |{}| {}", "/".cyan(), "=====".yellow().bold(), "\\".cyan());
    let r2_p = "  Active Mode: Agentic Engine initialized";
    print_complex_row(l2_p, &l2_c, r2_p, r2_p);

    // --- ROW 3 (Bee Body Stripe 2) ---
    let l3_p = "         |=====|";
    let l3_c = format!("         |{}|", "=====".yellow().bold());
    let r3_p = "  ───────────────────────────────────────────────────────────";
    let r3_c = r3_p.bright_black().to_string();
    print_complex_row(l3_p, &l3_c, r3_p, &r3_c);

    // --- ROW 4 (Bee Abdomen) ---
    let l4_p = "          \\___/";
    let l4_c = format!("          {}", "\\___/".cyan().bold());
    let r4_p = "  [ PRO TIPS & COMMANDS ]";
    let r4_c = r4_p.truecolor(15, 98, 254).to_string();
    print_complex_row(l4_p, &l4_c, r4_p, &r4_c);

    // --- ROW 5 (Gap between bee and status rows) ---
    let r5_p = "    * Ask me to execute bash commands, inspect code, or debug";
    let r5_c = format!("    {} Ask me to execute bash commands, inspect code, or debug", "*".cyan());
    print_complex_row("", "", r5_p, &r5_c);

    // --- ROW 6 ---
    let l6_p = "  [*] STATUS │ ONLINE";
    let l6_c = format!("  {} STATUS {} {}", "[*]".bright_black(), "│".bright_black(), "ONLINE".green().bold());
    let r6_p = "    * Type 'exit' / 'quit' to end session at any time";
    let r6_c = format!("    {} Type 'exit' / 'quit' to end session at any time", "*".cyan());
    print_complex_row(l6_p, &l6_c, r6_p, &r6_c);

    // --- ROW 7 ---
    let l7_p = format!("  [*] ENGINE │ {}", provider.to_uppercase());
    let l7_c = format!("  {} ENGINE {} {}", "[*]".bright_black(), "│".bright_black(), provider.to_uppercase().yellow());
    let r7_p = "    * Config auto-loaded from ~/.mano/settings.json";
    let r7_c = format!("    {} Config auto-loaded from ~/.mano/settings.json", "*".cyan());
    print_complex_row(&l7_p, &l7_c, r7_p, &r7_c);

    // --- ROW 8 ---
    let l8_p = format!("  [*] MODEL  │ {}", model);
    let l8_c = format!("  {} MODEL  {} {}", "[*]".bright_black(), "│".bright_black(), model.cyan());
    let r8_p = "  ───────────────────────────────────────────────────────────";
    let r8_c = r8_p.bright_black().to_string();
    print_complex_row(&l8_p, &l8_c, r8_p, &r8_c);

    // --- ROW 9 ---
    let l9_p = "  [*] TOOLS  │ read_file, run_command";
    let l9_c = format!("  {} TOOLS  {} {}", "[*]".bright_black(), "│".bright_black(), "read_file, run_command".normal());
    let r9_p = "  [ RECENT ADVANCEMENTS ]";
    let r9_c = r9_p.truecolor(15, 98, 254).to_string();
    print_complex_row(l9_p, &l9_c, r9_p, &r9_c);

    // --- ROW 10 ---
    let l10_p = format!("  [*] PATH   │ {}", dir_display);
    let l10_c = format!("  {} PATH   {} {}", "[*]".bright_black(), "│".bright_black(), dir_display.normal());
    let r10_p = "    * Multi-provider dynamic routing via Rig-Core";
    let r10_c = format!("    {} Multi-provider dynamic routing via Rig-Core", "*".cyan());
    print_complex_row(&l10_p, &l10_c, r10_p, &r10_c);

    // --- ROW 11 ---
    let r11_p = "    * Native host environment integration";
    let r11_c = format!("    {} Native host environment integration", "*".cyan());
    print_complex_row("", "", r11_p, &r11_c);

    // --- ROW 12 (Spacing) ---
    print_complex_row("", "", "", "");

    // Bottom border: Exactly 108 characters total (1 + 106 dashes + 1)
    println!("{}", border(&format!("╰{}╯", "─".repeat(106))));
    println!();
}

/// Runs the interactive REPL: prompts the user, sends input plus history to the agent, and prints replies until "exit"/"quit".
pub async fn run_chat_loop<M: CompletionModel>(agent: Agent<M>, provider: &str, model: &str) {
    print_banner(provider, model);

    // Initialize our persistent memory buffer
    let mut chat_history: Vec<Message> = Vec::new();

    loop {
        let input = Text::new("❯").prompt();

        match input {
            Ok(prompt) => {
                let prompt = prompt.trim();
                if prompt.eq_ignore_ascii_case("exit") || prompt.eq_ignore_ascii_case("quit") {
                    break;
                }
                if prompt.is_empty() {
                    continue;
                }

                let spinner = ProgressBar::new_spinner();
                spinner.set_style(
                    ProgressStyle::default_spinner()
                        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
                        .template("{spinner:.magenta} {msg}")
                        .unwrap(),
                );
                spinner.set_message("Mano is thinking and running tools...");
                spinner.enable_steady_tick(std::time::Duration::from_millis(100));

                // We use `chat` instead of `prompt` and pass our mutable history
                let response = agent.chat(prompt, chat_history.clone()).await;

                spinner.finish_and_clear();

                match response {
                    Ok(reply) => {
                        println!("🤖 {}\n", reply.white());
                        
                        // Append the user's prompt to memory
                        chat_history.push(Message {
                            role: "user".to_string(),
                            content: prompt.to_string(),
                        });
                        
                        // Append Mano's response to memory
                        chat_history.push(Message {
                            role: "assistant".to_string(),
                            content: reply,
                        });
                    },
                    Err(e) => eprintln!("{} {}\n", "Error:".red().bold(), e),
                }
            }
            Err(_) => {
                println!("Exiting Mano...");
                break;
            }
        }
    }
}

// pub async fn run_chat_loop_Prompt<M: CompletionModel>(agent: Agent<M>, provider: &str, model: &str) {
//     print_banner(provider, model);

//     loop {
//         let input = Text::new("❯").prompt();

//         match input {
//             Ok(prompt) => {
//                 let prompt = prompt.trim();
//                 if prompt.eq_ignore_ascii_case("exit") || prompt.eq_ignore_ascii_case("quit") {
//                     break;
//                 }
//                 if prompt.is_empty() {
//                     continue;
//                 }

//                 let spinner = ProgressBar::new_spinner();
//                 spinner.set_style(
//                     ProgressStyle::default_spinner()
//                         .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
//                         .template("{spinner:.magenta} {msg}")
//                         .unwrap(),
//                 );
//                 spinner.set_message("Mano is thinking and running tools...");
//                 spinner.enable_steady_tick(std::time::Duration::from_millis(100));

//                 let response = agent.prompt(prompt).await;

//                 spinner.finish_and_clear();

//                 match response {
//                     Ok(reply) => println!("🤖 {}\n", reply.white()),
//                     Err(e) => eprintln!("{} {}\n", "Error:".red().bold(), e),
//                 }
//             }
//             Err(_) => {
//                 println!("Exiting Mano...");
//                 break;
//             }
//         }
//     }
// }