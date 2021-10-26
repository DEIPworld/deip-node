#[async_trait::async_trait]
pub trait ActorI<I> {
    async fn recv(&mut self) -> Option<I>;
}

#[async_trait::async_trait]
pub trait ActorO<O> {
    async fn send(&mut self, output: O) -> Result<(), ()>;
}

#[async_trait::async_trait]
pub trait ActorIO<I, O>: ActorI<I> + ActorO<O> + Sized
{
    type Input: ActorI<I>;
    type Output: ActorO<O>;
 
    type Pair: ActorIO<O, I>;
    
    fn pair() -> (Self, Self::Pair);
    
    fn split(self) -> (Self::Input, Self::Output);
}
