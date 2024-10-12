program day10_part2
    
    character(len = 10) :: instruction
    integer :: ierror
    integer :: arg
    integer :: pixel_position = 0
    integer :: register_x = 1
    logical :: addx_first_cycle = .true.
    character(len = 40) :: output = ""
    
    ! Read entire line as a string
    read(*, "(A)", iostat=ierror) instruction

    do while(ierror == 0)
    
        if(pixel_position == 40) then
            print*, output
            output = ""
            pixel_position = 0
        end if

        if(pixel_position >= (register_x - 1) .and. pixel_position <= (register_x + 1)) then
            output = trim(output)//"#"
        else
            output = trim(output)//"."
        end if
        
        if(instruction == "noop") then
            read(*, "(A)", iostat=ierror) instruction
        
        else if(addx_first_cycle) then
            ! Line starts with 'addx'; parse argument afterwards
            read(instruction(5:), "(I5)") arg
            addx_first_cycle = .false.
            
        else
            register_x = register_x + arg
            read(*, "(A)", iostat=ierror) instruction
            addx_first_cycle = .true.
        end if
        
        pixel_position = pixel_position + 1
    end do
    
    print*, output
    
end program day10_part2
