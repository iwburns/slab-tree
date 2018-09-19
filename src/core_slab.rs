
#[derive(Copy, Clone)]
pub struct Index {
    index: usize,
    generation: u64,
}

enum Slot<T> {
    Empty { next_free_slot: Option<usize> },
    Filled { data: T, generation: u64 },
}

pub struct Slab<T> {
    data: Vec<Slot<T>>,
    first_free_slot: Option<usize>,
    generation: u64,
}

impl<T> Slab<T> {
    pub fn new(capacity: usize) -> Slab<T> {
        unimplemented!();
    }

    pub fn insert(&mut self, item: T) -> Index {
        unimplemented!()
    }

    pub fn remove(&mut self, index: Index) -> Option<T> {
        unimplemented!()
    }

    pub fn get(&self, index: Index) -> Option<&T> {
        unimplemented!()
    }

    pub fn get_mut(&mut self, index: Index) -> Option<&mut T> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capacity() {
        let capacity = 5;
        let slab = Slab::<i32>::new(capacity);

        assert_eq!(slab.data.capacity(), capacity);
        assert!(slab.first_free_slot.is_none());
        assert_eq!(slab.generation, 0);
    }

    #[test]
    fn insert() {
        let capacity = 2;
        let mut slab = Slab::new(capacity);

        let six = slab.insert(6);

        assert!(slab.first_free_slot.is_none());
        assert_eq!(slab.generation, 0);
        assert_eq!(slab.data.len(), 1);
        assert_eq!(slab.data.capacity(), capacity);

        assert_eq!(six.generation, 0);
        assert_eq!(six.index, 0);

        let seven = slab.insert(7);

        assert!(slab.first_free_slot.is_none());
        assert_eq!(slab.generation, 0);
        assert_eq!(slab.data.len(), 2);
        assert_eq!(slab.data.capacity(), capacity);

        assert_eq!(seven.generation, 0);
        assert_eq!(seven.index, 1);

        let eight = slab.insert(8);

        assert!(slab.first_free_slot.is_none());
        assert_eq!(slab.generation, 0);
        assert_eq!(slab.data.len(), 3);
        assert!(slab.data.capacity() >= capacity);

        assert_eq!(eight.generation, 0);
        assert_eq!(eight.index, 2);
    }

    #[test]
    fn remove() {
        let mut slab = Slab::new(5);
        let six = slab.insert(6);
        // |6|
        let seven = slab.insert(7);
        // |6|7|
        let eight = slab.insert(8);
        // |6|7|8|

        let seven_rem = slab.remove(seven);
        // |6|.|8|
        assert!(seven_rem.is_some());
        assert_eq!(seven_rem.unwrap(), 7);

        assert_eq!(slab.first_free_slot.unwrap_or(10), 1);
        assert_eq!(slab.generation, 1);

        {
            let six_slot = slab.data.get(1);
            assert!(six_slot.is_some());

            let six_slot = six_slot.unwrap();
            match six_slot {
                Slot::Empty { next_free_slot } => {
                    panic!("Slot should be filled after call to insert.");
                },
                Slot::Filled { data, generation } => {
                    assert_eq!(data, &6);
                    assert_eq!(generation, &0);
                },
            }

            let seven_slot = slab.data.get(1);
            assert!(seven_slot.is_some());

            let seven_slot = seven_slot.unwrap();
            match seven_slot {
                Slot::Empty { next_free_slot } => {
                    assert!(next_free_slot.is_none());
                },
                Slot::Filled { .. } => {
                    panic!("Slot should be empty after call to remove.");
                },
            }

            let eight_slot = slab.data.get(1);
            assert!(eight_slot.is_some());

            let eight_slot = eight_slot.unwrap();
            match eight_slot {
                Slot::Empty { next_free_slot } => {
                    panic!("Slot should be filled after call to insert.");
                },
                Slot::Filled { data, generation } => {
                    assert_eq!(data, &8);
                    assert_eq!(generation, &0);
                },
            }
        }

        let eight_rem = slab.remove(eight);
        // |6|.|.|
        assert!(eight_rem.is_some());
        assert_eq!(eight_rem.unwrap(), 8);

        assert_eq!(slab.first_free_slot.unwrap_or(10), 2);
        assert_eq!(slab.generation, 2);

        {
            let six_slot = slab.data.get(1);
            assert!(six_slot.is_some());

            let six_slot = six_slot.unwrap();
            match six_slot {
                Slot::Empty { next_free_slot } => {
                    panic!("Slot should be filled after call to insert.");
                },
                Slot::Filled { data, generation } => {
                    assert_eq!(data, &6);
                    assert_eq!(generation, &0);
                },
            }

            let seven_slot = slab.data.get(1);
            assert!(seven_slot.is_some());

            let seven_slot = seven_slot.unwrap();
            match seven_slot {
                Slot::Empty { next_free_slot } => {
                    assert!(next_free_slot.is_none());
                },
                Slot::Filled { .. } => {
                    panic!("Slot should be empty after call to remove.");
                },
            }

            let eight_slot = slab.data.get(2);
            assert!(eight_slot.is_some());

            let eight_slot = eight_slot.unwrap();
            match eight_slot {
                Slot::Empty { next_free_slot } => {
                    assert!(next_free_slot.is_some());
                    assert_eq!(next_free_slot.unwrap(), 1);
                },
                Slot::Filled { .. } => {
                    panic!("Slot should be empty after call to remove.");
                },
            }
        }

        let nine = slab.insert(9);
        // |6|.|9|
        assert_eq!(nine.index, 2);
        assert_eq!(nine.generation, 2);

        {
            let six_slot = slab.data.get(1);
            assert!(six_slot.is_some());

            let six_slot = six_slot.unwrap();
            match six_slot {
                Slot::Empty { next_free_slot } => {
                    panic!("Slot should be filled after call to insert.");
                },
                Slot::Filled { data, generation } => {
                    assert_eq!(data, &6);
                    assert_eq!(generation, &0);
                },
            }

            let seven_slot = slab.data.get(1);
            assert!(seven_slot.is_some());

            let seven_slot = seven_slot.unwrap();
            match seven_slot {
                Slot::Empty { next_free_slot } => {
                    assert!(next_free_slot.is_none());
                },
                Slot::Filled { .. } => {
                    panic!("Slot should be empty after call to remove.");
                },
            }

            let nine_slot = slab.data.get(2);
            assert!(nine_slot.is_some());

            let nine_slot = nine_slot.unwrap();
            match nine_slot {
                Slot::Empty { next_free_slot } => {
                    panic!("Slot should be filled after call to insert.");
                },
                Slot::Filled { data, generation } => {
                    assert_eq!(data, &9);
                    assert_eq!(generation, &2);
                },
            }
        }

        let six_rem = slab.remove(six);
        // |.|.|9|
        assert!(six_rem.is_some());
        assert_eq!(six_rem.unwrap(), 6);

        assert_eq!(slab.first_free_slot.unwrap_or(10), 0);
        assert_eq!(slab.generation, 3);

        {
            let six_slot = slab.data.get(1);
            assert!(six_slot.is_some());

            let six_slot = six_slot.unwrap();
            match six_slot {
                Slot::Empty { next_free_slot } => {
                    assert!(next_free_slot.is_some());
                    assert_eq!(next_free_slot.unwrap(), 1);
                },
                Slot::Filled { .. } => {
                    panic!("Slot should be empty after call to remove.");
                },
            }

            let seven_slot = slab.data.get(1);
            assert!(seven_slot.is_some());

            let seven_slot = seven_slot.unwrap();
            match seven_slot {
                Slot::Empty { next_free_slot } => {
                    assert!(next_free_slot.is_none());
                },
                Slot::Filled { .. } => {
                    panic!("Slot should be empty after call to remove.");
                },
            }

            let nine_slot = slab.data.get(2);
            assert!(nine_slot.is_some());

            let nine_slot = nine_slot.unwrap();
            match nine_slot {
                Slot::Empty { next_free_slot } => {
                    panic!("Slot should be filled after call to insert.");
                },
                Slot::Filled { data, generation } => {
                    assert_eq!(data, &9);
                    assert_eq!(generation, &2);
                },
            }
        }
    }

    #[test]
    fn get() {
        let mut slab = Slab::new(5);

        let six = slab.insert(6);
        assert_eq!(six.index, 0);
        assert_eq!(six.generation, 0);

        let seven = slab.insert(7);
        assert_eq!(seven.index, 1);
        assert_eq!(seven.generation, 0);

        {
            let six_ref = slab.get(six);
            assert!(six_ref.is_some());
            assert_eq!(six_ref.unwrap(), &6);
        }

        slab.remove(six);

        {
            let six_ref = slab.get(six);
            assert!(six_ref.is_none());
        }

        let eight = slab.insert(8);
        assert_eq!(eight.index, 0);
        assert_eq!(eight.generation, 1);

        let eight_ref = slab.get(eight);
        assert!(eight_ref.is_some());
        assert_eq!(eight_ref.unwrap(), &8);
    }

    #[test]
    fn get_mut() {
        let mut slab = Slab::new(5);

        let six = slab.insert(6);
        assert_eq!(six.index, 0);
        assert_eq!(six.generation, 0);

        let seven = slab.insert(7);
        assert_eq!(seven.index, 1);
        assert_eq!(seven.generation, 0);

        {
            let six_mut = slab.get_mut(six);
            assert!(six_mut.is_some());

            let six_mut = six_mut.unwrap();
            assert_eq!(six_mut, &mut 6);

            *six_mut = 60;
            assert_eq!(six_mut, &mut 60);
        }

        slab.remove(six);

        {
            let six_ref = slab.get_mut(six);
            assert!(six_ref.is_none());
        }

        let eight = slab.insert(8);
        assert_eq!(eight.index, 0);
        assert_eq!(eight.generation, 1);

        let eight_ref = slab.get_mut(eight);
        assert!(eight_ref.is_some());
        assert_eq!(eight_ref.unwrap(), &mut 8);
    }
}
