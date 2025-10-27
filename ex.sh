#! /usr/bin/env nix
#! nix shell nixpkgs#neofetch github:tomberek/-#python3With.prettytable --command sh -c ``neofetch && python ex.sh``

import prettytable

# Print a simple table.
#t = prettytable.PrettyTable(["N", "N^2"])
#for n in range(1, 10): t.add_row([n, n * n])
#	print(t)
my_var = "hi"
print(my_var)

