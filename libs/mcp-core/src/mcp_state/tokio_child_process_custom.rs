use rmcp::{
    service::ServiceRole,
    transport::{async_rw::TransportAdapterAsyncRW, IntoTransport, Transport},
};
use tokio::{
    io::AsyncRead,
    process::{Child, ChildStdin, ChildStdout},
};

#[cfg(windows)]
use crate::utils::command::CREATE_NO_WINDOW;

pub(crate) fn child_process(
    mut child: tokio::process::Child,
) -> std::io::Result<(tokio::process::Child, (ChildStdout, ChildStdin))> {
    let child_stdin = match child.stdin.take() {
        Some(stdin) => stdin,
        None => return Err(std::io::Error::other("std in was taken")),
    };
    let child_stdout = match child.stdout.take() {
        Some(stdout) => stdout,
        None => return Err(std::io::Error::other("std out was taken")),
    };
    Ok((child, (child_stdout, child_stdin)))
}

pub struct TokioChildProcessCustom {
    child: Child,
    child_stdin: ChildStdin,
    child_stdout: ChildStdout,
}

// we hold the child process with stdout, for it's easier to implement AsyncRead
pin_project_lite::pin_project! {
    pub struct TokioChildProcessOut {
        child: Child,
        #[pin]
        child_stdout: ChildStdout,
    }
}

impl AsyncRead for TokioChildProcessOut {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        self.project().child_stdout.poll_read(cx, buf)
    }
}

impl TokioChildProcessCustom {
    pub fn new(mut command: tokio::process::Command) -> std::io::Result<Self> {
        command
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped());
        command.kill_on_drop(true);
        #[cfg(windows)]
        {
            command.creation_flags(CREATE_NO_WINDOW);
        }
        let (child, (child_stdout, child_stdin)) = child_process(command.spawn()?)?;
        Ok(Self {
            child,
            child_stdin,
            child_stdout,
        })
    }

    pub fn split(self) -> (TokioChildProcessOut, ChildStdin) {
        let TokioChildProcessCustom {
            child,
            child_stdin,
            child_stdout,
        } = self;
        (
            TokioChildProcessOut {
                child,
                child_stdout,
            },
            child_stdin,
        )
    }
}

impl<R: ServiceRole> IntoTransport<R, std::io::Error, ()> for TokioChildProcessCustom {
    fn into_transport(self) -> impl Transport<R, Error = std::io::Error> + 'static {
        IntoTransport::<R, std::io::Error, TransportAdapterAsyncRW>::into_transport(self.split())
    }
}

pub trait ConfigureCommandExt {
    fn configure(self, f: impl FnOnce(&mut Self)) -> Self;
}

impl ConfigureCommandExt for tokio::process::Command {
    fn configure(mut self, f: impl FnOnce(&mut Self)) -> Self {
        f(&mut self);
        self
    }
}
