use std::io::{self, ErrorKind};
use std::env;
use std::borrow::Cow;
use std::path::{Component, Path, PathBuf};

//extern crate path_absolutize;
use path_absolutize::*;

/// Let `Path` and `PathBuf` have some path calculate methods.
pub trait Calculate {
    /// Get current env's home_dir if it exist.
    fn home_dir(&self) -> io::Result<Cow<Path>>;

    /// Get the absolute path, even if the path is not exist.
    fn as_absolute_path(&self) -> io::Result<Cow<Path>>;

    /// Get a relative root path betwwen two pathes.
    fn relative_root_with(&self, path_b: &Path) -> io::Result<Cow<Path>>;

    /// Get dst_path's relative path from the src_path.
    fn related_to(&self, src_path: &Path) -> io::Result<Cow<Path>>;
}

impl Calculate for Path {
    fn home_dir(&self) -> io::Result<Cow<Path>> {
        #[allow(deprecated)]
        let home_dir = env::home_dir().unwrap();
        if home_dir.to_str().unwrap() == "" {
            // do not set or support env $HOME/~
            return Err(io::Error::from(ErrorKind::InvalidInput))
        }

        Ok(Cow::from(home_dir))
    }
    fn as_absolute_path(&self) -> io::Result<Cow<Path>> {
        let mut iters = self.components();

        let first_component = iters.next();

        // if not start with `~`, return self.absolutize() directly.
        match first_component {
            Some(Component::Normal(dir)) => {
                if dir.to_str().unwrap() == "~" {} else {return self.absolutize()}
            },
            None => {
                return self.absolutize()
            }
            _ => {},
        }

        // here get replace HOME by abs_path
        let mut path_buf = PathBuf::new();

        let home_dir = self.home_dir()?;
        let home_iters = home_dir.components();
        for iter in home_iters {
            path_buf.push(iter)
        }

        for iter in iters {
            path_buf.push(iter)
        }

        Ok(Cow::from(path_buf))
    }

    fn relative_root_with(&self, path_b: &Path) -> io::Result<Cow<Path>> {
        // Absolutize 
        let pa = self.as_absolute_path()?;
        let pb = path_b.as_absolute_path()?;

        // new pathbuf
        let mut path_buf = PathBuf::new();

        let mut itera = pa.components();
        let mut iterb = pb.components();

        let first_componenta = itera.next().unwrap();
        let first_componentb = iterb.next().unwrap();

        // On Windows, do not support diff Prefix Pathes calculate.
        if first_componenta == first_componentb {
            path_buf.push(first_componenta);
        } else {
            return Err(io::Error::from(ErrorKind::InvalidInput))
        }

        loop {
            if let Some(componenta) = itera.next() {
                if let Some(componentb) = iterb.next() {
                    if componenta == componentb {
                        path_buf.push(componenta);
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        Ok(Cow::from(path_buf))
    }

    fn related_to(&self, src_path: &Path) -> io::Result<Cow<Path>> {
        // /home/cc/work/a related_to /home/cc => work/a
        // /home/cc/work/a related_to /home/cc/App/demo => ../../work/a
        // return a absolutily path
        let pa = self.as_absolute_path().unwrap();
        let pb = src_path.as_absolute_path().unwrap();
        let relative_root = self.relative_root_with(src_path).unwrap();

        let mut path_buf = PathBuf::new();

        // pop relative_root
        let mut itera = pa.components();
        let mut iterb = pb.components();

        let mut iterr = relative_root.components();
        loop {
            if let Some(_component) = iterr.next() {
                itera.next();
                iterb.next();
            } else {
                loop {
                    if let Some(_component) = iterb.next() {
                        path_buf.push(Component::ParentDir);
                    } else {
                        break;
                    }
                }

                loop {
                    if let Some(component) = itera.next() {
                        path_buf.push(component)
                    } else {
                        break;
                    }
                }

                break
            }
        }

        Ok(Cow::from(path_buf))
    }
}

impl Calculate for PathBuf {
    fn home_dir(&self) -> io::Result<Cow<Path>> {
        self.as_path().home_dir()
    }

    fn as_absolute_path(&self) -> io::Result<Cow<Path>> {
        self.as_path().as_absolute_path()
    }

    fn relative_root_with(&self, path_b: &Path) -> io::Result<Cow<Path>> {
        self.as_path().relative_root_with(path_b)
    }

    fn related_to(&self, src_path: &Path) -> io::Result<Cow<Path>> {
        self.as_path().related_to(src_path)
    }
}

