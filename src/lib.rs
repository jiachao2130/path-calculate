/*!
# Path Calculate

This is a library help you to calculate with `Path` or `PathBuf`, such as get absolute path, get the relate root or the relative path between two pathes, get the '~'(home_dir) if it exist.

This is based on path_absolutize library, I've use `as_absolute_path` replace the row `absolutize`, because it do not support '~' which is use recently, at least in UNIX System.

The following examples show the usage about this crate.

## Examples

There are some methods you can use.

### home_dir
Get the current user's HOME if it exist in your env, or it return an error(I'm lazy, so just put this method in the Calculate trait).

```rust
extern crate path_calculate;

use std::path::Path;

use path_calculate::*;

let p = Path::new("/home/cc");

if let Some(home_dir) = p.home_dir() {
    println!("Home path: {:?}", home_dir);
}

```

### as_absolute_path
This is almost like a shadow of the `absolutize` in `path-absolutize`, I only add `~`($HOME) support in the method(Unix or Windnows).

```rust
extern crate path_calculate;

use std::path::Path;

use path_calculate::*;

// I use the user Chao, homd dir is /home/chao
let p = Path::new("~/works/path-calculate");

assert_eq!("/home/chao/works/path-calculate", p.as_absolute_path().unwrap().to_str().unwrap());
```

```rust
extern crate path_calculate;

use std::path::Path;

use path_calculate::*;

// In my Windows
let p = Path::new("~\\works\\path-calculate");

assert_eq!("C:\\Users\\jiach\\works\\path-calculate", p.as_absolute_path().unwrap().to_str().unwrap());
```

### relative_root_with
Sometimes I would use the relative root of pathes, it return an absolutize relative_root path.
Behold, in Windows, it can not calculate the relative_root on different disks(C:\\, D:\\,,,), when you try to use this method it return an ioerror such as `io::ErrorKind::InvalidInput`.

```rust
extern crate path_calculate;

use std::path::Path;

use path_calculate::*;

// In Linux, run by standard user, HOME in /home/${USERNAME}
let p1 = Path::new("/home/gits/mkisos");
let p2 = Path::new("~/trash");

let relative_root = p1.relative_root_with(&p2);

assert_eq!("/home", relative_root.unwrap().to_str().unwrap())
```

```rust
extern crate path_calculate;

use std::io::ErrorKind;
use std::path::Path;

use path_calculate::*;

// Windows ok
let d1 = Path::new("D:\\Games\\Videos\\Replays");
let d2 = Path::new("D:\\Games\\Dota2");

assert_eq!("D:\\Games", d1.relative_root_with(&d2).unwrap().to_str().unwrap());

// Windows error
let c1 = Path::new("~");
assert_eq!(Err(ErrorKind::InvalidInput), c1.relative_root_with(&d1).unwrap_err().kind());

```

### related_to
This method is used to calculate the dst_path's relative path from the src_path.

```rust
extern crate path_calculate;

use std::path::Path;

use path_calculate::*;

// $HOME="/home/chao"
let dst_path = Path::new("/home/chao/works/demo/src");
let src_path = Path::new("~/trash");

assert_eq!("../works/demo/src", dst_path.related_to(&src_path).unwrap().to_str().unwrap());
```

*/
pub extern crate path_absolutize;

pub mod calculate;

pub use calculate::*;

