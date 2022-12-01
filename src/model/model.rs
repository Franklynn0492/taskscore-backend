pub trait Entity<I: 'static> where I: Send + Sync {
    fn get_id(&self) -> &I;
}