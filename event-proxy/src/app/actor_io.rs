
use tokio::sync::mpsc;

use crate::actor::*;


pub struct ActorJackI<I>(mpsc::Receiver<I>);
pub struct ActorJackO<O>(mpsc::Sender<O>);

pub struct ActorJack<I, O> {
    input: ActorJackI<I>,
    output: ActorJackO<O>,
}

#[async_trait::async_trait]
impl<I: Send> ActorI<I> for ActorJackI<I> {
    async fn recv(&mut self) -> Option<I> {
        self.0.recv().await
    }
}

#[async_trait::async_trait]
impl<O: Send> ActorO<O> for ActorJackO<O> {
    async fn send(&mut self, output: O) -> Result<(), ()> {
        self.0.send(output).await.map_err(|_| ())
    }
}

#[async_trait::async_trait]
impl<I: Send, O: Send> ActorI<I> for ActorJack<I, O> {
    async fn recv(&mut self) -> Option<I> {
        self.input.recv().await
    }
}

#[async_trait::async_trait]
impl<I: Send, O: Send> ActorO<O> for ActorJack<I, O> {
    async fn send(&mut self, output: O) -> Result<(), ()> {
        self.output.send(output).await
    }
}

#[async_trait::async_trait]
impl<I: Send, O: Send> ActorIO<I, O> for ActorJack<I, O> 
{
    type Input = ActorJackI<I>;
    type Output = ActorJackO<O>;

    type Pair = ActorJack<O, I>;

    fn pair() -> (Self, Self::Pair) {
        let (tx1, rx2) = mpsc::channel(1);
        let (tx2, rx1) = mpsc::channel(1);
        (Self { input: ActorJackI(rx1), output: ActorJackO(tx1) },
         ActorJack::<O, I> { input: ActorJackI(rx2), output: ActorJackO(tx2) })
    }

    fn split(self) -> (Self::Input, Self::Output) {
        let Self { input, output } = self;
        (input, output)
    }
}

#[allow(dead_code)]
pub type ActorJackPair<IO, I, O> = <IO as ActorIO<I, O>>::Pair;
