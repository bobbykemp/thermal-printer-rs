# Thermal Printer RS

A rewrite of the popular [node-thermal-printer](https://github.com/Klemen1337/node-thermal-printer) in Rust!

## Motivation

I currently have a goal of unifying all of the code for a project at work that interacts with an Epson thermal printer into a singular code base. Currently, the application itself is a web-based application wrapped in a Tauri application (in order to allow the user to configure some native system settings/position the apps multiple windows, etc. - this was a requirement added late in the project, and so Tauri was not used to build the entire app) and a third windows service that runs on the machine to handle interacting with the thermal printer. Rewriting node-thermal-printer in Rust will allow me to then rewrite that entire NodeJS service in Rust, and move it all into a singular code base.
