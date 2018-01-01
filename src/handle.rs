use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::sync::Mutex;

use error::{Result, UsersError};
use group::Group;
use user::User;

//Can't think of anything better to call it...
pub struct GU {
    users: HashMap<String, User>,
    groups: HashMap<String, Group>
}

impl GU {
    pub fn new() -> Result<GU> {
        let mut users = HashMap::new();
        let mut fpasswd = OpenOptions::new()
            .read(true)
            .open("/etc/passwd")?;
        
        let mut spasswd = String::new();
        fpasswd.read_to_string(&mut spasswd)?;
        
        for line in spasswd.lines() {
            let user = User::parse(line)?;
            users.insert(user.user.clone(), user);
        }
        
        let mut groups = HashMap::new();
        let mut fgroup = OpenOptions::new()
            .read(true)
            .open("/etc/group")?;
        
        let mut sgroup = String::new();
        fgroup.read_to_string(&mut sgroup)?;
        
        for line in sgroup.lines() {
            let group = Group::parse(line)?;
            groups.insert(group.group.clone(), group);
        }
        
        Ok(GU { users, groups })
    }
    
    pub fn add_group(&mut self, group: Group) -> Result<()> {
        if !self.groups.contains_key(&group.group) {
            self.groups.insert(group.group.clone(), group);
            Ok(())
        } else {
            Err(From::from(UsersError::AlreadyExists))
        }
    }
    
    pub fn add_user(&mut self, user: User) -> Result<()> {
        if !self.users.contains_key(&user.user) {
            self.users.insert(user.user.clone(), user);
            Ok(())
        } else {
            Err(From::from(UsersError::AlreadyExists))
        }
    }
}

pub struct Handle<'a> {
    uid: u32,
    gid: u32,
    path: String,
    gutex: &'a Mutex<GU>
}

impl<'a> Handle<'a> {
    pub fn new(path: String, uid: u32, gid: u32, gutex: &'a Mutex<GU>) -> Handle {
        
        Handle {
            uid,
            gid,
            path,
            gutex
        }
    }
}
