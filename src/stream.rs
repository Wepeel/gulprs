use std::collections::VecDeque;
use std::task::Poll;

pub struct GulpStream<Type: Unpin> {
    queue: VecDeque<Type>,
}

impl<Type: Unpin> GulpStream<Type> {
    pub fn new() -> Self {
        GulpStream {
            queue: VecDeque::new(),
        }
    }
}

impl<Type: Unpin> std::stream::Stream for GulpStream<Type> {
    type Item = Type;
    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::option::Option<<Self as std::stream::Stream>::Item>> {
        match self.get_mut().queue.pop_back() {
            Some(value) => Poll::Ready(Some(value)),
            None => Poll::Pending,
        }
    }
}
