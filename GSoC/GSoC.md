_! WIP !_
# Matrix as a backup/cloud service (application back-end)

To get to the point immediately: The idea of using matrix as a file server is very appealing. The matrix protocol checks all the boxes:
 - Tree like structure with spaces (a folder structure)
 - Rooms as folders
 - Amazing per "folder"/room permission system thanks the the nature of users as room participants
 - As a plus: it is distributed

I see it as a highly valuable idea to approach this field and figure out how this could be implemented and design sdk's for the usecase of using matrix as a file backend for applications.
One cool example could be managing all your .dotfiles using matrix! (Matrix would serve as a linux configuration cloud)

### How I imagine it to work
A SDK should allow any client to easily use matrix as a cloud. (Basically the SDK should providing the same service than Firebase for a some situations)
A lot of clients could benefit from some could service (notes app, text editor configurations, operating system configurations, Todo lists).

### User Story

I want to achieve the following user experience (I will continue to use the Todo app example. Can be anything. For the GSoC proposal I was thinking to implement this for a password manager):

 - The todo app gives me the option: "sync and backup your todos using matrix" (I click on it)
 - I get prompted with "please type in your matrix address to be invited" (i type in @toger5:matrix.org)
 - I get an element notification that: "I am invited to ToDo app space" joining this folder/space shows me the file with all the todos in some format (it would even be possible to change the todos there and all clients will be updated)
## The Goal
There are 3 specific areas that need work to allow for the presented user story:
 - At the end of GSoC there should be a fully functional SDK that can be used in any application as a cloud/sync tool. It should be well documented (With the example implementation inside a password manager app.).
 - The SDK should be implemented in Spectre-gtk. A password manager app. All the necessary information to generate the passwords are synced over matrix to multiple devices. This app should serve as a showcase application on how to implement the sdk.
 - The SDK should conform to the __File Tree Structure__ MSC. I want to iron out this MSC with the help of other matrix developers. I would really like to give the matrix-rust-sdk proper support for it, so that normal clients using the matrix-rust-sdk have an easier time to highlight folder spaces.

The Rust (or python) sdk (Build ontop of the [rust_sdk](https://github.com/matrix-org/matrix-rust-sdk)) should be the majority of the work (the password manager is already functional enough to add matrix-backup support. See this repository). The sdk should make it really easy to connect to the matrix network. The following is a draft of how this could look like:
```rust
/// Create a new space used for the file backup
/// # Arguments
/// * `dir_name` - A name for the space which contains all the configuration
fn create_root(dir_name: String) -> MatrixRoot;
/// Get the username representing this client. The user needs to invite this username so that this client can access the matrix dir.
fn matrix_id_for_invite() -> String;
/// Waits for the user to invite the matrix_id provided by `matrix_id_for_invite`. After that invite the MatrixRoot directory can be accessed.
fn wait_for_invite(invite_callback: Fn(MatrixRoot));

MatrixRoot{
    /// This invites a matrix user to the room. The user than can join the room.and has access to the files in the directory when logged into matrix with any client.
    /// # Arguments
    /// * `matrix_id` - the matrix id of the user in the form @username:server.domain
    add_user(&self, matrix_id: String) -> Result;
}

impl MatrixDir for matrixRoom

/// Api to work with the matrix directory
MatrixDir {
    /// adds a file to a matrix directory
    fn add_file(&self);
    /// adds a subdirectory to a matrix directory (or matrix root directory)
    fn add_dir(&self) -> MatrixDir;
    /// reads and downloads the file with the given name
    fn get_file(&self, file_name: String) -> MatrixFile;
    /// updates or creates the file with the given name.
    fn set_file(&self, file_name: String, file: File);
}
```
### The approach
The idea is to design that around a simple example: A password manager based on [Spectre ](https://gitlab.com/spectre.app)(former Maserpassord.app). 
> Spectre takes a username (example: `Max`) and a masterpassword(`verySensiblePassphrase`) and the name of the associated website (`github.com`) and generates a password. This stateless approach allows to always have access to the passwords without an encrypted file that can be lost.

Matrix should backup and sync the file where all the website/service names (like: `github.com`). So the names of the services are in a matrix room/directory, but **without the plain text passwords**.

The password manager app (written in gtk) is already implemented (bare-bones but usable...)

I think it is nice to have a finished product at the end of GSoC, so having the sdk implemented inside an app I use would be nice. The project of building a full file system infrastructure around matrix the way I imagine it is too big for a GSoC project. Even though the password manager is not helpful for matrix directly it helps to communicate the idea and helps me to not get sidetracked on other matrix file system related tasks.

time period | Task
------------|-------
_1._ Week | Familiarizing with the matrix-rust-sdk by deciding how good API would look like (similar to the one in the example). Especially how and what should be blocking/async. Deciding on the deployment needed (which server, spaces would be nice to be implemented!)
_2._ Week | Deciding on the method to authenticate. (see unsolved discussions)
_3.-6._ Week | Working on the API. Creating the rust library and adding it to the Spectre password app.
Rest | Polish documentation. Publish the rust crate. Publish the GTK app for Linux (maybe even for windows). Write a guide on how to use matrix as a cloud service
## Unsolved discussions
_This section refers to the Todo client from User Story._

The main issue I see is: 'how does the Todo-client connect to the matrix ecosystem'.

- **Guest accounts** are not sufficient to create rooms and invite users. Additionally I am not certain if the guest account can reconnect to the same room after a while (whould be able to, when not loosing its access token)
 - Would it be a good solution to **register a full matrix account** for the todo app? The app developer than can decide whihc server to use: @<hash1>:todo-app.com would than be the account registered and used to invite the actual user to the files room. When I use the todo app on another client another account @<hash2>:todo-app.com would be registered and I can invite this account to the todo files room so that the todo list is synced. (This is okay when using dedicated servers for each app. But when the developers dont want to host a server (todo-app.com) and instad use matirx.org all the @<hash>:matrix.org users would be annoying.)
 - **Sub-client**? I would log in with my actual matrix account and the Todo app would just be another client like fractal for example. Than it would be easy to create the room. And when I log in the second Todo app with my matrix account it would have access to the files immediately. The issue is that I don't want my Todo app or text editor to have full control over my matrix account. A Subclient would only have access to a limited list of rooms/spaces.
 - This solution would need a bigger MSC (defining subclients) which I don't like. Additionally the permission topic is already really well solved by room participation. It does not really make sense to me that there should be another permission system for subclients. (Its like the initial community implementation. It was not necessary to create a new system when the core parts are already available with rooms)

Other ideas: 
 - I could imagine that the client can just be a bot? 
 - Another idea is that the client is hosting its own matrix server (p2p matrix) and creates/joins/invites to the files room from that server. This would solve the potential issue of spamming servers like mozilla.org or matrix.org with hash users...

## GSoC
I would like to do this project in the realm of GSoC since there are some crucial decisions on how to approach (See previous section). Most importantly, how to make the client communicate with the matrix network.
Does the client need to create an account on the matrix network.

This repository is a Gtk front-end written in Rust for which binds to the Spectre algorithm. So this part (although it is still super rough) is already figured out and GSoC could immediately start with the matrix side of things.