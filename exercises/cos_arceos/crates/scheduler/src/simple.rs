use crate::BaseScheduler;
use alloc::{collections::VecDeque, sync::Arc};
use core::{ops::Deref, sync::atomic::AtomicU8};

/// A task wrapper for the [`SimpleScheduler`].
pub struct SimpleTask<T> {
    inner: T,
    remain_time_slice: AtomicU8,
}

impl<T> SimpleTask<T> {
    /// Creates a new [`SimpleTask`] from the inner task struct.
    pub const fn new(inner: T) -> Self {
        Self {
            inner,
            remain_time_slice: AtomicU8::new(5),
        }
    }

    /// Returns a reference to the inner task struct.
    pub const fn inner(&self) -> &T {
        &self.inner
    }
}

impl<T> Deref for SimpleTask<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// A simple scheduler.
///
/// When a task is added to the scheduler, it's placed at the end of the ready
/// queue. When picking the next task to run, the head of the ready queue is
/// taken.
///
/// As it's a cooperative scheduler, it does nothing when the timer tick occurs.
///
pub struct SimpleScheduler<T> {
    ready_queue: VecDeque<Arc<SimpleTask<T>>>,
}

impl<T> SimpleScheduler<T> {
    /// Creates a new empty [`SimpleScheduler`].
    pub const fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
        }
    }
    /// get the name of scheduler
    pub fn scheduler_name() -> &'static str {
        "Simple"
    }
}

impl<T> BaseScheduler for SimpleScheduler<T> {
    type SchedItem = Arc<SimpleTask<T>>;

    fn init(&mut self) {}

    fn add_task(&mut self, task: Self::SchedItem) {
        trace!("######### add_task");
        self.ready_queue.push_back(task);
    }

    fn remove_task(&mut self, task: &Self::SchedItem) -> Option<Self::SchedItem> {
        trace!("######### remove_task");
        self.ready_queue
            .iter()
            .position(|t| Arc::ptr_eq(t, task))
            .and_then(|idx| self.ready_queue.remove(idx))
    }

    fn pick_next_task(&mut self) -> Option<Self::SchedItem> {
        self.ready_queue.pop_front()
    }

    fn put_prev_task(&mut self, prev: Self::SchedItem, _preempt: bool) {
        self.ready_queue.push_back(prev);
    }

    fn task_tick(&mut self, current: &Self::SchedItem) -> bool {
        // warn!("{}", current.remain_time_slice);

        // these not work
        // let remain = &mut unsafe { *(&current.remain_time_slice as *const u8 as *mut u8)};
        // warn!("remain1 = {}", remain);
        // *remain -= 1;
        // warn!("remain2 = {}", remain);

        let old_remain = current
            .remain_time_slice
            .fetch_sub(1, core::sync::atomic::Ordering::Relaxed);
        old_remain == 1
    }

    fn set_priority(&mut self, _task: &Self::SchedItem, _prio: isize) -> bool {
        false
    }
}
