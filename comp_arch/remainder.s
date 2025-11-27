# Load the numbers :
li t1 2
li t2 5

# Calculate remanders:
## t3 = t0 % 2
remu t3 t0 t1
## t4 = t0 % 5
remu t4 t0 t2

# OR-sum remanders,
## which will only be zero if both are 0,
## which only happens if there's no remainder for 2 as well as 5,
## which means that t0 is divisible by 2 and 5:
or t0 t3 t4

## Invert so that 0 (the desired value) turns to all ones,
## or to a different arbitrary value otherwise:
not t0 t0
