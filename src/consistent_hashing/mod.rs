use std::{
    cell::RefCell,
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    rc::{Rc, Weak},
};

pub struct ConsistentHasing {
    servers: RefCell<Vec<Rc<RefCell<Server>>>>,
    virtual_nodes: RefCell<Vec<Weak<VirtualNode>>>,
}

struct Server {
    name: String,
    key: String,
    nodes: RefCell<Vec<Rc<VirtualNode>>>,
}

struct VirtualNode {
    server: RefCell<Weak<RefCell<Server>>>,
    node_key: String, // will be hashed to get hash value of the node.
    hash_value: u64,
}

impl ConsistentHasing {
    fn new() -> Self {
        ConsistentHasing {
            servers: RefCell::new(vec![]),
            virtual_nodes: RefCell::new(vec![]),
        }
    }

    fn server_exist(&self, server_key: &str) -> bool {
        return self
            .servers
            .borrow()
            .iter()
            .any(|server: &Rc<RefCell<Server>>| server.borrow().key.eq(server_key));
    }

    fn get_server(&self, server_key: &str) -> Option<Rc<RefCell<Server>>> {
        let list = self.servers.borrow();
        let item = list.iter().find(|server| server.borrow().key == server_key);
        if let Some(server) = item {
            return Some(Rc::clone(server));
        }
        None
    }

    fn add_node(&mut self, server_key: &str, virtual_node_count: u32) -> Result<(), &'static str> {
        let server = self.get_server(server_key);
        if let Some(s) = server {
            let start = s.borrow().nodes.borrow().len() as u32;
            let mut vec: Vec<Rc<VirtualNode>> = Vec::new();
            let mut vec_weak: Vec<Weak<VirtualNode>> = Vec::new();
            for _ in start..start + virtual_node_count {
                let node = Rc::new(VirtualNode{
                    hash_value: 1,
                    node_key: "".to_string(),
                    server: RefCell::new(Rc::downgrade(&s)),
                });
                vec.push(Rc::clone(&node));
                vec_weak.push(Rc::downgrade(&node));
            }
            s.borrow_mut().nodes.borrow_mut().append(&mut vec);
            self.virtual_nodes.borrow_mut().append(&mut vec_weak);
            return Ok(());
        }
        Err("server doesn't exist")
    }

    fn add(&mut self, server: Server, virtual_node_count: u32) -> Result<(), &'static str> {
        if self.server_exist(&server.key) {
            return Err("server already existed");
        }

        let mut nodes: Vec<Rc<VirtualNode>> = Vec::new();

        for i in 0..virtual_node_count {
            let mut node_key: String = server.key.clone();
            node_key.push('_');
            node_key.push_str(&i.to_string());

            let hash_value = ConsistentHasing::hash(&node_key);

            let node = Rc::new(VirtualNode {
                hash_value,
                node_key,
                server: RefCell::new(Weak::new()),
            });

            nodes.push(Rc::clone(&node));
            self.virtual_nodes.borrow_mut().push(Rc::downgrade(&node));
        }

        let server_ref = Rc::new(RefCell::new(Server {
            name: "".to_string(),
            key: "".to_string(),
            nodes: RefCell::new(nodes),
        }));

        // let a = server_ref.nodes.borrow();
        let a = server_ref.as_ref().borrow();
        for i in a.nodes.borrow().as_slice() {
            *i.server.borrow_mut() = Rc::downgrade(&server_ref);
        }

        self.servers.borrow_mut().push(Rc::clone(&server_ref));

        Ok(())
    }

    fn hash(value: &str) -> u64 {
        let mut s = DefaultHasher::new();
        value.hash(&mut s);
        s.finish()
    }
}

impl Server {
    fn new(name: &str, key: &str) -> Self {
        Server {
            name: name.to_string(),
            key: key.to_string(),
            nodes: RefCell::new(vec![]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ConsistentHasing, Server};

    #[test]
    fn default_test() {
        let mut ch = ConsistentHasing::new();
        let server = Server::new("Ha Noi", "hanoi");
        let virtual_node_count = 5;
        let add = ch.add(server, virtual_node_count);
        assert_eq!(add, Ok(()));
        assert_eq!(ch.servers.borrow().len(), 1);
        assert_eq!(ch.virtual_nodes.borrow().len(), virtual_node_count as usize);
    }

    #[test]
    fn my_test() {}
}
