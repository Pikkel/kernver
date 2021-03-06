use std::{fs, io::Read, io::Write, process::Command, thread, time};

// TODO: make it download and hopefully install the pkgs

#[allow(unused_must_use)]
fn main() {
    fs::create_dir_all("/tmp/.kernver/");
    loop {
        Command::new("bash")
            .arg("uname")
            .arg("-r")
            .arg(">")
            .arg("/tmp/.kernver/machinekernver")
            .spawn()
            .expect("failed to get machine kernel version");
        let mut machinekernver = std::fs::File::open("/tmp/.kernver/machinekernver").unwrap();
        let mut machinever = String::new();
        machinekernver.read_to_string(&mut machinever).unwrap();
        // curl -s https://kernel.org | grep -A1 'stable:' | grep -oP '(?<=strong>).*(?=</strong.*)' > /tmp/.kernver/kern-latest
        let mut getkernver = std::fs::File::create("/tmp/.kernver/getkernver")
            .expect("failed to create `/tmp/.kernver/getkernver`");
        getkernver.write_all("curl -s https://kernel.org | grep -A1 'stable:' | grep -oP '(?<=strong>).*(?=</strong.*)' > /tmp/.kernver/kern-latest".as_ref()).expect("failed to write to `/tmp/.kernver/getkernver`");
        Command::new("bash")
            // command from here >> https://stackoverflow.com/questions/25078065/how-to-check-latest-kernel-version-by-bash
            .arg("/tmp/.kernver/getkernver")
            .spawn()
            .expect("failed to execute `/tmp/.kernver/getkernver`");
        let mut kernver = std::fs::File::open("/tmp/.kernver/kern-latest").unwrap();
        let mut ver = String::new();
        kernver.read_to_string(&mut ver).unwrap();
        if machinever == ver {
            loop {
                /*
                // curl -s https://kernel.org | grep -A1 'stable:' | grep -oP '(?<=strong>).*(?=</strong.*)' > /tmp/.kernver/kern-latest
                let mut getkernver = std::fs::File::create("/tmp/.kernver/getkernver").expect("failed to create `/tmp/.kernver/getkernver`");
                getkernver.write_all("curl -s https://kernel.org | grep -A1 'stable:' | grep -oP '(?<=strong>).*(?=</strong.*)' > /tmp/.kernver/kern-latest".as_ref()).expect("failed to write to `/tmp/.kernver/getkernver`");
                Command::new("bash")
                    // command from here >> https://stackoverflow.com/questions/25078065/how-to-check-latest-kernel-version-by-bash
                    .arg("/tmp/.kernver/getkernver")
                    .spawn()
                    .expect("failed to execute `/tmp/.kernver/getkernver`");
                let mut kernver = std::fs::File::open("/tmp/.kernver/kern-latest").unwrap();
                let mut ver = String::new();
                kernver.read_to_string(&mut ver).unwrap();
                */
                let mut kernpkgs = std::fs::File::create("/tmp/.kernver/kernpkgs")
                    .expect("failed to create `/tmp/.kernver/kernpkgs`");
                // curl -s https://kernel.ubuntu.com/\~kernel-ppa/mainline/v5.16.8/amd64/ | grep -A1 'href=' | grep -oP '(?<=deb">).*(?=lowlatency).*(?=</a.*)'
                // curl -s https://kernel.ubuntu.com/\~kernel-ppa/mainline/v5.16.8/amd64/ | grep -A1 'href=' | grep -oP '(?<=all.deb">).*(?=</a.*)'
                let mut downloadkern = std::fs::File::create("/tmp/.kernver/downloadkern")
                    .expect("failed to create `/tmp/.kernver/downloadkern`");
                downloadkern.write_all(("curl -s https://kernel.ubuntu.com/\\~kernel-ppa/mainline/v".to_owned() + &ver.trim() + "/amd64/ | grep -A1 'href=' | grep -oP '(?<=deb\">).*(?=lowlatency).*(?=</a.*)' > /tmp/.kernver/kernpkgs\n").as_bytes())
            .expect("failed to write to `/tmp/.kernver/downloadkern`");
                downloadkern.write_all(
           ("curl -s https://kernel.ubuntu.com/\\~kernel-ppa/mainline/v".to_owned()
                + &ver.trim()
               + "/amd64/ | grep -A1 'href=' | grep -oP '(?<=all.deb\">).*(?=</a.*)' >> /tmp/.kernver/kernpkgs")
               .as_bytes(),
        )
        .expect("failed to write to `/tmp/.kernver/downloadkern`");
                //---------------------------------------------------------------
                Command::new("x-terminal-emulator").spawn();
                //---------------------------------------------------------------
                thread::sleep(time::Duration::from_secs(5/*86400*/));
            }
        }
    }
}
