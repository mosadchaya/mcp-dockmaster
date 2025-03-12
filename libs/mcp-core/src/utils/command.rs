use std::ffi::OsStr;
use tokio::process::Command;

pub fn get_default_shell() -> Option<String> {
    if cfg!(windows) {
        Some("powershell".to_string())
    } else {
        let shell_env = std::env::var("SHELL");
        match shell_env {
            Ok(shell_env) => Some(shell_env),
            Err(_) => None,
        }
    }
}

pub struct CommandWrappedInShellBuilder {
    program: String,
    args: Option<Vec<String>>,
}

impl CommandWrappedInShellBuilder {
    pub fn new<S: AsRef<OsStr>>(program: S) -> Self {
        Self {
            program: program.as_ref().to_string_lossy().to_string(),
            args: None,
        }
    }

    pub fn new_with_args<S, I>(program: S, args: I) -> Self
    where
        S: AsRef<OsStr>,
        I: IntoIterator<Item = S>,
    {
        Self {
            program: program.as_ref().to_string_lossy().to_string(),
            args: Some(
                args.into_iter()
                    .map(|s| s.as_ref().to_string_lossy().to_string())
                    .collect(),
            ),
        }
    }

    /// Appends arguments to the command.
    pub fn args<I, S>(&mut self, args: I) -> &mut CommandWrappedInShellBuilder
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.args = Some(
            args.into_iter()
                .map(|s| s.as_ref().to_string_lossy().to_string())
                .collect(),
        );
        self
    }

    /// Appends an argument to the command.
    pub fn arg<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut CommandWrappedInShellBuilder {
        match &mut self.args {
            Some(args_vec) => args_vec.push(arg.as_ref().to_string_lossy().to_string()),
            None => self.args = Some(vec![arg.as_ref().to_string_lossy().to_string()]),
        }
        self
    }

    pub fn build(self) -> Command {
        let command_with_args = if let Some(args) = self.args {
            self.program.clone() + " " + &args.join(" ")
        } else {
            self.program.clone()
        };
        Command::wrapped_in_shell(command_with_args)
    }
}

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;
pub trait CommandExt {
    fn wrapped_in_shell<S: AsRef<OsStr>>(program: S) -> Command;
}

impl CommandExt for Command {
    fn wrapped_in_shell<S: AsRef<OsStr>>(program_with_args: S) -> Command {
        let shell = get_default_shell();
        let mut command = match shell {
            Some(shell) => {
                let mut cmd = Self::new(shell);
                cmd.arg("-c").arg(program_with_args);
                cmd
            }
            None => Self::new(program_with_args),
        };
        #[cfg(windows)]
        {
            command.creation_flags(CREATE_NO_WINDOW);
        }
        command
    }
}
