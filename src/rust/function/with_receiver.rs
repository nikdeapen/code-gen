use crate::rust::Receiver;

/// An element with an optional function receiver.
pub trait WithReceiver: Sized {
    /// Gets the optional function receiver.
    fn receiver(&self) -> Option<Receiver>;

    /// Sets the function `receiver`.
    fn set_receiver(&mut self, receiver: Receiver);

    /// Sets the function `receiver`.
    fn with_receiver(mut self, receiver: Receiver) -> Self {
        self.set_receiver(receiver);
        self
    }
}
