# The assignment didn't explicitly define a palindrome.
## In my solution, i assumed that the input value would always be of 32 bits.
## That means that e.g. 0b11011 is not a palindrome, because it would only
## be one in a 5bit context.

# store low half of t0 in t1
li t6 0x0000FFFF
and t1 t0 t6

# store high half of t0 in t2
li t6 0xFFFF0000
and t2 t0 t6
srli t2 t2 16

# store reversed t2 in t4
## use t5 as a loop index
li t5 0

rev_loop:
    ### initialize n=16
    li t6 16

    ### check exit condition
    ### loop 16 times for the 16 bits in t2
    beq t5 t6 rev_end
    
    ### bit at current index (bit = (x >> i) - 1)
    srl t3 t2 t5
    andi t3 t3 1
    
    ### compute insertion index (n = 16 - i - 1)
    sub t6 t6 t5
    addi t6 t6 -1
    
    ### shift bit to target position (bit << n)
    sll t3 t3 t6
    
    ### insert shifted bit into target value (bits |= bit)
    or t4 t4 t3
    
    ### increment index and loop (i++)
    addi t5 t5 1
    j rev_loop

# compare t1 and t4
rev_end:
    ## end if equal (t0 is a palindrome), otherwise set t0 = -1
    beq t1 t4 end
    li t0 -1

end:
    nop
