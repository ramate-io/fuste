pub trait All: Sized {
	fn all() -> impl Iterator<Item = Self>;

	fn all_from(&self) -> impl Iterator<Item = Self> {
		Self::all()
	}
}
