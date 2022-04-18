# rudo
Minimalist CLI task management tool in written in Rust.

![sshot](https://i.imgur.com/yY4bQM9.png)

## Installation
### From source
* Clone the repo
* Run `cargo install --path=.` in the project's root

## Usage
#### Adding tasks
```
rudo -a "My task"
```

#### Listing tasks
```
rudo -l
```

#### Toggling 'done' status on tasks
```
rudo -m 1
```
(Where `1` is the task's index)

#### Removing tasks
```
rudo -d 1
```
(Where `1` is the task's index)
