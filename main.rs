/*
=================== CIS 400/700 ASSIGNMENT COVER SHEET =================== 

1.  Assignment (e.g, Homework 17): HW3

2a. Name & email address (First author): Curtis Oliver <cuoliver@syr.edu>

2b. Name & email address (Second author, if any): -

3a. Did you consult with anyone on parts of this assignment?
    (Yes/No): No

3b. If so, give the details of these consultations (e.g., who, what,
    where)? - 


4a. Did you consulted an outside source on parts of this assignment 
    (e.g., an Internet site)?  (Yes/No): Yes

4b. If so, give the details of these consultations (e.g., who, what, 
    where)? Just the provided sources


5.  The authors attest that the above is correct, (Yes/No): Yes

=====================================================================
*/
use std::cmp;

fn main() {
    let initial = [66, 70, 52, 93, 44, 67, 47, 10, 11, 13, 94, 99, 51, 12];
    let mut to_sort;
    println!("initial:          {:?}",initial);

    to_sort  = initial.clone();
    bubble_sort(&mut to_sort);
    println!("bubble-sorted:    {:?}",to_sort);

    to_sort  = initial.clone();
    sel_sort(&mut to_sort);
    println!("selection-sorted: {:?}",to_sort);

    to_sort  = initial.clone();
    insert_sort(&mut to_sort);
    println!("insertion-sorted: {:?}",to_sort);

    println!();
    println!("unordered search:");
    report_search(44,unordered_search(44,&initial[..]));
    report_search(43,unordered_search(43,&initial[..]));

    println!();
    println!("binary search:");
    report_search(44,binary_search(44,&to_sort[..]));
    report_search(43,binary_search(43,&to_sort[..]));

    println!();
    println!("the min and max of initial are {:?}",
             min_max(&initial[..]));
}

/*
// NOTE!! The following will not compile: It needs lifetime annotations.
// We'll fix this later on.
fn swap(x :&mut u32, y : &mut u32) {
    let t = x;
    x = y;
    y = t;
}
*/

fn bubble_sort(a : &mut [u32]) {
    let len = a.len();
    for i in 0..len {
        for j in 0..(len-i-1) {
            if a[j]>a[j+1] {
                // swap the values of a[j] and a[j+1]
                let t = a[j];
                a[j] = a[j+1];
                a[j+1] = t;
            }
        }
    }
}

fn report_search(x : u32, r : Option<usize>) {
    print!("\t {} ",x);
    match r {
        None    => { println!("not found"); },
        Some(i) => { println!("found at index {}",i); },
    }
}

fn unordered_search(x : u32, a : &[u32]) -> Option<usize> {
    for i in 0..a.len() {
        if x==a[i] { return Some(i); }
    }
    None
}


fn sel_sort(a : &mut [u32]) {
	let len = a.len();
	for i in 0..len{
		let mut min = i;
		for j in i..len{
			if a[j] < a[min]{
				min = j;
			}
		}
		if min != i{
		    let t = a[i];
            a[i] = a[min];
            a[min] = t;
		}
	}
}

fn insert_sort(a : &mut [u32]) {
	let len = a.len();
	for i in 0..len{
		let sentinel = a[i];
		let mut j = i.clone();
		while j>0 && a[j-1]>sentinel {
			a[j]= a[j-1];
			j = j-1;
		
		}
		a[j] = sentinel;
	}
}

fn binary_search(x : u32, a : &[u32]) -> Option<usize> {
	let mut l = 0;
	let mut r = a.len();
	let mut m;
	while l <= r {
		m = (l+r)/2;
		if a[m] < x {l = m+1;}
		else if a[m] > x {r = m-1;}
		else if a[m] == x {return Some(m);}
	}
	None
}

fn min_max(a : &[u32]) -> (u32,u32) {
    let len = a.len();
    assert!(len>0);
    
    if len <= 1 {
    	return (cmp::min(a[0],a[len-1]), cmp::max(a[0],a[len-1]));
    }
    else {
    	let (min1, max1) = min_max(&a[0..len/2]);
    	let (min2, max2) = min_max(&a[(len/2)..len]);
    	let min = cmp::min(min1,min2);
    	let max = cmp::max(max1,max2);
    	return (min, max);
    }
}
    





    
// NOTE:
// cmp::min(a,b) returns the minimum of a and b
// cmp::max(a,b) returns the maximum of a and b


