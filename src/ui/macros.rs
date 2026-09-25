#[macro_export]
macro_rules! boxed_vec {
    ($($x:expr),* $(,)?) => {
        vec![
            $(Box::new($x) as Box<dyn $crate::ui::traits::UiElement>),*
        ]
    };
}
