mod grid1;

pub trait Grid {
    type Item;

    fn with_size(width: usize, height: usize) -> Self;

    fn get(&self, x: usize, y: usize) -> &Self::Item;
    fn get_mut(&mut self, x: usize, y: usize) -> &mut Self::Item;

    fn set(&mut self, x: usize, y: usize, value: Self::Item);

    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item=(usize, usize, &'a Self::Item)> + 'a>;
    fn iter_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item=(usize, usize, &'a mut Self::Item)> + 'a>;
}
