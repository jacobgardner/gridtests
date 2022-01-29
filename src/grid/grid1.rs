use crate::grid::Grid;

pub struct Grid1<T> {
    spaces: Vec<T>,
    width: usize,
    height: usize,
}

struct GridIter<'a, T> {
    iter: core::slice::Iter<'a, T>,
    width: usize,
    x: usize,
    y: usize,
}

impl<'a, T> GridIter<'a, T> {
    fn new(grid: &'a Grid1<T>) -> Self {
        Self {
            iter: grid.spaces.iter(),
            width: grid.width,
            x: 0,
            y: 0,
        }
    }
}

struct GridIterMut<'a, T> {
    iter: core::slice::IterMut<'a, T>,
    width: usize,
    x: usize,
    y: usize,
}

impl<'a, T> GridIterMut<'a, T> {
    fn new(grid: &'a mut Grid1<T>) -> Self {
        Self {
            iter: grid.spaces.iter_mut(),
            width: grid.width,
            x: 0,
            y: 0,
        }
    }
}

impl<'a, T> Iterator for GridIter<'a, T> {
    type Item = (usize, usize, &'a T);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let r_value = self.iter.next().map(|i| (self.x, self.y, i));

        self.x += 1;
        if self.x >= self.width {
            self.y += 1;
            self.x = 0;
        }

        r_value
    }
}

impl<'a, T> Iterator for GridIterMut<'a, T> {
    type Item = (usize, usize, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        let r_value = self.iter.next().map(|i| (self.x, self.y, i));

        self.x += 1;
        if self.x >= self.width {
            self.y += 1;
            self.x = 0;
        }

        r_value
    }
}

impl<T> Grid1<T> {
    #[inline]
    fn get_offset(&self, x: usize, y: usize) -> usize {
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);
        x + y * self.width
    }
}

impl<T: Default + Clone> Grid for Grid1<T> {
    type Item = T;

    // Apparently Clone is needed here because Default is only called for the first
    //  element and then cloned for all proceeding
    fn with_size(width: usize, height: usize) -> Self {
        Self {
            spaces: vec![T::default(); width * height],
            width,
            height,
        }
    }

    #[inline]
    fn get(&self, x: usize, y: usize) -> &T {
        &self.spaces[self.get_offset(x, y)]
    }

    #[inline]
    fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        let offset = self.get_offset(x, y);
        &mut self.spaces[offset]
    }

    #[inline]
    fn set(&mut self, x: usize, y: usize, value: T) {
        let offset = self.get_offset(x, y);
        self.spaces[offset] = value;
    }

    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = (usize, usize, &'a Self::Item)> + 'a> {
        Box::new(GridIter::new(self))
    }

    fn iter_mut<'a>(
        &'a mut self,
    ) -> Box<dyn Iterator<Item = (usize, usize, &'a mut Self::Item)> + 'a> {
        Box::new(GridIterMut::new(self))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[should_panic]
    fn cannot_exceed_width() {
        let g: Grid1<u32> = Grid1::with_size(100, 100);
        g.get(100, 0);
    }

    #[test]
    #[should_panic]
    fn cannot_exceed_height() {
        let g: Grid1<u32> = Grid1::with_size(100, 100);
        g.get(0, 100);
    }

    #[test]
    fn test_iter_mut() {
        let mut grid: Grid1<u32> = Grid1::with_size(100, 100);

        for (idx, (x, y, v)) in grid.iter_mut().enumerate() {
            assert_eq!(y * 100 + x, idx);
            *v = idx as u32;
        }

        assert_eq!(*grid.get(0, 0), 0);
        assert_eq!(*grid.get(99, 0), 99);
        assert_eq!(*grid.get(0, 2), 200);
        assert_eq!(*grid.get(99, 99), 9999);
    }

    #[test]
    fn test_iter() {
        let mut grid: Grid1<u32> = Grid1::with_size(100, 100);

        let mut idx = 0;
        for y in 0..100 {
            for x in 0..100 {
                grid.set(x, y, idx);
                idx += 1;
            }
        }

        let mut sum = 0;
        for (x, y, i) in grid.iter() {
            assert_eq!(y * 100 + x, *i as usize);
            sum += i;
        }

        assert_eq!(sum, (100 * 100 / 2) * ((100 * 100) - 1));
    }
}
