use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor};
use prj::{prt_red, prt_green, now, parse_input};
use log::{info, warn, error, debug, trace};
use async_nats::ConnectOptions;

mod sub;

const TIMEFORMAT_ISO8601:&str = "%Y-%m-%dT%H:%M:%S%:z";

const LOG_FILE:&str = "local.log";
const LOG_FILTER:log::LevelFilter = log::LevelFilter::Info;

const HISTORY_FILE:&str = "history.txt";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    dotenvy::from_filename(".env").ok();

    setup_logger().expect("Logger konnte nicht initialisiert werden");

    info!("🟢 Info-Log");
    warn!("🟡 Warnung");
    error!("🔴 Fehler!");
    debug!("🔵 Debug (nur mit RUST_LOG=debug)");
    trace!("🟣 Trace (sehr detailliert)");

    let client = match ConnectOptions::with_token(std::env::var("NATS_TOKEN").unwrap())
        .connect(format!("nats://{}:4222", std::env::var("NATS_HOST").unwrap()))
        .await {
            Ok(c) => {
                println!("✅ Verbunden!");
                c
            },
            Err(e) => {
                eprintln!("❌ Verbindungsfehler: {}", e);
                return Err(e.into());
            }
        };

    let _ = sub::subscribe(client.clone()).await;

    let mut rl = DefaultEditor::new().unwrap();

    // Optional: Vorherige History laden
    let _ = rl.load_history(HISTORY_FILE);

    
    loop {
        let readline = rl.readline("\x1b[33m>> \x1b[0m");
        match readline {
            Ok(line) => {
                let input = line.trim();
                let _ = rl.add_history_entry(input);
                
                let (cmd, args, opts) = parse_input(input);// Zur History hinzufügen
                
                println!("befehl: {}, args: {:?}", cmd, args);

                match cmd.as_str() {
                    "hallo" => prt_green("Was geht, Bestermann?"),
                    "exit" | "q" => {
                        prt_red("Ciao Kakao!");
                        break;
                    }
                    "say" => {
                        let _ = sub::publish(client.clone(), &args[0]).await;
                    }
                    _ => println!("Unbekannter Befehl: {}", input),
                }
            }
            Err(ReadlineError::Interrupted) => {
                prt_red("Strg+C gedrückt. Abbruch...");
                break;
            }
            Err(ReadlineError::Eof) => {
                prt_red("Strg+D gedrückt. Auf Wiedersehen!");
                break;
            }
            Err(err) => {
                println!("Fehler: {:?}", err);
                break;
            }
        }
    }

    // History speichern
    let _ = rl.save_history(HISTORY_FILE);

    Ok(())
}

fn setup_logger() -> Result<(), fern::InitError> {
    
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}] {}",
                now().format(TIMEFORMAT_ISO8601),
                record.level(),
                message
            ))
        })
        .level(LOG_FILTER)
        .chain(std::io::stdout())
        .chain(fern::log_file(LOG_FILE)?)
        .apply()?;
    Ok(())
}