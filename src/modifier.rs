use serde::Deserialize;

pub struct Modifier<T> {
    pub field: String,
    runner: Box<dyn Fn(&mut T) -> () + 'static>,
}

impl<T> Modifier<T>
where
    T: Clone + for<'de> Deserialize<'de>,
{
    /// Construct the new custom rule
    pub fn new<F>(field_name: &str, runner: F) -> Self
    where
        F: Fn(&mut T) -> () + 'static,
    {
        Modifier::<T> {
            field: field_name.to_string(),
            runner: Box::new(runner),
        }
    }

    /// Handle the rule validation once its generated
    pub fn handle(&self, item: &mut T) {
        (self.runner)(item);
    }
}
