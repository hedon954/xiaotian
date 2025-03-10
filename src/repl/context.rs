use std::ops::Deref;
use std::sync::Arc;
use std::thread;

use crossbeam_channel as mpsc;
use tokio::runtime::Runtime;

use crate::process::Processor;
use crate::sources::DefaultSourceFactory;
use crate::storage::MemoryStorage;

use super::CmdExector;
use super::cmd::ReplCommand;

pub struct ReplContext {
    pub tx: mpsc::Sender<ReplMsg>,
}

pub struct ReplMsg {
    cmd: ReplCommand,
    tx: oneshot::Sender<String>,
}

impl ReplContext {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded::<ReplMsg>();
        let rt = Runtime::new().expect("Failed to create tokio runtime");

        let storage = Arc::new(MemoryStorage::new());
        let source_factory = Arc::new(DefaultSourceFactory::new(None).unwrap());
        let mut processor = Processor::new(storage, source_factory);

        thread::Builder::new()
            .name("ReplBackend".to_string())
            .spawn(move || {
                while let Ok(msg) = rx.recv() {
                    if let Err(e) = rt.block_on(async {
                        let ret = msg.cmd.execute(&mut processor).await?;
                        msg.tx.send(ret)?;
                        Ok::<_, anyhow::Error>(())
                    }) {
                        eprintln!("Failed to execute command: {}", e);
                    }
                }
            })
            .unwrap();
        Self { tx }
    }

    pub fn send(&self, msg: ReplMsg, rx: oneshot::Receiver<String>) -> Option<String> {
        if let Err(e) = self.tx.send(msg) {
            eprintln!("Failed to send message to channel: {}", e);
            std::process::exit(1);
        }
        rx.recv().ok()
    }
}

impl ReplMsg {
    pub fn new(cmd: impl Into<ReplCommand>) -> (Self, oneshot::Receiver<String>) {
        let (tx, rx) = oneshot::channel();
        (
            Self {
                cmd: cmd.into(),
                tx,
            },
            rx,
        )
    }
}

impl Deref for ReplContext {
    type Target = mpsc::Sender<ReplMsg>;

    fn deref(&self) -> &Self::Target {
        &self.tx
    }
}

impl Default for ReplContext {
    fn default() -> Self {
        Self::new()
    }
}
