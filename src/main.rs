use gtk4::prelude::*;
use gtk4::{glib, Application, ApplicationWindow};
use gtk4::{ButtonsType, MessageDialog, MessageType};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::rc::Rc;
use tempfile::NamedTempFile;

const APP_ID: &str = "org.ussur.ActivateLinuxToggle";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let output = Command::new("bash")
        .arg("activate-linux-is-enabled")
        .output()
        .expect("Failed to execute command `activate-linux-is-enabled`");

    let is_enabled;
    println!("Output length is: {}", output.stdout.len());
    if let Ok(string) = String::from_utf8(output.stdout) {
        println!("Output is: {}", string);
        is_enabled = string.starts_with("1");
    } else {
        println!("Output is not utf8");
        is_enabled = false;
    }

    let provider = gtk4::CssProvider::new();
    provider.load_from_data(
        r#"
        .big { font-size: 20px; }
        switch { border-radius: 50px; }
        switch slider { border-radius: 50px; }
    "#,
    );
    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().unwrap(),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let toggle = gtk4::Switch::builder()
        .margin_bottom(10)
        .margin_top(10)
        .margin_start(10)
        .margin_end(10)
        .vexpand(true)
        .build();

    toggle.set_active(is_enabled);

    let boxx = gtk4::Box::new(gtk4::Orientation::Vertical, 10);
    let label = gtk4::Label::new(Some("Show 'Activate Linux' text?"));
    label.add_css_class("big");

    boxx.append(&label);
    boxx.append(&toggle);
    boxx.set_margin_top(10);
    boxx.set_margin_bottom(10);
    boxx.set_margin_start(10);
    boxx.set_margin_end(10);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Toggle 'Activate Linux'")
        .child(&boxx)
        .build();

    window.set_default_size(300, 150);
    window.present();

    let window_rc = Rc::new(window);

    toggle.connect_state_set(move |_button: &gtk4::Switch, state: bool| {
        println!("State set to {}", state);

        if state {
            execute_command("activate-linux-enable".to_string(), &window_rc);
        } else {
            execute_command("activate-linux-disable".to_string(), &window_rc);
        }

        glib::Propagation::Proceed
    });
}

fn save_output_to_file(output: std::process::Output) -> (PathBuf, File) {
    let error_message = format!(
        "{}\n\nOutput:\n```\n{}\n```\nStderr:\n```\n{}\n```\n",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
    save_text_to_file(&error_message)
}

fn save_text_to_file(text: &str) -> (PathBuf, File) {
    let temp_file = NamedTempFile::new().expect("Failed to create temp file");
    let path = temp_file.path().to_path_buf();
    let mut file = temp_file.persist(&path).unwrap();

    println!("Written to the file {:?}", file);

    file.write_all(text.as_bytes())
        .expect("Failed to write stderr to temp file");

    (path, file)
}

fn show_file_error_dialogue(file: PathBuf, msg: &str, window: &gtk4::ApplicationWindow) {
    let dialog = MessageDialog::new(
        // None as Option<&gtk4::Window>,
        Some(window),
        gtk4::DialogFlags::MODAL,
        MessageType::Error,
        ButtonsType::OkCancel,
        msg,
    );

    dialog.add_button("Command output", gtk4::ResponseType::Other(42));

    dialog.connect_response(move |dialog, response| {
        if let gtk4::ResponseType::Other(42) = response {
            // Open the temporary file in the system's default text editor
            if let Err(e) = open::that(file.clone()) {
                eprintln!("Failed to open file: {}", e);
            }
        }
        dialog.close();
    });
    dialog.show();
}

fn execute_command(command: String, window: &gtk4::ApplicationWindow) {
    let mut cmd = Command::new("bash");
    cmd.arg(&command);

    let output = cmd.output();

    match output {
        Ok(result) if result.status.success() => {
            println!("Command succeeded: {}", command);
        }
        Ok(result) => {
            let (path, file) = save_output_to_file(result);
            show_file_error_dialogue(path, "Command failed to execute", window);
            drop(file);
        }
        Err(e) => {
            let (path, file) = save_text_to_file(&e.to_string());
            show_file_error_dialogue(path, "Failed to start command", window);
            drop(file);
        }
    }
}
