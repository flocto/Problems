Problem located at https://cses.fi/problemset/task/1691/

Notes:
I'm not sure if there is a way to solve this in stable.
The reason is that there is no fast method of retrieving the first element of a set in Rust Stable. There is .first(), but this is not a stable feature so I don't believe it would work when submitted.
This causes the test case with many edges stemming from 1 to take way longer than it should, as the iter().next().unwrap() creates an iterator out of the massive set every single time.
Naively storing just the iterators does not work either, as there is no method of removing the edges this way (I guess you could technically store the removed edges? But this probably makes runtime even worse.)
The code here should be the best solution at the moment. Until first() gets pushed to stable, I don't think this problem will be solveable in Rust.

The C++ file is a solution written by someone else. I used it to try and figure out why that one test case was timing out.