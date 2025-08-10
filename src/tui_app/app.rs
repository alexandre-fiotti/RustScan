//! Main TUI Application Entrypoint
//!
//! Provides a pure function `run_tui()` that owns setup, loop, and teardown.

use crossterm::{
    event::{
        self, DisableBracketedPaste, DisableMouseCapture, EnableBracketedPaste, EnableMouseCapture,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

use crate::tui_app::events::handle_event;
use crate::tui_app::{message::Message, results::ResultsMsg};
use crate::tui_app::{update::update, view::view, Model};
use std::sync::mpsc::{channel, Sender};
use std::time::Duration as StdDuration;

/// Run the TUI application
pub fn run_tui() -> io::Result<()> {
    // Create model
    let mut model = Model::new();

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        EnableMouseCapture,
        EnableBracketedPaste
    )?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Add welcome message to results
    model.results_mut().push_lines(vec![
        "RustScan TUI".to_string(),
        "Enter targets and ports, then press Tab to navigate or Enter to confirm".to_string(),
        "Use Shift+Up/Down, PageUp/PageDown, or mouse wheel to scroll".to_string(),
        "".to_string(),
    ]);

    // Run the loop
    let res = run_loop(&mut terminal, &mut model);

    // Restore terminal - more thorough cleanup
    disable_raw_mode()?;

    // Clear any pending input/mouse events before disabling mouse capture
    while event::poll(Duration::from_millis(0))? {
        let _ = event::read()?; // Drain the event queue
    }

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
        DisableBracketedPaste
    )?;
    terminal.show_cursor()?;

    // Ensure all terminal commands are flushed
    terminal.backend_mut().flush()?;

    // Small delay to let terminal process the disable commands
    thread::sleep(Duration::from_millis(50));

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

/// Run the main application loop
fn run_loop<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    model: &mut Model,
) -> io::Result<()> {
    loop {
        // Render current screen
        terminal.draw(|f| view(model, f))?;

        // Handle events
        if event::poll(std::time::Duration::from_millis(50))? {
            let event = event::read()?;
            if let Some(mut msg) = handle_event(model, event)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{e:?}")))?
            {
                // Cascade updates as long as update returns a follow-up message
                loop {
                    if let Some(next) = update(model, msg) {
                        msg = next;
                        continue;
                    }
                    break;
                }
            }
        }

        // If a short-lived activation is in progress, finish it when due and then start scan
        if model.scan_config_mut().maybe_finish_button_activation() {
            if let Some(next) = update(model, crate::tui_app::message::AppMsg::StartScan.into()) {
                // Handle any cascaded follow-ups
                let mut msg = next;
                loop {
                    if let Some(next2) = update(model, msg) {
                        msg = next2;
                        continue;
                    }
                    break;
                }
            }
        }

        // Handle scan lifecycle
        match model.scan_state() {
            crate::tui_app::model::ScanState::Requested => {
                // Build Opts from ScanConfig
                let cfg = model.scan_config().clone();
                let (tx, rx) = channel::<Message>();
                model.set_scan_results_rx(rx);
                crate::tui_app::results::set_results_sender(tx.clone());
                spawn_scan_worker(cfg, tx);
                model.set_scan_state(crate::tui_app::model::ScanState::Running);
            }
            crate::tui_app::model::ScanState::Running => {
                // Drain channel without blocking and detect completion
                use std::sync::mpsc::TryRecvError;
                let done = loop {
                    let maybe_msg = {
                        if let Some(rx) = model.scan_results_rx_ref() {
                            match rx.try_recv() {
                                Ok(m) => Some(m),
                                Err(TryRecvError::Empty) => None,
                                Err(TryRecvError::Disconnected) => break true,
                            }
                        } else {
                            None
                        }
                    };
                    if let Some(mut m) = maybe_msg {
                        loop {
                            if let Some(next) = update(model, m) {
                                m = next;
                                continue;
                            }
                            break;
                        }
                    } else {
                        break false;
                    }
                };
                if done {
                    crate::tui_app::results::clear_results_sender();
                    // Ensure any lingering worker join handle is cleared next start
                    model.set_scan_state(crate::tui_app::model::ScanState::Completed);
                }
            }
            _ => {}
        }

        if model.should_quit() {
            break;
        }
    }
    Ok(())
}

fn spawn_scan_worker(cfg: crate::tui_app::scan_config::ScanConfig, tx: Sender<Message>) {
    std::thread::spawn(move || {
        // Build Opts
        let opts_res = crate::tui_app::scan_config::build_opts_from_scan_config(&cfg);
        match opts_res {
            Ok(opts) => {
                let _ = tx.send(Message::Results(ResultsMsg::AppendLine(
                    "[Scan starting]".to_string(),
                )));
                let ips = crate::address::parse_addresses(&opts);
                if ips.is_empty() {
                    let _ = tx.send(Message::Results(ResultsMsg::AppendLine(
                        "No IPs could be resolved, aborting scan.".to_string(),
                    )));
                    return;
                }

                let strategy = crate::port_strategy::PortStrategy::pick(
                    &opts.range,
                    opts.ports.clone(),
                    opts.scan_order,
                );
                let scanner = crate::scanner::Scanner::new(
                    &ips,
                    cfg.batch_size,
                    StdDuration::from_millis(opts.timeout.into()),
                    opts.tries,
                    opts.greppable,
                    strategy,
                    opts.accessible,
                    opts.exclude_ports.clone().unwrap_or_default(),
                    opts.udp,
                );

                let scan_result = futures::executor::block_on(scanner.run());
                use std::collections::HashMap;
                use std::string::ToString;
                let mut ports_per_ip: HashMap<std::net::IpAddr, Vec<u16>> = HashMap::new();
                for socket in scan_result {
                    ports_per_ip
                        .entry(socket.ip())
                        .or_insert_with(Vec::new)
                        .push(socket.port());
                }

                for ip in &ips {
                    if let Some(ports) = ports_per_ip.get(ip) {
                        let vec_str_ports: Vec<String> =
                            ports.iter().map(ToString::to_string).collect();
                        let ports_str = vec_str_ports.join(",");
                        let _ = tx.send(Message::Results(ResultsMsg::AppendLine("".to_string())));
                        let _ = tx.send(Message::Results(ResultsMsg::AppendLine(format!(
                            "{} -> [{}]",
                            ip, ports_str
                        ))));
                    } else {
                        let x = format!(
                            "Looks like I didn't find any open ports for {:?}. This is usually caused by a high batch size.\n*I used {} batch size, consider lowering it with {} or a comfortable number for your system.\n Alternatively, increase the timeout if your ping is high. Rustscan -t 2000 for 2000 milliseconds (2s) timeout.\n",
                            ip, cfg.batch_size, "'rustscan -b <batch_size> -a <ip address>'"
                        );
                        let _ = tx.send(Message::Results(ResultsMsg::AppendLine("".to_string())));
                        let _ = tx.send(Message::Results(ResultsMsg::AppendLine(x)));
                    }
                }
                // Explicitly close channel by dropping sender
                drop(tx);
            }
            Err(e) => {
                let _ = tx.send(Message::Results(ResultsMsg::AppendLine(format!(
                    "[Error] {:?}",
                    e
                ))));
                drop(tx);
            }
        }
    });
}
