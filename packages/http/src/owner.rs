#[derive(Debug, Default)]
pub enum Owner<T> {
    Control(T),
    #[default]
    Lose
}

impl<T> Owner<T> {
    pub fn take(&mut self) -> Self {
        core::mem::take(self)
    }
}
