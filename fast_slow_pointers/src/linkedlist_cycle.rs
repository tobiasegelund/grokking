struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

fn has_cycle(head: &Node) -> bool {
    let mut slow = head;
    let mut fast = head;
    while let Some(f) = &fast.next {
        fast = f;
        if let Some(s) = &slow.next {
            slow = s;
            if slow.value == fast.value {
                return true;
            }
        } else {
            return false;
        }
        if let Some(f) = &fast.next {
            fast = f;
        } else {
            return false;
        }
    }
    return false;
}

fn main() {
    let mut head = Box::new(Node {
        value: 1,
        next: None,
    });
    head.next = Some(Box::new(Node {
        value: 2,
        next: None,
    }));
    head.next.as_mut().unwrap().next = Some(Box::new(Node {
        value: 3,
        next: None,
    }));
    head.next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(Node {
        value: 4,
        next: None,
    }));
    head.next
        .as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next = Some(Box::new(Node {
        value: 5,
        next: None,
    }));
    head.next
        .as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next = Some(Box::new(Node {
        value: 6,
        next: None,
    }));
    println!("LinkedList has cycle: {}", has_cycle(&head));

    head.next
        .as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next = Some(head.next.as_ref().unwrap().next.as_ref().unwrap());
    println!("LinkedList has cycle: {}", has_cycle(&head));

    head.next
        .as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next = Some(
        head.next
            .as_ref()
            .unwrap()
            .next
            .as_ref()
            .unwrap()
            .next
            .as_ref()
            .unwrap(),
    );
    println!("LinkedList has cycle: {}", has_cycle(&head));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
