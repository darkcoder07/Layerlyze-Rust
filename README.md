Run instructions:

main.rs contains the classes that perform generation. "typical stuff" runs simulated annealing, "genetic" runs asexual genetic optimization, "load and score" scores the loaded layout in optimized_layout.json, and "load and print" prints out a representation of the loaded layout that's more human-readable. There are lots of magic constants and a lack of easy config and workflow due to my coding inexperience and the WIP nature of the project. Hopefully that can be fixed later...

To run the generation or scoring process you'd like, comment out the other function calls in main.rs and run the program. Remind me to figure out packages and crates sometime so things can be actually organized, xD.
