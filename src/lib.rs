use std::ops::{ Index, IndexMut };

#[derive(Debug)]
pub struct Mdvec<T: Clone> {
    internal: Vec<T>,
    dim: Vec<usize>
}

impl<T: Clone> Mdvec<T> {
    pub fn new() -> Self {
        Self {
            internal: Vec::new(),
            dim: Vec::new()
        }
    }

    pub fn with_capacity(dim: &[usize]) -> Self {
        let internal_cap = dim.iter().product();
        Self {
            internal: Vec::with_capacity(internal_cap),
            dim: Vec::from(dim)
        }
    }

        /// Converts multidimensional indices to a single flat index.
        fn flat_index(&self, indices: &[usize]) -> usize {
            assert_eq!(indices.len(), self.dim.len(), "Invalid number of indices");
            indices.iter().zip(&self.dim).fold(0, |acc, (&idx, &dim)| {
                assert!(idx < dim, "Index out of bounds");
                acc * dim + idx
            })
        }
    
        /// Access an immutable reference to an element at the given indices.
        pub fn get(&self, indices: &[usize]) -> &T {
            let flat_index = self.flat_index(indices);
            &self.internal[flat_index]
        }
    
        /// Access a mutable reference to an element at the given indices.
        pub fn get_mut(&mut self, indices: &[usize]) -> &mut T {
            let flat_index = self.flat_index(indices);
            &mut self.internal[flat_index]
        }
    
        /// Sets the value at the given indices.
        pub fn set(&mut self, indices: &[usize], value: T) {
            let flat_index = self.flat_index(indices);
            self.internal[flat_index] = value;
        }
}

impl<T: Clone> Index<&[usize]> for Mdvec<T> {
    type Output = T;

    fn index(&self, indices: &[usize]) -> &Self::Output {
        self.get(indices)
    }
}

impl<T: Clone> IndexMut<&[usize]> for Mdvec<T> {
    fn index_mut(&mut self, indices: &[usize]) -> &mut Self::Output {
        self.get_mut(indices)
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
