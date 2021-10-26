mod io;

pub use io::*;


pub trait ActorDirectiveT<Data> {
    fn exit_loop(&self) -> bool;
    fn input(self) -> Option<Data>;
}

#[async_trait::async_trait]
pub trait Actor<D, I, O, IO>
    where I: ActorDirectiveT<D>,
          IO: ActorI<I> + ActorO<O> + Send + 'static,
          O: Send,
          D: Send,
{
    async fn actor_loop(&mut self, mut io: IO) {
        loop {
            let data = match io.recv().await {
                Some(d) if d.exit_loop() => return,
                Some(d) => d.input().expect("input directive"),
                None => return,
            };
            let output = self.on_input(data).await;
            if io.send(output).await.is_err() {
                return;
            }
        }
    }
    
    async fn on_input(&mut self, data: D) -> O;
}

pub enum ActorDirective<D> {
    #[allow(dead_code)]
    ExitLoop,
    Input(D),
}

impl<D> ActorDirectiveT<D> for ActorDirective<D> {
    
    fn exit_loop(&self) -> bool {
        matches!(self, Self::ExitLoop)
    }

    fn input(self) -> Option<D> {
        if let Self::Input(data) = self {
            Some(data)
        } else { 
            None
        }
    }
}
