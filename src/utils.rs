use crossterm::{
    style::{Color, Print, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use notify::{RecursiveMode, Watcher};
use std::{
    env::{args, consts::OS, Args},
    io::stdout,
    path::Path,
    process::Command,
};

fn watch() -> notify::Result<()> {
    let mut watcher =
        notify::recommended_watcher(|res: notify::Result<notify::Event>| match res {
            Ok(event) => {
                for file in &event.paths {
                    match file.extension() {
                        Some(extension) => {
                            if extension == "py" {
                                print_colored_text(
                                    "warning",
                                    "Restarting due to file changes...\n",
                                )
                                .err();
                                let mut args: Args = args();
                                let file_name: &str = &args.nth(1).unwrap() as &str;
                                run(file_name);
                            }
                        }
                        None => {}
                    }
                }
            }
            Err(err) => println!("watch error: {err:?}"),
        })?;

    watcher.watch(Path::new("."), RecursiveMode::Recursive)?;

    Ok(())
}

pub(crate) fn run(file_name: &str) {
    if Path::new(file_name).exists() {
        match OS {
            "linux" | "macos" => {
                let stdout = Command::new("python3")
                    .arg(file_name)
                    .status()
                    .expect("[pymon] Error: Failed to run file");
                let output: &str = &stdout.to_string() as &str;
                println!("{output}");

                loop {
                    watch().err();
                }
            }
            "windows" => {
                let stdout = Command::new("python")
                    .arg(file_name)
                    .status()
                    .expect("[pymon] Error: Failed to run file");
                let output: &str = &stdout.to_string() as &str;
                println!("{output}");

                loop {
                    watch().err();
                }
            }
            _ => panic!("[pymon] Error: Operating System not supported"),
        }
    } else {
        panic!("[pymon] Error: No files matching the pattern '{file_name}' were found.")
    }
}

pub(crate) fn print_colored_text(output_type: &str, msg: &str) -> crossterm::Result<()> {
    match output_type {
        "success" => {
            stdout()
                .execute(SetForegroundColor(Color::Green))?
                .execute(Print(msg))?
                .execute(ResetColor)?;
            Ok(())
        }
        "warning" => {
            stdout()
                .execute(SetForegroundColor(Color::Yellow))?
                .execute(Print(msg))?
                .execute(ResetColor)?;
            Ok(())
        }
        "error" => {
            stdout()
                .execute(SetForegroundColor(Color::Red))?
                .execute(Print(msg))?
                .execute(ResetColor)?;
            Ok(())
        }
        _ => panic!("Error: Invalid output type provided"),
    }
}
