# copy the original values of t0 and t1
mv t3 t0
mv t4 t1

# perform the addition
add t0 t0 t1

# check for overflow:
# if the sum (now t0) is less than either of the summands, an overflow must have occured 
sltu t1 t0 t3
sltu t1 t0 t4
