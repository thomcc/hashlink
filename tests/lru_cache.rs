use hashlink::LruCache;

#[test]
fn test_put_and_get() {
    let mut cache = LruCache::new(2);
    cache.insert(1, 10);
    cache.insert(2, 20);
    assert_eq!(cache.get_mut(&1), Some(&mut 10));
    assert_eq!(cache.get_mut(&2), Some(&mut 20));
    assert_eq!(cache.len(), 2);
}

#[test]
fn test_put_update() {
    let mut cache = LruCache::new(1);
    cache.insert("1", 10);
    cache.insert("1", 19);
    assert_eq!(cache.get_mut("1"), Some(&mut 19));
    assert_eq!(cache.len(), 1);
}

#[test]
fn test_contains_key() {
    let mut cache = LruCache::new(1);
    cache.insert("1", 10);
    assert_eq!(cache.contains_key("1"), true);
}

#[test]
fn test_expire_lru() {
    let mut cache = LruCache::new(2);
    cache.insert("foo1", "bar1");
    cache.insert("foo2", "bar2");
    cache.insert("foo3", "bar3");
    assert!(cache.get_mut("foo1").is_none());
    cache.insert("foo2", "bar2update");
    cache.insert("foo4", "bar4");
    assert!(cache.get_mut("foo3").is_none());
}

#[test]
fn test_pop() {
    let mut cache = LruCache::new(2);
    cache.insert(1, 10);
    cache.insert(2, 20);
    assert_eq!(cache.len(), 2);
    let opt1 = cache.remove(&1);
    assert!(opt1.is_some());
    assert_eq!(opt1.unwrap(), 10);
    assert!(cache.get_mut(&1).is_none());
    assert_eq!(cache.len(), 1);
}

#[test]
fn test_change_capacity() {
    let mut cache = LruCache::new(2);
    assert_eq!(cache.capacity(), 2);
    cache.insert(1, 10);
    cache.insert(2, 20);
    cache.set_capacity(1);
    assert!(cache.get_mut(&1).is_none());
    assert_eq!(cache.capacity(), 1);
}

#[test]
fn test_remove() {
    let mut cache = LruCache::new(3);
    cache.insert(1, 10);
    cache.insert(2, 20);
    cache.insert(3, 30);
    cache.insert(4, 40);
    cache.insert(5, 50);
    cache.remove(&3);
    cache.remove(&4);
    assert!(cache.get_mut(&3).is_none());
    assert!(cache.get_mut(&4).is_none());
    cache.insert(6, 60);
    cache.insert(7, 70);
    cache.insert(8, 80);
    assert!(cache.get_mut(&5).is_none());
    assert_eq!(cache.get_mut(&6), Some(&mut 60));
    assert_eq!(cache.get_mut(&7), Some(&mut 70));
    assert_eq!(cache.get_mut(&8), Some(&mut 80));
}

#[test]
fn test_clear() {
    let mut cache = LruCache::new(2);
    cache.insert(1, 10);
    cache.insert(2, 20);
    cache.clear();
    assert!(cache.get_mut(&1).is_none());
    assert!(cache.get_mut(&2).is_none());
    assert!(cache.is_empty())
}

#[test]
fn test_iter() {
    let mut cache = LruCache::new(3);
    cache.insert(1, 10);
    cache.insert(2, 20);
    cache.insert(3, 30);
    cache.insert(4, 40);
    cache.insert(5, 50);
    assert_eq!(
        cache.iter().collect::<Vec<_>>(),
        [(&3, &30), (&4, &40), (&5, &50)]
    );
    assert_eq!(
        cache.iter_mut().collect::<Vec<_>>(),
        [(&3, &mut 30), (&4, &mut 40), (&5, &mut 50)]
    );
    assert_eq!(
        cache.iter().rev().collect::<Vec<_>>(),
        [(&5, &50), (&4, &40), (&3, &30)]
    );
    assert_eq!(
        cache.iter_mut().rev().collect::<Vec<_>>(),
        [(&5, &mut 50), (&4, &mut 40), (&3, &mut 30)]
    );
}
