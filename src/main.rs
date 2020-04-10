use anyhow::Result;
use neovim_lib::{Neovim, NeovimApi, Session, Value};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "Control nvim from the CLI!")]
struct Control {
    #[structopt(flatten)]
    subcmds: Subcommand,
    #[structopt(default_value = "/tmp/nvim_rpc")]
    sockets_dir: PathBuf,
}

#[derive(StructOpt)]
enum Subcommand {
    /// set option=value
    Set { name: String, value: String },
    /// run an arbitrary command
    Run { cmd: String },
}

impl Control {
    fn run(&self, nvim: &mut Neovim) -> Result<()> {
        match &self.subcmds {
            Subcommand::Set { name, value } => NeovimApi::set_option(
                nvim,
                name,
                Value::String(value.as_str().into()),
            )?,
            Subcommand::Run { cmd } => nvim.command(cmd)?,
        };
        Ok(())
    }
}

fn main() -> Result<()> {
    let args = Control::from_args();

    match std::fs::read_dir(&args.sockets_dir) {
        Ok(dir) => dir
            .filter_map(|f| f.ok())
            .filter_map(|d| Session::new_unix_socket(d.path()).ok())
            .map(|mut session| {
                session.start_event_loop();
                Neovim::new(session)
            })
            .for_each(|mut nvim| {
                let _ =
                    args.run(&mut nvim).map_err(|e| eprintln!("Error: {}", e));
            }),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(e) => Err(e)?,
    }

    Ok(())
}
