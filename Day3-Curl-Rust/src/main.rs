/*
 * @Date: 2021-09-28 02:12:45
 * @LastEditors: LIULIJING
 * @LastEditTime: 2021-09-29 17:19:34
 */

#![allow(dead_code)]
#![allow(unused)]

use std::fs;

mod arithmatic{
    pub fn apply(value: i32, f:fn(i32)->i32)-> i32{
        f(value)
    }
    
    pub fn square(value: i32) -> i32 {
        value * value
    }
    
    pub fn cube(value: i32)->i32 {
        value * value * value
    }
}

 
mod data_structure {

    #[derive(Debug)]
    #[derive(Clone, Copy)]
    #[derive(PartialEq, Eq)]
    struct UserId(u64);
    #[derive(Clone, Copy)]
    #[derive(Debug)]
    #[derive(PartialEq, Eq)]
    struct TopicId(u64);
    enum States {
        JOIN((UserId, TopicId)),
        LEAVE((UserId, TopicId)),
        Message((UserId, TopicId, String))
    }

    #[derive(Debug)]
    struct Topic {
        id: TopicId,
        name: String,
        owner: UserId
    }
    
    #[derive(Debug)]
    struct User {
        id: UserId,
        name: String,
        gender: Gender
    }

    #[derive(Debug)]
    enum Gender {
        Unspecified = 0,
        Female = 1,
        Male = 2
    }

    /* pattern match */
    fn feed_msg(state: &States) {
        if let States::Message((_, _, msg)) = state {
            println!("broadcast: {}", msg);
        } 
    }

    #[test]
    pub fn run() {
        use rand::random;
        let alice = User { id:UserId(random()), name:"Alice".into(), gender: Gender::Female};
        let bob = User {id:UserId(random()), name:"Bob".into(), gender: Gender::Male};
        let news = Topic{id:TopicId(random()), name:"news".into(), owner: alice.id};
        let event1 = States::JOIN((alice.id.into(), news.id));
        let event2 = States::LEAVE((bob.id, news.id));
        let event3 = States::Message((alice.id, news.id, "The World Changes.\n".into()));
        assert_eq!(news.owner, alice.id);
        feed_msg(& event3);
        feed_msg(&event2);
        feed_msg(&event1);
    }
}

mod control_syntax {

    

    fn fib_recur(n: u64)->u64 {
        match n {
            0 => 0,
            1 => 1,
            x => fib_recur(x - 1) + fib_recur(x - 2)
        }
    }

    fn swap_add(a: &mut u64, b: &mut u64) {
        let tmp = *a;
        *a = *a + *b;
        *b = tmp;
    }
    
    fn fib_while(n: u64)->u64 {
        let mut res:u64 = 1;
        let mut pre = 0;
        let mut i = 1;
        while i < n {
            swap_add(&mut res, &mut pre);
            i += 1;
        }
        res
    }

    fn fib_loop(n: u64)->u64 {
        let mut res:u64 = 1;
        let mut pre = 0;
        let mut i = 1;
        loop {
            swap_add(&mut res, &mut pre);
            i += 1;
            if i >= n {
                break;
            }
        }
        res
    }

    fn fib_for(n: u64)->u64 {
        let mut res:u64 = 1;
        let mut pre = 0;
        for _i in 1..n {
            swap_add(&mut res, &mut pre);
        }
        res
    }

    #[test]
    fn check_fib() {
        assert_eq!(5, fib_recur(5));
        assert_eq!(5, fib_loop(5));
        assert_eq!(5, fib_while(5));
        assert_eq!(5, fib_for(5));
    }
    
}




#[cfg(test)]
mod test_suits{
    #[test]
    fn test_arith() {
        use super::arithmatic::*;
        assert!(4 == apply(2, square));
        assert_eq!(8, apply(2, cube));
        assert_eq!(6, apply(3, |a|a*2));
        assert_eq!(6, apply(3, |a|{a * 2}));
        assert_eq!(9, apply(3, |a:i32|->i32{a + 6}));
        let x = 4;
        let equal_to_x = |z| z == x;
        assert!(equal_to_x(4));

    }
}

#[test]
fn tests() {
    for arg in std::env::args() {
        println!("\n{}", arg)
    }
   
}

pub fn main() {
    println!("Hello, world!");
    let mut url = String::from("https://www.rust-lang.org/");
    let mut output = String::from("rust.md");
    for (i, str) in std::env::args().enumerate() {
        match i {
            1 => {url = str;}
            2 => {output = str;}
            _ => ()
        }
    }

    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("Converting html to markdown..");
    let md = html2md::parse_html(&body);

    fs::write(&output, md.as_bytes()).unwrap();
    println!("Converted markdown has been saved in {}.", &output);
}
