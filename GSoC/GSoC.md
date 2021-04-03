## Matrix as a file server

To get to the point immediately: The idea of using matrix as a file server is very appealing. The matrix protocol checks all the boxes:
 - Tree like structure with spaces (=> a folder structure)
 - Rooms as folders
 - Amazing per "folder"/room permission system thanks the the nature of users as room participants
 - As a plus: it is distributed

I see it as a highly valuable idee to approach this field and figure out how this could be implemented and design sdk's for that usecase.
One cool example could be managing all your .dotfiles using matrix!
### The approach
The idea is to design that around a simple example: A password manager based on [Spectre ](https://gitlab.com/spectre.app)(former Maserpassord.app). 
> Spectre takes a username (example: `Max`) and a masterpassword(`verySensiblePassphrase`) and the name of the associated website (`github.com`) and generates a password. This stateless approach allows to always have access to the passwords without an encrypted file that can be lost.

It is nice to backup and sync the file where all the sitenames (like: `github.com`) without the passwords are saved. This is where matrix could be used.

**How I imagine it to work:** \
In the password app is the option to backup via matrix. You will be prompted with a field to enter your matrix id. On submit, you will be invited to an encrypted room by a bot/guest_user/(There are multiple ideas which I need to discuss). In this room the data will be stored (in the room state or as events).

## GSoC
I would like to do this project in the realm of GSoC since there are some crucial decisions on how to approach. Most importantly, how to make the client communicate with the matrix network.
Does the client need to create an account on the matrix network. Is there a way to just create a room with a gues account? Is sth like a subclient for the actual user necassary: The password manager needs to log in like a normal client with a matrix account but only has limited access to only the one room/space where the files are stored.

This repository is a gtk frontend written in rust for which binds to the spectre algorithm. So this part (although it is still super rough) is already figured out and gsoc could immediatly start with the matrix side of things.