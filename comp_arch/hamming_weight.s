# loop 32 times (for a 32-bit word size)
# use t1 as a count-down variable
li t1 32
loop:
    beqz t1 end
    
    # check LSB:
    # t3 += t2 & 1
    andi t2 t0 1
    add t3 t3 t2
    
    # shift next bit to LSB
    # t0 >>= 1
    srli t0 t0 1
    
    # decrement count-down and loop
    # t1--
    addi t1 t1 -1
    j loop

# move result t3 into t0
# t0 = t3
end:
    mv t0 t3
