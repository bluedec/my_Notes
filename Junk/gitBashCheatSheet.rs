Here is a list of Git Commands I believe I might need in the future, an interesting thing is that i am at the same time finding out which words are special in Rust, which is pretty nice as a side effect.
I might just start writing like this to discover a little bit about the language, little by little. 

    mkdir 
        create a directory.
    rmdir
        removes a directory.
    pwd
        print the "present working directory"
    cd
        get into a certain file. just do "cd (file)"  can also be used to get into places further away from current location. cd ()
    cd ..  
        goes back to the last directory!
    touch
        creates a new file. just do "touch (file)" PD   you can create more than one file at the same time. (e.g)"touch scripts.js styles.css"
   mv    move a certain file or something into somthing else, like a folder.
----------do  "mv (file to move) (where to move it)"
   ^C    will cancel the current command, in case you want to delete it
----------instantly. >  ^  Means CTRL.<
   ls    gives a list of the list of the files in the current directory.
   ls     + (file) shows the list of items inside a certain directory or file.
 ls -la  gives the list with details about the files in the c. directory.
  start  opens a folder or file. do "start (file name)"
start .  opens the file where we are standing with the console.

-- you can also run several commands at the same time using &&.--
--(command) + "--help" will give a ton of help about the command.--

   rm    will remove the object file. Can delete multiple files at once.
rm (-r ) applies the command to all elements inside the container. Important
----------to move/copy something with items inside, to somewhere else.
   echo  prints something in the console??
   cat   prints whatever is in the a file?? any file??
   cp    copies whatever into somewhere else.
   >     makes/places something inside something else. 
"  /*  " applies whatever you command you used to everything. for example
----------"$cp newdir/* website" = copiar todo en newdir a website.

Basic repository cheatsheet 
git init                         
git add                           before you commit, you have to add the files.
git commit + file                 commit the added files. 
git commit                        will open text editor to insert comments.
git remote add origin <gitlink>   especifies where the commits will go when
                                   pushed.
git remote --v                    checks the permissions.
git add .                         move everything in the path into the repo. 
git 

Advanced Git Commands 
git status                        checks the states of the commits.
git log                           shows history of commits!
git checkout + hash               goes back in time! can make changes to a past
                                   commit!
git branch -M main                selects the branch where u want to work on
git config --list                 gives a list of the personal user config.
git diff (branches)..(branches)   shows differences between two branches.

