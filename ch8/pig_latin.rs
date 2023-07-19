

pub fn to_pig_latin(s: &String) -> String {
    let mut tail = String::new();
    let mut rest = String::new();

    for (i, c) in s.chars().enumerate() {
	if i == 0 {
	    match c {
		'a' => {
		    tail.push_str("hay");
		    rest.push(c);
		}
		'e' => {
		    tail.push_str("hay");
		    rest.push(c);
		}
		'i' => {
		    tail.push_str("hay");
		    rest.push(c);
		}
		'o' => {
		    tail.push_str("hay");
		    rest.push(c);
		}
		'u' => {
		    tail.push_str("hay");
		    rest.push(c);
		}
		_other => {
		    tail.push(c);
		    tail.push_str("ay");
		}
	    }
	} else {
	    rest.push(c)
	}
    }

    format!("{rest}-{tail}")
    
    //match &s[0..1] {
	//"a" => format!("{s}-hay"),
	//"e" => format!("{s}-hay"),
	//"i" => format!("{s}-hay"),
	//"o" => format!("{s}-hay"),
	//"u" => format!("{s}-hay"),
	//other => {
	//    let rest = &s[1..];
	//    format!("{rest}-{other}ay")
	//}
    //}
}
