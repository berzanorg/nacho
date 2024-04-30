use tokio::sync::mpsc;

#[derive(Clone, Copy, Debug)]
pub struct Processor {
    pub(crate) sender: &'static mpsc::Sender<u32>,
}

impl Processor {
    pub async fn start_merge(&self, at: u32) {
        self.sender.send(at).await.unwrap()
    }
}
