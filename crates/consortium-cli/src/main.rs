use std::process;

use consortium_app::remoteproc::{RemoteProc, RemoteProcCoredump};

const HELP: &str = "\
consortium-cli — manage Consortium remote processors

USAGE:
    consortium-cli <COMMAND> [OPTIONS]

COMMANDS:
    status   [--index <N>]              Print the state of remoteproc(s)
    start    --index <N>                Start a remote processor
    stop     --index <N>                Stop a remote processor
    firmware --index <N> --path <PATH>  Set the firmware image for a remote processor
    coredump --index <N> --mode <MODE>  Set coredump mode (enabled | disabled | inline)
    recovery --index <N> --enable       Enable automatic recovery
    recovery --index <N> --disable      Disable automatic recovery

OPTIONS:
    -h, --help      Print this help message
    -V, --version   Print version information
";

fn usage_error(msg: &str) -> ! {
    eprintln!("error: {msg}");
    eprintln!("Run `consortium-cli --help` for usage.");
    process::exit(1);
}

fn parse_index(args: &[String], flag: &str) -> usize {
    let pos = args
        .iter()
        .position(|a| a == flag)
        .unwrap_or_else(|| usage_error(&format!("missing required flag {flag}")));
    let val = args
        .get(pos + 1)
        .unwrap_or_else(|| usage_error(&format!("flag {flag} requires a value")));
    val.parse::<usize>()
        .unwrap_or_else(|_| usage_error(&format!("value for {flag} must be an integer, got {val:?}")))
}

fn parse_flag_value<'a>(args: &'a [String], flag: &str) -> &'a str {
    let pos = args
        .iter()
        .position(|a| a == flag)
        .unwrap_or_else(|| usage_error(&format!("missing required flag {flag}")));
    args.get(pos + 1)
        .map(String::as_str)
        .unwrap_or_else(|| usage_error(&format!("flag {flag} requires a value")))
}

fn cmd_status(args: &[String]) {
    if let Some(pos) = args.iter().position(|a| a == "--index") {
        // Single remoteproc
        let idx: usize = args
            .get(pos + 1)
            .and_then(|v| v.parse().ok())
            .unwrap_or_else(|| usage_error("--index requires an integer value"));
        let rp = RemoteProc::new(idx);
        match rp.state() {
            Ok(state) => println!("remoteproc{idx}: {state}"),
            Err(e) => {
                eprintln!("error reading remoteproc{idx}: {e}");
                process::exit(1);
            }
        }
    } else {
        // Scan a reasonable range of remoteproc indices
        let mut found = false;
        for idx in 0..8 {
            let rp = RemoteProc::new(idx);
            match rp.state() {
                Ok(state) => {
                    println!("remoteproc{idx}: {state}");
                    found = true;
                }
                Err(_) => {
                    // Stop at the first missing entry
                    break;
                }
            }
        }
        if !found {
            println!("No remote processors found.");
        }
    }
}

fn cmd_start(args: &[String]) {
    let idx = parse_index(args, "--index");
    let rp = RemoteProc::new(idx);
    rp.start().unwrap_or_else(|e| {
        eprintln!("error: failed to start remoteproc{idx}: {e}");
        process::exit(1);
    });
    println!("remoteproc{idx}: started");
}

fn cmd_stop(args: &[String]) {
    let idx = parse_index(args, "--index");
    let rp = RemoteProc::new(idx);
    rp.stop().unwrap_or_else(|e| {
        eprintln!("error: failed to stop remoteproc{idx}: {e}");
        process::exit(1);
    });
    println!("remoteproc{idx}: stopped");
}

fn cmd_firmware(args: &[String]) {
    let idx = parse_index(args, "--index");
    let path = parse_flag_value(args, "--path");
    let rp = RemoteProc::new(idx);
    rp.set_firmware(path).unwrap_or_else(|e| {
        eprintln!("error: failed to set firmware for remoteproc{idx}: {e}");
        process::exit(1);
    });
    println!("remoteproc{idx}: firmware set to {path:?}");
}

fn cmd_coredump(args: &[String]) {
    let idx = parse_index(args, "--index");
    let mode_str = parse_flag_value(args, "--mode");
    let mode = RemoteProcCoredump::try_from(mode_str.to_string()).unwrap_or_else(|e| {
        eprintln!("error: {e}");
        process::exit(1);
    });
    let rp = RemoteProc::new(idx);
    rp.set_coredump(mode).unwrap_or_else(|e| {
        eprintln!("error: failed to set coredump mode for remoteproc{idx}: {e}");
        process::exit(1);
    });
    println!("remoteproc{idx}: coredump mode set to {mode_str:?}");
}

fn cmd_recovery(args: &[String]) {
    let idx = parse_index(args, "--index");
    let enable = if args.iter().any(|a| a == "--enable") {
        true
    } else if args.iter().any(|a| a == "--disable") {
        false
    } else {
        usage_error("recovery requires either --enable or --disable");
    };
    let rp = RemoteProc::new(idx);
    rp.set_recovery(enable).unwrap_or_else(|e| {
        eprintln!("error: failed to set recovery for remoteproc{idx}: {e}");
        process::exit(1);
    });
    println!(
        "remoteproc{idx}: recovery {}",
        if enable { "enabled" } else { "disabled" }
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // args[0] is the binary name; the subcommand is args[1].
    let subcommand = args.get(1).map(String::as_str).unwrap_or("--help");

    match subcommand {
        "--help" | "-h" => {
            print!("{HELP}");
        }
        "--version" | "-V" => {
            println!("consortium-cli {}", env!("CARGO_PKG_VERSION"));
        }
        "status" => cmd_status(&args[2..]),
        "start" => cmd_start(&args[2..]),
        "stop" => cmd_stop(&args[2..]),
        "firmware" => cmd_firmware(&args[2..]),
        "coredump" => cmd_coredump(&args[2..]),
        "recovery" => cmd_recovery(&args[2..]),
        other => usage_error(&format!("unknown subcommand {other:?}")),
    }
}
