use std::fs;
use daemonize::{Daemonize, DaemonizeError};

#[derive(Debug, Clone)]
pub struct Daemon<'s> {
    port:           u16,
    pub log:        Option<&'s str>,
    pub pid_file:   Option<&'s str>,
    pub workdir:    Option<&'s str>, 
}

impl<'s> Daemon<'s> {
    pub fn new(port: u16) -> Self {
        Daemon {
            port: port,
            log: None,
            pid_file: None,
            workdir: None
        }
    }

    pub fn daemonize(&self) -> Result<&str, DaemonizeError> {
        let stdout = fs::File::create(self.log.unwrap_or("/tmp/gmdp.log")).unwrap();
        let stderr = stdout.try_clone().unwrap();
        let port = self.port;
        let daemonize = Daemonize::new()
            .pid_file(self.pid_file.unwrap_or("/tmp/gmdp.pid"))
            .chown_pid_file(true)
            .working_directory(self.workdir.unwrap_or("/tmp"))
            .user("nobody")
            .group("daemon")
            .stdout(stdout)
            .stderr(stderr)
            .exit_action(move || println!("Server running 127.0.0.1:{}", port))
            .privileged_action(|| "Executed before drop privileged");

        daemonize.start()
    }
}


