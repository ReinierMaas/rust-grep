# rust-grep

Toy grep implementation. 

Line based search with filename, line number and match highlighting. 

A single input file/directory or stdin is used. 

Enumerates all paths from directory, a single path is generated for a file.
Rayon walks those paths possibly in parallel searching for files in UTF8.
Regex matches the given regex with the lines from those files.
It collects all matches and dumps them to stdout.

Project for a Rust meetup with a workshop. 
