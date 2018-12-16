use std::mem;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub(super) struct Index {
    index: usize,
    generation: u64,
}

#[derive(Debug, PartialEq)]
enum Slot<T> {
    Empty { next_free_slot: Option<usize> },
    Filled { item: T, generation: u64 },
}

#[derive(Debug, PartialEq)]
pub(super) struct Slab<T> {
    data: Vec<Slot<T>>,
    first_free_slot: Option<usize>,
    generation: u64,
}

impl<T> Slab<T> {
    pub(super) fn new(capacity: usize) -> Slab<T> {
        Slab {
            data: Vec::with_capacity(capacity),
            first_free_slot: None,
            generation: 0,
        }
    }

    pub(super) fn capacity(&self) -> usize {
        self.data.capacity()
    }

    pub(super) fn insert(&mut self, item: T) -> Index {
        let new_slot = Slot::Filled {
            item,
            generation: self.generation,
        };

        if let Some(index) = self.first_free_slot {
            match mem::replace(
                &mut self.data[index],
                new_slot
            ) {
                Slot::Empty { next_free_slot } => {
                    self.first_free_slot = next_free_slot;
                }
                _ => unreachable!(),
            };

            Index {
                index,
                generation: self.generation,
            }
        } else {
            self.data.push(new_slot);
            Index {
                index: self.data.len() - 1,
                generation: self.generation,
            }
        }
    }

    pub(super) fn remove(&mut self, index: Index) -> Option<T> {
        if index.index >= self.data.len() {
            return None;
        }

        let slot = mem::replace(
            &mut self.data[index.index],
            Slot::Empty {
                next_free_slot: self.first_free_slot
            },
        );

        match slot {
            Slot::Filled { item, generation } => {
                if index.generation == generation {
                    self.generation += 1;
                    self.first_free_slot = Some(index.index);
                    Some(item)
                } else {
                    self.data[index.index] = Slot::Filled { item, generation };
                    None
                }
            },
            s =>  {
                self.data[index.index] = s;
                None
            }
        }
    }

    pub(super) fn get(&self, index: Index) -> Option<&T> {
        self.data.get(index.index)
            .and_then(|slot| {
                match slot {
                    Slot::Filled { item, generation } => {
                        if index.generation == *generation {
                            return Some(item);
                        }
                        None
                    },
                    _ => None,
                }
            })
    }

    pub(super) fn get_mut(&mut self, index: Index) -> Option<&mut T> {
        self.data.get_mut(index.index)
            .and_then(|slot| {
                match slot {
                    Slot::Filled { item, generation } => {
                        if index.generation == *generation {
                            return Some(item);
                        }
                        None
                    },
                    _ => None,
                }
            })
    }
}

#[cfg_attr(tarpaulin, skip)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capacity() {
        let capacity = 5;
        let slab = Slab::<i32>::new(capacity);

        assert_eq!(slab.capacity(), capacity);
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
    fn remove_basic() {
        let mut slab = Slab::new(5);
        let _six = slab.insert(6);
        let seven = slab.insert(7);
        let _eight = slab.insert(8);
        // |6|7|8|

        let seven_rem = slab.remove(seven);
        // |6|.|8|
        assert!(seven_rem.is_some());
        assert_eq!(seven_rem.unwrap(), 7);

        assert_eq!(slab.first_free_slot.unwrap_or(10), 1);
        assert_eq!(slab.generation, 1);

        let six_slot = slab.data.get(0);
        assert!(six_slot.is_some());

        match six_slot.unwrap() {
            Slot::Empty { .. } => {
                panic!("Slot should be filled after call to insert.");
            }
            Slot::Filled { item, generation } => {
                assert_eq!(item, &6);
                assert_eq!(generation, &0);
            }
        }

        let seven_slot = slab.data.get(1);
        assert!(seven_slot.is_some());

        match seven_slot.unwrap() {
            Slot::Empty { next_free_slot } => {
                assert!(next_free_slot.is_none());
            }
            Slot::Filled { .. } => {
                panic!("Slot should be empty after call to remove.");
            }
        }

        let eight_slot = slab.data.get(2);
        assert!(eight_slot.is_some());

        match eight_slot.unwrap() {
            Slot::Empty { .. } => {
                panic!("Slot should be filled after call to insert.");
            }
            Slot::Filled { item, generation } => {
                assert_eq!(item, &8);
                assert_eq!(generation, &0);
            }
        }
    }

    #[test]
    fn double_remove() {
        let mut slab = Slab::new(5);
        let _six = slab.insert(6);
        let seven = slab.insert(7);
        let _eight = slab.insert(8);
        // |6|7|8|

        let seven_rem = slab.remove(seven);
        // |6|.|8|
        assert!(seven_rem.is_some());
        assert_eq!(seven_rem.unwrap(), 7);

        let seven_again = slab.remove(seven);
        assert!(seven_again.is_none());
    }

    #[test]
    fn remove_multiple() {
        let mut slab = Slab::new(5);
        let _six = slab.insert(6);
        let seven = slab.insert(7);
        let eight = slab.insert(8);
        // |6|7|8|

        let seven_rem = slab.remove(seven);
        // |6|.|8|
        assert!(seven_rem.is_some());
        assert_eq!(seven_rem.unwrap(), 7);

        assert_eq!(slab.first_free_slot.unwrap_or(10), 1);
        assert_eq!(slab.generation, 1);

        {
            let six_slot = slab.data.get(0);
            assert!(six_slot.is_some());

            match six_slot.unwrap() {
                Slot::Empty { .. } => {
                    panic!("Slot should be filled after call to insert.");
                }
                Slot::Filled { item, generation } => {
                    assert_eq!(item, &6);
                    assert_eq!(generation, &0);
                }
            }

            let seven_slot = slab.data.get(1);
            assert!(seven_slot.is_some());

            match seven_slot.unwrap() {
                Slot::Empty { next_free_slot } => {
                    assert!(next_free_slot.is_none());
                }
                Slot::Filled { .. } => {
                    panic!("Slot should be empty after call to remove.");
                }
            }

            let eight_slot = slab.data.get(2);
            assert!(eight_slot.is_some());

            match eight_slot.unwrap() {
                Slot::Empty { .. } => {
                    panic!("Slot should be filled after call to insert.");
                }
                Slot::Filled { item, generation } => {
                    assert_eq!(item, &8);
                    assert_eq!(generation, &0);
                }
            }
        }

        let eight_rem = slab.remove(eight);
        // |6|.|.|
        assert!(eight_rem.is_some());
        assert_eq!(eight_rem.unwrap(), 8);

        assert_eq!(slab.first_free_slot.unwrap_or(10), 2);
        assert_eq!(slab.generation, 2);

        {
            let six_slot = slab.data.get(0);
            assert!(six_slot.is_some());

            match six_slot.unwrap() {
                Slot::Empty { .. } => {
                    panic!("Slot should be filled after call to insert.");
                }
                Slot::Filled { item, generation } => {
                    assert_eq!(item, &6);
                    assert_eq!(generation, &0);
                }
            }

            let seven_slot = slab.data.get(1);
            assert!(seven_slot.is_some());

            match seven_slot.unwrap() {
                Slot::Empty { next_free_slot } => {
                    assert!(next_free_slot.is_none());
                }
                Slot::Filled { .. } => {
                    panic!("Slot should be empty after call to remove.");
                }
            }

            let eight_slot = slab.data.get(2);
            assert!(eight_slot.is_some());

            match eight_slot.unwrap() {
                Slot::Empty { next_free_slot } => {
                    assert!(next_free_slot.is_some());
                    assert_eq!(next_free_slot.unwrap(), 1);
                }
                Slot::Filled { .. } => {
                    panic!("Slot should be empty after call to remove.");
                }
            }
        }
    }

    #[test]
    fn remove_and_reinsert() {
        let mut slab = Slab::new(5);
        let _six = slab.insert(6);
        let seven = slab.insert(7);
        let eight = slab.insert(8);
        // |6|7|8|

        let seven_rem = slab.remove(seven);
        // |6|.|8|
        assert!(seven_rem.is_some());
        assert_eq!(seven_rem.unwrap(), 7);

        assert_eq!(slab.first_free_slot.unwrap_or(10), 1);
        assert_eq!(slab.generation, 1);

        let eight_rem = slab.remove(eight);
        // |6|.|.|
        assert!(eight_rem.is_some());
        assert_eq!(eight_rem.unwrap(), 8);

        assert_eq!(slab.first_free_slot.unwrap_or(10), 2);
        assert_eq!(slab.generation, 2);

        let nine = slab.insert(9);
        // |6|.|9|
        assert_eq!(nine.index, 2);
        assert_eq!(nine.generation, 2);

        let eight_again = slab.remove(eight);
        assert!(eight_again.is_none());

        {
            let six_slot = slab.data.get(0);
            assert!(six_slot.is_some());

            match six_slot.unwrap() {
                Slot::Empty { .. } => {
                    panic!("Slot should be filled after call to insert.");
                }
                Slot::Filled { item, generation } => {
                    assert_eq!(item, &6);
                    assert_eq!(generation, &0);
                }
            }

            let seven_slot = slab.data.get(1);
            assert!(seven_slot.is_some());

            match seven_slot.unwrap() {
                Slot::Empty { next_free_slot } => {
                    assert!(next_free_slot.is_none());
                }
                Slot::Filled { .. } => {
                    panic!("Slot should be empty after call to remove.");
                }
            }

            let nine_slot = slab.data.get(2);
            assert!(nine_slot.is_some());

            match nine_slot.unwrap() {
                Slot::Empty { .. } => {
                    panic!("Slot should be filled after call to insert.");
                }
                Slot::Filled { item, generation } => {
                    assert_eq!(item, &9);
                    assert_eq!(generation, &2);
                }
            }
        }
    }

    #[test]
    fn remove_with_bad_index() {
        let mut slab = Slab::new(5);
        let _six = slab.insert(6);
        let _seven = slab.insert(7);
        let mut eight = slab.insert(8);
        // |0|1|2| index
        // |6|7|8| value

        eight.index = 3; // oops, this should be 2

        let eight_rem = slab.remove(eight);
        assert!(eight_rem.is_none());
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
        {
            let eight_ref = slab.get(eight);
            assert!(eight_ref.is_some());
            assert_eq!(eight_ref.unwrap(), &8);
        }
        {
            let six_ref = slab.get(six);
            assert!(six_ref.is_none());
        }
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

        {
            let eight_ref = slab.get_mut(eight);
            assert!(eight_ref.is_some());
            assert_eq!(eight_ref.unwrap(), &mut 8);
        }
        {
            let six_ref = slab.get_mut(six);
            assert!(six_ref.is_none());
        }
    }
}
