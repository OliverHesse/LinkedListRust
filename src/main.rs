
struct Node<T>{
    value:T,
    next_node: Option<Box<Node<T>>>,
}
impl<'a,T: std::fmt::Display> Node<T>{
    fn incr(&self) -> Option<& Node<T>>{
        return Some(&*(self.next_node.as_ref().unwrap()));
    }
    fn print(&self){
        println!("{}",self.value);
    }
}

fn main() {
    let node2 = Node{value:2,next_node:None};
    let node1 = Node{value:1,next_node:Some(Box::new(node2))};

    //try printing out the value of node2 using node 1.
    //first increment node1. his should return an option with a node pointer
    //then unwrap the node ref and try printing
    node1.incr().unwrap().print();
}
