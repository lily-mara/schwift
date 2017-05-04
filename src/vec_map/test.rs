use ::vec_map::VecMap;

#[test]
fn should_store_1_and_4() {
    let mut map: VecMap<i32, i32> = VecMap::new();
    map.insert(1, 4);

    assert_eq!(map.get(&1), Some(&4));
}

#[test]
fn should_remove_1_and_4() {
    let mut map: VecMap<i32, i32> = VecMap::new();
    map.insert(1, 4);
    map.remove(&1);

    assert_eq!(map.get(&1), None);
}

#[test]
fn should_mut_borrow_1_and_4() {
    let mut map: VecMap<i32, i32> = VecMap::new();
    map.insert(1, 4);

    match map.get_mut(&1) {
        Some(mut value) => *value = 10,
        None => panic!(),
    }

    assert_eq!(map.get(&1), Some(&10));
}
