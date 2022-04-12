use nix::unistd::{fork, close, pipe, ForkResult, write, getpid, getppid};

fn main() {
  let (r, w) = pipe().unwrap();

  match unsafe{fork()} { 
    Ok(result) => {
        match result {
            ForkResult::Parent { child } => {
                close(r);
            }

            ForkResult::Child => {
                close(w);
            }

        }
    }

    Err(_) => println!("Fork failed"),


}