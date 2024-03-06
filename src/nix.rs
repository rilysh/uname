use crate::utsname::Utsname;

impl Utsname {
    // Initialize the main structure
    pub fn new() -> Utsname {
        Self {
            sysname: "".to_string(),
            nodename: "".to_string(),
            release: "".to_string(),
            version: "".to_string(),
            machine: "".to_string(),
            domainname: "".to_string(),
        }
    }

    // Call the uname() function
    pub fn uname(self: &mut Self) -> Result<(), &str> {
        let mut uts = libc::utsname {
            sysname: [0; 65],
            nodename: [0; 65],
            release: [0; 65],
            version: [0; 65],
            machine: [0; 65],
            domainname: [0; 65],
        };

        unsafe {
            if libc::uname(&mut uts) == -1 {
                return Err("libc::uname()");
            }
        }

        uts.sysname.iter().for_each(|each| {
            if *each != 0 {
                self.sysname.push(*each as u8 as char);
            }
        });

        uts.nodename.iter().for_each(|each| {
            if *each != 0 {
                self.nodename.push(*each as u8 as char);
            }
        });

        uts.release.iter().for_each(|each| {
            if *each != 0 {
                self.release.push(*each as u8 as char);
            }
        });

        uts.version.iter().for_each(|each| {
            if *each != 0 {
                self.version.push(*each as u8 as char);
            }
        });

        uts.machine.iter().for_each(|each| {
            if *each != 0 {
                self.machine.push(*each as u8 as char);
            }
        });

        #[cfg(target_os = "linux")]
        // Hidden under _GNU_SOURCE
        uts.domainname.iter().for_each(|each| {
            self.domainname.push(*each as u8 as char);
        });

        Ok(())
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "linux")]
    #[test]
    fn it_works_sysname() {
        let mut uts = Utsname::new();
        if Utsname::uname(&mut uts).is_err() {
            panic!("Utsname::uname() panicked.");
        }

        assert_ne!(uts.sysname, "");
        assert_ne!(uts.nodename, "");
        assert_ne!(uts.release, "");
        assert_ne!(uts.version, "");
        assert_ne!(uts.machine, "");
        assert_ne!(uts.domainname, "");
    }

    #[cfg(any(
        target_os = "freebsd",
        target_os = "dragonflybsd",
        target_os = "openbsd",
        target_os = "netbsd"))]
    #[test]
    fn it_works_sysname() {
        let mut uts = Utsname::new();
        if Utsname::uname(&mut uts).is_err() {
            panic!("Utsname::uname() panicked.");
        }

        assert_ne!(uts.sysname, "");
        assert_ne!(uts.nodename, "");
        assert_ne!(uts.release, "");
        assert_ne!(uts.version, "");
        assert_ne!(uts.machine, "");
    }
}
