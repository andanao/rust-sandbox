

fn main() {
    let names = vec![
	"Alice",
	"Bob",
	"Eve",
    ];

    for name in names.into_iter(){
	if name == "Bob" {
	    continue
	}
	println!("{}", name);
    }
}
