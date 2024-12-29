A Linux/GTK4 application meant to assist in playing word-based games like crosswords.

The Linux system dictionary is read, and words are filtered according to one of 3 criteria. 

1. The word-length, either as a single value {5} or a range {3-5} 
2. List of letters the word can be comprised of. "Strict" flag looks for  words that can only be comprised of the letters
3. A pattern of known letters and unknown letters e.g -p--e matches "apace","apple","spice"

e.g -p--e matches "apace","apple","spice"

## Query
Searches the Linux system dictionary for all words that meet the criteria
## Generate
Produces at most one random word that meets the criteria.
## Dev
Requires Rust and libgtk-4-dev. Run install.sh. 


