#[actix_rt::main] 
async fn main() {
    // start new actor
    const COUNT_VALUE: usize = 20;
    let addr = MyActor{ count: COUNT_VALUE}.start(); //gives addr value of COUNT_VALUE

    // send message and get future for result
    const PING_VALUE: usize = 5;
    let res = addr.send(Ping(PING_VALUE)).await; //adds PING_VALUE to current addr value

    // handle() returns tokio handle
    println!("RESULT: {}", res.unwrap());

    // stop system and exit
    System::current().stop();
}


use actix::prelude::*;

struct MyActor {
    count: usize,
}

impl Actor for MyActor {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "usize")]
struct Ping(usize);

impl Handler<Ping> for MyActor {
    type Result = usize;

    fn handle(&mut self, msg: Ping, _ctx: &mut Context<Self>) -> Self::Result {
        self.count += msg.0;

        self.count
    }
}