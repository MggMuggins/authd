use std::collections::HashMap;
use std::str::from_utf8;
use std::sync::Mutex;

use syscall::data::Stat;
use syscall::error::{EINVAL, ENOTDIR, Error};
use syscall::error::Result;
use syscall::scheme::SchemeMut;

use user::User;
use group::Group;
use handle::{GU, Handle};
use error::Result as UsersResult;

const LVL_ONE_URL: [&str; 6] = ["user", "useradd", "userdel", "group", "groupadd", "groupdel"];
const USER_URLS: [&str; 6] = ["auth", "uid", "gid", "name", "home", "shell"];
const GROUP_URLS: [&str; 2] = ["gid", "users"];

/*
fn authstat(path: String) -> Result<()> {
    let url_parts: Vec<&str> = path.split('/').collect();
    //Check that the first part of the url is valid
    //if !LVL_ONE_URL.iter().any(|part| part == url_parts.get(0).unwrap_or(&"") ) {
    //    return Err
    //}
    if url_parts.len() > 3 {
        return Err(Error::new(ENOTDIR));
    }
    for (i, part) in url_parts.iter().enumerate() {
        match i {
            0 => if !LVL_ONE_URL.iter().any(|p| part == p ) {
                return Err(Error::new(ENOTDIR));
            },
            //Check for users/groups names
            1 => {},
            //Check for Appropriate user or group URLs
            2 => if !USER_URLS.iter().any(|p| part == p ) || !GROUP_URLS.iter().any(|p| part == p ) {
                return Err(Error::new(ENOTDIR));
            },
            _ => unreachable!()
        }
    }
    Ok(())
}
*/

pub struct AuthScheme<'a> {
    //I think this needs to live for 'a as well, not sure how to annotate that
    gutex: Mutex<GU>,
    handles: HashMap<usize, Handle<'a>>
}

impl<'a> AuthScheme<'a> {
    pub fn new() -> UsersResult<AuthScheme> {
        Ok(AuthScheme {
            gutex: Mutex::new(GU::new()?),
            handles: HashMap::new()
        })
    }
}

impl<'a> SchemeMut for AuthScheme<'a> {
    fn open(&mut self, path: &[u8], flags: usize, uid: u32, gid: u32) -> Result<usize> {
        let path = from_utf8(path).or(Err(Error::new(EINVAL)))?;
        let path = String::from(path);
        
        //Select an available id
        let id = (0..self.handles.len() + 1)
            .into_iter()
            .find(|i| !self.handles.contains_key(&i))
            .unwrap_or_else(|| unreachable!() );
        
        println!("Path: {:?}, FLAGS:{}, UID: {}, GID: {}, ConID: {}", path, flags, uid, gid, id);
        self.handles.insert(id, Handle::new(path, uid, gid, &self.gutex));
        Ok(id)
    }
    
    /*
    fn dup(&self, _file: usize, buf: &[u8]) -> Result<usize> {
        if ! buf.is_empty() {
            return Err(Error::new(EINVAL));
        }
        
        Ok(0)
    }*/
    
    fn read(&mut self, id: usize, buf: &mut [u8]) -> Result<usize> {
        println!("ID: {}", id);
        let text = &mut b"Hello everything!".to_owned();
        Ok(0)
    }
    
    fn write(&mut self, id: usize, buf: &[u8]) -> Result<usize> {
        println!("{}", from_utf8(buf).unwrap());
        Ok(buf.len())
    }
    
    /*
    fn fcntl(&self, _id: usize, _cmd: usize, _arg: usize) -> Result<usize> {
        Ok(0)
    }
    
    fn fsync(&self, _file: usize) -> Result<usize> {
        Ok(0)
    }*/
    
    fn fstat(&mut self, id: usize, _stat: &mut Stat) -> Result<usize> {
        /*let _handle = self.handles.get(&id).ok_or(Error::new(EBADF))?;

        *stat = Stat {
            st_mode: MODE_CHR | 0o666,
            ..Default::default()
        };*/
        println!("STAT: {}", id);
        Ok(id)
    }
    
    fn close(&mut self, id: usize) -> Result<usize> {
        println!("CLOSE: {}", id);
        //self.connections.remove(&id);
        Ok(0)
    }
}
