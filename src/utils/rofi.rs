pub mod rofi {
    use std::io::{stdin, BufReader, Write};
    use std::process::{Command, Stdio};

    pub fn rofi(commands: &[&str]) -> String {
        let input = commands.join("\n");

        let mut child = Command::new("rofi")
            .arg("-dmenu")
            .arg("-p")
            .arg("Select the image to edit:")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to spawn rofi process");

        if let Some(stdin) = child.stdin.as_mut() {
            stdin.write_all(input.as_bytes()).expect("Failed to write to rofi stdin");
        }

        let output = child.wait_with_output().expect("Failed to read rofi output");

        String::from_utf8_lossy(&output.stdout).trim().to_string()
    }
}