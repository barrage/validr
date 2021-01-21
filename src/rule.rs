use super::error::ValidationError;
use serde::Deserialize;

pub struct Rule<T> {
    pub field: String,
    runner: Box<dyn Fn(&T, &mut ValidationError) -> () + 'static>,
}

impl<T> Rule<T>
where
    T: Clone + for<'de> Deserialize<'de>,
{
    /// Construct the new custom rule
    pub fn new<F>(field_name: &str, runner: F) -> Self
    where
        F: Fn(&T, &mut ValidationError) -> () + 'static,
    {
        Rule::<T> {
            field: field_name.to_string(),
            runner: Box::new(runner),
        }
    }

    /// Handle the rule validation once its generated
    pub fn handle(&self, item: &T, error: &mut ValidationError) {
        error.set_field_name(&self.field);

        (self.runner)(item, error);
    }
}
