#[derive(Debug)]
pub struct Bin <T> {
    pub items: Vec<T>,
    pub remaining_space: u64,
}

impl<T: Pack> Bin<T> {
    pub fn new(capacity: u64) -> Bin<T> {
        Bin {
            items: Vec::new(),
            remaining_space: capacity as u64
        }
    }

    pub fn add_item(&mut self, new_item: T) {
        self.remaining_space -= new_item.get_size();
        self.items.push(new_item);
    }

    pub fn does_fit(&self, new_item: &T) -> bool {
        return new_item.get_size() <= self.remaining_space;
    }

    pub fn new_from(new_item: T, capacity: u64) -> Bin<T> {
        return Bin {
            remaining_space: capacity-new_item.get_size(),
            items: vec!(new_item),
        };
    }

    pub fn get_weights_pretty(&self) -> String {
        return self.items.iter().map(|x| format!("{}", x.get_size())).collect::<Vec<String>>().join(", ");
    }

    pub fn get_count(&self) -> usize {
        return self.items.len()
    }
}

pub trait Pack {
    fn get_size(&self) -> u64;
}

#[derive(Debug)]
pub struct ObjectTooBigError(u64, u64);

impl std::fmt::Display for ObjectTooBigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Object too big! {} can't fit in {}", self.0, self.1)
    }
}

// Use this one for fast runtime at the cost of potentially many bins...
pub fn online_nf<T: Pack>(mut items: Vec<T>, capacity: u64) -> Result<Vec<Bin<T>>, ObjectTooBigError> {
    // No sorting because that would raise our runtime from O(n) to at least O(n log(n)).
    // TODO - Replace the Vec with a stream or some similar type

    let mut current_bin: Bin<T> = Bin::new(capacity);
    let mut closed_bins: Vec<Bin<T>> = Vec::new();

    for item in items.drain(..) {
        let item_size = item.get_size();

        if item_size > current_bin.remaining_space   {
            if item_size > capacity {
                return Err(ObjectTooBigError(item_size, capacity));
            }

            closed_bins.push(current_bin);
            current_bin = Bin::new(capacity);
        }

        current_bin.add_item(item);
    }
    closed_bins.push(current_bin);

    return Ok(closed_bins);
}

// https://en.wikipedia.org/wiki/Bin_packing_problem#Modified_First_Fit_Decreasing_(MFFD)
// Wikipedia says it's O(n*log(n)) but I don't see how this is not O(n^2)
pub fn modified_ffd<T: Pack>(mut items: Vec<T>, capacity: u64) -> Result<Vec<Bin<T>>, ObjectTooBigError> {
    items.sort_unstable_by_key(|i| -(i.get_size() as i64)); // Sort largest first...

    let mut bins: Vec<Bin<T>> = Vec::new();
    let mut medium: Vec<T> = Vec::new();
    let mut small: Vec<T> = Vec::new();
    let mut tiny: Vec<T> = Vec::new();

    // Step 1: Make bins from 'large' items
    for item in items.drain(..) {
        match item {
            _ if item.get_size() > capacity/2 => bins.push(Bin::new_from(item, capacity)), // large
            _ if item.get_size() > capacity/3 => medium.push(item), // medium
            _ if item.get_size() > capacity/6 => small.push(item), //small
            _ => tiny.push(item) //tiny
        }
    }

    // Step 2 - Add medium items where possible and split the bins
    for bin in bins.iter_mut() {
        if let Some(med_index) = largest_that_fits(&medium, &bin) {
            bin.add_item(medium.remove(med_index));
        }
    }

    // Step 3 - Add smalls
    for bin in bins.iter_mut().rev() {
        // Test if there is no small here, also test if there are even two items to place
        if bin.items.len() == 1 && small.len() > 2 {
            // All unwraps are safe since we know that small has at least 2 items in it
            let smallest_smalls = small.last().unwrap().get_size() + small.get(small.len()-2).unwrap().get_size();

            if bin.remaining_space >= smallest_smalls {
                bin.add_item(small.pop().unwrap()); // First item
                if let Some(i) = largest_that_fits(&small, &bin) {
                    bin.add_item(small.remove(i));
                }
            }
        }
    }

    medium.append(&mut small); // I wish this consumed...
    medium.append(&mut tiny); // I wish this consumed...

    return Ok(first_fit_decreasing_prv(medium, bins, capacity));
}

pub fn ffd<T: Pack>(mut items: Vec<T>, capacity: u64) -> Result<Vec<Bin<T>>, ObjectTooBigError> {
    items.sort_unstable_by_key(|i| -(i.get_size() as i64)); // Sort largest first...
    return Ok(first_fit_decreasing_prv(items, Vec::new(), capacity))
}


fn first_fit_decreasing_prv<T: Pack>(mut items: Vec<T>, mut bins: Vec<Bin<T>>, capacity: u64) -> Vec<Bin<T>>{
    // Assumes the list is sorted since this is a private function.

    for item in items.drain(..) { // Finish off the items
        let mut bin_that_fits: Option<&mut Bin<T>> = None;

        for bin in bins.iter_mut() {
            if bin.does_fit(&item) {
                bin_that_fits = Some(bin);
                break;
            }
        }
        match bin_that_fits {
            Some(bin) => bin.add_item(item),
            None => bins.push(Bin::new_from(item, capacity))
        }
    }

    return bins;
}

// Finds index of largest item that fits in the bin
pub fn largest_that_fits<T: Pack>(items: &Vec<T>, bin: &Bin<T>) -> Option<usize> {
    let mut i: isize = items.len() as isize - 1;

    // Unwrap is garaunteed here since the list is not empty (and so must have a last element)
    if items.is_empty() || !bin.does_fit(items.last().unwrap()) {
        return None;
    }

    while i >= 0 {
        if let Some(item) = items.get(i as usize) { // This will always return since it can't oob
            if !bin.does_fit(&item) {
                return Some(i as usize+1);
            }
        }
        i = i - 1;
    }

    return Some(0); // If we've finished the loop
}