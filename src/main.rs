use anyhow::Result;
use neovim_lib::{Neovim, NeovimApi, Session};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "Control nvim from the CLI!")]
struct Control {
    /// run an arbitrary command
    cmd: String,
    #[structopt(default_value = "nvim", long)]
    vim_type: String,
    #[structopt(long)]
    file: Option<String>
}

fn main() -> Result<()> {
    let args = Control::from_args();

    let full_file = match args.file {
        None => None,
        Some(ref f) => std::fs::canonicalize(f).ok()
    };

    vec![std::env::var("TMPDIR").ok(), Some(String::from("/tmp"))]
        .into_iter()
        .flatten()
        .filter_map(|tmpdir| std::fs::read_dir(tmpdir).ok())
        .for_each(|dir| {
            dir
                .filter_map(|f| f.ok())
                .filter(|f| {
                    let is_dir =
                        matches!(f.file_type().map(|t| t.is_dir()), Ok(true));
                    let name_heuristic =
                        f.file_name().to_string_lossy().starts_with(&args.vim_type);
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
                    if let Some(full_file) = full_file.clone() {
                        let full_buf_name = nvim.get_current_buf()
                            .iter().flat_map(|buf| buf.get_name(&mut nvim))
                            .flat_map(|bufname| std::fs::canonicalize(bufname))
                            .collect::<Vec<_>>();
                        if Some(&full_file) == full_buf_name.get(0) {
                            let _ = nvim
                                .command(&args.cmd)
                                .map_err(|e| eprintln!("Error: {}", e));
                        } else {
                            println!("Skipping non-matching file {:?}", full_buf_name.get(0))
                        }
                    } else {
                        let _ = nvim
                            .command(&args.cmd)
                            .map_err(|e| eprintln!("Error: {}", e));
                    }
                })
        });

    Ok(())
}
