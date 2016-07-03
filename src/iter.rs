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
        self.sort_by_fn(|a, b| {
            if let Some(a) = a.priority() {
                if let Some(b) = b.priority() {
                    a.clone().cmp(b)
                } else {
                    Ordering::Less
                }
            } else {
                Ordering::Greater
            }
        })
    }

    /// Sort the tasks by entry date
    pub fn sort_by_entry(self) -> Vec<Task> {
        unimplemented!()
    }

    /// Sort the tasks by due date
    pub fn sort_by_due(self) -> Vec<Task> {
        unimplemented!()
    }

    /// Sort the tasks by scheduled date
    pub fn sort_by_scheduled(self) -> Vec<Task> {
        unimplemented!()
    }

    /// Sort the tasks by number of annotations
    pub fn sort_by_num_annotations(self) -> Vec<Task> {
        unimplemented!()
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

