use nacho_burns_db::BurnsDb;
use nacho_data_structures::Burn;
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
                    let burn = burns_db
                        .get(&burner, &token_id)
                        .await
                        .map(|burn| burn.token_amount);

                    sender.send(burn.ok()).unwrap();
                }
                Request::GetBurns { sender, burner } => {
                    let burns = burns_db.get_many(&burner).await.map(|burns| {
                        burns
                            .into_iter()
                            .map(|burns| (burns.token_id, burns.token_amount))
                            .collect()
                    });

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
                Request::PushBurn {
                    sender,
                    burner,
                    token_id,
                    token_amount,
                } => {
                    let result = burns_db
                        .push(&Burn {
                            burner,
                            token_id,
                            token_amount,
                        })
                        .await;

                    sender.send(result.ok()).unwrap()
                }
                Request::UpdateBurn {
                    sender,
                    burner,
                    token_id,
                    token_amount,
                } => {
                    let result = burns_db
                        .update(&Burn {
                            burner,
                            token_id,
                            token_amount,
                        })
                        .await;

                    sender.send(result.ok()).unwrap()
                }
                Request::PushLeaf {
                    sender,
                    burner,
                    token_id,
                    token_amount,
                } => {
                    let result = burns_db
                        .push_leaf(&Burn {
                            burner,
                            token_id,
                            token_amount,
                        })
                        .await;

                    sender.send(result.ok()).unwrap()
                }
                Request::UpdateLeaf {
                    sender,
                    burner,
                    token_id,
                    token_amount,
                } => {
                    let result = burns_db
                        .update_leaf(&Burn {
                            burner,
                            token_id,
                            token_amount,
                        })
                        .await;

                    sender.send(result.ok()).unwrap()
                }
            }
        }
    });

    Processor {
        sender: Box::leak(Box::new(sender)),
    }
}
