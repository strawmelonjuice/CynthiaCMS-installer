use inline_colorization::*;
use normalize_path::NormalizePath;
use rand::Rng;
use std::fs::File;
use fs_extra::dir::copy;
use fs_extra::dir::CopyOptions;
use flate2::read::GzDecoder;
use tar::Archive;
use std::{env, process::Command};
use std::{
    fs::{self},
    path::Path,
};
const VERSION: &str = env!("CARGO_PKG_VERSION");
const TOTALSTEPS: i32 = 5;



#[cfg(windows)]
pub const NPM: &'static str = "npm.cmd";

#[cfg(not(windows))]
pub const NPM: &'static str = "npm";

fn main() {
    let cynthiapkg = format!(
        "@cynthiacms/cynthiacms@{0}",
        env::args().nth(1).unwrap_or("latest".to_string())
    );
    println!("{style_bold}{color_cyan}Cynthia Directory Downloader -- cyninst{color_reset} v{VERSION}\n{style_reset}By {color_red}Straw{color_green}melon{color_yellow}juice {color_magenta}Mar{color_reset}.\n\nInstalls Cynthia into the current directory.\nUsage: cyninst <[version]>\n\n\r[1/{TOTALSTEPS}] Asking NPM where the specified Cynthia tarball can be found...");
    let output = Command::new(NPM)
        .arg("view")
        .arg(cynthiapkg)
        .arg("dist.tarball")
        .output()
        .expect("Could not find NPM.");

    let tarballurl = format!("{}", String::from_utf8_lossy(&output.stdout));
    if format!("{}", output.status) == "exit code: 0".to_string() {
        print!(
            "{color_green}->{color_blue}{tarballurl}{color_reset}"
        );
    } else {
        println!("\rError: {}", String::from_utf8_lossy(&output.stderr));
    }
    let mut rng = rand::thread_rng();
    let tempdir = Path::new(&format!(
        "{0}/{1}_cyninstdir/",
        env::temp_dir().display(),
        rng.gen_range(10000000..999999999)
    ))
    .normalize();
    fs::create_dir_all((&tempdir).as_path()).unwrap();
    let tarballfilepath = format!(
        "{}",
        Path::new(&format!("{0}/cynthia.tgz", tempdir.display()))
            .normalize()
            .display()
    );
    println!("\r[2/{TOTALSTEPS}] Downloading to '{}'...", tarballfilepath);
    let resp = reqwest::blocking::get(tarballurl.as_str()).expect("request failed");
    let body = resp.bytes().expect("body invalid");
    std::fs::write(&tarballfilepath, &body).expect("failed to download Cynthia.");

    println!("\r[3/{TOTALSTEPS}] Download completed, unpacking...");

    let tar_gz = File::open((&tarballfilepath).as_str()).expect("Could not unpack Cynthia.");
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(&tempdir)
    .expect("Could not unpack Cynthia.");
    let packagedir = Path::new(&format!(
        "{0}/package",
        &tempdir.display()
    ))
    .normalize();
    let cynthiareadme = Path::new(&format!(
        "{0}/package/README.MD",
        &tempdir.display()
    ))
    .normalize();
    let cd = env::current_dir().unwrap();
    let mut options = CopyOptions::new();
    options.overwrite = true;
    options.content_only = true;
    println!("\r[4/{TOTALSTEPS}] Pruning unpacked files...");
    fs::remove_file(cynthiareadme).unwrap();
    println!("\r[5/{TOTALSTEPS}] Copying Cynthia files into current directory...");
    copy(packagedir, cd, &options).expect("Could not create target files.");
    println!("\r...Complete!");
}