use std::error::Error;

use async_trait::async_trait;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}

#[async_trait]
pub trait DoThing: Send + Sync + 'static {
    async fn do_thing(&self, what: String) -> Result<DoThingRes, DoThingError>;
}

pub struct DoThingRes {
    what: String,
}

pub struct DoThingError {
    pub kind: DoThingErrorKind,
    pub source: Option<Box<dyn Error + Send + Sync + 'static>>,
}

pub enum DoThingErrorKind {
    Unknwon,
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct DoThinkMock(fn() -> Result<DoThingRes, DoThingError>);

    #[async_trait]
    impl DoThing for DoThinkMock {
        async fn do_thing(&self, what: String) -> Result<DoThingRes, DoThingError> {
            (self.0)()
        }
    }

    #[tokio::test]
    async fn it_should_work() {
        let service = DoThinkMock(|| {
            Ok(DoThingRes {
                what: "PASS".to_owned(),
            })
        });

        let result = service.do_thing("PASS".to_owned()).await;

        match result {
            Ok(value) => assert_eq!(value.what, "PASS".to_owned()),
            Err(_) => unreachable!(),
        }
    }
}
