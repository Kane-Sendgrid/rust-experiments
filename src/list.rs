// use std::fmt::Debug;

// #[derive(Debug)]
// struct Node<T>
//     where T: Debug
// {
//     data: T,
//     prev: Option<Box<Node<T>>>,
// }

// impl<T> Drop for Node<T>
//     where T: Debug
// {
//     fn drop(&mut self) {
//         println!("Dropping! {:?}", self);
//     }
// }

// struct Stack<T>
//     where T: Debug
// {
//     tail: Option<*mut Node<T>>,
// }

// impl<T> Stack<T>
//     where T: Debug
// {
//     fn new() -> Self {
//         Stack { tail: None }
//     }

//     fn push(&mut self, mut node: Box<Node<T>>) {
//         match self.tail {
//             Some(tail_node) => {
//                 println!("Some: {:?}", tail_node);
//                 unsafe { node.prev = Some(Box::from_raw(tail_node)) }
//                 self.tail = Some(Box::into_raw(node));
//             }
//             None => {
//                 println!("None");
//                 self.tail = Some(Box::into_raw(node));
//             }
//         }
//     }

//     fn pop(&mut self) -> Option<Box<Node<T>>> {
//         match self.tail {
//             Some(tail_node) => {
//                 println!("Some {:?}", tail_node);
//                 // self.tail = Box::into_raw(**tail_node.prev);
//                 let b = unsafe { Box::from_raw(tail_node) };
//                 let prev = b.prev;
//                 // self.tail = b.prev.map(|n| Box::into_raw(n));
//                 Some(b)
//             }
//             None => None,
//         }

//     }
// }

// fn main() {
//     let mut s: Stack<i32> = Stack::new();
//     s.push(Box::new(Node {
//         data: 0,
//         prev: None,
//     }));
//     s.push(Box::new(Node {
//         data: 1,
//         prev: None,
//     }));
//     let n = s.pop();
//     println!("Popped {:?}", n);
//     let n = s.pop();
//     println!("Popped {:?}", n);
//     // s.push(&mut Node { i: 1, prev: None });
//     // s.push(&mut Node { i: 2, prev: None });
// }