#![allow(dead_code,unused_variables)]
use std::{
    env,
    io::{self},
    process::Stdio,
    sync::Arc,
};

mod handle_context;
mod kakoune;

use tokio::{fs::OpenOptions, io::AsyncWriteExt};

use crate::kakoune::Kakoune;
use crate::{
    handle_context::Context,
    kakoune::{KakClient, KakSession},
};

/// main - main entrypoint for runner
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.iter().any(|a| a == "--worker") {
        println!("Starting worker");
        return start_worker(args);
    }

    spawn_background(args);
    Ok(())
}

/// spawn_background runs the command in background process
fn spawn_background(args: Vec<String>) {
    let current_exe = env::current_exe().expect("Failed to get current exe");
    println!("Starting background");

    let _ = std::process::Command::new(current_exe)
        .args(&args[1..])
        .arg("--worker")
        .stdin(Stdio::null())
        // Uncomment these two lines for debug
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .spawn();
}

/// start_worker - background worker that parses
fn start_worker(args: Vec<String>) -> io::Result<()> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Failed to build runtime");

    rt.block_on(async {
        let clean_args: Vec<String> = args.into_iter().filter(|a| a != "--worker").collect();

        if clean_args.len() < 2 {
            return;
        }

        let session = &clean_args[1];
        let client = &clean_args[2];
        let args = &clean_args[3..];

        let kakoune = Kakoune::new(KakSession(session), KakClient(client));
        let context = Arc::new(Context::new(&[]).expect("Can't create context"));

        handle_command(kakoune, context.clone(), args)
            .await
            .expect("Failed to handle command");
    });
    Ok(())
}

/// handle_command is the main logic processor for kak-tool;
///                modify as needed
async fn handle_command(
    kakoune: Kakoune,
    context: Arc<Context>,
    args: &[String],
) -> io::Result<()> {
    let fifo_path = Context::get_or_create_fifo_path("example")?;

    kakoune
        .run_command_in_client(&format!(
            "edit -fifo {} *example*",
            fifo_path.to_string_lossy()
        ))
        .await
        .expect("Kakoune command failed");

    let mut file: tokio::fs::File = OpenOptions::new()
        .write(true)
        .read(false)
        .open(&fifo_path)
        .await?;

    file.write_all(b"Hello world").await?;
    drop(file);
    kakoune
        .run_command_in_client("echo -markup {Error}PONG PONG")
        .await
        .expect("Kakoune command failed");
    Ok(())
}

/// _searchable_args - helper for use with tools like fd/rg
/// it errors when there are no arguments or first one is empty one
fn _searchable_args(args: &[String]) -> Result<(), io::Error> {
    if args.is_empty() || args[0].is_empty() {
        return err("Search entry empty");
    }
    Ok(())
}

/// _all_args_start_with_dash - helper for use with tools like fd/rg
/// it errors when all args start with dash
fn _all_args_start_with_dash(args: &[String]) -> Result<(), io::Error> {
    let non_dashed_arg =
        args.iter()
            .fold(false, |acc, v| if acc { true } else { !v.starts_with("-") });

    if non_dashed_arg {
        Ok(())
    } else {
        return err("All args start with dash");
    }
}

/// err is a error creation helper
fn err(msg: &str) -> Result<(), std::io::Error> {
    Err(std::io::Error::other(msg))
}
