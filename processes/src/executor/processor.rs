use tokio::sync::{mpsc, oneshot};

use super::Request;

#[derive(Clone, Copy, Debug)]
pub struct Processor {
    pub(crate) sender: &'static mpsc::Sender<Request>,
}

impl Processor {
    pub async fn execute_next(&self) -> Option<()> {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();

        self.sender
            .send(Request::ExecuteNext {
                sender: oneshot_sender,
            })
            .await
            .ok()?;

        let result = oneshot_receiver.await.ok()?;

        result
    }
}
