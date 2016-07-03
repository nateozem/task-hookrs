//! Tools for iterating over tasks

use std::cmp::Ordering;
use std::iter::Iterator;

use task::Task;

use itertools::Itertools;

/// A convenience iterator for tasks, providing some functionality for sorting and filtering for
/// the tasks it iterates over
pub struct TaskIterator<I: Iterator<Item = Task>> {
    i: I
}

impl<I: Iterator<Item = Task>> TaskIterator<I> {

    /// Create a new TaskIterator object from the passed iterator object
    pub fn new(i: I) -> TaskIterator<I> {
        TaskIterator {
            i: i
        }
    }

    /// Sort the tasks from the iterator with a function
    pub fn sort_by_fn<F>(self, f: F) -> Vec<Task>
        where F: FnMut(&Task, &Task) -> Ordering
    {
        self.sorted_by(f)
    }

    /// Sort the tasks by prio
    pub fn sort_by_prio(self) -> Vec<Task> {
        use self::sorting;
        self.sort_by_fn(sorting::sort_by_prio)
    }

    /// Sort the tasks by entry date
    pub fn sort_by_entry(self) -> Vec<Task> {
        use self::sorting;
        self.sort_by_fn(sorting::sort_by_entry)
    }

    /// Sort the tasks by due date
    pub fn sort_by_due(self) -> Vec<Task> {
        use self::sorting;
        self.sort_by_fn(sorting::sort_by_due)
    }

    /// Sort the tasks by scheduled date
    pub fn sort_by_scheduled(self) -> Vec<Task> {
        use self::sorting;
        self.sort_by_fn(sorting::sort_by_scheduled)
    }

    /// Sort the tasks by number of annotations
    pub fn sort_by_num_annotations(self) -> Vec<Task> {
        use self::sorting;
        self.sort_by_fn(sorting::sort_by_num_annotations)
    }

    /// Filter the tasks with a custom function
    pub fn filter_by_fn<F>(self, f: F) -> TaskIterator<I>
        where F: FnOnce(&Task) -> bool
    {
        unimplemented!()
    }

}

impl<I: Iterator<Item = Task>> Iterator for TaskIterator<I> {
    type Item = Task;

    fn next(&mut self) -> Option<Task> {
        self.i.next()
    }

}

/// A module which contains functions that can be used to sort tasks
/// Use these functions if you want to sort by two predicates, for example sorting by prio and by
/// due date.
pub mod sorting {
    use std::cmp::Ordering;
    use task::Task;

    /// sort the tasks by prio
    pub fn sort_by_prio(a: &Task, b: &Task) -> Ordering {
        if let Some(a) = a.priority() {
            if let Some(b) = b.priority() {
                a.clone().cmp(b)
            } else {
                Ordering::Less
            }
        } else {
            Ordering::Greater
        }
    }

    /// Sort the tasks by entry date
    pub fn sort_by_entry(a: &Task, b: &Task) -> Ordering {
        unimplemented!()
    }

    /// Sort the tasks by due date
    pub fn sort_by_due(a: &Task, b: &Task) -> Ordering {
        unimplemented!()
    }

    /// Sort the tasks by scheduled date
    pub fn sort_by_scheduled(a: &Task, b: &Task) -> Ordering {
        unimplemented!()
    }

    /// Sort the tasks by number of annotations
    pub fn sort_by_num_annotations(a: &Task, b: &Task) -> Ordering {
        unimplemented!()
    }

}
