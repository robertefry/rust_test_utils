
use test_utils::lifetime;

#[test]
fn test_lifetime_tracker()
{
    let tracker = lifetime::Tracker::new();
    {
        let _a = tracker.track(1);
        let _b = tracker.track(2);
        let _c = tracker.track(3);
    }
    assert_eq!(tracker.tracked_order(), vec![1,2,3]);
    assert_eq!(tracker.dropped_order(), vec![3,2,1]);
}
