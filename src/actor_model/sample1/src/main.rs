use ractor::{ cast, Actor, ActorProcessingErr, ActorRef, async_trait};


struct PingPong;

#[derive(Debug, Clone)]
pub enum Message {
    Ping, Pong,
}


impl Message {
    fn next(&self) -> Self {
        match self {
            Self::Ping => Self::Pong,
            Self::Pong => Self::Ping,
        }
    }

    fn print(&self) {
        match self {
            Self::Ping => print!("PING!"),
            Self::Pong => print!("PONG!!!"),
        }
    }
}

#[async_trait]
impl Actor for PingPong {
    type Msg = Message;
    type State = u8;
    type Arguments = ();

    async fn pre_start(
        &self,
        mySelf: ActorRef<Self::Msg>,
        _arguments: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr>{
        cast!(mySelf,Message::Ping);
        Ok(0u8)
    }
    
    async fn handle(
            &self,
            myself: ActorRef<Self::Msg>,
            message: Self::Msg,
            state: &mut Self::State,
        ) -> Result<(), ActorProcessingErr> {
        if *state < 10u8 {
            message.print();
            cast!(myself, message.next());
            *state += 1;
        }else{
            println!();
            myself.stop(None);
        }
        Ok(())
    }
}


#[tokio::main]
async fn main() {
    println!("こんにちは、a/sub_project から！");

    let (_actor, handle) = Actor::spawn(None, PingPong, ()).await
        .expect("なんかだめ");
    handle.await.expect("なんかだめ");
}
