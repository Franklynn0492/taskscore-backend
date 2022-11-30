pub trait Entity<I> {
    fn get_id(&self) -> &I;
}