#![feature(globs,macro_rules,phase)]

#[phase(plugin,link)]
extern crate nom;

use nom::{IResult,Producer,FileProducer,Mapper,Mapper2};
use nom::IResult::*;
use nom::ProducerState::*;

use std::str;
use std::fmt::Show;

#[test]
fn map_test_x() {
  let res = Done((),"abcd".as_bytes()).map(|data| { str::from_utf8(data).unwrap() }); 
  assert_eq!(res, Done((), "abcd"));
}

#[test]
fn tag_test() {
  FileProducer::new("links.txt", 20).map(|producer: FileProducer| {
    let mut p = producer;
    tag!(f "https://".as_bytes());
    //p.push(|par| par.flat_map(f).flat_map(print));
    fn pr(par: IResult<(),&[u8]>) -> IResult<&[u8],()> {
      let p = par.flat_map(f).map_opt(str::from_utf8).flat_map(print);
      println!("p : {}", p);
      Done("".as_bytes(), ())
    }
    pusher!(ps, pr)
    ps(&mut p);
    //assert!(false);
  }); 
}

pub fn print<'a,T: Show>(input: T) -> IResult<T, ()> {
  println!("{}", input);
  Done(input, ())
}


#[test]
fn is_not_test() {
  is_not!(foo "\r\n".as_bytes())
  let a = "ab12cd\nefgh".as_bytes();
  assert_eq!(Done((), a).flat_map(foo), Done("\nefgh".as_bytes(), "ab12cd".as_bytes()))
}