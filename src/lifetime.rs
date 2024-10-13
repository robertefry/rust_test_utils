
use std::sync::{Mutex, MutexGuard};

struct TrackLists
{
    tracked: Vec<u32>,
    dropped: Vec<u32>,
}

pub struct Tracker
{
    track_lists: Mutex<TrackLists>,
}

impl Tracker
{
    pub fn new() -> Self
    {
        let track_lists = TrackLists
        {
            tracked: Vec::new(),
            dropped: Vec::new(),
        };
        Tracker
        {
            track_lists: Mutex::new(track_lists),
        }
    }

    fn lock_lists<'a>(&'a self) -> MutexGuard<'a, TrackLists>
    {
        self.track_lists.lock().expect("Invalid test: Unable to lock the track lists.")
    }

    pub fn tracked_order(&self) -> Vec<u32>
    {
        self.lock_lists().tracked.clone()
    }

    pub fn dropped_order(&self) -> Vec<u32>
    {
        self.lock_lists().dropped.clone()
    }
}

pub struct Tracked<'a>
{
    tracker: &'a Tracker,
    id: u32,
}

impl Tracker
{
    pub fn track(&self, id: u32) -> Tracked
    {
        let mut track_lists = self.lock_lists();

        if track_lists.tracked.contains(&id)
        {
            panic!("Invalid test: Cannot track the same id {} more than once.", id);
        }

        track_lists.tracked.push(id);
        Tracked{ tracker: self, id }
    }
}

impl<'a> Drop for Tracked<'a>
{
    fn drop(&mut self)
    {
        let mut track_lists = self.tracker.lock_lists();
        track_lists.dropped.push(self.id);
    }
}
