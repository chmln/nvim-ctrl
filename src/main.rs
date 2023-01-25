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
    let tmp = std::env::var("TMPDIR").unwrap_or("/tmp".to_owned());

    match std::fs::read_dir(tmp) {
        Ok(dir) => dir
            .filter_map(|f| f.ok())
            .filter(|f| {
                let is_dir =
                    matches!(f.file_type().map(|t| t.is_dir()), Ok(true));
                let name_heuristic =
                    f.file_name().to_string_lossy().starts_with("nvim");
                is_dir && name_heuristic
            })
            .filter_map(|dir| {
                Some(
                    std::fs::read_dir(dir.path())
                        .ok()?
                        .filter_map(Result::ok)
                        .map(|d| d.path()),
                )
            })
            .flatten()
            .filter_map(|dir| {
                Some(
                    std::fs::read_dir(dir)
                        .ok()?
                        .filter_map(Result::ok)
                        .map(|d| d.path()),
                )
            })
            .flatten()
            .filter_map(|d| Session::new_unix_socket(d).ok())
            .map(|mut session| {
                session.start_event_loop();
                Neovim::new(session)
            })
            .for_each(|mut nvim| {
                let _ = nvim
                    .command(&args.cmd)
                    .map_err(|e| eprintln!("Error: {}", e));
            }),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(e) => Err(e)?,
    }

    Ok(())
}
