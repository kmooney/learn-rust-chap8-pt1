use itertools::Itertools;
use std::collections::HashMap;

fn mode(vv: &Vec<i32>) -> Option<i32> {
	let mut h: HashMap<i32, i32> = HashMap::new();
	// count 'em up
	for i in vv.iter() {
		let c = h.entry(*i).or_insert(0);
		*c+=1;
	}

	let mut most_used: Option<i32> = None;
	let mut highest_count: i32 = 0;

	for i in vv.iter() {
		let count = h[i];
		if count > highest_count {
			most_used = Some(h[i]);
			highest_count = count;	
		}
	}

	return most_used
}

fn mean(vv: &Vec<i32>) -> f32 {
	let len: f32 = vv.iter().len() as f32;
	if len == 0.0 {
		0.0
	} else {
		vv.iter().sum::<i32>() as f32 / vv.iter().len() as f32
	}
}

fn median(vv: &Vec<i32>) -> Option<&i32> {
	vv.iter().sorted().skip(vv.iter().len() / 2).next()
}

fn main() {
    let v: Vec<i32> = vec!(1);
    //let v : Vec<i32> = Vec::new();
    match mode(&v) {
    	None => println!("Vector {:?} has no mode", v),
    	Some(x) => println!("Vector {:?} mode is {}", v, x),
    }

    println!("Vector {:?} mean is {}", v, mean(&v));

    match median(&v) {
    	None => println!("Vector {:?} has no median", v),
    	Some(x) => println!("Vector {:?} median is {}", v, *x),
    }
}

