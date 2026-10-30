# Task Archive

## Summary

Task Archive is a lightweight task management application built with Rust. It provides a command-line interface for managing tasks and tracking progress, alongside a simple read-only web dashboard for visualizing task completion.

## Features
- Create standard and goal-based tasks.
- Track task progress through the CLI.
- Automatically mark tasks as completed when their goals are reached.
- Remove existing tasks.
- Persist task data using JSON.
- View task statistics through a web dashboard.

## Backend Modules Used
- Serde
- Clap

## Installation
Download as a ZIP file 

OR

Clone the repository using the following commands:

```
git clone https://github.com/IceCreamKei/Task-Archive
cd task-archive
```

Then, build the rust file:

```
cargo build
```

## Command List
Use the same CLI to do the following commands
- Adding a standard task: 'cargo run -- add "[item]"' (ex. 'cargo run -- add "exercise"')
- Adding a numbered task: 'cargo run -- add "[item]" --goal [number]' (ex. 'cargo run -- add "drink water" --goal 8')
- Display list of tasks: 'cargo run -- list'
- Update progress (this completes a standard task or increases count for numbered task by 1 until completed): 'cargo run -- progress [id]' (ex. 'cargo run -- progress 2')
- Remove a task: 'cargo run -- remove [id]' (ex. 'cargo run -- remove 1')

## Web Interface 
The project also includes a simple read-only web interface that displays:
- Total number of tasks
- Completed tasks
- Overall completion rate
- Individual task progress bars
To access the interface open 'index.html' through a local web server to view the dashboard. Make sure tasks.json is accessible at the configured relative path (../tasks.json).

It is recommended to view this via Node.js using the following commands in the 'task-archive' folder:
```
npx http-server -o ui/index.html
```
