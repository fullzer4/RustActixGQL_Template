use juniper::{RootNode, EmptyMutation};

pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
    fn pings() -> Vec<Iping> {
        vec![
            Iping {
                pong: "Pong!"
            },
            Iping {
                pong: "Pong!"
            }
        ]
    }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new())
}
