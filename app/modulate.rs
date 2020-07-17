use bitvec::*;
use bitvec::prelude::BitVec;

#[derive(Debug)]
enum State {
    NumComplete,
    Pos,
    Len(bool),
    Val(usize, bool),
}

// To the inevitable reader of this code, I wanted to do this with nom,
// but I couldn't figure it out fast enough, so I did it the pragmatic way, which I am aware is terrible.
// ¯\_(ツ)_/¯
fn parseToken(input : &BitVec) -> Vec<i64> {
    let mut inputIter = input.iter();
    let mut myState = State::NumComplete;
    let mut numbers: Vec<i64> = Vec::new();
    loop {
        match &mut myState {
            State::NumComplete => {
                if *inputIter.next().unwrap() {
                    //1
                    if *inputIter.next().unwrap() {
                        //11
                        myState = State::Pos;
                    } else {
                        //10
                        panic!("found a 10 when I shouldn't have!")
                    };
                } else {
                    if *inputIter.next().unwrap() {
                        //01
                        panic!("found a 01 when I shouldn't have!")
                    } else {
                        //00 end of message
                        break;
                    };
                };
            },
            State::Pos => {
                if *inputIter.next().unwrap() {
                    //1
                    if *inputIter.next().unwrap() {
                        //11
                        panic!("found a sign of \'11\'")
                    } else {
                        //10
                        myState = State::Len(false);
                    }
                } else {
                    if *inputIter.next().unwrap() {
                        //01
                        myState = State::Len(true);
                    } else {
                        //00 
                        //panic!("found a sign of \'00\'")
                        //don't panic, it's the empty vec, probably. /shrug
                        break;
                    }
                }
            },
            State::Len(positive) => {
                let isPositive = *positive;
                let mut currentLength = 0;
                while let Some(bit) = inputIter.next() {
                    if *bit  {
                        currentLength += 1;
                    } else {
                        myState = State::Val(currentLength*4, isPositive);
                        break;
                    }
                }
            }
            State::Val(size, isPositive) => {
                let mut magnitude = 0u64;
                for _ in 0..*size {
                    magnitude = magnitude << 1;
                    if *inputIter.next().unwrap() {
                        magnitude += 1;
                    }
                }
                let mut num : i64 = magnitude as i64;
                numbers.push(num);
                myState = State::NumComplete;
            }
        };
    };
    numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty_list() {
        let msg = bitvec![1,1,0,0];
        let result = parseToken(&msg);
        assert_eq!(Vec::<i64>::new(), result);
    }
    #[test]
    fn parse_their_msg() {
        let msg = bitvec![1,1,0,1,1,0,0,0,0,1,1,1,0,1,1,1,1,1,1,0,0,0,0,1,0,0,1,1,1,0,1,0,1,1,1,0,0,1,0,0,0,0];
        let result = parseToken(&msg);
        assert_eq!(vec![1,80612], result);
    }
}