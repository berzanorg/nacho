use nacho_burns_db::BurnsDb;
use tokio::sync::mpsc;

use super::{Processor, Request};

pub fn process(path: &str) -> Processor {
    let path = path.to_owned();

    let (sender, mut receiver) = mpsc::channel::<Request>(1000);

    tokio::spawn(async move {
        let mut burns_db = BurnsDb::new(path).await.unwrap();

        while let Some(request) = receiver.recv().await {
            match request {
                Request::GetBurn {
                    sender,
                    burner,
                    token_id,
                } => {
                    let burn = burns_db.get(&burner, &token_id).await;

                    sender.send(burn.ok()).unwrap();
                }
                Request::GetBurns { sender, burner } => {
                    let burns = burns_db.get_many(&burner).await;

                    sender.send(burns.ok()).unwrap();
                }
                Request::GetWitness {
                    sender,
                    burner,
                    token_id,
                } => {
                    let single_witness = burns_db.get_single_witness(&burner, &token_id).await;

                    sender.send(single_witness.ok()).unwrap();
                }
                Request::GetNewWitness { sender } => {
                    let new_witness = burns_db.get_new_single_witness().await;

                    sender.send(new_witness.ok()).unwrap();
                }
                Request::PushBurn { sender, burn } => {
                    let result = burns_db.push(&burn).await;

                    sender.send(result.ok()).unwrap()
                }
                Request::UpdateBurn { sender, burn } => {
                    let result = burns_db.update(&burn).await;

                    sender.send(result.ok()).unwrap()
                }
                Request::PushLeaf { sender, burn } => {
                    let result = burns_db.push_leaf(&burn).await;

                    sender.send(result.ok()).unwrap()
                }
                Request::UpdateLeaf { sender, burn } => {
                    let result = burns_db.update_leaf(&burn).await;

                    sender.send(result.ok()).unwrap()
                }
                Request::GetRoot { sender } => {
                    let result = burns_db.get_root().await;

                    sender.send(result.ok().map(|root| root.into())).unwrap();
                }
            }
        }
    });

    Processor {
        sender: Box::leak(Box::new(sender)),
    }
}
