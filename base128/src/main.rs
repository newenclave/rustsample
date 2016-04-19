fn pack128( mut x: u64 ) -> Vec<u8> {
	let mut res = Vec::new( );
	res.reserve(8);
	while x > 0x7F {
		res.push(((x & 0x7F) | 0x80) as u8);
		x >>= 7
	}
	res.push((x & 0xFF) as u8);
	res
}

fn unpack128( vec: &[u8] ) -> (u64, usize) {
	let mut res   = 0 as u64;
	let mut shift = 0;
	let mut count = 0 as usize;
	for v in vec {
		res |= (((v & 0x7F) as u64) << shift) as u64;
		shift += 7;
		count += 1;
		if (*v & 0x80) == 0 {
			break
		}
	}
	(res, count)
}

fn test( ) {
	 let tests = vec![0, 0, 0, 1, 2, 255, 728,
                456, 11111111,
                99998888, 1234567890,
                0987654321, 0, 1, 2 ];

	let mut res: Vec<u8> = Vec::new( );
	res.reserve( 32 );
	for t in &tests {
	    res.extend( pack128(*t).iter( ) )
	}

	println!("\nUnpacking {} bytes;", res.len( ));
	let mut cnt = 0 as usize;
	let mut i   = 0;
	while cnt < res.len( ) {
	    let (v, c) = unpack128(&res[cnt..]);
	    cnt += c;
	    //println!("res[{}] {}", i, v );
	    if tests[i] != v {
	        panic!("!!!")
	    }
	    i += 1
	}

}

fn main( ) {
	// for _ in 0..5_000_00 {
	// 	test( )
	// }
	for _ in 0..10 {
		test( )
	}
}
