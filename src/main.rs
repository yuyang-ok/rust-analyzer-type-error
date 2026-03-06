#![allow(unused)]
use std::any::Any;

fn main() {}

#[async_trait::async_trait]
trait DBTrait {
    async fn xxx(&self);
}

struct Mysql {}

#[async_trait::async_trait]
impl DBTrait for Mysql {
    async fn xxx(&self) {
        todo!()
    }
}

fn xxx(x: &dyn Any) -> Option<&dyn DBTrait> {
    if let Some(x) = x.downcast_ref::<Mysql>() {
        Some(x as &dyn DBTrait)
    } else {
        None
    }
}
