use anyhow::Result;

pub trait InputTrait {}
pub trait OutputTrait {}

#[trait_variant::make(Send)]
pub trait UsecaseTrait<I, O>
where
    I: InputTrait,
    O: OutputTrait,
{
    fn new(input: I) -> Self;
    async fn handle(&self) -> Result<O>;
}

#[derive(Debug)]