pub futures
use path_table::PathTable;
pub use type Handler = fn(Request<Body>, Params) -> Body;


pub use type Handler = impl Furure<Output=>

pub struct Repo{
    tree: PathTree::<Handler>;
    // root: Node<Root>
}

use path_table::PathTable;
