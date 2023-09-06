mod nixopts;
use nixopts::nixopts::parse_options;

fn main() {
    print!("{:#?}", parse_options());
}
