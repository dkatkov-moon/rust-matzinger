use std::cell::RefCell;
use std::mem;

#[derive(Debug, Clone)]
pub struct IotDevice {
    pub numeric_id: u64,
    pub address: String,
    pub path: String,
}

impl IotDevice {
    pub fn new(id: u64, address: impl Into<String>, path: impl Into<String>) -> IotDevice {
        IotDevice {
            numeric_id: id,
            address: address.into(),
            path: path.into(),
        }
    }
}

type Tree = Option<Box<Node>>;

#[derive(Debug)]
struct Node {
    pub dev: IotDevice,
    left: Tree,
    right: Tree,
}

impl Node {
    pub fn new(dev: IotDevice) -> Tree {
        Some(Box::new(Node {
            dev: dev,
            left: None,
            right: None,
        }))
    }
}

#[derive(Debug)]
struct DeviceRegistry {
    root: Tree,
    pub length: u64,
}

impl DeviceRegistry {
    pub fn new_empty() -> DeviceRegistry {
        DeviceRegistry {
            root: None,
            length: 0,
        }
    }

    pub fn add(&mut self, device: IotDevice) {
        self.length += 1;
        let root = mem::replace(&mut self.root, None);
        self.root = self.add_rec(root, device);
    }

    fn add_rec(&mut self, node: Tree, device: IotDevice) -> Tree {
        match node {
            Some(mut n) => {
                if n.dev.numeric_id <= device.numeric_id {
                    n.left = self.add_rec(n.left, device);
                } else {
                    n.right = self.add_rec(n.right, device);
                }
                Some(n)
            }
            _ => Node::new(device),
        }
    }

    pub fn find(&self, numeric_id: u64) -> Option<IotDevice> {
        self.find_rec(&self.root, numeric_id)
    }

    fn find_rec(&self, node: &Tree, numeric_id: u64) -> Option<IotDevice> {
        match node {
            Some(n) => {
                if n.dev.numeric_id == numeric_id {
                    Some(n.dev.clone())
                } else if n.dev.numeric_id <= numeric_id {
                    self.find_rec(&n.left, numeric_id)
                } else {
                    self.find_rec(&n.right, numeric_id)
                }
            }
            _ => None,
        }
    }

    pub fn walk(&self, callback: impl Fn(&IotDevice) -> ()) {
        self.walk_in_order(&self.root, &callback);
    }

    fn walk_in_order(&self, node: &Tree, callback: &impl Fn(&IotDevice) -> ()) {
        if let Some(n) = node {
            self.walk_in_order(&n.left, callback);
            callback(&n.dev);
            self.walk_in_order(&n.right, callback);
        }
    }
}

fn main() {
    let mut registry = DeviceRegistry::new_empty();
    registry.add(IotDevice::new(100, "root", "100"));
    registry.add(IotDevice::new(1, "address_1", "1"));
    registry.add(IotDevice::new(101, "address_2", "101"));
    registry.add(IotDevice::new(2, "address_3", "2"));

    let my_devices: RefCell<Vec<IotDevice>> = RefCell::new(vec![]);
    registry.walk(|n| my_devices.borrow_mut().push(n.clone()));

    println!("{:?}", my_devices);
}
