
struct Node<T>{
    value:T,
    next_node: Option<Box<Node<T>>>,
}
impl<'a,T: std::fmt::Display> Node<T>{
    fn incr(&mut self) -> Option<&mut Node<T>>{
        return Some(&mut *(self.next_node.as_mut().unwrap()));
    }
    fn print(&self){
        println!("{}",self.value);
    }
    fn add_node(&mut self,value: T){
        let new_node = Node{value,next_node:None};
        //should transfer ownership
        self.next_node =  Some(Box::new(new_node));
    }
}

fn main() {
    let node2 = Node{value:2,next_node:None};
    let mut node1 = Node{value:1,next_node:Some(Box::new(node2))};

    //try printing out the value of node2 using node 1.
    //first increment node1. his should return an option with a node pointer
    //then unwrap the node ref and try printing
    node1.incr().unwrap().print();

    //should add a 3rd node with the value of 5
    node1.incr().unwrap().add_node(5);
    //should print out the new value
    node1.incr().unwrap().incr().unwrap().print();
    //i know want to make it easier to mutate these values.
}
