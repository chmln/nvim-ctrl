use anyhow::Result;
use neovim_lib::{Neovim, NeovimApi, Session};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "Control nvim from the CLI!")]
struct Control {
    /// run an arbitrary command
    cmd: String,
}

fn main() -> Result<()> {
    let args = Control::from_args();
    let socket_dir = std::env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| {
        let user_id = users::get_current_uid();
        format!("/run/user/{user_id}")
    });

    std::fs::read_dir(socket_dir)?
        .filter_map(|f| f.ok())
        .filter(|f| f.file_name().to_string_lossy().starts_with("nvim"))
        .filter_map(|d| {
            Session::new_unix_socket(d.path())
                .inspect_err(|e| {
                    eprint!("Error connecting to socket: {e}");
                })
                .ok()
        })
        .map(|mut session| {
            session.start_event_loop();
            Ok(Neovim::new(session).command(&args.cmd)?)
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(())
}
