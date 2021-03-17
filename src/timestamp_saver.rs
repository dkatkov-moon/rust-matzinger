use std::cmp;

type Node = Option<u64>;

const MIN_SIZE: usize = 3;

#[derive(Debug)]
pub struct TimestampSaver {
    buf: Box<[Node]>,
    cap: usize,
    pub length: usize,
}

impl TimestampSaver {
    pub fn new_empty() -> TimestampSaver {
        TimestampSaver {
            buf: Box::new([None; MIN_SIZE]),
            cap: MIN_SIZE,
            length: 0,
        }
    }

    fn grow(&mut self, min_cap: usize) {
        let old_cap = self.buf.len();
        let mut new_cap = old_cap + (old_cap << 1);

        new_cap = cmp::max(new_cap, min_cap);
        new_cap = cmp::min(new_cap, usize::max_value());

        let current = self.buf.clone();
        self.cap = new_cap;
        self.buf = vec![None; new_cap].into_boxed_slice();
        self.buf[..current.len()].clone_from_slice(&current);
    }

    pub fn append(&mut self, value: u64) {
        if self.length == self.cap {
            self.grow(self.length + 1);
        }

        self.buf[self.length] = Some(value);
        self.length += 1;
    }

    pub fn at(&self, index: usize) -> Node {
        if self.length > index {
            self.buf[index]
        } else {
            None
        }
    }
}

pub struct ListIterator {
    current: usize,
    data: Box<[Node]>,
}

impl ListIterator {
    pub fn new(index: usize, buf: Box<[Node]>) -> ListIterator {
        ListIterator {
            current: index,
            data: buf,
        }
    }
}

impl Iterator for ListIterator {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.current < self.data.len() {
            let item = self.data[self.current];
            self.current += 1;
            item
        } else {
            None
        }
    }
}

impl IntoIterator for TimestampSaver {
    type Item = u64;
    type IntoIter = ListIterator;

    fn into_iter(self) -> Self::IntoIter {
        ListIterator::new(0, self.buf)
    }
}

fn main() {
    let mut timestamp = TimestampSaver::new_empty();
    timestamp.append(1);
    timestamp.append(1);
    timestamp.append(1);
    timestamp.append(2);
    println!("{:?}", timestamp);

    let mut ts_iter = timestamp.into_iter();
    println!("{:?}", ts_iter.next());
    println!("{:?}", ts_iter.next());
    println!("{:?}", ts_iter.next());
    println!("{:?}", ts_iter.next());
    println!("{:?}", ts_iter.next());
}
