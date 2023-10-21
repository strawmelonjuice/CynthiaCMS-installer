use flate2::read::GzDecoder;
use fs_extra::dir::copy;
use fs_extra::dir::CopyOptions;
use inline_colorization::*;
use normalize_path::NormalizePath;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::process;
use std::{env, process::Command};
use std::{
    fs::{self},
    path::Path,
};
use tar::Archive;

const PLUGIN_REPO_URL: &str = "https://cdn.jsdelivr.net/gh/strawmelonjuice/CynthiaCMS-installer@main/cynthia-plugin-repository.json";
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Deserialize, Debug, Serialize)]
struct CynthiaPluginRepoItem {
    id: String,
    host: String,
    referrer: String,
}

#[derive(Deserialize, Debug, Serialize)]
struct CynthiaPluginManifestItem {
    id: String,
    version: String,
}

#[cfg(windows)]
pub const NPM: &'static str = "npm.cmd";

#[cfg(not(windows))]
pub const NPM: &'static str = "npm";

fn main() {
    println!("{style_bold}{color_cyan}Cynthia Directory Downloader -- cyninst{color_reset} v{VERSION}\n{style_reset}By {color_red}Straw{color_green}melon{color_yellow}juice {color_magenta}Mar{color_reset}.");
    println!(
        r#"
Installs Cynthia into the current directory.
    Usage: 
            {color_yellow}cyninst <[version]>{color_reset}
    
    or, for plugins:

            {color_yellow}cyninst -p <plugin name> <[plugin version]>{color_reset}
"#
    );
    if (env::args().nth(1).unwrap_or("unset".to_string())) != "-p".to_string() {
        defaultmode(format!(
            "@cynthiacms/cynthiacms@{0}",
            env::args().nth(1).unwrap_or("latest".to_string())
        ));
    }
    pluginmode(
        format!("{0}", env::args().nth(2).unwrap_or("none".to_string())),
        env::args().nth(3).unwrap_or("latest".to_string()),
    );
}

fn defaultmode(cynthiapkg: String) {
    const TOTALSTEPS: i32 = 7;
    println!(
        "\n\n\n\n\r─────────────────────────────────────{0}─────────────────────────────────────",
        "Installing Cynthia"
    );
    println!("\n\r[1/{TOTALSTEPS}] Asking NPM where the specified Cynthia tarball can be found...");
    let output = Command::new(NPM)
        .arg("view")
        .arg(cynthiapkg)
        .arg("dist.tarball")
        .output()
        .expect("Could not find NPM.");

    let tarballurl = format!("{}", String::from_utf8_lossy(&output.stdout));
    if format!("{}", output.status) == "exit code: 0".to_string() {
        print!("{color_green}->{color_blue}{tarballurl}{color_reset}");
    } else {
        if String::from_utf8_lossy(&output.stderr) != "".to_string() {
            println!("\rError: {}", String::from_utf8_lossy(&output.stderr));
        }
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
    archive.unpack(&tempdir).expect("Could not unpack Cynthia.");
    let packagedir = Path::new(&format!("{0}/package", &tempdir.display())).normalize();
    let cynthiareadme =
        Path::new(&format!("{0}/package/README.MD", &tempdir.display())).normalize();
    let cd = env::current_dir().unwrap();
    let mut options = CopyOptions::new();
    options.overwrite = true;
    options.content_only = true;
    println!("\r[4/{TOTALSTEPS}] Pruning unpacked files...");
    fs::remove_file(cynthiareadme).unwrap();
    println!("\r[5/{TOTALSTEPS}] Copying Cynthia files into current directory...");
    copy(packagedir, &cd, &options).expect("Could not create target files.");
    println!("\r[6/{TOTALSTEPS}] Cleaning temp files...");
    fs::remove_dir_all(tempdir).unwrap();
    println!("\r[7/{TOTALSTEPS}] Installing Cynthia dependencies...");
    Command::new(NPM)
        .arg("install")
        .current_dir(&cd)
        .output()
        .expect("Could not find NPM.");
    print!("\r...{color_green}Complete!");
    let pluginmanjson = Path::new(&format!("{0}/cynthiapmanifest.json", &cd.display())).normalize();
    if pluginmanjson.exists() {
        println!(
            " {color_blue}Installing plugins specified in {color_magenta}'{0}'{color_blue} now...{color_reset}",
            pluginmanjson.display()
        );
        let mut o = File::open(format!("{}", &pluginmanjson.display()).as_str())
            .expect("Could not read Cynthia plugin manifest file.");
        let mut contents = String::new();
        o.read_to_string(&mut contents)
            .expect("Could not read Cynthia plugin manifest file.");
        let unparsed: &str = &contents.as_str();
        let cynplmn: Vec<CynthiaPluginManifestItem> = serde_json::from_str(unparsed)
            .expect("Could not read from Cynthia plugin manifest file.");
        // let mut totalplugins: i32 = 0;
        let totalplugins: &usize = &cynplmn.len();
        let mut currentplugin: i32 = 1;
        // for _plugin in cynplmn { totalplugins +=1 }
        for plugin in cynplmn {
            println!(
                "Installing plugin {0}/{1}: {2}",
                currentplugin, totalplugins, plugin.id
            );
            pluginmode(plugin.id, plugin.version);
            currentplugin += 1;
        }
    }
    println!(" {color_cyan}Bye!");

    process::exit(0);
}

fn pluginmode(wantedplugin: String, wantedpluginv: String) {
    println!(
        "\n\n\n\n\r─────────────────────────────────────{0}─────────────────────────────────────",
        "Installing Plugin"
    );
    if wantedplugin == "none".to_string() {
        println!("{color_red}ERROR:{color_reset} No plugin selected.")
    }
    const TOTALSTEPS: i32 = 10;
    println!("\r[1/{TOTALSTEPS}] Creating temporary directories...");
    let mut rng = rand::thread_rng();
    let tempdir = Path::new(&format!(
        "{0}/{1}_cyninstdir/",
        env::temp_dir().display(),
        rng.gen_range(10000000..999999999)
    ))
    .normalize();
    fs::create_dir_all((&tempdir).as_path()).unwrap();

    println!("\r[2/{TOTALSTEPS}] Downloading Cynthia Plugin Index...");
    let resp = reqwest::blocking::get(PLUGIN_REPO_URL).expect("request failed");
    let body = resp.bytes().expect("body invalid");

    let repositoryfile = format!(
        "{}",
        Path::new(&format!(
            "{0}/cynthia-plugin-repository.json",
            tempdir.display()
        ))
        .normalize()
        .display()
    );

    std::fs::write(&repositoryfile, &body).expect("failed to download Cynthia Plugin Index.");

    println!("\r[3/{TOTALSTEPS}] Loading Cynthia Plugin Index...");

    let mut o = File::open(&repositoryfile.as_str()).expect("Could not read Cynthia Plugin Index.");
    let mut contents = String::new();
    o.read_to_string(&mut contents)
        .expect("Could not read Cynthia Plugin Index.");
    let unparsed: &str = &contents.as_str();
    let cynplind: Vec<CynthiaPluginRepoItem> =
        serde_json::from_str(unparsed).expect("Could not read from Cynthia Plugin Index");
    println!("[4/{TOTALSTEPS}] Searching Cynthia plugin index for '{wantedplugin}'...");
    let mut wantedpkg: &CynthiaPluginRepoItem = &CynthiaPluginRepoItem {
        id: "none".to_string(),
        host: "none".to_string(),
        referrer: "none".to_string(),
    };
    for cynplug in &cynplind {
        if cynplug.id == wantedplugin {
            print!("\r          {color_magenta}Found!{color_reset}");
            wantedpkg = cynplug;
            break;
        }
        // println!("{:#?}", cynplug);
    }
    if wantedpkg.id.to_lowercase() == "none".to_string() {
        println!("\r{color_red}Plugin not found.{color_reset}");
        process::exit(1);
    }
    let mut tarballurl = "unknown".to_string();
    if wantedpkg.host.to_lowercase() == "npm".to_string() {
        println!(
            " --> Cynthia Plugin Index: {0} is on NPM as {1}!",
            wantedplugin, wantedpkg.referrer
        );
        println!("[5/{TOTALSTEPS}] Asking NPM about this...");
        let npmpackagename = format!("{1}@{0}", wantedpluginv, wantedpkg.referrer);
        let output = Command::new(NPM)
            .arg("view")
            .arg(npmpackagename)
            .arg("dist.tarball")
            .output()
            .expect("Could not find NPM.");

        tarballurl = format!("{}", String::from_utf8_lossy(&output.stdout));
        if format!("{}", output.status) == "exit code: 0".to_string() {
            print!("{color_green}->{color_blue}{tarballurl}{color_reset}");
        } else {
            if String::from_utf8_lossy(&output.stderr) != "".to_string() {
                println!("\rError: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
    } else if wantedpkg.host.to_lowercase() == "direct-tar" {
        println!("Skipping step 5... Archive is not hosted on NPM.");
        tarballurl = wantedpkg.referrer.to_owned();
    }
    if tarballurl == "none".to_string() {
        print!("Error: Could not fetch tarball url for some reason.");
        process::exit(1);
    }
    let tarballfilepath = format!(
        "{}",
        Path::new(&format!("{0}/{1}.tgz", tempdir.display(), wantedplugin))
            .normalize()
            .display()
    );
    println!(
        "\r[6/{TOTALSTEPS}] Downloading {1} to '{0}'...",
        tarballfilepath, wantedplugin
    );
    let resp = reqwest::blocking::get(tarballurl.as_str()).expect("request failed");
    let body = resp.bytes().expect("body invalid");
    std::fs::write(&tarballfilepath, &body).expect("failed to download plugin package.");
    println!("\r[7/{TOTALSTEPS}] Download completed, unpacking...");

    let tar_gz = File::open((&tarballfilepath).as_str()).expect("Could not unpack plugin.");
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(&tempdir).expect("Could not unpack plugin.");

    print!("\r[8/{TOTALSTEPS}] Success! installing plugin to: ");
    let packagedir = Path::new(&format!("{0}/package", &tempdir.display())).normalize();
    let mut options = CopyOptions::new();
    options.overwrite = true;
    options.content_only = true;
    let pd = Path::new(&format!(
        "{0}/plugins",
        env::current_dir().unwrap().display()
    ))
    .normalize();
    println!("{}", pd.display());
    let pdp = format!("{0}/{1}", pd.display(), wantedplugin);
    fs::create_dir_all(&pdp).expect("Could not create plugin folders.");
    copy(packagedir, &pdp, &options).expect("Could not create target files.");
    println!("\r[9/{TOTALSTEPS}] Cleaning temp files...");
    fs::remove_dir_all(tempdir).unwrap();
    println!("\r[10/{TOTALSTEPS}] Installing dependencies for this plugin...");
    Command::new(NPM)
        .arg("install")
        .current_dir(pdp)
        .output()
        .expect("Could not find NPM.");
    println!("\r...{color_green}Complete!");
}
