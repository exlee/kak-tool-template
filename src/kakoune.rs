use std::process::Stdio;

use tokio::{
    io::{self, AsyncWriteExt},
    process::Command,
};

pub struct KakClient<'a>(pub &'a str);
pub struct KakSession<'a>(pub &'a str);

pub struct Kakoune {
    session: String,
    client: String,
}

impl Kakoune {
    pub fn new(session: KakSession, client: KakClient) -> Self {
        Self {
            session: session.0.into(),
            client: client.0.into(),
        }
    }

    /// `run_command` function runs Kakoune command in Session context.
    /// 
    /// For session-client context see [`Self::run_command_in_client`].
    ///
    /// # Panics
    ///
    /// Panics if Command panics or stdin can't be opened.
    ///
    /// # Errors
    ///
    /// This function will return an error if write to stdin fails or
    /// `kak` command fails.
    pub async fn run_command(&self, command: &str) -> io::Result<()> {
        println!("Kakoune {}", command);

        let mut kak_process = Command::new("kak")
            .arg("-p")
            .arg(&self.session)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;

        let mut kak_stdin = kak_process.stdin.take().expect("Failed to open kak stdin");
        kak_stdin.write_all(command.as_bytes()).await?;

        Ok(())
    }

    /// This function runs command in Kakoune in client context,
    ///
    /// i.e. `evaluate-commands -client CLIENT %{ ... }.`
    ///
    /// # Errors
    ///
    /// This function will return an error in same cases as [`Self::run_command`].
    pub async fn run_command_in_client(&self, command: &str) -> io::Result<()> {
        let cmd = format!(
            "evaluate-commands -client {} %{{ {} }}",
            &self.client, command
        );
        self.run_command(&cmd).await
    }
}
