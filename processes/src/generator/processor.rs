use tokio::sync::Notify;

#[derive(Clone, Copy, Debug)]
pub struct Processor {
    pub(crate) notify: &'static Notify,
}

impl Processor {
    pub fn keep_generating(&self) {
        self.notify.notify_one();
    }
}
