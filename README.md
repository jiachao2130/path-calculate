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

let p = Path::new("/tmp");

if let Ok(home_dir) = p.home_dir() {
    println!("Home path: {:?}", home_dir);
}

```

### as_absolute_path
This is almost like a shadow of the `absolutize` in `path-absolutize`, I only add `~`($HOME) support in the method(Unix or Windnows).

```rust
extern crate path_calculate;

use std::path::Path;

use path_calculate::*;

// If u have $HOME, test `~` support
let p = Path::new("./whatever");
if let Ok(home_dir) = p.home_dir() {
    let p = Path::new("~");
    assert_eq!(home_dir.to_str().unwrap(), p.as_absolute_path().unwrap().to_str().unwrap());
}

let p2 = Path::new("/tmp/a/b/c/../../e/f");

assert_eq!("/tmp/a/e/f", p2.as_absolute_path().unwrap().to_str().unwrap());
```

### relative_root_with
Sometimes I would use the relative root of pathes, it return an absolutize relative_root path.
Behold, in Windows, it can not calculate the relative_root on different disks(C:\\, D:\\,,,), when you try to use this method it return an ioerror such as `io::ErrorKind::InvalidInput`.

```rust
extern crate path_calculate;

use std::path::Path;

use path_calculate::*;

let p1 = Path::new("/home/gits/mkisos");
let p2 = Path::new("/home/cc/trash");

let relative_root = p1.relative_root_with(&p2);

assert_eq!("/home", relative_root.unwrap().to_str().unwrap())
```

```rust
extern crate path_calculate;

use std::io::ErrorKind;
use std::path::Path;

use path_calculate::*;

// Pass Test when in Unix
if cfg!(target_os = "windows") {
    // Windows ok
    let d1 = Path::new("D:\\Games\\Videos\\Replays");
    let d2 = Path::new("D:\\Games\\Dota2");
    
    assert_eq!("D:\\Games", d1.relative_root_with(&d2).unwrap().to_str().unwrap());
    
    // Windows err
    let c1 = Path::new("~");

    assert_eq!(ErrorKind::InvalidInput, c1.relative_root_with(&d1).unwrap_err().kind());
}

```

### related_to
This method is used to calculate the dst_path's relative path from the src_path.

```rust
extern crate path_calculate;

use std::path::Path;

use path_calculate::*;

// $HOME="/home/chao"
let dst_path = Path::new("/home/chao/works/demo/src");
let src_path = Path::new("/home/chao/trash");

assert_eq!("../works/demo/src", dst_path.related_to(&src_path).unwrap().to_str().unwrap());
```

### add_path
Band path_a with path_b(Path_b should be a relative path, can not contain `~`, too).
It return an abs path, I think u must want this.
```rust
extern crate path_calculate;

use std::path::Path;

use path_calculate::*;

let base_path = Path::new("/home/chao");

let extra_path = Path::new("./works/demo");

assert_eq!("/home/chao/works/demo", base_path.add_path(&extra_path).unwrap().to_str().unwrap());

let empty_path = Path::new("works/../../gits/kernel");

assert_eq!("/home/gits/kernel", base_path.add_path(&empty_path).unwrap().to_str().unwrap());
```
