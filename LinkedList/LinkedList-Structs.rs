use std::io::{self, BufRead};


#[derive(Clone)]
struct Node {
    next : Option<Box<Node>>,
    value : u32
}

struct LinkedList {
    head : Option<Box<Node>>
}

trait List {
    fn new() -> Self;
    fn length(&self) -> usize;
    fn traverse(&self);
    fn add(&mut self , val : u32);
    fn delete(&mut self , value : u32);
}

impl  List for LinkedList {
    fn new() -> Self {
        Self {
            head:None,
        }
    }

    fn length(&self) -> usize {
        if let Some(current_list) = &self.head {
            current_list.length()
        }else {
            0
        }
    }

    fn add(&mut self, val: u32)  {
        let new_head = Node {
            next: self.head.take(),
            value: val,
        };
        self.head = Some(Box::new(new_head));
    }
    

    fn traverse(&self) {
        if let Some(node) = self.head.as_ref() {
            node.traverse();
        }
    }

    fn delete(&mut self , value : u32) {
        if let Some(_node) = self.head.as_ref() {
            if let Some(mut head) = self.head.take() {
                let new_head = head.delete(value);
                self.head = new_head;
            }    
        }
    }
}

impl Node {
    fn length(&self) -> usize {
        if let Some(node) = &self.next {
            1 + node.length()
        } else {
            1
        }
    }

    fn traverse(&self) {
        println!("Element {}" , self.value);
        if let Some(node) = self.next.as_ref() {
            node.traverse();
        }else{
            println!("The end ")
        }
    }

    fn delete(&mut self ,val : u32) -> Option<Box<Node>> {
        if self.value == val {
            return self.next.take();
        } else {
            if let Some(next) = self.next.as_mut() {
                self.next = next.delete(val);
            }
        }
        Some(Box::new(self.clone()))
    }
}

fn main () {
    let stdin = io::stdin();
    let mut LinkedList = LinkedList::new();
    loop {
        let mut input = String::new();
        stdin.lock().read_line(&mut input).unwrap();
        let input = input.trim();
        let args : Vec<&str> = input.split_whitespace().collect();
        println!("{:?}" , args);
        if input == "exit" {
            break;
        }
        if args.len() > 0 {
            match  args[0] {
                "add" => {
                    LinkedList.add(args[1].parse::<u32>().unwrap());
                },
                "delete" => {
                    LinkedList.delete(args[1].parse::<u32>().unwrap());
                },
                "print" => {
                    LinkedList.traverse();
                },
                "len" => {
                    println!("{:?}" , LinkedList.length());
                }
                _ => {}
            }
        }

        // println!("{}", input);
    }
}