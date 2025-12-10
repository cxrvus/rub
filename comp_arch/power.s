li t0 5    # n
li t1 3    # x

# t2 = next acc (m)
# t3 = accumulator (acc)

# set acc = n
mv t3 t0

loop:
    # decrement iterator (x--)
    addi t1 t1 -1
    
    # jump to [done] if x == 0
    beqz t1 done
    
    # set m = acc * n
    mul t2 t3 t0
    
    # overflow if m < acc
    blt t2 t3 overflow
    
    # else set acc = m
    mv t3 t2
    
    # jump to beginning of loop
    j loop

done:
    jal result
    
    # set t1 = 0, because no overflow occured
    li t1 0
    
    j end

overflow:
    # set t1 = -1
    li t1 -1
    
    j end

result:
    # set result n^x = acc
    mv t0 t3
    ret

end:
    nop
