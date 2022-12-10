program day10_part1
    
    character(len = 10) :: instruction
    integer :: ierror
    integer :: arg
    integer :: cycle_n = 1
    integer :: register_x = 1
    
    integer :: signal_strength
    integer :: sum_of_signal_strengths = 0
    
    ! Read entire line as a string
    read(*, "(A)", iostat=ierror) instruction

    do while(ierror == 0)
    
        if(mod(cycle_n, 40) == 20) then
            signal_strength = cycle_n * register_x
            print*, "At cycle ", cycle_n, " signal strength is ", signal_strength
            
            sum_of_signal_strengths = sum_of_signal_strengths + signal_strength
        end if
        
        if(instruction == "noop") then
        
            cycle_n = cycle_n + 1
        else
            ! Line starts with 'addx'; parse argument afterwards
            read(instruction(5:), "(I5)") arg
            print*, arg
            
            ! Since we advance cycle_n by two, the above 'if' may miss some cases
            if(mod(cycle_n, 40) == 19) then
                signal_strength = (cycle_n + 1) * register_x
                print*, "At cycle ", cycle_n, " (+1) signal strength is ", signal_strength
                
                sum_of_signal_strengths = sum_of_signal_strengths + signal_strength
            end if
            
            cycle_n = cycle_n + 2
            register_x = register_x + arg
            
        end if
        
        read(*, "(A)", iostat=ierror) instruction
    end do
    
    print*, "Sum of signal strengths is ", sum_of_signal_strengths
    
end program day10_part1
