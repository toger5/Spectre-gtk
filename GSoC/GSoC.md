# Matrix GSoC Project:<br>Matrix as a backup/cloud service (Application back-end)
Timo Kandra
@toger5:matrix.org
timo-kandra@hotmail.de



The idea of using Matrix as a file server is very appealing. The Matrix protocol ticks all the boxes:

 - Matrix features a tree like structure with spaces (a folder structure).
 - Rooms serve as folders.
 - Matrix has an amazing per "folder"/room permission system, based on users being room participants.
 - As a plus: Matrix is decentralized.

I regard this idea and its implementation as highly valuable. The latter would mean the incorporation and design of an SDK in order to use Matrix as a file back-end for applications;
I keep dreaming about things like managing all my .dotfiles using Matrix! Matrix would then serve as a Linux desktop configuration cloud.

### How I imagine it to work
The SDK should allow any client to easily use Matrix as a cloud, basically the SDK should providing the same service than Firebase Store. 
A lot of clients could benefit from some kind of cloud service (notes app, text editor configurations, operating system configurations, ToDo lists).

### User Story
For the following User Story, I will continue to use the ToDo app as an example, but it can be any kind of application:

 - The ToDo app gives me the option: "Sync and backup your ToDos using Matrix" (I click on it)
 - I get prompted with "Please type in your Matrix address to be invited" (I type in @toger5:Matrix.org)
 - I get a notification from Element: "New Invitation for 'ToDo app space/room' ". Joining this folder/space shows me the file with all the ToDos in some format. It would even be possible to change the ToDos there and all clients will be updated.

Adding an additional client to an existing backup/sync room.

- In the ToDo app I choose: "connect to an existing ToDo room/folder"
- I get prompted with a bot address `@TodoApp<id or hash>:server.domain` and a message: "Invite this address to an existing backup/sync room/folder/space."
- After inviting the bot, it can read the ToDo list hosted in the matrix room and populate the app.

In this GSoC proposal, I want to implement this for a password manager.





## The Goal
There are 3 specific areas that need to be solved, to fulfill the presented user story:
 - At the end of GSoC there should be a **fully functional SDK** that can be used in any application as a cloud/sync tool. It should be well documented (the documentation would be aided with an example implementation using an open source password manager app).
 - The SDK should be **implemented in Spectre-gtk** - a password manager app. All the necessary information to generate the passwords are synced over Matrix to multiple devices (Further information in the section: The approach). This app should serve as a showcase application on how to implement the SDK.
 - The SDK should conform to the __File Tree Structure__ MSC. I want to **iron out this MSC** with the help of other Matrix developers (particularly with @travis:t2l.io). I would really like to give the Matrix-rust-sdk proper support for it, so that normal clients using the Matrix-rust-sdk have an easier time to support file tree spaces.

The Rust (or Python) SDK (Build on top of the [rust_sdk](https://github.com/Matrix-org/Matrix-rust-sdk)) should be the majority of the work. The password manager itself is already functional enough. (See this repository: https://github.com/toger5/Spectre-gtk ). The SDK should make it really easy to connect to the Matrix network. The following is a draft of how this could look like:
```rust
/// Create a new space used for the file backup.
/// # Arguments
/// * `dir_name` - A name for the space which contains all the configuration
fn create_root(dir_name: String) -> MatrixRoot;
/// Get the username representing this client.
/// The user needs to invite this username so that this client can access the Matrix dir.
fn Matrix_id_for_invite() -> String;
/// Waits for the user to invite the matrix_id provided by `matrix_id_for_invite`. 
/// After that invite the MatrixRoot directory can be accessed.
fn wait_for_invite(invite_callback: Fn(MatrixRoot));

MatrixRoot {
    /// This invites a Matrix user to the room. The user can then join the room
    /// and has access to the files in the directory when logged into Matrix with any client.
    /// # Arguments
    /// * `matrix_id` - the Matrix id of the user in the form @username:server.domain
    add_user(&self, matrix_id: String) -> Result;
}

impl MatrixDir for MatrixRoom
/// Api to work with the Matrix directory
MatrixDir {
    /// adds a file to a Matrix directory
    fn add_file(&self);
    /// adds a subdirectory to a Matrix directory (or Matrix root directory)
    fn add_dir(&self) -> MatrixDir;
    /// reads and downloads the file with the given name
    fn get_file(&self, file_name: String) -> MatrixFile;
    /// updates or creates the file with the given name
    fn set_file(&self, file_name: String, file: File);
}
```
### The approach
The idea is to develop the SDK around a simple example: A password manager based on [Spectre](https://gitlab.com/spectre.app) (former Masterpassord.app).
Spectre takes a user-name (example: `Max`) and a master-password (`verySensiblePassphrase`) and the name of the associated website (`github.com`) and generates a password. In theory this stateless approach allows to always have access to the passwords without an encrypted file that can be lost. In practice it is much more convenient to store all the website names in a file. Saving and loading to such a file is supported by the gtk4 Spectre app. The file looks like this:

```
# Spectre site export
#     Export of site names.
# Format: 1
# Date: 2021-04-07T18:02:42Z
# User Name: Max
# Full Name: Max
# Passwords: PROTECTED
#               Last     Times  Password                      Login	                     Site	Site
#               used      used      type                       name	                     name	password
2021-03-31T13:10:17Z         0    17:3:1                           	                   Matrix	
2021-04-03T12:44:34Z         0    17:3:1                           	               github.com	
2021-04-03T12:51:59Z         0    17:3:1                           	               google.com	
2021-04-07T18:02:42Z         0    17:3:1                           	                   github	
```

Matrix should backup and sync this file. Therefore, the names of the websites are in a Matrix room/directory, but **without the plain text passwords**. And any client connecting to the Matrix folder has the list of website names to easily generate the according password.

The prospect of having a finished product at the end of GSoC is very encouraging. I find it exciting having the SDK implemented inside an app that I, myself, use frequently. The password manager not only helps to communicate my project idea, but would also help me not to get sidetracked on other Matrix file system related tasks. To put it short: What makes this goal so special to me and well suited for GSoC is the prospect of an immediate application.

**Community Bonding**
May 17, 2021 - June 7, 2021

During The community bonding phase I am planning to contribute to [fractal-next](https://gitlab.gnome.org/GNOME/fractal/-/tree/fractal-next) and try to open some MR's. I already made myself familiar with the code-base. It is using the same stack (gtk4 + rust + Matrix-rust-sdk) as the password app for GSoC and I used it as a resource, on how things are done properly. Additionally I love fractal and cannot wait for it to be ported to gtk4 and support e2ee! 
I think this would be a great preparation for this GSoC project.

**Coding**
June 7, 2021 - August 16, 2021

time period | Task
------------|-------
_1._ Week | Familiarizing with the Matrix-rust-sdk by deciding how a good API would look like (similar to the one in the example), especially how and what should be blocking/async, deciding on the deployment if necessary. Also, the terminology should be figured out (cloud, backup, sync, folder, sync-room ...). In the User Story the issue of a missing terminology can be observed. 
_2._ Week | Deciding on the method to authenticate. (see unsolved discussions)
_3.-7._ Week | Working on the API. Creating the rust library.
_8.-9._ Week| Adding the SDK to the Spectre password app and improving the app to look nicer and be feature complete. 
_9._ Week | Publish the rust crate and publish the GTK app for Linux (maybe even for windows). Write a guide on how to use Matrix as a cloud service. 
_10._ Week | Additional time. Might be needed to finish or clean up some tasks. Otherwise can be used for the **Follow up tasks** listed below. 
## Unsolved discussions
_This section refers to the ToDo client from User Story._

The main issue I see is: 'how does the ToDo-client connect to the Matrix ecosystem'.

- **Guest accounts** are not sufficient to create rooms and invite users. Additionally I am not certain if the guest account can reconnect to the same room after a while (would be able to, when not loosing its access token)

 - Would it be a good solution to **register a full Matrix account** for the ToDo app? The app developer can then decide which server to use: @\<hash1\>:ToDo-app.com would then be the account registered and used to invite the actual user to the files room. When I use the ToDo app on another client, a different account @\<hash2\>:ToDo-app.com would be registered and I could invite this account to the ToDo files room. That way, the ToDo list stays synced. (This is okay when using dedicated servers for each app. But when the developers don't want to host a server (ToDo-app.com) and use Matrix.org instead, all the @\<hash\>:Matrix.org users would be annoying.). 

   Comment: _This solution is currently the preferred one.  @travis:t2l.io offered to option to host the bot on t2I.io. This makes me really confident in the success of the User Story of this proposal during GSoC. Even though a long term solution could be nicer._

 - **Sub-client**? I would log in with my actual Matrix account in the ToDo app. The app would just be another client like fractal for example. It would be easy to create the room. When I log into the second ToDo app with my Matrix account it would have access to the files immediately. The issue is that I <u>don't</u> want my ToDo app or text editor <u>to have full control over my Matrix account</u>. A Sub-client would only have access to a limited list of rooms/spaces. This solution would need a bigger MSC (defining sub-clients) which I don't like. Additionally, the permission topic is already fairly well solved by room participation. It does not really make sense to me, that there should be another permission system for sub-clients. The situation is comparable to the initial community implementation which is now superseded by spaces; It was not necessary to create a new system when the core parts are already available with rooms.

   Comment: _This solution is well suited for things like a dropbox like desktop client, or a .dotfile sync tool. For applications like the ToDo app or the password manager logging in via the actual matrix account might be a security concern for some ppl. Additionally this might be worth waiting for the efforts on OAuth._

 - Another consideration for the future: the **client is hosting its own Matrix server** (p2p Matrix) and creates/joins/invites to the files room from a bot hosted on that server. This would solve the potential issue of spamming servers like mozilla.org or Matrix.org with `hash` users... 

   Comment: _For me, this sounds like the perfect solution for a simple backup/cloud configuration systems like the ToDo app or the password manager. First it is a minimal security risk (the app only has access to one room!). It is very nice to implement: The developer does not need to care about any back-end infrastructure. It is all handled by the built-in server of the SDK and the backup is hosted on the users own home-server. It would be easier to implement than any other service, skipping all the annoying steps like requesting tokens from dropbox and registering for a developer account. I am aware, that p2p Matrix is not quiet there yet._

## GSoC
I would like to do this project in the realm of GSoC since there are some crucial decisions on the approach (See previous section). Most importantly, how to make the client communicate with the Matrix network. Those are much easier to solve with a mentor experienced with the matrix ecosystem.

This [repository](https://github.com/toger5/Spectre-gtk) is a gtk4 front-end written in Rust which binds to the Spectre algorithm. It will need some more work to be nice looking and feature complete, but there are also some more days until GSoC starts 😉. So this part is already figured out and GSoC could immediately start with the Matrix side of things.

### Follow up tasks 
The project can be extended in multiple ways after GSoC or in case the goal is met before the end of GSoC.
 - Documentation (having good documentation for the rust library and a well written explanation of how to implement the file backup feature in any app.)
 - With the gained experience I could see myself in a good position to implement support for the file tree MSC in a client (probably element). This is something I would be interested in working on after GSoC as well.
 - Using the SDK to write a .dotfile backup client. Making it easy to sync Linux and other configuration files over multiple computers.
 - Investigating on p2p approach of hosting the bot on the client itself.

## About me
I am a physics student at the Universität Konstanz.
My background in programming started in around 2015. Since then, I became excited about coding and found a lot of satisfaction in it. During my studies, it was hard to fit larger coding projects into my every day life at the university, so coding had to stay smaller hobby. For the upcoming semester I really want to change that! So I decided to make some room for my passion; I did most of my mandatory courses for the masters in the first master semester leaving me a lot of free time during this summer. My weeks in this upcoming semester are freed for coding, except for a single day each week.
I gathered lots of my programming experience from contributing to the Godot engine project. I have done other bigger projects with swift (a summer school (_MakeSchool_), multiple ios apps), C++ (_Godot Engine_), Python (lots of small projects for example with the _BGE_), java (multiple games, a course at university), and dart (An Android app for defining and sharing boulder problems (climbing)).
I have some experience in rust, mostly because of the GTK password app I coded prior to GSoC. I am confident that I am well prepared for the the scope of the GSoC project.

I also would like to mention my excitement for the Matrix ecosystem. The possibilities and importance of an open protocol for communication is fascinating to me - from a technical and social standpoint.
This is a huge source of motivation for me and it would be exhilarating for me to explore these possibilities in areas outside of a chat system and to communicate and showcase what is possible.