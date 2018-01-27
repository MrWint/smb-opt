use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use state::{Dist, State, StateCompressor, StateDistCompressor};

pub trait StateStore {
  fn new(search_space_size_hint: usize) -> Self;
  fn len(&self) -> usize;
  fn check_and_update_dist(&mut self, &State, Dist) -> bool;
  fn increment_all_dists(&mut self) -> ();
}

impl<T: StateCompressor + Eq + Hash> StateStore for ::std::collections::HashMap<T, Dist> {
  fn new(search_space_size_hint: usize) -> Self {
    ::std::collections::HashMap::<T, Dist>::with_capacity(search_space_size_hint)
  }
  fn len(&self) -> usize {
    self.len()
  }
  fn check_and_update_dist(&mut self, s: &State, d: Dist) -> bool {
    match self.entry(T::from_state(s)) {
      ::std::collections::hash_map::Entry::Vacant(ve) => {ve.insert(d); true}
      ::std::collections::hash_map::Entry::Occupied(mut oe) => {
        let e = oe.get_mut();
        if *e > d {*e = d; true} else {false}
      }
    }
  }
  fn increment_all_dists(&mut self) -> () {
    for (_, d) in self.iter_mut() {
      *d += 1;
    }
  }
}

impl<T: StateCompressor + Ord> StateStore for ::std::collections::BTreeMap<T, Dist> {
  fn new(_search_space_size_hint: usize) -> Self {
    ::std::collections::BTreeMap::<T, Dist>::new()
  }
  fn len(&self) -> usize {
    self.len()
  }
  fn check_and_update_dist(&mut self, s: &State, d: Dist) -> bool {
    match self.entry(T::from_state(s)) {
      ::std::collections::btree_map::Entry::Vacant(ve) => {ve.insert(d); true}
      ::std::collections::btree_map::Entry::Occupied(mut oe) => {
        let e = oe.get_mut();
        if *e > d {*e = d; true} else {false}
      }
    }
  }
  fn increment_all_dists(&mut self) -> () {
    for (_, d) in self.iter_mut() {
      *d += 1;
    }
  }
}

impl<T: StateCompressor + VecHashKey> StateStore for VecHashMap<T, Dist> {
  fn new(search_space_size_hint: usize) -> Self {
    VecHashMap::<T, Dist>::with_capacity(search_space_size_hint)
  }
  fn len(&self) -> usize {
    self.len()
  }
  fn check_and_update_dist(&mut self, s: &State, d: Dist) -> bool {
    match self.entry(T::from_state(s)) {
      VecHashMapEntry::Vacant(v) => { *v = d; true }
      VecHashMapEntry::Occupied((key, k, v)) => {
        if *v > d {
          *v = d;
          ::std::mem::replace(k, key);
          true
        } else { false }
      }
    }
  }
  fn increment_all_dists(&mut self) -> () {
    for i in 0..self.keys.len() {
      if self.keys[i].is_valid() {
        self.values[i] += 1;
      }
    }
  }
}

impl<T: StateDistCompressor + VecHashKey> StateStore for VecHashMap<T, ()> {
  fn new(search_space_size_hint: usize) -> Self {
    VecHashMap::<T, ()>::with_capacity(search_space_size_hint)
  }
  fn len(&self) -> usize {
    self.len()
  }
  fn check_and_update_dist(&mut self, s: &State, d: Dist) -> bool {
    match self.entry(T::from_state_dist(s, d)) {
      VecHashMapEntry::Vacant(_) => { true }
      VecHashMapEntry::Occupied((key, k, _)) => {
        if k.dist() > d {
          ::std::mem::replace(k, key);
          true
        } else { false }
      }
    }
  }
  fn increment_all_dists(&mut self) -> () {
    for key in self.keys.iter_mut() {
      if key.is_valid() {
        key.increment_dist();
      }
    }
  }
}


pub trait VecHashKey: Eq + Hash + Clone {
  fn is_valid(&self) -> bool;
  fn invalid() -> Self;
}

pub struct VecHashMap<K: VecHashKey, V: Clone + Default> {
  keys: Vec<K>,
  values: Vec<V>,
  size: usize,
}

pub enum VecHashMapEntry<'a, K: 'a, V: 'a> {
  Vacant(&'a mut V),
  Occupied((K, &'a mut K, &'a mut V))
}

impl<K: VecHashKey, V: Clone + Default> VecHashMap<K, V> {
  pub fn with_capacity(capacity: usize) -> Self {
    let raw_capacity = capacity * 11 / 10 + 1;
    let mut keys = Vec::with_capacity(raw_capacity);
    let mut values = Vec::with_capacity(raw_capacity);
    keys.resize(raw_capacity, K::invalid());
    values.resize(raw_capacity, V::default());
    VecHashMap { keys, values, size: 0 }
  }
  fn len(&self) -> usize {
    self.size
  }
  #[allow(dead_code)]
  pub fn debug_info(&self) -> String {
    let mut non_empty_trailing = 0;
    while non_empty_trailing < self.keys.len() && self.keys[self.keys.len() - 1 - non_empty_trailing].is_valid() {
      non_empty_trailing += 1;
    }
    let mut num_nonempty: usize = 0;
    let mut sum_dist: usize = 0;
    let mut min_dist: usize = self.keys.len() + 1;
    let mut max_dist: usize = 0;
    let mut consecutive_nonempty: usize = 0;
    let mut invalid_fields: usize = 0;
    for i in 0..self.keys.len() {
      if self.keys[i].is_valid() {
        num_nonempty += 1;
        let dist = (self.keys.len() + i - self.hash_of(&self.keys[i])) % self.keys.len();
        min_dist = ::std::cmp::min(dist, min_dist);
        max_dist = ::std::cmp::max(dist, max_dist);
        sum_dist += dist;
        if (consecutive_nonempty < i && consecutive_nonempty < dist) || consecutive_nonempty + non_empty_trailing < dist {
          invalid_fields += 1;
        }
        consecutive_nonempty += 1;
      } else {
        consecutive_nonempty = 0;
      }
    }
    format!("size: {}, capacity: {}, filled fields: {}, invalid: {}, sum dist: {} (min: {}, max: {}, avg: {})", self.size, self.keys.len(), num_nonempty, invalid_fields, sum_dist, min_dist, max_dist, (sum_dist as f64) / (self.size as f64))
  }

  /// Adds the given element to this set. Returns the element it replaced (if any).
  pub fn entry(&mut self, mut key: K) -> VecHashMapEntry<K, V> {
    assert!(key.is_valid());
    // Check if set is at max capacity (~90%), resize if necessary.
    if self.size * 11 / 10 >= self.keys.len() {
      self.resize_and_rehash();
    }

    let mut value = V::default();
    let mut hash = self.hash_of(&key);
    let mut probe: usize = 0;
    let mut probe_pos = (hash + probe) % self.keys.len();
    let mut first_steal_probe_pos: usize = <usize>::max_value();
    while self.keys[probe_pos].is_valid() {
      if self.keys[probe_pos] == key {
        return VecHashMapEntry::Occupied((key, &mut self.keys[probe_pos], &mut self.values[probe_pos]));
      }
      let alt_hash = self.hash_of(&self.keys[probe_pos]);
      let alt_dist = (self.keys.len() + probe_pos - alt_hash) % self.keys.len();
      if alt_dist < probe {
        // Found a luckier element. Take its place and continue probing with the replaced
        // element. At this point the return value is bound to be None.
        key = ::std::mem::replace(&mut self.keys[probe_pos], key);
        value = ::std::mem::replace(&mut self.values[probe_pos], value);
        hash = alt_hash;
        probe = alt_dist;
        if first_steal_probe_pos == <usize>::max_value() {
          first_steal_probe_pos = probe_pos;
        }
      }
      probe += 1;
      probe_pos = (hash + probe) % self.keys.len();
    }
    ::std::mem::replace(&mut self.keys[probe_pos], key);
    ::std::mem::replace(&mut self.values[probe_pos], value);
    if first_steal_probe_pos == <usize>::max_value() {
      first_steal_probe_pos = probe_pos;
    }
    self.size += 1;
    VecHashMapEntry::Vacant(&mut self.values[first_steal_probe_pos])
  }

  /// Doubles the capacity of the set and repositions the elements to form a valid hash table
  /// in the new larger array.
  fn resize_and_rehash(&mut self) {
    // Find first position with an element at its ideal location. This element is guaranteed
    // to exist, in fact the first element in each block of consecutively filled spots is
    // necessarily in its ideal position.
    let mut first_ideal_position = 0;
    while !self.keys[first_ideal_position].is_valid() || self.hash_of(&self.keys[first_ideal_position]) != first_ideal_position {
      first_ideal_position += 1;
    }

    let old_len = self.keys.len();
    let new_len = old_len * 2;

    println!("resizing to hold {} states", new_len);

    // Double the size of the array (n -> 2n). Due to how the hash is calculated, an element
    // with hash h will now have a hash of either h or (h + n).
    self.keys.reserve_exact(new_len);
    self.keys.resize(new_len, K::invalid());
    self.values.reserve_exact(new_len);
    self.values.resize(new_len, V::default());

    let mut write_position_low = first_ideal_position;
    let mut write_position_high = old_len + first_ideal_position;
    for i in first_ideal_position..(first_ideal_position + old_len) {
      let index = i % old_len;
      if self.keys[index].is_valid() {
        let hash = self.hash_of(&self.keys[index]);
        if hash >= first_ideal_position && hash < old_len + first_ideal_position {
          if write_position_low < hash {
            write_position_low = hash;
          }
          if write_position_low != index {
            self.keys.swap(index, write_position_low);
            self.values.swap(index, write_position_low);
          }
          write_position_low += 1; // low index never wraps around
        } else {
          if (write_position_high >= first_ideal_position && hash < first_ideal_position)
              || ((write_position_high >= first_ideal_position || hash < first_ideal_position) && write_position_high < hash) {
            write_position_high = hash;
          }
          if write_position_high != index {
            self.keys.swap(index, write_position_high);
            self.values.swap(index, write_position_high);
          }
          write_position_high = (write_position_high + 1) % new_len;
        }
      }
    }
  }

  /// Computes the hash position of an element.
  fn hash_of(&self, key: &K) -> usize {
    let mut s = DefaultHasher::new();
    key.hash(&mut s);
    (s.finish() as usize) % self.keys.len()
  }
}
