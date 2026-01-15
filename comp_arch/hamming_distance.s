# we can re-use our hamming-weight code,
# since HD=HW(XOR(a,b))

xor t0 t1 t0
li t1 0

# calculate hamming-weight of t0...

# loop 32 times (for a 32-bit word size)
# use t4 as a count-down variable
li t4 32

loop:
    beqz t4 end
    
    # check LSB:
    # t3 += t2 & 1
    andi t2 t0 1
    add t3 t3 t2
    
    # shift next bit to LSB
    # t0 >>= 1
    srli t0 t0 1
    
    # decrement count-down and loop
    # t4--
    addi t4 t4 -1
    j loop

# move result t3 into t0
# t0 = t3
end:
    mv t0 t3
