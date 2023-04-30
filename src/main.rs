//! eelu-login is a command-line tool that allows stuff and students to log in to the The Egyptian E-Learning University(EELU) Moodle platform quickly and easily through the command-line interface.
//! > This is a fork of [Crypt00o/eelu-login](https://github.com/Crypt00o/eelu-login) with more features and improvements and bugs =D
//!
//! # Installation
//! You can install the latest stable version of eelu-login via Cargo:
//! ```sh
//! cargo install eelu-login
//! ```
//!
//! Or you can get the latest git version from the repository:
//! ```sh
//! cargo install --git https://github.com/anas-elgarhy/eelu-login.git
//! ```
//!
//! Or you can install it from the AUR with your favorite AUR helper:
//! ```sh
//! yay -S eelu-login # or any other AUR helper you use
//! ```
//!
//! # Usage
//! `eelu-login --help` will show you the usage of the tool:
//! ```text
//! [+] Usage : eelu-login [--user <username>] [--pass <password>] [--type <staff| sys-user | student>]
//! Args:
//! [-user | --user | --username | -username |  -u]   <username>  :  username to login with
//! [-pass | --pass | --password | -p]   <password>  :  password to login with
//! [-type | --type | --usertype | -usertype | -t]  : <usertype>
//!
//! Flags:
//! [-o | --open | -open] : open browser after login
//! [-v | --verbose | -verbose] : verbose mode
//! [-V | --version | -version] : print version
//! [-h | --help | -help] : print this help message
//!
//! usertype can be :
//!     [ staff | 3 ] for staff privilege
//!     [ sys-user | 1] for system user privilege
//!     [ student | 2] for student privilege"#
//! ```
//!
//! Replace `<username>` and `<password>` with your EELU Moodle login credentials, and `< staff | sys-user | student>` with your user type.
//!
//! If you don't want to enter your credentials every time you run the tool, you can set the `SIS_EELU_USERNAME` and `SIS_EELU_PASSWORD` environment variables to your username and password respectively.
//!
//! You can don't need to specify the user, and the tool will be try to login as a student and if it fails it will try to login as a staff and if it fails it will try to login as a system user.
//!
//! # Contributing
//! If you want to contribute to this project, please read the [contributing guidelines](./CONTRIBUTING.md) first.
//!
//! # Useful Links
//! - [sis-login crate](https://crates.io/crates/sis-login)
//! - [EELU SIS](https://sis.eelu.edu.eg/)
//! - [EELU Moodle](https://moodle.eelu.edu.eg/)
use sis_login::Sis;

mod cli;

use cli::cli_session;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse Arguments
    let mut args = cli::arg_parser::Arguments::parse_args_and_env();

    // Print the banner if the verbose flag is set
    if args.verbose {
        cli_session::banner();
    }

    // Check the user entered the username and password, if not, prompt the user to enter them
    if args.username.is_none() {
        args.username = cli_session::prompt("Username", true).into();
    }
    if args.password.is_none() {
        args.password = cli_session::prompt("Password", true).into();
    }

    if args.verbose {
        println!("[+] Preparing to login ...");
        println!("    [+] Username: {}", args.username.clone().unwrap());
        println!("    [+] Password: {}", args.password.clone().unwrap());
        println!("[+] Initializing the SIS session ...");
    }

    let headers_builder = sis_login::headers_builder::DefaultHeadersBuilder::new(
        "Mozilla/5.0 (X11; Linux x86_64; rv:78.0) Gecko/20100101 Firefox/78.0".to_string(),
        "https://sis.eelu.edu.eg/static/PortalStudent.html".to_string(),
    );
    let mut sis = Sis::new(
        "https://sis.eelu.edu.eg/studentLogin",
        "https://sis.eelu.edu.eg/getJCI",
        &headers_builder,
    );

    if args.verbose {
        println!("[+] SIS session initialized successfully");
        println!("[+] Login ...");
    }

    cli_session::login(&mut sis, &mut args).await;
    Ok(())
}
